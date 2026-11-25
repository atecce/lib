-- Which currency a listing reports in, which its market values are in, and
-- whether it reports quarters: the dashboards ask these for every row they
-- filter (listing_facts' currency, the market panels' currency), and the
-- functions answered by aggregating all of the company's facts, about a
-- second a call. With the call in a row filter, a panel took minutes.
--
-- They change only when a company's facts or market values are written, so
-- they are kept in listing_summary, set by refresh_listing() after each
-- write (store's write_facts and write_market), and the functions read it:
-- an index lookup. Same answers, same signatures; the dashboards don't
-- change. (A table of its own rather than columns of listing, so creating
-- it takes no lock that would stall the dashboards' running queries.)

CREATE TABLE listing_summary (
    ticker             text PRIMARY KEY REFERENCES listing (ticker),
    reporting_currency text,
    market_currency    text,
    reports_quarters   boolean NOT NULL DEFAULT false,
    refreshed_at       timestamptz NOT NULL DEFAULT now()
);

-- The old functions' logic, driven from the company's own facts (the fact
-- primary key starts with issuer_id) rather than from every filing.
CREATE FUNCTION refresh_listing(p_ticker text) RETURNS void
LANGUAGE sql AS $$
    INSERT INTO listing_summary (ticker, reporting_currency, market_currency, reports_quarters)
    SELECT l.ticker,
        (SELECT f.currency FROM fact f JOIN filing g ON g.accession = f.accession
         WHERE f.issuer_id = l.issuer_id AND g.source = listing_source(l.ticker)
         GROUP BY f.currency ORDER BY count(*) DESC, f.currency LIMIT 1),
        (SELECT m.currency FROM market m WHERE m.ticker = l.ticker
         GROUP BY m.currency ORDER BY count(*) DESC, m.currency LIMIT 1),
        EXISTS (SELECT FROM fact f JOIN filing g ON g.accession = f.accession
                WHERE f.issuer_id = l.issuer_id AND f.item = 'Revenue' AND f.period = '3m'
                  AND g.source = listing_source(l.ticker))
    FROM listing l WHERE l.ticker = upper(p_ticker)
    ON CONFLICT (ticker) DO UPDATE SET
        reporting_currency = EXCLUDED.reporting_currency, market_currency = EXCLUDED.market_currency,
        reports_quarters = EXCLUDED.reports_quarters, refreshed_at = now()
$$;

SELECT refresh_listing(ticker) FROM listing;

CREATE OR REPLACE FUNCTION reporting_currency(p_ticker text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT reporting_currency FROM listing_summary WHERE ticker = upper(p_ticker)
$$;

CREATE OR REPLACE FUNCTION market_currency(p_ticker text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT market_currency FROM listing_summary WHERE ticker = upper(p_ticker)
$$;

CREATE OR REPLACE FUNCTION flow_period(p_ticker text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT CASE WHEN coalesce((SELECT reports_quarters FROM listing_summary WHERE ticker = upper(p_ticker)), false)
                THEN '3m' ELSE '12m' END
$$;

-- A period other than 'auto': the requested one where the listing reports
-- revenue for it, else annual. Driven from the listing's issuer.
CREATE OR REPLACE FUNCTION flow_period(p_ticker text, p_requested text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT CASE
        WHEN p_requested = 'auto' OR p_requested IS NULL THEN flow_period(p_ticker)
        WHEN EXISTS (
            SELECT FROM fact f JOIN filing g ON g.accession = f.accession
            WHERE f.issuer_id = (SELECT issuer_id FROM listing WHERE ticker = upper(p_ticker))
              AND f.item = 'Revenue' AND f.period = p_requested
              AND g.source = listing_source(upper(p_ticker))
        ) THEN p_requested
        ELSE '12m'
    END
$$;
