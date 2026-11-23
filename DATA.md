# Data: where it lives, and where it's going

The plan for storing and analysing the financials, as decided on 2026-11-22.
Details elsewhere: the relational model in `store/README.md` (on branch
`store` until merged), statistics and backtests in
`equities/FORECASTING.md`, valuation in `equities/VALUATION.md`.

## Layers

```
caches (edgar/, edinet/, tiingo/)     the source of truth: filings and prices
        │ readers (equities)          as downloaded; everything else rebuilds
        ▼                             from them
Postgres (store)                      the record: relational, with each value's
        │                             filing, every restatement kept; serves
        │                             Grafana, the API, the app
        ├──► DuckDB attaches it       analysis, with no copying, to start
        └──► Parquet, nightly         when scans get heavy: prices and facts as
                                      columns, for DuckDB, Polars, backtest
```

- **Caches** are never edited, only added to. A schema change, or a bug fixed
  in a reader, is a rebuild, not a migration of data.
- **Postgres** (`financials` on localhost:5432) is the system of record:
  issuers and listings, filings with their filing dates, facts in every
  version (`fact_first`: what the market knew then; `fact_latest`: with
  restatements), prices, market values. Row-oriented, indexed: right for
  "everything about AAPL's 2019 10-K", and for joins.
- **Columnar analysis** for "every stock's return on every day": DuckDB
  (below), over Postgres to begin with, over Parquet files once the data is
  large (intraday prices, say).

## InfluxDB: retired

InfluxDB 3 (`financials`: `market_v1` and `reported_item` through
`reported_item_v15`) holds only this project, and fits it badly: a file per
day makes thirty years of sparse quarterly data slow to scan; a value has one
time, so restatements overwrite originals; and a measurement can't change
shape, hence fifteen dead copies. Its strengths (high-rate ingest, expiry,
downsampling) go unused at one write a night, kept forever.

Order, so nothing breaks in between:

1. Grafana's dashboards (`equities.json`, `auto.json`) on Postgres, checked
   against the InfluxDB panels.
2. The app on a read-only API (`api.atec.pub`, behind Cloudflare Access);
   the `influx.atec.pub` route and its Access application removed.
3. The nightly push to Postgres only.
4. InfluxDB stopped (`brew services stop influxdb`); its data directory kept
   a while as an archive. Nothing to migrate out: it all rebuilds from the
   caches.

## Statistics, noise, and why the store should be columnar

Daily returns are mostly noise, and statistics beats noise only as √N. To
tell a strategy with a Sharpe ratio of 0.5 a year from zero at two standard
errors takes t = SR·√T ≥ 2, so T ≥ (2/0.5)² = 16 years, if the market stayed
the same for sixteen years, which it doesn't: longer histories partly
measure a different market.

The way out is breadth rather than length: 500 stocks over 5 years carry far
more independent information than one stock over 50. Hence cross-sectional
methods (Fama–MacBeth regressions, portfolio sorts, panels), and a store
that makes wide panels fast: every stock, every day, a few columns, scanned
and grouped at once. That is a columnar engine, not a row store.

## DuckDB

An embedded, columnar SQL database: "SQLite for analytics". No server; a
library (with a command-line shell), open source (MIT), from CWI Amsterdam,
the institute where Python was created.

- **Columnar and vectorized**: stores and processes data by column, in
  batches, on all cores; scans and aggregates over millions of rows in
  milliseconds.
- **Reads files directly**: `SELECT … FROM 'prices/*.parquet'`, CSV, JSON;
  no loading step.
- **Attaches other databases**: `ATTACH 'dbname=financials' AS pg (TYPE
  postgres)` queries Postgres tables with DuckDB's engine, so analysis can
  start without exporting anything.
- **`ASOF JOIN`**: joins each row to the latest row of another table at or
  before its time: each day's price to the most recent filing known that
  day. The point-in-time join backtests depend on, in one line.
- **Window functions, `QUALIFY`, `PIVOT`**, and friendly SQL for analysis.
- **Bindings** for Rust (`duckdb` crate), Python, R, and Arrow, so results
  move to Polars or numpy without copying.

Its limits: one process writes at a time, and it's built for analysis, not
for serving an app many small queries. Hence Postgres as the record and
DuckDB beside it.

Alternatives considered: **Polars** (Rust DataFrames on Arrow; complements
DuckDB, for computation in the backtest crate), **kdb+/q** (the Wall Street
standard for price data; proprietary, expensive), **ArcticDB** (Man Group's
open-source DataFrame store; Python-centric), ClickHouse and TimescaleDB
(servers, more than this needs).

## Drivers: one interface, several stores

Like Core Data's model in front of interchangeable stores (SQLite, binary,
in-memory), a Rust trait for the questions the code asks, with a driver per
store:

```rust
trait MarketData {
    fn universe(&self, as_of: NaiveDate) -> Result<Vec<Company>>;
    fn prices(&self, tickers: &[Ticker], from: NaiveDate, to: NaiveDate) -> Result<Prices>;
    fn facts_known(&self, issuer: IssuerId, as_of: NaiveDate) -> Result<Vec<Fact>>;  // point in time
    fn dashboard(&self, ticker: &Ticker) -> Result<Vec<TimeSeries>>;
}
// drivers: Postgres (the record), DuckDB/Parquet (analysis), memory (tests),
// InfluxDB (while it lasts)
```

The caveat: keep the trait at the level of these domain questions, not a
general query layer. Each store is good because of what it does its own way
(Postgres's `COPY` and `DISTINCT ON`, DuckDB's `ASOF JOIN` and vectorized
scans); a lowest-common-denominator interface throws that away. Core Data
was criticized for this: SQLite with an object graph on top, its other
stores second-class.

Built when two real users need it (likely `backtest` and the API), so its
shape comes from use.

## Next

1. Postgres: rebase `store` onto `claude`, write one company at a time
   (progress visible, resumable, flat memory), rerun the backfill.
2. DuckDB beside Postgres; the first cross-sectional test there.
3. Grafana and the app on Postgres; InfluxDB retired.
4. The `MarketData` trait, then Parquet when scans get heavy.
5. Survivorship: the tickerless companies (fetching from the next nightly
   EDGAR run), prices for delisted companies, historical index membership.
