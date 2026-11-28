-- When a listing's company was last written in full (facts, prices, market
-- values): what a push or backfill has done so far, for store-progress, and
-- how fresh each company is.
ALTER TABLE listing ADD COLUMN written_at timestamptz;
CREATE INDEX ON listing (written_at);
