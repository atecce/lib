-- Taiwan's MOPS as a source: its company code on the issuer (2330 for TSMC,
-- whose US listing TSM is the same issuer), and 'mops' among the filing
-- sources; the dashboard functions read a listing's facts from its own
-- regulator: EDINET for Tokyo (.T), MOPS for Taipei (.TW), EDGAR otherwise.
ALTER TABLE issuer ADD COLUMN mops_code text UNIQUE;
ALTER TABLE filing DROP CONSTRAINT filing_source_check;
ALTER TABLE filing ADD CONSTRAINT filing_source_check CHECK (source IN ('edgar', 'edinet', 'mops'));

CREATE FUNCTION listing_source(p_ticker text) RETURNS text
LANGUAGE sql IMMUTABLE AS $$
    SELECT CASE WHEN upper(p_ticker) LIKE '%.TW' THEN 'mops'
                WHEN upper(p_ticker) LIKE '%.T' THEN 'edinet'
                ELSE 'edgar' END
$$;

CREATE OR REPLACE FUNCTION reporting_currency(p_ticker text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT f.currency
    FROM listing l
    JOIN fact f ON f.issuer_id = l.issuer_id
    JOIN filing g ON g.accession = f.accession
    WHERE l.ticker = upper(p_ticker) AND g.source = listing_source(l.ticker)
    GROUP BY f.currency
    ORDER BY count(*) DESC, f.currency
    LIMIT 1
$$;

CREATE OR REPLACE FUNCTION listing_facts(p_ticker text, p_period text, p_currency text, p_items text[])
RETURNS TABLE (period_end date, item text, value double precision)
LANGUAGE sql STABLE AS $$
    SELECT DISTINCT ON (f.item, f.period_end) f.period_end, f.item, f.value
    FROM listing l
    JOIN fact f ON f.issuer_id = l.issuer_id
    JOIN filing g ON g.accession = f.accession
    WHERE l.ticker = upper(p_ticker) AND f.period = p_period
      AND f.currency = CASE WHEN p_currency = 'reporting' THEN reporting_currency(p_ticker) ELSE p_currency END
      AND f.item = ANY (p_items)
      AND g.source = listing_source(l.ticker)
    ORDER BY f.item, f.period_end, g.filed_at DESC, g.accession DESC
$$;

CREATE OR REPLACE FUNCTION flow_period(p_ticker text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT CASE WHEN EXISTS (
        SELECT FROM listing l
        JOIN fact f ON f.issuer_id = l.issuer_id
        JOIN filing g ON g.accession = f.accession
        WHERE l.ticker = upper(p_ticker) AND f.period = '3m' AND f.item = 'Revenue'
          AND g.source = listing_source(l.ticker)
    ) THEN '3m' ELSE '12m' END
$$;

CREATE OR REPLACE FUNCTION flow_period(p_ticker text, p_requested text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT CASE
        WHEN p_requested = 'auto' OR p_requested IS NULL THEN flow_period(p_ticker)
        WHEN EXISTS (
            SELECT FROM listing l
            JOIN fact f ON f.issuer_id = l.issuer_id
            JOIN filing g ON g.accession = f.accession
            WHERE l.ticker = upper(p_ticker) AND f.period = p_requested AND f.item = 'Revenue'
              AND g.source = listing_source(l.ticker)
        ) THEN p_requested
        ELSE '12m'
    END
$$;
