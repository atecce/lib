-- The issuer's industry: its SEC Standard Industrial Classification code
-- (four digits; 3571 is electronic computers), as EDGAR's submissions give
-- it. Filled by `store-sic` from `universe`'s output; EDINET and MOPS
-- issuers have none unless they also file with the SEC.
ALTER TABLE issuer ADD COLUMN sic text;
CREATE INDEX issuer_sic_idx ON issuer (sic);
