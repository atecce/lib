//! The equities batch job: each company's filings (EDGAR, EDINET, MOPS
//! caches) and prices (tiingo) read, passed through `equities` (items,
//! derived quarters, valuation), and written to Postgres, one company at a
//! time. Run nightly by equities/nightly/run.sh and by equities/sp500/run.sh.
//!
//!     store-equities --postgres-only AAPL TM 7203.T
//!     store-equities --dry-run AAPL
//!
//! It was the root package's src/main.rs; it lives with the other jobs that
//! read a source and write Postgres (store-fred, store-plaid, ...), so the
//! root crate is a library and equities needs none of the IO.

use std::error::Error;

use equities::Reader;

/// Read when no tickers are given on the command line.
const DEFAULT_TICKERS: [equities::Ticker; 35] = [
    equities::Ticker::NVDA,
    equities::Ticker::TSLA,
    equities::Ticker::AAPL,
    equities::Ticker::BRK,
    equities::Ticker::TSM,
    equities::Ticker::KO,
    equities::Ticker::MCD,
    equities::Ticker::new("PEP"),
    equities::Ticker::new("OSCR"),
    equities::Ticker::new("NET"),
    // Block, formerly Square (SQ until January 2025).
    equities::Ticker::new("XYZ"),
    equities::Ticker::new("PLTR"),
    equities::Ticker::new("AMZN"),
    equities::Ticker::new("MSFT"),
    // Alphabet; also listed as GOOG (same company). Google Inc.'s filings
    // before the 2015 reorganization are under a different CIK.
    equities::Ticker::new("GOOGL"),
    equities::Ticker::new("IBM"),
    equities::Ticker::new("ORCL"),
    // Hardware behind the AI build-out: compute, memory and storage,
    // networking, servers, power and cooling, chip equipment.
    equities::Ticker::new("AMD"),
    equities::Ticker::new("AVGO"),
    equities::Ticker::new("MU"),
    equities::Ticker::new("SNDK"),
    equities::Ticker::new("WDC"),
    equities::Ticker::new("STX"),
    equities::Ticker::new("ANET"),
    equities::Ticker::new("MRVL"),
    equities::Ticker::new("CSCO"),
    equities::Ticker::new("COHR"),
    equities::Ticker::new("CIEN"),
    equities::Ticker::new("SMCI"),
    equities::Ticker::new("DELL"),
    equities::Ticker::new("HPE"),
    equities::Ticker::new("VRT"),
    equities::Ticker::new("AMAT"),
    equities::Ticker::new("LRCX"),
    equities::Ticker::new("KLAC"),
];

/// Usage: lib [--dry-run [--cash-flow] | --dump [--latest]] [TICKER ...]
/// --dump prints every value read; with --latest, only the values pushed.
/// Filings are read from the EDGAR cache (edgar/cache), which `fetch`
/// (the edgar crate's executable) brings up to date; nothing here touches
/// the network but the push to InfluxDB.

fn main() -> Result<(), Box<dyn Error>> {
    let flag = |f: &str| std::env::args().any(|a| a == f);
    let dry_run = flag("--dry-run");
    let dump = flag("--dump");
    // --postgres writes to Postgres too (store crate, STORE_URL); with
    // --postgres-only, to Postgres alone. Every version of every value, with
    // its filing, rather than the latest only.
    let postgres_only = flag("--postgres-only");
    let postgres = postgres_only || flag("--postgres");
    let influx = !postgres_only;

    let mut tickers = Vec::new();
    for arg in std::env::args().skip(1).filter(|a| !a.starts_with("--")) {
        match arg.parse::<equities::Ticker>() {
            Ok(t) => tickers.push(t),
            Err(_) => return Err(format!("not a ticker symbol: {}", arg).into()),
        }
    }
    if tickers.is_empty() {
        tickers = DEFAULT_TICKERS.to_vec();
    }

    let cache = edgar::Cache::new(edgar::Cache::default_dir());
    let japan = edinet::Cache::new(edinet::Cache::default_dir());
    let taiwan = mops::Cache::new(mops::Cache::default_dir());
    let prices = tiingo::Cache::new(tiingo::Cache::default_dir());

    // One company at a time: read, derive, write, then the next, so what's
    // written shows as it goes, an interruption loses one company, and memory
    // stays flat however long the list. (Every step below works per ticker.)
    // --dump and --dry-run print everything together, so they collect it.
    let collect = dump || dry_run;
    let mut db = if postgres && !collect { Some(open_store()?) } else { None };
    let (mut all_items, mut all_market) = (Vec::new(), Vec::new());
    let mut totals = Totals::default();
    for equity in tickers {
        let mut items = Vec::new();
        // Tokyo tickers ("7203.T") come from EDINET, the rest from EDGAR.
        let read = if is_mops(equity) {
            read_mops(&taiwan, equity, &mut items)
        } else if is_edinet(equity) {
            read_edinet(&japan, equity, &mut items)
        } else {
            read_edgar(&cache, equity, &mut items)
        };
        if let Err(e) = read {
            eprintln!("{}: {}", equity, e);
        }
        if items.is_empty() {
            continue;
        }
        equities::merge_nearby_dates(&mut items);
        equities::derive_quarters(&mut items);
        let mut book = equities::book_value_per_share(&items);
        items.append(&mut book);

        // Daily market values, if the tiingo cache has the ticker's prices.
        // What the company's module adds for its valuation (TSMC's quarters
        // in dollars), ahead of what it reported.
        // A sibling listing's reports too, when the module names one (TSM:
        // TSMC's Taipei listing, for its quarters).
        let mut sibling = Vec::new();
        if let Some(other) = equities::company::valued_with(equity) {
            let read = if is_mops(other) { read_mops(&taiwan, other, &mut sibling) }
                       else if is_edinet(other) { read_edinet(&japan, other, &mut sibling) }
                       else { read_edgar(&cache, other, &mut sibling) };
            if let Err(e) = read { eprintln!("{}: {}", other, e) }
            equities::merge_nearby_dates(&mut sibling);
            equities::derive_quarters(&mut sibling);
            sibling.extend(items.iter().cloned());
        }
        // The FRED series its valuation needs, from the fred cache.
        let mut series = equities::company::Series::new();
        for id in equities::company::valuation_series(equity) {
            match fred::Cache::new(fred::Cache::default_dir()).series(id) {
                Ok(s) => { series.insert(id, s.observations.iter().filter_map(|o| Some((o.date, o.value?))).collect()); }
                Err(e) => eprintln!("{}: no {} ({}): valued without it", equity, id, e),
            }
        }
        let valued: Vec<_> = equities::company::valuation_items(equity, if sibling.is_empty() { &items } else { &sibling }, &series)
            .into_iter().chain(items.iter().cloned()).collect();
        let company_prices = prices.prices(equity.as_str()).ok();
        let market = company_prices.as_ref().map(|p| equities::valuation::market(equity, p, &valued)).unwrap_or_default();
        if let Some(last) = market.last() {
            eprintln!("{}: market cap {:.1}B on {}{}{}{}", equity, last.market_cap / 1e9, last.date,
                last.price_to_book.map(|pb| format!(", price-to-book {:.2}", pb)).unwrap_or_default(),
                last.return_on_equity.map(|r| format!(", ROE {:.1}%", r * 100.0)).unwrap_or_default(),
                last.earnings_yield.map(|e| format!(", earnings yield {:.1}%", e * 100.0)).unwrap_or_default());
        }

        if collect {
            all_items.append(&mut items);
            all_market.extend(market);
            continue;
        }
        if let Some(db) = db.as_mut() {
            match write_company(db, &cache, &japan, equity, &items, company_prices.as_deref(), &market) {
                Ok(t) => totals.add(t),
                Err(e) => eprintln!("postgres: {}: {}", equity, e),
            }
        }
        if influx {
            let latest_items = latest(items);
            push_to_influx(&latest_items)?;
            totals.influx_items += latest_items.len();
            if !market.is_empty() {
                push_market(&market)?;
                totals.influx_market += market.len();
            }
        }
    }

    // Every item read, one per line, sorted: for diffing the output of two versions.
    if dump {
        let items = if flag("--latest") { latest(all_items) } else { all_items };
        let mut lines: Vec<String> = items.iter()
            .map(|it| format!("{}\t{}\t{}\t{}\t{}\t{}", it.ticker, it.date, it.p, it.item, it.currency, it.val))
            .collect();
        lines.sort();
        lines.dedup();
        println!("{}", lines.join("\n"));
        return Ok(());
    }
    if dry_run {
        print_series(&all_items, flag("--cash-flow"));
        return Ok(());
    }

    if postgres {
        println!("Successfully wrote {} companies, {} filings, {} facts, {} price days, {} market days to Postgres",
            totals.companies, totals.filings, totals.facts, totals.prices, totals.market);
    }
    if influx {
        println!("Successfully pushed {} items to InfluxDB", totals.influx_items);
        println!("Successfully pushed {} market days to InfluxDB", totals.influx_market);
    }
    Ok(())
}

/// What a run wrote, for its summary.
#[derive(Default)]
struct Totals {
    companies: usize,
    filings: usize,
    facts: usize,
    prices: usize,
    market: usize,
    influx_items: usize,
    influx_market: usize,
}

impl Totals {
    fn add(&mut self, t: Totals) {
        self.companies += t.companies;
        self.filings += t.filings;
        self.facts += t.facts;
        self.prices += t.prices;
        self.market += t.market;
    }
}

/// Write each company read to Postgres: its issuer and listing, its filings
/// and every value they reported, its prices and market values.
/// The store, with its schema up to date and the items table in step with
/// the Rust enum.
fn open_store() -> Result<store::Store, Box<dyn Error>> {
    let mut db = store::Store::connect(&store::default_url())?;
    for m in db.migrate()? {
        eprintln!("postgres: applied migration {}", m);
    }
    db.sync_items()?;
    Ok(db)
}

/// One company to Postgres: issuer and listing, its filings and every
/// version of every value, its prices and market values, each in its own
/// transaction.
fn write_company(db: &mut store::Store, edgar: &edgar::Cache, edinet: &edinet::Cache, ticker: equities::Ticker,
                 items: &[equities::item::Reported], prices: Option<&[tiingo::Price]>,
                 market: &[equities::valuation::Market]) -> Result<Totals, Box<dyn Error>> {
    let company = company(edgar, edinet, ticker)?;
    let issuer = db.write_company(&company)?;
    let (filings, facts) = db.write_facts(issuer, items)?;
    let prices = match prices { Some(p) => db.write_prices(ticker.as_str(), p)?, None => 0 };
    let market = db.write_market(market)?;
    db.mark_written(ticker.as_str())?;
    Ok(Totals { companies: 1, filings, facts, prices, market, ..Default::default() })
}

/// What the caches know of a listing's issuer.
fn company(edgar: &edgar::Cache, edinet: &edinet::Cache, t: equities::Ticker) -> Result<store::Company, Box<dyn Error>> {
    if is_mops(t) {
        let code = mops::code(t.as_str());
        // The same issuer on EDGAR, through its US depositary shares.
        let cik = store::SAME_ISSUER_MOPS.iter().find(|(m, _)| *m == code)
            .and_then(|(_, us)| edgar.cik(us).ok()).map(|c| c as i64);
        return Ok(store::Company { ticker: t.as_str().to_string(), name: None, cik, edinet_code: None,
            mops_code: Some(code), exchange: Some("TWSE".to_string()), currency: "TWD".to_string() });
    }
    if is_edinet(t) {
        let last = edinet.reports(t.as_str())?.pop().ok_or("no EDINET reports")?;
        let code = last.edinet_code.clone();
        // The same issuer on EDGAR, through its US depositary shares.
        let cik = code.as_deref()
            .and_then(|c| store::SAME_ISSUER.iter().find(|(e, _)| *e == c))
            .and_then(|(_, us)| edgar.cik(us).ok())
            .map(|c| c as i64);
        Ok(store::Company { ticker: t.as_str().to_string(), name: last.filer_name.clone(), cik, edinet_code: code,
            mops_code: None, exchange: Some("TSE".to_string()), currency: "JPY".to_string() })
    } else {
        let cik = edgar.cik(t.sec_symbol())?;
        let info = edgar.company(cik).ok();
        let exchange = info.as_ref().and_then(|i| {
            let at = i.tickers.iter().position(|x| x.eq_ignore_ascii_case(t.sec_symbol()))?;
            i.exchanges.get(at).cloned().flatten()
        });
        let edinet_code = store::SAME_ISSUER.iter().find(|(_, us)| *us == t.as_str()).map(|(e, _)| e.to_string());
        let mops_code = store::SAME_ISSUER_MOPS.iter().find(|(_, us)| *us == t.as_str()).map(|(m, _)| m.to_string());
        Ok(store::Company { ticker: t.as_str().to_string(), name: info.map(|i| i.name), cik: Some(cik as i64),
            edinet_code, mops_code, exchange, currency: "USD".to_string() })
    }
}

/// One value per (ticker, item, period, currency, date): the last one read.
/// Filings are read oldest first, so a restatement replaces the original.
/// Resolved here rather than left to InfluxDB, which does not reliably keep
/// the last of two points with the same series and timestamp in one write.
fn latest(items: Vec<equities::item::Reported>) -> Vec<equities::item::Reported> {
    let mut index = std::collections::HashMap::new();
    let mut out: Vec<equities::item::Reported> = Vec::with_capacity(items.len());
    for it in items {
        match index.entry((it.ticker, it.item, it.p, it.currency, it.date)) {
            std::collections::hash_map::Entry::Occupied(e) => out[*e.get()] = it,
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(out.len());
                out.push(it);
            }
        }
    }
    out
}

/// Every 10-Q and 10-K the company filed with EDGAR, oldest first, so a later
/// filing's restatement of an earlier period overwrites the original.
fn read_edgar(cache: &edgar::Cache, equity: equities::Ticker, reported_items: &mut Vec<equities::item::Reported>) -> Result<(), Box<dyn Error>> {
    let cik = cache.cik(equity.sec_symbol())?;
    eprintln!("{}: CIK {}", equity, cik);
    let filings = cache.filings(cik, &edgar::FORMS)?;
    let (mut skipped, mut missing) = (0, 0);
    for f in &filings {
        let name = format!("{} {} {} ({})", equity, f.form, f.report_date, f.accession);
        // Everything read from this filing carries it.
        let start = reported_items.len();
        let filed = equities::item::Filed {
            accession: f.accession.clone(),
            filed: f.filing_date,
            source: equities::item::Source::Edgar,
            form: f.form.clone(),
            period_end: Some(f.report_date),
        };
        match cache.filing(f) {
            Ok(dir) => match equities::filing::new_reader(&dir, equity) {
                Ok(mut r) => {
                    read_statements(&mut r, &name, reported_items);
                    match r.process_cash_flow_statement() {
                        Ok(mut ret) => reported_items.append(&mut ret),
                        Err(e) => eprintln!("failed to process cash flow statement for {}; {}", name, e),
                    }
                    // Older filings' cover pages may not be fetched yet.
                    if let Ok(mut ret) = r.process_cover() {
                        reported_items.append(&mut ret);
                    }
                    for w in r.warnings() { eprintln!("{}: {}", name, w) }
                },
                Err(e) => eprintln!("failed to read {}: {}", name, e),
            },
            Err(e) => match e.downcast_ref::<edgar::EdgarError>() {
                Some(edgar::EdgarError::NoFilingSummary) => skipped += 1,
                Some(edgar::EdgarError::NotFetched(_)) => missing += 1,
                _ => eprintln!("failed to read {}: {}", name, e),
            },
        }
        for it in &mut reported_items[start..] {
            it.filing = Some(filed.clone());
        }
    }
    eprintln!("{}: read {} EDGAR filings, {} predate rendered statements{}", equity, filings.len() - skipped - missing, skipped,
        if missing > 0 { format!(", {} not fetched yet", missing) } else { String::new() });
    Ok(())
}

/// A Tokyo Stock Exchange ticker, read from EDINET.
fn is_edinet(t: equities::Ticker) -> bool {
    t.as_str().ends_with(".T")
}

fn is_mops(t: equities::Ticker) -> bool {
    t.as_str().ends_with(".TW")
}

fn read_mops(cache: &mops::Cache, equity: equities::Ticker, reported_items: &mut Vec<equities::item::Reported>) -> Result<(), Box<dyn Error>> {
    let (mut items, read, failed) = equities::mops::read_company(cache, equity)?;
    eprintln!("{}: read {} MOPS reports{}", equity, read,
        if failed > 0 { format!(", {} failed", failed) } else { String::new() });
    reported_items.append(&mut items);
    Ok(())
}

fn read_edinet(cache: &edinet::Cache, equity: equities::Ticker, reported_items: &mut Vec<equities::item::Reported>) -> Result<(), Box<dyn Error>> {
    let (mut items, read, missing) = equities::edinet::read_company(cache, equity)?;
    eprintln!("{}: read {} EDINET reports{}", equity, read,
        if missing > 0 { format!(", {} not fetched yet", missing) } else { String::new() });
    reported_items.append(&mut items);
    Ok(())
}

fn read_statements(r: &mut impl Reader, name: &str, reported_items: &mut Vec<equities::item::Reported>) {
    match r.process_balance_sheet() {
        Ok(mut ret) => reported_items.append(&mut ret),
        Err(e) => eprintln!("failed to process balance sheet for {}; {}", name, e),
    }
    match r.process_income_statement() {
        Ok(mut ret) => reported_items.append(&mut ret),
        Err(e) => eprintln!("failed to process income statement for {}; {}", name, e),
    }
}

/// Print the series charted in Grafana, one row per (ticker, date), to check
/// the data before pushing it. Later reports restate earlier dates, so where a
/// (ticker, item, period, date) was reported more than once with different
/// values, the row is marked with '*'.
fn print_series(items: &[equities::item::Reported], cash_flow: bool) {
    use equities::item::Item;
    use equities::Period;
    use std::collections::BTreeMap;

    let cols: [(Item, Period, &str); 5] = if cash_flow { [
        (Item::NetCashProvidedByOperatingActivities, Period::ThreeMonths, "operating"),
        (Item::NetCashUsedInInvestingActivities, Period::ThreeMonths, "investing"),
        (Item::NetCashUsedInFinancingActivities, Period::ThreeMonths, "financing"),
        (Item::PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets, Period::ThreeMonths, "capex"),
        (Item::ChangeInCashAndCashEquivalents, Period::ThreeMonths, "chg cash"),
    ] } else { [
        (Item::Revenue, Period::ThreeMonths, "revenue"),
        (Item::NetIncome, Period::ThreeMonths, "net income"),
        (Item::CashAndCashEquivalents, Period::PointInTime, "cash"),
        (Item::MarketableSecurities, Period::PointInTime, "mkt sec"),
        (Item::Inventories, Period::PointInTime, "inventory"),
    ] };

    let mut table: BTreeMap<(String, chrono::NaiveDate), [Vec<f64>; 5]> = BTreeMap::new();
    for it in items {
        if let Some(i) = cols.iter().position(|(item, p, _)| *item == it.item && *p == it.p) {
            // Values in other currencies get their own rows, e.g. "TSM:TWD".
            let name = match it.currency {
                equities::Currency::USD => it.ticker.to_string(),
                c => format!("{}:{}", it.ticker, c),
            };
            let row = table.entry((name, it.date)).or_default();
            if !row[i].contains(&it.val) { row[i].push(it.val); }
        }
    }

    print!("{:<9}{:<12}", "ticker", "date");
    for (_, _, name) in &cols { print!("{:>12}", name); }
    println!();
    for ((ticker, date), row) in &table {
        print!("{:<9}{:<12}", ticker, date.to_string());
        for vals in row {
            let cell = match vals.as_slice() {
                [] => "-".to_string(),
                [v] => format!("{:.2}B", v / 1e9),
                vs => vs.iter().map(|v| format!("{:.2}", v / 1e9)).collect::<Vec<_>>().join("/"),
            };
            print!("{:>12}", cell);
        }
        println!();
    }
}

/// Daily market values, in their own measurement: one point per ticker and
/// day, with the close, market cap and (where known) price-to-book.
fn push_market(days: &[equities::valuation::Market]) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let auth_token = std::env::var("INFLUXDB3_AUTH_TOKEN")
        .map_err(|_| "INFLUXDB3_AUTH_TOKEN environment variable not set")?;
    for chunk in days.chunks(5000) {
        let mut payload = String::new();
        for m in chunk {
            let ts = m.date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_nanos_opt().unwrap();
            payload.push_str(&format!("market_v1,ticker={},currency={} close={},market_cap={}{} {}\n",
                m.ticker.to_string().to_lowercase(), m.currency, m.close, m.market_cap,
                [("price_to_book", m.price_to_book), ("return_on_equity", m.return_on_equity), ("earnings_yield", m.earnings_yield)]
                    .iter().filter_map(|(k, v)| v.map(|v| format!(",{}={}", k, v))).collect::<String>(), ts));
        }
        let res = client.post("http://localhost:8181/write?db=financials")
            .bearer_auth(&auth_token)
            .body(payload)
            .send()?;
        if !res.status().is_success() {
            return Err(format!("InfluxDB write failed: {}", res.text()?).into());
        }
    }
    Ok(())
}

fn push_to_influx(items: &[equities::item::Reported]) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let auth_token = std::env::var("INFLUXDB3_AUTH_TOKEN")
        .map_err(|_| "INFLUXDB3_AUTH_TOKEN environment variable not set")?;

    for chunk in items.chunks(1000) {
        let mut payload = String::new();
        for item in chunk {
            let ts = item.date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_nanos_opt().unwrap();
            payload.push_str(&format!(
                "reported_item_v15,ticker={},item={},period={},currency={}{} value={} {}\n",
                // lowercase: the Grafana dashboards query ticker = 'tsla' / 'nvda'
                item.ticker.to_string().to_lowercase(),
                item.item,
                item.p,
                item.currency,
                // EDGAR values carry no source tag (they came first).
                if is_edinet(item.ticker) { ",source=edinet" } else { "" },
                item.val,
                ts
            ));

        }

        let res = client.post("http://localhost:8181/write?db=financials")
            .bearer_auth(&auth_token)
            .body(payload)
            .send()?;

        if !res.status().is_success() {
            return Err(format!("InfluxDB write failed: {}", res.text()?).into());
        }
    }

    Ok(())
}
