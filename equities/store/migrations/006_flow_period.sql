-- The period a listing's flows are reported over: quarters ('3m') where it
-- reports them, else years ('12m'). Companies filing 20-Fs with the SEC
-- (TSMC, Toyota's TM, ...) report annually only. For the dashboards' default
-- period, "auto", so a company without quarters still draws its flows.
CREATE FUNCTION flow_period(p_ticker text) RETURNS text
LANGUAGE sql STABLE AS $$
    SELECT CASE WHEN EXISTS (
        SELECT FROM listing l
        JOIN fact f ON f.issuer_id = l.issuer_id
        JOIN filing g ON g.accession = f.accession
        WHERE l.ticker = upper(p_ticker) AND f.period = '3m' AND f.item = 'Revenue'
          AND g.source = CASE WHEN l.ticker LIKE '%.T' THEN 'edinet' ELSE 'edgar' END
    ) THEN '3m' ELSE '12m' END
$$;
