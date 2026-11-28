-- Brokerage holdings and investment transactions, read-only, through Plaid
-- (the plaid crate; store-plaid writes them). Ids are Plaid's.

CREATE TABLE portfolio_account (
    account_id  text PRIMARY KEY,
    item_id     text NOT NULL,
    institution text,
    name        text NOT NULL,
    type        text,
    subtype     text,
    balance     double precision,
    updated_at  timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE portfolio_security (
    security_id text PRIMARY KEY,
    ticker      text,
    name        text,
    type        text,            -- equity, etf, mutual fund, cryptocurrency, derivative, cash, ...
    subtype     text,
    option_type text,            -- call, put: a derivative's contract
    strike      double precision,
    expiration  date,
    underlying  text
);

-- What each account held, as of a day: one snapshot per sync day, replaced
-- by a later sync the same day.
CREATE TABLE portfolio_holding (
    as_of       date NOT NULL,
    account_id  text NOT NULL REFERENCES portfolio_account,
    security_id text NOT NULL REFERENCES portfolio_security,
    quantity    double precision NOT NULL,
    price       double precision,
    value       double precision,
    cost_basis  double precision,  -- the position's total cost, as the institution reports it
    price_as_of date,
    PRIMARY KEY (as_of, account_id, security_id)
);

CREATE TABLE portfolio_transaction (
    transaction_id text PRIMARY KEY,
    account_id     text NOT NULL REFERENCES portfolio_account,
    security_id    text REFERENCES portfolio_security,
    date           date NOT NULL,
    type           text,
    subtype        text,
    quantity       double precision,
    price          double precision,
    amount         double precision,
    fees           double precision,
    name           text             -- the institution's description: Robinhood's starts "sell_short ...", "buy ...", ...
);
CREATE INDEX portfolio_transaction_security ON portfolio_transaction (account_id, security_id, date);

CREATE TABLE portfolio_sync (
    item_id   text PRIMARY KEY,
    synced_at timestamptz NOT NULL
);
