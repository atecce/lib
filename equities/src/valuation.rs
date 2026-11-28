//! Market values from daily closes (the caller's: tiingo's cache) and what filings had
//! reported by each day: market cap and price-to-book, and the two flows that
//! explain it, return on equity and earnings yield:
//!
//! ```text
//! price-to-book = return on equity / earnings yield
//! ```
//!
//! (profit over book equity, and profit over market cap). A company earning
//! more on its book equity than the market's earnings yield trades above book.
//!
//! Each day uses only what was public then: the shares on the latest cover
//! page filed on or before it (or dated, when its filing isn't known), and equity as first reported. Between a split
//! and the next cover page, the cover's count is multiplied by the split, so
//! the count matches the price.

use chrono::NaiveDate;

use crate::item::{Item, Reported};
use crate::series::Series;
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
    depositary_ratios(ticker).iter().rev()
        .find(|(from, _)| from.parse::<NaiveDate>().is_ok_and(|f| f <= date))
        .map_or(1.0, |(_, n)| *n)
}

/// How many of the company's shares one listed share represents, from each
/// date it changed: empty for an ordinary listing (always 1). TSMC's have
/// always been 5 common shares.
pub fn depositary_ratios(ticker: Ticker) -> &'static [(&'static str, f64)] {
    match ticker.as_str() {
        "TM" => &[("1900-01-01", 2.0), ("2021-10-01", 10.0)],
        "HMC" => &[("1900-01-01", 1.0), ("2023-10-01", 3.0)],
        "TSM" => &[("1900-01-01", 5.0)],
        _ => &[],
    }
}

/// The day a figure became public: its filing's date when known, else the
/// date it is as of. A 20-F's cover counts the shares on December 31 but
/// is filed in April; a quarter's balance sheet is filed weeks after it.
fn known(r: &Reported) -> NaiveDate {
    r.filing.as_ref().map_or(r.date, |f| f.filed.max(r.date))
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

/// Daily market values for `ticker`, from its closes (in dollars: US
/// listings), its splits (the days it split, each the new shares per old)
/// and its reported items, read in filing order.
pub fn market(ticker: Ticker, closes: &Series, splits: &Series, items: &[Reported]) -> Vec<Market> {
    let shares = as_first_reported(items, ticker, Item::SharesOutstanding);
    let equity = as_first_reported(items, ticker, Item::StockholdersEquity);
    let income = trailing_net_income(items, ticker);
    let usd_equity = |d: NaiveDate| equity.iter().find(|e| e.currency == Currency::USD && e.date == d).map(|e| e.val);
    let mut out = Vec::new();
    let mut cover = 0;          // index into shares of the cover in effect
    let mut split_since = 1.0;  // splits since that cover
    for &(day, close) in closes.points() {
        // A new cover page replaces the count and the splits since the last.
        let mut changed = false;
        while cover + 1 < shares.len() && known(shares[cover + 1]) <= day {
            cover += 1;
            changed = true;
        }
        if changed { split_since = 1.0; }
        let Some(s) = shares.get(cover).filter(|s| known(s) <= day) else { continue };
        let factor = splits.on(day).unwrap_or(1.0);
        if factor > 0.0 && factor != 1.0 && day > s.date {
            split_since *= factor;
        }
        let listed = s.val / shares_per_listed(ticker, s.date) * split_since;
        let market_cap = close * listed;
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
            close,
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

    /// The closes and the splits of (day, close, split factor) rows.
    fn prices(rows: &[(&str, f64, f64)]) -> (Series, Series) {
        let day = |s: &str| s.parse::<NaiveDate>().unwrap();
        (Series::new(rows.iter().map(|r| (day(r.0), r.1))),
         Series::new(rows.iter().filter(|r| r.2 != 1.0).map(|r| (day(r.0), r.2))))
    }

    fn fact(item: Item, date: &str, val: f64) -> Reported {
        Reported { ticker: Ticker::TSLA, date: date.parse().unwrap(), p: Period::PointInTime, item, currency: Currency::USD, val, filing: None, derived: false }
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
        let rows = [
            ("2022-07-14", 900.0, 1.0),   // before any cover: nothing
            ("2022-07-15", 900.0, 1.0),
            ("2022-08-25", 300.0, 3.0),   // the split
            ("2022-10-17", 330.0, 1.0),
        ];
        let m = { let (closes, splits) = prices(&rows); market(Ticker::TSLA, &closes, &splits, &items) };
        let caps: Vec<f64> = m.iter().map(|m| m.market_cap).collect();
        assert_eq!(caps, [900_000.0, 900_000.0, 990_000.0]);
        assert_eq!(m[0].price_to_book, Some(30.0));
    }

    /// A cover dated December 31 but filed in April (a 20-F's) counts
    /// from April: until then the year before's cover, and its book.
    #[test]
    fn a_cover_counts_from_its_filing() {
        let filed = |date: &str, on: &str| Reported {
            filing: Some(crate::item::Filed { accession: on.into(), filed: on.parse().unwrap(), source: crate::item::Source::Edgar,
                                              form: "20-F".into(), period_end: Some(date.parse().unwrap()) }),
            ..fact(Item::SharesOutstanding, date, 0.0)
        };
        let items = [
            Reported { item: Item::StockholdersEquity, val: 10_000.0, ..filed("2023-12-31", "2024-04-18") },
            Reported { val: 100.0, ..filed("2023-12-31", "2024-04-18") },
            Reported { item: Item::StockholdersEquity, val: 20_000.0, ..filed("2024-12-31", "2025-04-17") },
            Reported { val: 100.0, ..filed("2024-12-31", "2025-04-17") },
        ];
        let rows = [("2025-02-03", 1_000.0, 1.0), ("2025-04-17", 1_000.0, 1.0)];
        let m = { let (closes, splits) = prices(&rows); market(Ticker::TSLA, &closes, &splits, &items) };
        let pb: Vec<Option<f64>> = m.iter().map(|m| m.price_to_book).collect();
        // February: 2023's book; from the filing day, 2024's.
        assert_eq!(pb, [Some(10.0), Some(5.0)]);
    }
}
