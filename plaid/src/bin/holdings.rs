//! Each connected Item's holdings, with their unrealized gains where the
//! institution reports a cost basis:
//!
//!     cargo run -p plaid --bin plaid-holdings
//!
//! Plaid's cost_basis is the position's total cost (not per share), and may
//! be null: then there's no gain to show until it is computed from
//! transactions.

use std::collections::HashMap;
use std::error::Error;

use plaid::{Client, Item};

fn money(v: f64) -> String {
    let s = format!("{:.2}", v.abs());
    let (int, frac) = s.split_once('.').unwrap();
    let mut grouped = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 { grouped.push(',') }
        grouped.push(c);
    }
    format!("{}{}.{}", if v < 0.0 { "-" } else { "" }, grouped, frac)
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::from_env_file()?;
    let items = Item::all()?;
    if items.is_empty() {
        return Err("no connections yet: run plaid-link".into());
    }
    for item in &items {
        let v = client.holdings(item)?;
        let securities: HashMap<&str, &serde_json::Value> = v["securities"].as_array().into_iter().flatten()
            .filter_map(|s| Some((s["security_id"].as_str()?, s))).collect();
        println!("{}", item.institution.as_deref().unwrap_or(&item.item_id));
        for account in v["accounts"].as_array().into_iter().flatten() {
            let id = account["account_id"].as_str().unwrap_or("");
            println!("\n  {} ({} {}), balance {}", account["name"].as_str().unwrap_or("?"),
                     account["type"].as_str().unwrap_or(""), account["subtype"].as_str().unwrap_or(""),
                     account["balances"]["current"].as_f64().map(money).unwrap_or("?".into()));
            println!("  {:<8} {:>12} {:>10} {:>14} {:>14} {:>14} {:>8}  {}", "ticker", "quantity", "price", "value", "cost", "gain", "%", "as of");
            let (mut value_sum, mut cost_sum, mut gain_value_sum, mut without) = (0.0, 0.0, 0.0, 0);
            let mut rows: Vec<&serde_json::Value> = v["holdings"].as_array().into_iter().flatten()
                .filter(|h| h["account_id"].as_str() == Some(id)).collect();
            rows.sort_by(|a, b| b["institution_value"].as_f64().partial_cmp(&a["institution_value"].as_f64()).unwrap());
            for h in rows {
                let s = securities.get(h["security_id"].as_str().unwrap_or("")).copied();
                let ticker = s.and_then(|s| s["ticker_symbol"].as_str()).unwrap_or("?");
                let quantity = h["quantity"].as_f64().unwrap_or(0.0);
                let price = h["institution_price"].as_f64().unwrap_or(0.0);
                let value = h["institution_value"].as_f64().unwrap_or(quantity * price);
                let as_of = h["institution_price_as_of"].as_str().unwrap_or("");
                value_sum += value;
                match h["cost_basis"].as_f64() {
                    Some(cost) => {
                        let gain = value - cost;
                        cost_sum += cost;
                        gain_value_sum += value;
                        println!("  {:<8} {:>12.4} {:>10} {:>14} {:>14} {:>14} {:>7.1}%  {}", ticker, quantity, money(price), money(value),
                                 money(cost), money(gain), if cost != 0.0 { 100.0 * gain / cost } else { 0.0 }, as_of);
                    }
                    None => {
                        without += 1;
                        println!("  {:<8} {:>12.4} {:>10} {:>14} {:>14} {:>14} {:>8}  {}", ticker, quantity, money(price), money(value), "—", "—", "", as_of);
                    }
                }
            }
            println!("  {:<8} {:>12} {:>10} {:>14} {:>14} {:>14} {:>7.1}%", "total", "", "", money(value_sum), money(cost_sum),
                     money(gain_value_sum - cost_sum), if cost_sum != 0.0 { 100.0 * (gain_value_sum - cost_sum) / cost_sum } else { 0.0 });
            if without > 0 {
                println!("  ({} holding{} without a cost basis: not in the gain)", without, if without == 1 { "" } else { "s" });
            }
        }
    }
    Ok(())
}
