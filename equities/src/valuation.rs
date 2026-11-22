//! Market values from daily prices (the `tiingo` cache) and what filings had
//! reported by each day: market cap and price-to-book, and the two flows that
//! explain it, return on equity and earnings yield:
//!
//!     price-to-book = return on equity / earnings yield
//!
//! (profit over book equity, and profit over market cap). A company earning
//! more on its book equity than the market's earnings yield trades above book.
//!
//! Each day uses only what was public then: the shares on the latest cover
//! page dated on or before it, and equity as first reported. Between a split
//! and the next cover page, the cover's count is multiplied by the split, so
//! the count matches the price.

use chrono::NaiveDate;

use crate::item::{Item, Reported};
use crate::{Currency, Period, Ticker};

/// One trading day's values.
#[derive(Debug, Clone)]
pub struct Market {
    pub ticker: Ticker,
    pub date: NaiveDate,
    pub currency: Currency,
    /// As traded that day.
    pub close: f64,
    pub market_cap: f64,
    /// Market cap over stockholders' equity, when both are in one currency.
    pub price_to_book: Option<f64>,
    /// Net income over the last four quarters, over average stockholders'
    /// equity (now and a year before; or now, without that).
    pub return_on_equity: Option<f64>,
    /// The same net income over market cap: the inverse of the P/E ratio.
    pub earnings_yield: Option<f64>,
}

/// How many of the company's shares one traded share represents, for US
/// depositary shares. When a foreign company splits its shares, the US price
/// may not move: the ratio changes instead (and Tiingo records no split).
/// Toyota's (TM) were 2 ordinary shares until its 5-for-1 split on
/// 2021-10-01, then 10; Honda's (HMC) 1 until its 3-for-1 split on
/// 2023-10-01, then 3; TSMC's (TSM) 5. Checked against each price series
/// around the split.
fn shares_per_listed(ticker: Ticker, date: NaiveDate) -> f64 {
    let before = |y, m, d| date < NaiveDate::from_ymd_opt(y, m, d).unwrap();
    match ticker.as_str() {
        "TM" if before(2021, 10, 1) => 2.0,
        "TM" => 10.0,
        "HMC" if before(2023, 10, 1) => 1.0,
        "HMC" => 3.0,
        // TSMC's have always been 5 common shares.
        "TSM" => 5.0,
        _ => 1.0,
    }
}

/// First-reported values of a point-in-time item, by date, oldest first.
fn as_first_reported(items: &[Reported], ticker: Ticker, item: Item) -> Vec<&Reported> {
    first_reported(items, ticker, item, Period::PointInTime)
}

fn first_reported(items: &[Reported], ticker: Ticker, item: Item, period: Period) -> Vec<&Reported> {
    let mut out: Vec<&Reported> = Vec::new();
    for i in items.iter().filter(|i| i.ticker == ticker && i.item == item && i.p == period) {
        if !out.iter().any(|o| o.date == i.date && o.currency == i.currency) {
            out.push(i);
        }
    }
    out.sort_by_key(|i| i.date);
    out
}

/// Net income over the four quarters ending at each quarter end, in dollars,
/// from quarters as first reported (a fiscal year's fourth quarter derived
/// from the year less nine months). Where four quarters within a year are
/// not there, a twelve-month figure ending that day.
fn trailing_net_income(items: &[Reported], ticker: Ticker) -> Vec<(NaiveDate, f64)> {
    let quarters: Vec<&Reported> = first_reported(items, ticker, Item::NetIncome, Period::ThreeMonths)
        .into_iter().filter(|q| q.currency == Currency::USD).collect();
    let years: Vec<&Reported> = first_reported(items, ticker, Item::NetIncome, Period::TwelveMonths)
        .into_iter().filter(|y| y.currency == Currency::USD).collect();
    let mut ends: Vec<NaiveDate> = quarters.iter().map(|q| q.date).chain(years.iter().map(|y| y.date)).collect();
    ends.sort();
    ends.dedup();
    let mut out = Vec::new();
    for end in ends {
        let last4: Vec<&&Reported> = quarters.iter().filter(|q| q.date <= end).rev().take(4).collect();
        let four = last4.len() == 4 && last4[0].date == end && (end - last4[3].date).num_days() <= 300;
        if four {
            out.push((end, last4.iter().map(|q| q.val).sum()));
        } else if let Some(y) = years.iter().find(|y| y.date == end) {
            out.push((end, y.val));
        }
    }
    out
}

/// Daily market values for `ticker`, from its prices (in dollars: Tiingo's
/// prices are US listings) and its reported items, read in filing order.
pub fn market(ticker: Ticker, prices: &[tiingo::Price], items: &[Reported]) -> Vec<Market> {
    let shares = as_first_reported(items, ticker, Item::SharesOutstanding);
    let equity = as_first_reported(items, ticker, Item::StockholdersEquity);
    let income = trailing_net_income(items, ticker);
    let usd_equity = |d: NaiveDate| equity.iter().find(|e| e.currency == Currency::USD && e.date == d).map(|e| e.val);
    let mut out = Vec::new();
    let mut cover = 0;          // index into shares of the cover in effect
    let mut splits = 1.0;       // splits since that cover
    for p in prices {
        let Some(day) = p.day() else { continue };
        // A new cover page replaces the count and the splits since the last.
        let mut changed = false;
        while cover + 1 < shares.len() && shares[cover + 1].date <= day {
            cover += 1;
            changed = true;
        }
        if changed { splits = 1.0; }
        let Some(s) = shares.get(cover).filter(|s| s.date <= day) else { continue };
        if p.split_factor > 0.0 && p.split_factor != 1.0 && day > s.date {
            splits *= p.split_factor;
        }
        let listed = s.val / shares_per_listed(ticker, s.date) * splits;
        let market_cap = p.close * listed;
        let book = equity.iter().rev()
            .find(|e| e.currency == Currency::USD && e.date <= s.date && (s.date - e.date).num_days() <= 400);
        // The last four quarters reported by this cover's filing.
        let ttm = income.iter().rev().find(|(end, _)| *end <= s.date && (s.date - *end).num_days() <= 400);
        let return_on_equity = ttm.and_then(|(end, ni)| {
            let now = usd_equity(*end)?;
            let year_before = equity.iter()
                .find(|e| e.currency == Currency::USD && (*end - e.date).num_days().abs_diff(365) <= 10)
                .map(|e| e.val);
            let avg = year_before.map(|b| (now + b) / 2.0).unwrap_or(now);
            (avg > 0.0).then(|| ni / avg)
        });
        out.push(Market {
            ticker,
            date: day,
            currency: Currency::USD,
            close: p.close,
            market_cap,
            price_to_book: book.filter(|b| b.val > 0.0).map(|b| market_cap / b.val),
            return_on_equity,
            earnings_yield: ttm.map(|(_, ni)| ni / market_cap),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn price(date: &str, close: f64, split: f64) -> tiingo::Price {
        tiingo::Price { date: format!("{}T00:00:00.000Z", date), close, adj_close: close, split_factor: split, dividend: 0.0 }
    }

    fn fact(item: Item, date: &str, val: f64) -> Reported {
        Reported { ticker: Ticker::TSLA, date: date.parse().unwrap(), p: Period::PointInTime, item, currency: Currency::USD, val }
    }

    #[test]
    fn split_between_covers() {
        // 1,000 shares on the July cover; a 3-for-1 split in August; the
        // October cover then states 3,000.
        let items = [
            fact(Item::StockholdersEquity, "2022-06-30", 30_000.0),
            fact(Item::SharesOutstanding, "2022-07-15", 1_000.0),
            fact(Item::SharesOutstanding, "2022-10-15", 3_000.0),
        ];
        let prices = [
            price("2022-07-14", 900.0, 1.0),   // before any cover: nothing
            price("2022-07-15", 900.0, 1.0),
            price("2022-08-25", 300.0, 3.0),   // the split
            price("2022-10-17", 330.0, 1.0),
        ];
        let m = market(Ticker::TSLA, &prices, &items);
        let caps: Vec<f64> = m.iter().map(|m| m.market_cap).collect();
        assert_eq!(caps, [900_000.0, 900_000.0, 990_000.0]);
        assert_eq!(m[0].price_to_book, Some(30.0));
    }
}
