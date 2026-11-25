//! Brokerage holdings and investment transactions (Plaid's JSON, as the
//! plaid crate returns it) into the portfolio_* tables.

use std::error::Error;

use chrono::NaiveDate;
use serde_json::Value;

use crate::Store;

fn date(v: &Value) -> Option<NaiveDate> {
    v.as_str().and_then(|s| NaiveDate::parse_from_str(s.get(..10)?, "%Y-%m-%d").ok())
}

/// Written by a sync: accounts, holdings and transactions.
#[derive(Debug, Default)]
pub struct Written {
    pub accounts: usize,
    pub holdings: usize,
    pub transactions: usize,
}

impl Store {
    /// When `item_id` was last synced.
    pub fn portfolio_synced(&mut self, item_id: &str) -> Result<Option<chrono::DateTime<chrono::Utc>>, Box<dyn Error>> {
        Ok(self.db.query_opt("SELECT synced_at FROM portfolio_sync WHERE item_id = $1", &[&item_id])?.map(|r| r.get(0)))
    }

    /// One Item's `/investments/holdings/get` reply, as the holdings of
    /// `as_of` (replacing that day's), and its transactions with the
    /// securities they name (added or updated).
    pub fn write_portfolio(&mut self, item_id: &str, institution: Option<&str>, holdings: &Value,
                           transactions: &[Value], securities: &[Value], as_of: NaiveDate) -> Result<Written, Box<dyn Error>> {
        let mut written = Written::default();
        let mut tx = self.db.transaction()?;

        let all_securities = holdings["securities"].as_array().into_iter().flatten().chain(securities);
        for s in all_securities {
            let o = &s["option_contract"];
            tx.execute(
                "INSERT INTO portfolio_security (security_id, ticker, name, type, subtype, option_type, strike, expiration, underlying)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                 ON CONFLICT (security_id) DO UPDATE SET ticker = EXCLUDED.ticker, name = EXCLUDED.name, type = EXCLUDED.type,
                   subtype = EXCLUDED.subtype, option_type = EXCLUDED.option_type, strike = EXCLUDED.strike,
                   expiration = EXCLUDED.expiration, underlying = EXCLUDED.underlying",
                &[&s["security_id"].as_str(), &s["ticker_symbol"].as_str(), &s["name"].as_str(), &s["type"].as_str(),
                  &s["subtype"].as_str(), &o["contract_type"].as_str(), &o["strike_price"].as_f64(),
                  &date(&o["expiration_date"]), &o["underlying_security_ticker"].as_str()])?;
        }

        for a in holdings["accounts"].as_array().into_iter().flatten() {
            tx.execute(
                "INSERT INTO portfolio_account (account_id, item_id, institution, name, type, subtype, balance, updated_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, now())
                 ON CONFLICT (account_id) DO UPDATE SET item_id = EXCLUDED.item_id, institution = EXCLUDED.institution,
                   name = EXCLUDED.name, type = EXCLUDED.type, subtype = EXCLUDED.subtype, balance = EXCLUDED.balance, updated_at = now()",
                &[&a["account_id"].as_str(), &item_id, &institution, &a["name"].as_str().unwrap_or("account"),
                  &a["type"].as_str(), &a["subtype"].as_str(), &a["balances"]["current"].as_f64()])?;
            tx.execute("DELETE FROM portfolio_holding WHERE as_of = $1 AND account_id = $2", &[&as_of, &a["account_id"].as_str()])?;
            written.accounts += 1;
        }

        for h in holdings["holdings"].as_array().into_iter().flatten() {
            written.holdings += tx.execute(
                "INSERT INTO portfolio_holding (as_of, account_id, security_id, quantity, price, value, cost_basis, price_as_of)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[&as_of, &h["account_id"].as_str(), &h["security_id"].as_str(), &h["quantity"].as_f64().unwrap_or(0.0),
                  &h["institution_price"].as_f64(), &h["institution_value"].as_f64(), &h["cost_basis"].as_f64(),
                  &date(&h["institution_price_as_of"])])? as usize;
        }

        for t in transactions {
            written.transactions += tx.execute(
                "INSERT INTO portfolio_transaction (transaction_id, account_id, security_id, date, type, subtype, quantity, price, amount, fees, name)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                 ON CONFLICT (transaction_id) DO UPDATE SET date = EXCLUDED.date, type = EXCLUDED.type, subtype = EXCLUDED.subtype,
                   quantity = EXCLUDED.quantity, price = EXCLUDED.price, amount = EXCLUDED.amount, fees = EXCLUDED.fees, name = EXCLUDED.name",
                &[&t["investment_transaction_id"].as_str(), &t["account_id"].as_str(), &t["security_id"].as_str(),
                  &date(&t["date"]).ok_or("a transaction without a date")?, &t["type"].as_str(), &t["subtype"].as_str(),
                  &t["quantity"].as_f64(), &t["price"].as_f64(), &t["amount"].as_f64(), &t["fees"].as_f64(), &t["name"].as_str()])? as usize;
        }

        tx.execute("INSERT INTO portfolio_sync (item_id, synced_at) VALUES ($1, now())
                    ON CONFLICT (item_id) DO UPDATE SET synced_at = now()", &[&item_id])?;
        tx.commit()?;
        Ok(written)
    }
}
