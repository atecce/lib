//! TSMC (TSM; 台積電, Táijīdiàn): what its valuation needs that the generic
//! code can't read from its filings.
//!
//! TSMC reports in New Taiwan dollars. Its quarterly statements (from the
//! Taiwan filings) give equity and net income in TWD only; its 20-F adds a
//! convenience translation into dollars once a year, of the year-end balance
//! sheet, dated December 31 though filed in April. Its share count is on the
//! 20-F's cover, also once a year. Valued from those alone, its book in
//! dollars changed once a year and a year late in effect: price-to-book
//! rose with the price all year and fell 12–18% each December 31.
//!
//! `in_dollars` gives the valuation what it lacks: each quarter's equity and
//! net income translated at the Taiwan dollar's rate (FRED's DEXTAUS), and
//! the latest share count on each quarter's date. These are for valuation
//! only, not stored as reported facts.

use chrono::NaiveDate;

use crate::item::{Item, Reported};
use crate::{Currency, Period, Ticker};

/// TSMC's Taipei listing, whose quarterly reports (MOPS) the valuation of
/// its US depositary shares reads.
pub const TAIPEI: Ticker = Ticker::new("2330.TW");

/// The rate on `day` or the last trading day before it (within ten days):
/// Taiwan dollars per US dollar, as (date, rate), oldest first.
fn rate_on(day: NaiveDate, twd_per_usd: &[(NaiveDate, f64)]) -> Option<f64> {
    let i = twd_per_usd.partition_point(|(d, _)| *d <= day);
    let (d, r) = *twd_per_usd.get(i.checked_sub(1)?)?;
    ((day - d).num_days() <= 10).then_some(r)
}

/// The mean rate over the period that ends on `end` (for a flow: net income
/// is earned through the period, not on its last day).
fn mean_rate(end: NaiveDate, months: u32, twd_per_usd: &[(NaiveDate, f64)]) -> Option<f64> {
    let start = end.checked_sub_months(chrono::Months::new(months))?;
    let within: Vec<f64> = twd_per_usd.iter().filter(|(d, _)| *d > start && *d <= end).map(|(_, r)| *r).collect();
    (within.len() >= 20 * months as usize / 3).then(|| within.iter().sum::<f64>() / within.len() as f64)
}

/// Dollar figures for valuing TSM, from its reported `items` and the Taiwan
/// dollar's rate (`twd_per_usd`: date, TWD per USD, oldest first):
///
/// - stockholders' equity, each quarter end's TWD balance at that day's rate;
/// - net income, each quarter (and year) in TWD at the period's mean rate;
/// - shares outstanding on each of those quarter ends: the latest 20-F cover
///   count dated on or before it (the count was public; only repeated).
pub fn in_dollars(items: &[Reported], twd_per_usd: &[(NaiveDate, f64)]) -> Vec<Reported> {
    let tsm = Ticker::TSM;
    let mine = |i: &&Reported| (i.ticker == tsm || i.ticker == TAIPEI) && i.currency == Currency::TWD;
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for i in items.iter().filter(mine) {
        let rate = match (i.item, i.p) {
            (Item::StockholdersEquity, Period::PointInTime) => rate_on(i.date, twd_per_usd),
            (Item::NetIncome, Period::ThreeMonths) => mean_rate(i.date, 3, twd_per_usd),
            (Item::NetIncome, Period::TwelveMonths) => mean_rate(i.date, 12, twd_per_usd),
            _ => continue,
        };
        let Some(rate) = rate else { continue };
        if !seen.insert((i.item, i.p, i.date)) { continue }
        out.push(Reported { ticker: tsm, currency: Currency::USD, val: i.val / rate, derived: true, ..i.clone() });
    }

    let mut covers: Vec<&Reported> = items.iter()
        .filter(|i| i.ticker == tsm && i.item == Item::SharesOutstanding && i.p == Period::PointInTime)
        .collect();
    covers.sort_by_key(|c| c.date);
    let quarter_ends: Vec<NaiveDate> = out.iter().filter(|r| r.item == Item::StockholdersEquity).map(|r| r.date).collect();
    for day in quarter_ends {
        if covers.iter().any(|c| c.date == day) { continue }
        if let Some(c) = covers.iter().rev().find(|c| c.date < day) {
            out.push(Reported { date: day, derived: true, ..(*c).clone() });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate { NaiveDate::from_ymd_opt(y, m, day).unwrap() }

    fn item(item: Item, p: Period, date: NaiveDate, currency: Currency, val: f64) -> Reported {
        Reported { ticker: Ticker::TSM, date, p, item, currency, val, filing: None, derived: false }
    }

    /// Invented: a rate of 30 through 2024 (every weekday), equity of
    /// TWD 3,000 at the year end and 3,300 at the first quarter's end, net
    /// income of TWD 300 in the quarter, and 10 shares on the year's cover.
    #[test]
    fn quarters_in_dollars_and_the_count_carried_forward() {
        use chrono::Datelike;
        // 30 through 2024, 33 in 2025.
        let rates: Vec<(NaiveDate, f64)> = d(2024, 1, 1).iter_days().take_while(|x| *x <= d(2025, 6, 30))
            .filter(|x| x.weekday().number_from_monday() <= 5)
            .map(|x| (x, if x.year() == 2024 { 30.0 } else { 33.0 })).collect();
        let items = vec![
            item(Item::StockholdersEquity, Period::PointInTime, d(2024, 12, 31), Currency::TWD, 3000.0),
            item(Item::StockholdersEquity, Period::PointInTime, d(2025, 3, 31), Currency::TWD, 3300.0),
            item(Item::NetIncome, Period::ThreeMonths, d(2025, 3, 31), Currency::TWD, 330.0),
            item(Item::SharesOutstanding, Period::PointInTime, d(2024, 12, 31), Currency::USD, 10.0),
            // Another company's TWD figures are not TSMC's.
            Reported { ticker: Ticker::NVDA, ..item(Item::StockholdersEquity, Period::PointInTime, d(2025, 3, 31), Currency::TWD, 1.0) },
        ];
        let out = in_dollars(&items, &rates);
        let find = |it: Item, day| out.iter().find(|r| r.item == it && r.date == day).map(|r| (r.val, r.currency));
        assert_eq!(find(Item::StockholdersEquity, d(2024, 12, 31)), Some((100.0, Currency::USD)));
        assert_eq!(find(Item::StockholdersEquity, d(2025, 3, 31)), Some((100.0, Currency::USD)));
        assert_eq!(find(Item::NetIncome, d(2025, 3, 31)), Some((10.0, Currency::USD)));
        // The year's cover count, repeated at the quarter's end; not at the
        // cover's own date, which has it already.
        assert_eq!(find(Item::SharesOutstanding, d(2025, 3, 31)).map(|s| s.0), Some(10.0));
        assert_eq!(out.iter().filter(|r| r.item == Item::SharesOutstanding).count(), 1);
        assert!(out.iter().all(|r| r.derived && r.ticker == Ticker::TSM));
    }

    #[test]
    fn no_rate_no_dollars() {
        let items = vec![item(Item::StockholdersEquity, Period::PointInTime, d(2024, 12, 31), Currency::TWD, 3000.0)];
        assert!(in_dollars(&items, &[(d(2024, 1, 2), 30.0)]).is_empty());
    }
}
