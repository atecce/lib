-- Economic series from FRED, as the fred crate caches them: one row per
-- series, one per observation (the start of its period: a monthly series'
-- first of the month). A missing value (FRED's ".") is NULL.
CREATE TABLE fred_series (
    id           text PRIMARY KEY,
    title        text,
    first_date   date,
    last_date    date,
    observations integer NOT NULL DEFAULT 0,
    loaded_at    timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE fred_observation (
    series_id text NOT NULL REFERENCES fred_series (id),
    date      date NOT NULL,
    value     double precision,
    PRIMARY KEY (series_id, date)
);
