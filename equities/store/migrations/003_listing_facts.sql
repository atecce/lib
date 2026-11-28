-- A listing's facts as last reported, from its own regulator: what a chart
-- of a company draws. EDGAR for US listings, EDINET for Tokyo ones (7203.T),
-- so the two listings of one issuer (Toyota: TM, 7203.T) don't mix sources,
-- as fact_latest, which spans them, would. For the dashboards: one call a
-- panel, all of its items.
CREATE FUNCTION listing_facts(p_ticker text, p_period text, p_currency text, p_items text[])
RETURNS TABLE (period_end date, item text, value double precision)
LANGUAGE sql STABLE AS $$
    SELECT DISTINCT ON (f.item, f.period_end) f.period_end, f.item, f.value
    FROM listing l
    JOIN fact f ON f.issuer_id = l.issuer_id
    JOIN filing g ON g.accession = f.accession
    WHERE l.ticker = p_ticker AND f.period = p_period AND f.currency = p_currency
      AND f.item = ANY (p_items)
      AND g.source = CASE WHEN l.ticker LIKE '%.T' THEN 'edinet' ELSE 'edgar' END
    ORDER BY f.item, f.period_end, g.filed_at DESC, g.accession DESC
$$;
