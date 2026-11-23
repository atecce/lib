-- The financials, relational: issuers and their listings, filings, the facts
-- each filing reports, and daily prices. See ../README.md for the model and
-- the queries it is for.

-- A company that issues shares and files reports. One issuer can have
-- several listings (Toyota: TM in New York, 7203.T in Tokyo).
CREATE TABLE issuer (
    id          serial PRIMARY KEY,
    name        text NOT NULL,
    cik         bigint UNIQUE,     -- SEC's Central Index Key
    edinet_code text UNIQUE        -- e.g. E02144 (Toyota)
);

-- A ticker: where the shares trade, in what currency.
CREATE TABLE listing (
    ticker    text PRIMARY KEY,    -- AAPL, BRK-B, 7203.T
    issuer_id int NOT NULL REFERENCES issuer,
    exchange  text,
    currency  char(3) NOT NULL DEFAULT 'USD'
);
CREATE INDEX ON listing (issuer_id);

-- How many of the issuer's shares one listed share is (US depositary
-- receipts), from a date: TM was 2 until 2021-10-01, then 10.
CREATE TABLE depositary_ratio (
    ticker          text REFERENCES listing,
    from_date       date,
    shares_per_unit numeric NOT NULL,
    PRIMARY KEY (ticker, from_date)
);

-- A report as filed: EDGAR's 10-K, 10-Q, 20-F, or EDINET's 有価証券報告書
-- (yūka shōken hōkokusho, annual, docTypeCode 120) and its kin.
CREATE TABLE filing (
    accession  text PRIMARY KEY,   -- EDGAR accession number, or EDINET docID
    issuer_id  int NOT NULL REFERENCES issuer,
    source     text NOT NULL CHECK (source IN ('edgar', 'edinet')),
    form       text NOT NULL,      -- 10-K, 10-Q, 20-F; EDINET: 120, 140, 160 ...
    period_end date,
    filed_at   date NOT NULL       -- when the market learned of it
);
CREATE INDEX ON filing (issuer_id, filed_at);

-- The items, mirroring equities::item::Item (the Rust enum is the source of
-- truth; the push keeps this in step).
CREATE TABLE item (
    name    text PRIMARY KEY,      -- TotalAssets, Revenue, ...
    unit    text NOT NULL CHECK (unit IN ('currency', 'shares', 'per_share')),
    part_of text REFERENCES item   -- AutomotiveRevenue is part of Revenue
);

-- A value, as one filing reported it. The same (issuer, item, period,
-- period_end, currency) can come from several filings: the original and
-- later restatements, and from both EDGAR and EDINET for a company on both.
CREATE TABLE fact (
    issuer_id  int  NOT NULL REFERENCES issuer,
    item       text NOT NULL REFERENCES item,
    period     text NOT NULL CHECK (period IN ('pit', '3m', '6m', '9m', '12m')),
    period_end date NOT NULL,
    currency   char(3) NOT NULL,
    value      double precision NOT NULL,
    accession  text NOT NULL REFERENCES filing,
    derived    boolean NOT NULL DEFAULT false,  -- computed, e.g. Q4 = year − nine months
    PRIMARY KEY (issuer_id, item, period, period_end, currency, accession)
);
CREATE INDEX ON fact (accession);

-- Each value as first reported: what the market knew at the time, for
-- valuations that must not see the future.
CREATE VIEW fact_first AS
SELECT DISTINCT ON (f.issuer_id, f.item, f.period, f.period_end, f.currency)
       f.*, g.filed_at, g.source
FROM fact f JOIN filing g USING (accession)
ORDER BY f.issuer_id, f.item, f.period, f.period_end, f.currency, g.filed_at ASC, g.accession;

-- Each value as last reported: restatements included, for charts of what
-- happened.
CREATE VIEW fact_latest AS
SELECT DISTINCT ON (f.issuer_id, f.item, f.period, f.period_end, f.currency)
       f.*, g.filed_at, g.source
FROM fact f JOIN filing g USING (accession)
ORDER BY f.issuer_id, f.item, f.period, f.period_end, f.currency, g.filed_at DESC, g.accession DESC;

-- Daily prices of a listing, unadjusted, with what it takes to adjust them.
CREATE TABLE price (
    ticker       text REFERENCES listing,
    date         date,
    close        double precision NOT NULL,
    adj_close    double precision,
    split_factor double precision NOT NULL DEFAULT 1,
    dividend     double precision NOT NULL DEFAULT 0,
    PRIMARY KEY (ticker, date)
);

-- Daily valuations of a listing, computed in Rust (equities::valuation) from
-- price and fact_first; a materialized view of that join later, perhaps.
CREATE TABLE market (
    ticker           text REFERENCES listing,
    date             date,
    currency         char(3),
    close            double precision NOT NULL,
    market_cap       double precision NOT NULL,
    price_to_book    double precision,
    return_on_equity double precision,
    earnings_yield   double precision,
    PRIMARY KEY (ticker, date, currency)
);
