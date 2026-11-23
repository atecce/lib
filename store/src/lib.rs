//! The financials in Postgres (see README.md): issuers and their listings,
//! filings, every value each filing reported, daily prices and market
//! values. Written by the `lib` executable with `--postgres`.
//!
//! Every write is an upsert keyed as the schema's primary keys, so writing a
//! company again (the nightly push, a backfill) changes nothing that didn't
//! change. Bulk rows go through `COPY` into a temporary table, then one
//! `INSERT … ON CONFLICT` from it.

use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::io::Write;

use chrono::NaiveDate;
use equities::Ticker;
use equities::item::{Item, Reported, Unit};
use equities::valuation::Market;
use postgres::{Client, NoTls, Transaction};

/// The schema, in order. Each runs once, recorded in `schema_migrations`.
const MIGRATIONS: &[(&str, &str)] = &[
    ("001_schema", include_str!("../migrations/001_schema.sql")),
    ("002_listing_written_at", include_str!("../migrations/002_listing_written_at.sql")),
    ("003_listing_facts", include_str!("../migrations/003_listing_facts.sql")),
    ("004_reporting_currency", include_str!("../migrations/004_reporting_currency.sql")),
    ("005_ticker_case", include_str!("../migrations/005_ticker_case.sql")),
    ("006_flow_period", include_str!("../migrations/006_flow_period.sql")),
    ("007_flow_period_fallback", include_str!("../migrations/007_flow_period_fallback.sql")),
    ("008_mops", include_str!("../migrations/008_mops.sql")),
];

/// `host=localhost dbname=financials` unless `STORE_URL` says otherwise.
pub fn default_url() -> String {
    std::env::var("STORE_URL").unwrap_or_else(|_| "host=localhost dbname=financials".to_string())
}

/// A listing and what's known of its issuer.
#[derive(Clone, Debug, Default)]
pub struct Company {
    pub ticker: String,
    pub name: Option<String>,
    /// SEC's Central Index Key.
    pub cik: Option<i64>,
    /// EDINET's code, e.g. E02144 (Toyota).
    pub edinet_code: Option<String>,
    /// MOPS's code, e.g. 2330 (TSMC).
    pub mops_code: Option<String>,
    pub exchange: Option<String>,
    /// The currency the shares trade in.
    pub currency: String,
}

/// The same issuer under both regulators: a Tokyo company with US-listed
/// depositary shares. Its EDINET code and the US ticker whose CIK it files
/// under on EDGAR.
pub const SAME_ISSUER: &[(&str, &str)] = &[
    ("E02144", "TM"),    // Toyota (トヨタ自動車, Toyota Jidōsha)
];

/// MOPS codes whose companies are on EDGAR too, through US depositary shares.
pub const SAME_ISSUER_MOPS: &[(&str, &str)] = &[
    ("2330", "TSM"),     // TSMC (台積電, Táijīdiàn)
    ("2303", "UMC"),     // United Microelectronics (聯電, Liándiàn)
    ("3711", "ASX"),     // ASE Technology (日月光投控, Rìyuèguāng Tóukòng)
    ("2412", "CHT"),     // Chunghwa Telecom (中華電信, Zhōnghuá Diànxìn)
];

pub struct Store {
    db: Client,
}

impl Store {
    pub fn connect(url: &str) -> Result<Store, Box<dyn Error>> {
        Ok(Store { db: Client::connect(url, NoTls)? })
    }

    /// Apply the migrations not yet applied; returns their names.
    pub fn migrate(&mut self) -> Result<Vec<&'static str>, Box<dyn Error>> {
        self.db.batch_execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                 version text PRIMARY KEY, applied_at timestamptz NOT NULL DEFAULT now())")?;
        let mut applied = Vec::new();
        for (version, sql) in MIGRATIONS {
            let done = self.db.query_opt("SELECT 1 FROM schema_migrations WHERE version = $1", &[version])?.is_some();
            if done { continue; }
            let mut tx = self.db.transaction()?;
            tx.batch_execute(sql)?;
            tx.execute("INSERT INTO schema_migrations (version) VALUES ($1)", &[version])?;
            tx.commit()?;
            applied.push(*version);
        }
        Ok(applied)
    }

    /// Bring the `item` table in step with `equities::item::Item`.
    pub fn sync_items(&mut self) -> Result<(), Box<dyn Error>> {
        let mut tx = self.db.transaction()?;
        // Names first, then the part_of references between them.
        for item in Item::ALL {
            let unit = match item.unit() {
                Unit::Money => "currency",
                Unit::Shares => "shares",
                Unit::PerShare => "per_share",
            };
            tx.execute("INSERT INTO item (name, unit) VALUES ($1, $2)
                        ON CONFLICT (name) DO UPDATE SET unit = EXCLUDED.unit",
                &[&item.to_string(), &unit])?;
        }
        for item in Item::ALL {
            let part_of = item.part_of().map(|p| p.to_string());
            tx.execute("UPDATE item SET part_of = $2 WHERE name = $1 AND part_of IS DISTINCT FROM $2",
                &[&item.to_string(), &part_of])?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Upsert the issuer and listing; returns the issuer's id. An issuer is
    /// found by its CIK or EDINET code, so a company on both regulators
    /// (`SAME_ISSUER`) is one issuer with two listings.
    pub fn write_company(&mut self, c: &Company) -> Result<i32, Box<dyn Error>> {
        let mut tx = self.db.transaction()?;
        let found = tx.query_opt(
            "SELECT id FROM issuer WHERE ($1::bigint IS NOT NULL AND cik = $1) OR ($2::text IS NOT NULL AND edinet_code = $2)
                OR ($3::text IS NOT NULL AND mops_code = $3)
             ORDER BY id LIMIT 1", &[&c.cik, &c.edinet_code, &c.mops_code])?;
        let name = c.name.clone().unwrap_or_else(|| c.ticker.clone());
        let id: i32 = match found {
            Some(row) => {
                let id: i32 = row.get(0);
                tx.execute("UPDATE issuer SET cik = coalesce(cik, $2), edinet_code = coalesce(edinet_code, $3),
                            mops_code = coalesce(mops_code, $6), name = CASE WHEN $4 THEN $5 ELSE name END WHERE id = $1",
                    &[&id, &c.cik, &c.edinet_code, &c.name.is_some(), &name, &c.mops_code])?;
                id
            },
            None => tx.query_one("INSERT INTO issuer (name, cik, edinet_code, mops_code) VALUES ($1, $2, $3, $4) RETURNING id",
                &[&name, &c.cik, &c.edinet_code, &c.mops_code])?.get(0),
        };
        tx.execute("INSERT INTO listing (ticker, issuer_id, exchange, currency) VALUES ($1, $2, $3, $4)
                    ON CONFLICT (ticker) DO UPDATE SET issuer_id = EXCLUDED.issuer_id,
                        exchange = coalesce(EXCLUDED.exchange, listing.exchange), currency = EXCLUDED.currency",
            &[&c.ticker, &id, &c.exchange, &c.currency])?;
        if let Ok(t) = c.ticker.parse::<Ticker>() {
            for (from, n) in equities::valuation::depositary_ratios(t) {
                let from: NaiveDate = from.parse()?;
                let n = format!("{}", n);
                tx.execute("INSERT INTO depositary_ratio (ticker, from_date, shares_per_unit) VALUES ($1, $2, $3::text::numeric)
                            ON CONFLICT (ticker, from_date) DO UPDATE SET shares_per_unit = EXCLUDED.shares_per_unit",
                    &[&c.ticker, &from, &n])?;
            }
        }
        tx.commit()?;
        Ok(id)
    }

    /// The filings and every value they reported, for one issuer. Values
    /// without a filing (none, from the EDGAR and EDINET readers) are
    /// skipped. Returns (filings, facts) written.
    /// Record that `ticker`'s company has been written in full.
    pub fn mark_written(&mut self, ticker: &str) -> Result<(), Box<dyn Error>> {
        self.db.execute("UPDATE listing SET written_at = now() WHERE ticker = $1", &[&ticker])?;
        Ok(())
    }

        pub fn write_facts(&mut self, issuer_id: i32, items: &[Reported]) -> Result<(usize, usize), Box<dyn Error>> {
        // One row per key: a filing can state a value twice (two columns
        // merged to one date); the last read wins, as in the push to InfluxDB.
        let mut filings = BTreeMap::new();
        let mut facts: HashMap<(Item, String, NaiveDate, String, String), (f64, bool)> = HashMap::new();
        for it in items {
            let Some(f) = &it.filing else { continue };
            if !it.val.is_finite() { continue; }
            filings.insert(f.accession.clone(), f);
            facts.insert((it.item, it.p.to_string(), it.date, it.currency.to_string(), f.accession.clone()),
                (it.val, it.derived));
        }
        let mut tx = self.db.transaction()?;

        let mut buf = Vec::new();
        for (accession, f) in &filings {
            writeln!(buf, "{}\t{}\t{}\t{}\t{}\t{}", accession, issuer_id, f.source.as_str(), f.form,
                f.period_end.map_or("\\N".to_string(), |d| d.to_string()), f.filed)?;
        }
        copy(&mut tx, "CREATE TEMP TABLE tmp_filing (LIKE filing) ON COMMIT DROP",
            "COPY tmp_filing (accession, issuer_id, source, form, period_end, filed_at) FROM STDIN", &buf)?;
        tx.execute("INSERT INTO filing SELECT * FROM tmp_filing
                    ON CONFLICT (accession) DO UPDATE SET issuer_id = EXCLUDED.issuer_id, source = EXCLUDED.source,
                        form = EXCLUDED.form, period_end = EXCLUDED.period_end, filed_at = EXCLUDED.filed_at", &[])?;

        let mut buf = Vec::new();
        for ((item, period, end, currency, accession), (val, derived)) in &facts {
            writeln!(buf, "{}\t{}\t{}\t{}\t{}\t{:e}\t{}\t{}", issuer_id, item, period, end, currency, val, accession,
                if *derived { "t" } else { "f" })?;
        }
        copy(&mut tx, "CREATE TEMP TABLE tmp_fact (LIKE fact) ON COMMIT DROP",
            "COPY tmp_fact (issuer_id, item, period, period_end, currency, value, accession, derived) FROM STDIN", &buf)?;
        // Replace, not merge: a filing's facts are what the reader reads from it
        // now, so a value a reader fix no longer produces (a zero read from an
        // empty cell) goes. Only the filings read this run; one that failed to
        // read keeps what it had. One transaction, so never half replaced.
        tx.execute("DELETE FROM fact WHERE issuer_id = $1 AND accession IN (SELECT accession FROM tmp_filing)",
            &[&issuer_id])?;
        tx.execute("INSERT INTO fact SELECT * FROM tmp_fact
                    ON CONFLICT (issuer_id, item, period, period_end, currency, accession)
                    DO UPDATE SET value = EXCLUDED.value, derived = EXCLUDED.derived", &[])?;
        tx.commit()?;
        Ok((filings.len(), facts.len()))
    }

    /// A listing's daily prices, as the tiingo cache has them.
    pub fn write_prices(&mut self, ticker: &str, prices: &[tiingo::Price]) -> Result<usize, Box<dyn Error>> {
        let mut days = BTreeMap::new();
        for p in prices {
            if let Some(d) = p.day() { days.insert(d, p); }
        }
        let mut buf = Vec::new();
        for (d, p) in &days {
            writeln!(buf, "{}\t{}\t{:e}\t{:e}\t{:e}\t{:e}", ticker, d, p.close, p.adj_close, p.split_factor, p.dividend)?;
        }
        let mut tx = self.db.transaction()?;
        copy(&mut tx, "CREATE TEMP TABLE tmp_price (LIKE price) ON COMMIT DROP",
            "COPY tmp_price (ticker, date, close, adj_close, split_factor, dividend) FROM STDIN", &buf)?;
        tx.execute("INSERT INTO price SELECT * FROM tmp_price ON CONFLICT (ticker, date) DO UPDATE SET
                    close = EXCLUDED.close, adj_close = EXCLUDED.adj_close,
                    split_factor = EXCLUDED.split_factor, dividend = EXCLUDED.dividend", &[])?;
        tx.commit()?;
        Ok(days.len())
    }

    /// Daily market values, as `equities::valuation::market` computed them.
    pub fn write_market(&mut self, days: &[Market]) -> Result<usize, Box<dyn Error>> {
        let opt = |v: Option<f64>| v.filter(|x| x.is_finite()).map_or("\\N".to_string(), |x| format!("{:e}", x));
        let mut rows = BTreeMap::new();
        for m in days {
            if !m.market_cap.is_finite() { continue; }
            rows.insert((m.ticker.to_string(), m.date, m.currency.to_string()), m);
        }
        let mut buf = Vec::new();
        for ((t, d, c), m) in &rows {
            writeln!(buf, "{}\t{}\t{}\t{:e}\t{:e}\t{}\t{}\t{}", t, d, c, m.close, m.market_cap,
                opt(m.price_to_book), opt(m.return_on_equity), opt(m.earnings_yield))?;
        }
        let mut tx = self.db.transaction()?;
        copy(&mut tx, "CREATE TEMP TABLE tmp_market (LIKE market) ON COMMIT DROP",
            "COPY tmp_market (ticker, date, currency, close, market_cap, price_to_book, return_on_equity, earnings_yield) FROM STDIN", &buf)?;
        tx.execute("INSERT INTO market SELECT * FROM tmp_market ON CONFLICT (ticker, date, currency) DO UPDATE SET
                    close = EXCLUDED.close, market_cap = EXCLUDED.market_cap, price_to_book = EXCLUDED.price_to_book,
                    return_on_equity = EXCLUDED.return_on_equity, earnings_yield = EXCLUDED.earnings_yield", &[])?;
        tx.commit()?;
        Ok(rows.len())
    }
}

/// Create a temporary table and COPY tab-separated rows into it.
fn copy(tx: &mut Transaction, create: &str, copy: &str, rows: &[u8]) -> Result<(), Box<dyn Error>> {
    tx.batch_execute(create)?;
    let mut w = tx.copy_in(copy)?;
    w.write_all(rows)?;
    w.finish()?;
    Ok(())
}
