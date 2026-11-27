-- The period to draw a listing's flows over, given the one asked for: as
-- asked, unless the listing has no flows over it (quarters, for a company
-- that files 20-Fs, as TSMC does), then years; 'auto' is quarters where
-- there are any. So a chart left on '3m' still draws TSMC's flows.
CREATE FUNCTION flow_period(p_ticker text, p_requested text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT CASE
        WHEN p_requested = 'auto' OR p_requested IS NULL THEN flow_period(p_ticker)
        WHEN EXISTS (
            SELECT FROM listing l
            JOIN fact f ON f.issuer_id = l.issuer_id
            JOIN filing g ON g.accession = f.accession
            WHERE l.ticker = upper(p_ticker) AND f.period = p_requested AND f.item = 'Revenue'
              AND g.source = CASE WHEN l.ticker LIKE '%.T' THEN 'edinet' ELSE 'edgar' END
        ) THEN p_requested
        ELSE '12m'
    END
$$;
