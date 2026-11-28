# store

The financials in Postgres: issuers and their listings, filings, the facts
each filing reports, and daily prices. A destination, as `edgar`, `edinet`
and `tiingo` are sources: `equities` reads the sources, and the `lib`
executable writes what it read here (and, for now, to InfluxDB too).

## Why not InfluxDB

InfluxDB is built for metrics: dense points, recent windows, old data
thrown away. Financial statements are the opposite.

1. **Sparse, with a long history.** A few values per company a quarter, back
   to the 1990s. InfluxDB 3 keeps a file per day, so a company's history is
   spread over thousands of files: listing every ticker over all time took
   17 s, and the server needs `--query-file-limit` raised.
2. **Two dates per value.** A quarter's revenue has the period it covers and
   the date it was reported, and it can be restated. InfluxDB has one time
   per point, and a point written again with the same tags and time replaces
   the first, so the original and the restatement can't both be kept.
3. **Questions across companies.** "Every company with price-to-book under 1
   and return on equity over 10%" is a join with indexes in SQL, and a scan
   of everything in InfluxDB, which is organized by time first.
4. **Things that aren't time series.** Issuers, listings, filings and items
   are tables, related by keys.

Daily prices are what InfluxDB is good at, but at this size one store with
joins between prices and statements (market cap against book value) beats
two.

## The model

The distinction that shapes it: a **company that issues shares** (issuer) is
not a **ticker** (listing). Toyota is one issuer with two listings, `TM` in
New York, which files with the SEC, and `7203.T` in Tokyo, which files with
EDINET. Statements belong to the issuer; prices to the listing.

```
                         ┌──────────────────────────┐
                         │ issuer                   │
                         ├──────────────────────────┤
                         │ PK id           serial   │
                         │    name         text     │
                         │ UQ cik          bigint   │  SEC's Central Index Key
                         │ UQ edinet_code  text     │  e.g. E02144
                         └──────────────────────────┘
                            ▲ 1        ▲ 1        ▲ 1
             ┌──────────────┘          │          └─────────────────┐
             │ n                       │ n                          │ n
┌────────────────────────┐ ┌──────────────────────────┐ ┌──────────────────────────────┐
│ listing                │ │ filing                   │ │ fact                         │
├────────────────────────┤ ├──────────────────────────┤ ├──────────────────────────────┤
│ PK ticker     text     │ │ PK accession   text      │◀┤ FK accession   text          │
│ FK issuer_id  int      │ │ FK issuer_id   int       │1│ FK issuer_id   int           │
│    exchange   text     │ │    source      text      │ n│ FK item       text ─────────┐│
│    currency   char(3)  │ │      'edgar' | 'edinet'  │ │    period      text          ││
└────────────────────────┘ │    form        text      │ │      pit|3m|6m|9m|12m        ││
   ▲ 1        ▲ 1          │    period_end  date      │ │    period_end  date          ││
   │ n        │ n          │    filed_at    date      │ │    currency    char(3)       ││
┌──────────────────┐ ┌───────────────────────┐       │ │    value       float8        ││
│ price            │ │ depositary_ratio      │       │ │    derived     bool          ││
├──────────────────┤ ├───────────────────────┤       │ ├──────────────────────────────┤│
│ PK FK ticker     │ │ PK FK ticker          │       │ │ PK (issuer_id, item, period, ││
│ PK    date       │ │ PK    from_date       │       │ │     period_end, currency,    ││
│    close         │ │       shares_per_unit │       │ │     accession)               ││
│    adj_close     │ │   TM: 2 → 10 (2021)   │       │ └──────────────────────────────┘│
│    split_factor  │ └───────────────────────┘       └──────────────────────────┘     │
│    dividend      │                                  ┌──────────────────────────┐    │
└──────────────────┘                                  │ item                     │◀───┘
┌──────────────────┐                                  ├──────────────────────────┤
│ market           │  daily valuations, computed in   │ PK name      text        │
├──────────────────┤  Rust from price and fact_first  │    unit      text        │
│ PK FK ticker     │                                  │ FK part_of   text → item │
│ PK    date       │                                  └──────────────────────────┘
│ PK    currency   │
│    market_cap, price_to_book, return_on_equity, earnings_yield
└──────────────────┘
```

PK primary key, FK foreign key, UQ unique. The schema is
[`migrations/001_schema.sql`](migrations/001_schema.sql).

- **`filing.accession`** is EDGAR's accession number
  (`0000320193-25-000079`) or EDINET's document ID (`S100W67T`), unique
  either way.
- **`filing.filed_at`** is a fact's second date: `period_end` is the period
  it covers, `filed_at` (through its filing) when the market learned it.
- **`fact.derived`** marks values computed rather than read, such as a
  fourth quarter (the year less nine months); its accession is the annual
  report it was derived from.
- **`item`** mirrors the Rust enum `equities::item::Item`, which stays the
  source of truth; the push keeps the table in step, for units and the
  `part_of` hierarchy (automotive revenue is part of revenue).
- **`market`** is derived, kept as a table so a dashboard doesn't redo the
  price-against-shares join for 30 years of days on every view.

## The joins it's for

**First reported against latest.** The views pick one value per (issuer,
item, period, period_end, currency) by filing date:

```sql
CREATE VIEW fact_first AS
SELECT DISTINCT ON (f.issuer_id, f.item, f.period, f.period_end, f.currency)
       f.*, g.filed_at, g.source
FROM fact f JOIN filing g USING (accession)
ORDER BY f.issuer_id, f.item, f.period, f.period_end, f.currency, g.filed_at ASC;
-- fact_latest: the same, filed_at DESC
```

Valuations use `fact_first`, so they see only what investors knew at the
time; charts use either.

**A company's dashboard, in one query.** Listing to issuer to facts:

```sql
SELECT f.item, f.period, f.currency, f.period_end, f.value
FROM listing l JOIN fact_latest f USING (issuer_id)
WHERE l.ticker = 'AAPL' AND f.item IN ('TotalAssets', 'Revenue', ...)
  AND f.period IN ('pit', '3m', '12m');
```

**Market cap as of each day.** Each price against the share count most
recently filed before that day, per listed share:

```sql
SELECT p.date, p.close * s.value / coalesce(d.shares_per_unit, 1) AS market_cap
FROM price p
JOIN listing l USING (ticker)
CROSS JOIN LATERAL (
    SELECT value FROM fact_first
    WHERE issuer_id = l.issuer_id AND item = 'SharesOutstanding' AND filed_at <= p.date
    ORDER BY filed_at DESC LIMIT 1) s
LEFT JOIN LATERAL (
    SELECT shares_per_unit FROM depositary_ratio
    WHERE ticker = p.ticker AND from_date <= p.date
    ORDER BY from_date DESC LIMIT 1) d ON true
WHERE p.ticker = 'TM';
```

What `equities::valuation` does in Rust, as a query that can check it, and
stricter: it takes the share count as of the day it was *filed*, where
`valuation.rs` takes it as of the date on the cover page (weeks earlier).
They agree away from those weeks. It also shows bad first reports plainly:
Toyota's 2019 20-F cover gives 283,243,167 shares, a tenth of the real
count, and the query uses it until the next 20-F is filed. Not adjusted for
splits between filings either (`price.split_factor` has them), so near a
split it is off by the split's factor.

**EDGAR against EDINET.** Toyota's figures from both (144 of 166 matched
when compared by hand):

```sql
SELECT a.item, a.period, a.period_end, a.value AS edgar, b.value AS edinet
FROM fact_first a JOIN fact_first b
  USING (issuer_id, item, period, period_end, currency)
WHERE a.source = 'edgar' AND b.source = 'edinet' AND a.value <> b.value;
```

**Across companies.** Issuers whose return on equity is above their earnings
yield, the latest day:

```sql
SELECT DISTINCT ON (ticker) ticker, date, return_on_equity, earnings_yield
FROM market WHERE return_on_equity > earnings_yield
ORDER BY ticker, date DESC;
```

## Using it

```sh
# Postgres from Homebrew (postgresql@17), database financials:
createdb financials
# Write companies to it (migrations run on connect; STORE_URL overrides
# host=localhost dbname=financials):
store-equities --postgres-only AAPL TM 7203.T   # read, derive, value, write
store-equities --dry-run AAPL                   # the series, printed, nothing written
# Each issuer's SIC industry code, from EDGAR's list of filers:
universe | store-sic
# Residual income persistence against the industry (equities/VALUATION.md §8):
cargo run --release -p backtest --example persistence -- AAPL
# FRED's series, from the fred cache (fred-fetch first; daily:
# fred/nightly/run.sh):
store-fred                  # every cached series
store-fred GS10 DGS3MO      # just these
```

FRED's series are two tables beside the rest, joined to prices by date:

```sql
-- Goldman Sachs's price against the 2-year Treasury yield, 2022
SELECT p.date, p.adj_close, o.value AS two_year
FROM price p JOIN fred_observation o ON o.series_id = 'DGS2' AND o.date = p.date
WHERE p.ticker = 'GS' AND p.date >= '2022-01-01';
```

(Prices are only the listings `lib` writes, the companies; funds such as
MINT and SPY are in the tiingo cache but not yet here.)

Every write is an upsert: running it again changes nothing that didn't
change. `SAME_ISSUER` (in `src/lib.rs`) links a Tokyo company's EDINET code
to the US listing it files on EDGAR under, so Toyota is one issuer with two
listings, `TM` and `7203.T`.

## The plan

Steps 1 to 3 are done; 4 to 6 aren't.

1. **Postgres on the Mac** (Homebrew, `postgresql@17`), a `financials`
   database, this schema.
2. **Facts know their filing.** `equities::item::Reported` gains the
   accession and filing date it came from (EDGAR: the filing being read;
   EDINET: the document; derived quarters: the annual report).
3. **The `store` crate writes** issuers, listings, filings, items, facts,
   prices and market values, idempotently (upserts keyed as above), and the
   `lib` executable writes to it alongside InfluxDB.
4. **A small read-only HTTP service** (`store` executable, Rust) for the app:
   `/companies`, `/dashboard/{ticker}`. Postgres speaks its own protocol, not
   HTTP, and shouldn't be on the internet; the Cloudflare route
   `influx.atec.pub` gives way to an `api.atec.pub` in front of this, behind
   the same Access service token. The app's Rust client changes; its Swift
   doesn't.
5. **Grafana's Postgres data source**, the dashboards' panels in SQL, one
   query a panel.
6. **InfluxDB retired** once the app and Grafana read Postgres and the
   nightly push has run against it for a while.
