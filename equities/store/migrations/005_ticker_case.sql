-- Tickers are stored as listed (AAPL, 7203.T); the dashboards' functions
-- take them in any case (InfluxDB's tags, and links made for them, were
-- lower case).
CREATE OR REPLACE FUNCTION reporting_currency(p_ticker text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT f.currency
    FROM listing l
    JOIN fact f ON f.issuer_id = l.issuer_id
    JOIN filing g ON g.accession = f.accession
    WHERE l.ticker = upper(p_ticker)
      AND g.source = CASE WHEN l.ticker LIKE '%.T' THEN 'edinet' ELSE 'edgar' END
    GROUP BY f.currency
    ORDER BY count(*) DESC, f.currency
    LIMIT 1
$$;

CREATE OR REPLACE FUNCTION market_currency(p_ticker text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT currency FROM market WHERE ticker = upper(p_ticker)
    GROUP BY currency ORDER BY count(*) DESC, currency LIMIT 1
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
      AND g.source = CASE WHEN l.ticker LIKE '%.T' THEN 'edinet' ELSE 'edgar' END
    ORDER BY f.item, f.period_end, g.filed_at DESC, g.accession DESC
$$;
