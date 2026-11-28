//! Robinhood's account activity reports (Account → Reports and statements →
//! Reports, a CSV per account): the history before Plaid's two years.
//!
//! Columns: "Activity Date","Process Date","Settle Date","Instrument",
//! "Description","Trans Code","Quantity","Price","Amount". Descriptions run
//! over several lines ("Oracle\nCUSIP: 68389X105"); amounts are "$1,234.56"
//! in and "($10.00)" out; the file ends with a disclaimer row.
//!
//! Rows are read into Plaid's shape (portfolio_transaction), so what reads
//! Plaid's reads these: buys "buy …" with quantity and amount positive,
//! sells "sell …" both negative, cash dividends "Cash dividend of $x from
//! TICKER - DIVIDEND" with the amount negative (money in, as Plaid writes it).

use chrono::NaiveDate;

/// One report row, in Plaid's shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    /// Stable across re-imports of the same report: the row's own fields,
    /// and its number among identical rows.
    pub id: String,
    pub date: NaiveDate,
    pub ticker: String,
    pub kind: Kind,
    pub quantity: f64,
    pub price: Option<f64>,
    pub amount: f64,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind { Buy, Sell, Dividend, Split }

impl Kind {
    /// Plaid's (type, subtype).
    pub fn plaid(self) -> (&'static str, &'static str) {
        match self {
            Kind::Buy => ("buy", "buy"),
            Kind::Sell => ("sell", "sell"),
            Kind::Dividend => ("cash", "dividend"),
            Kind::Split => ("split", "split"),
        }
    }
}

/// "$1,234.56" → 1234.56, "($10.00)" → -10.0, "" → None.
fn money(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() { return None }
    let (neg, s) = match s.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
        Some(inner) => (true, inner),
        None => (false, s),
    };
    let v: f64 = s.trim_start_matches('$').replace(',', "").parse().ok()?;
    Some(if neg { -v } else { v })
}

/// The report's buys, sells and cash dividends ("Buy", "Sell", "CDIV").
/// Everything else (options, transfers, fees, interest, lending) is counted
/// in `skipped` by its code, not read.
pub fn parse(csv: &str) -> Result<(Vec<Row>, std::collections::BTreeMap<String, usize>), Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(csv.as_bytes());
    let mut rows = Vec::new();
    let mut skipped = std::collections::BTreeMap::new();
    let mut seen: std::collections::HashMap<String, usize> = Default::default();
    let mut moves = Vec::new();
    for record in reader.records() {
        let r = record?;
        if r.len() < 9 { continue }
        let Ok(date) = NaiveDate::parse_from_str(&r[0], "%m/%d/%Y") else { continue };
        let (ticker, code) = (r[3].trim().to_string(), r[5].trim());
        if matches!(code, "Buy" | "Sell" | "SPL" | "SPR" | "SXCH") && !ticker.is_empty() {
            let q = r[6].trim();
            let out = q.ends_with('S');
            if let Ok(q) = q.trim_end_matches('S').replace(',', "").parse::<f64>() {
                moves.push((date, ticker.clone(), code.to_string(), q, out));
            }
        }
        let kind = match code {
            "Buy" => Kind::Buy,
            "Sell" => Kind::Sell,
            "CDIV" => Kind::Dividend,
            _ => { *skipped.entry(code.to_string()).or_insert(0) += 1; continue }
        };
        let (Some(amount), false) = (money(&r[8]), ticker.is_empty()) else {
            *skipped.entry(format!("{} (no amount or ticker)", code)).or_insert(0) += 1;
            continue;
        };
        let quantity: f64 = r[6].trim().replace(',', "").parse().unwrap_or(0.0);
        let price = money(&r[7]);
        let key = format!("{}:{}:{}:{}:{}", date, code, ticker, &r[6], &r[8]);
        let n = seen.entry(key.clone()).or_insert(0);
        *n += 1;
        let id = format!("{}:{}", key, n);
        let (quantity, amount, name) = match kind {
            Kind::Buy => (quantity, amount.abs(), format!("buy {} shares of {} (Robinhood report)", quantity, ticker)),
            Kind::Sell => (-quantity, -amount.abs(), format!("sell {} shares of {} (Robinhood report)", quantity, ticker)),
            Kind::Dividend => (0.0, -amount, format!("Cash dividend of ${:.2} from {} - DIVIDEND (Robinhood report)", amount, ticker)),
            Kind::Split => unreachable!(),
        };
        rows.push(Row { id, date, ticker, kind, quantity, price, amount, name });
    }
    rows.extend(splits(&moves));
    Ok((rows, skipped))
}

/// Splits and exchanges as ratios (new shares per old), from the report's
/// share movements in date order (`moves`: date, ticker, code, quantity,
/// and whether it is the "S" side, the shares taken out). A forward split
/// (SPL) lists the shares added, so its ratio needs the shares held before,
/// tracked here from the buys, sells and earlier splits; a reverse split
/// (SPR) or an exchange (SXCH) lists the old shares out ("S") and the new in.
/// Ratios on fewer than a hundredth of a share held are not trusted.
///
/// The shares held are tracked from this report alone, so shares that came
/// in another way (a transfer, a merger) make a ratio inexact. Two repairs:
/// a split's shares credited in parts within a week are one split, and a
/// forward split's ratio within 15% of a usual one (simple) is that one,
/// since companies split in simple ratios. Reverse splits and exchanges
/// list both sides, so their ratios are exact as they are.
fn splits(moves: &[(NaiveDate, String, String, f64, bool)]) -> Vec<Row> {
    let mut ordered: Vec<_> = moves.iter().rev().collect();
    // In date order; on a split's day, the split first (the day's trades are
    // in the new shares).
    ordered.sort_by_key(|m| (m.0, !matches!(m.2.as_str(), "SPL" | "SPR" | "SXCH")));
    let mut held: std::collections::HashMap<&str, f64> = Default::default();
    let mut out_today: std::collections::HashMap<(&str, NaiveDate), (Option<f64>, Option<f64>)> = Default::default();
    // (date, ticker, code, shares before, shares after): one per split.
    let mut found: Vec<(NaiveDate, &str, &str, f64, f64)> = Vec::new();
    for (date, ticker, code, q, out) in ordered {
        let h = held.entry(ticker.as_str()).or_insert(0.0);
        match (code.as_str(), out) {
            ("Buy", _) => *h += q,
            ("Sell", _) => *h -= q,
            ("SPL", _) => {
                // Another part of a split credited in the last week.
                let earlier = found.iter_mut().rev()
                    .find(|f| f.1 == ticker && f.2 == "SPL" && (*date - f.0).num_days() <= 7);
                match earlier {
                    Some(f) => f.4 += q,
                    None if *h > 0.01 => found.push((*date, ticker, code, *h, *h + q)),
                    None => {}
                }
                *h += q;
            }
            // The two sides of a reverse split or an exchange, in either
            // order: the ratio once both are seen.
            (_, out) => {
                let side = out_today.entry((ticker.as_str(), *date)).or_insert((None, None));
                if *out { side.0 = Some(*q) } else { side.1 = Some(*q) }
                if let (Some(old), Some(new)) = *side {
                    out_today.remove(&(ticker.as_str(), *date));
                    if old > 0.01 { found.push((*date, ticker, code, old, new)) }
                    *h = *h - old + new;
                }
            }
        }
    }
    found.into_iter().filter_map(|(date, ticker, code, before, after)| {
        let ratio = if code == "SPL" { simple(after / before) } else { after / before };
        (ratio.is_finite() && ratio > 0.0 && (ratio - 1.0).abs() > 1e-9).then(|| Row {
            id: format!("{}:{}:{}:split", date, code, ticker), date, ticker: ticker.to_string(), kind: Kind::Split,
            quantity: ratio, price: None, amount: 0.0,
            name: format!("split {} new per old of {} (Robinhood report, {})", ratio, ticker, code),
        })
    }).collect()
}

/// The usual forward split ratio (5:4, 3:2, 2 to 10, 12, 15, 20, 25, 30,
/// 40, 50 for 1) within 15% of `r`, the nearest by log distance; else `r`.
fn simple(r: f64) -> f64 {
    [1.25, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 12.0, 15.0, 20.0, 25.0, 30.0, 40.0, 50.0].into_iter()
        .map(|c| (c, (r / c).ln().abs()))
        .filter(|(_, d)| *d < 1.15f64.ln())
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(c, _)| c)
        .unwrap_or(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Invented rows, in the report's shape.
    const REPORT: &str = "\"Activity Date\",\"Process Date\",\"Settle Date\",\"Instrument\",\"Description\",\"Trans Code\",\"Quantity\",\"Price\",\"Amount\"
\"3/2/2021\",\"3/2/2021\",\"3/4/2021\",\"ABC\",\"Abc Corp
CUSIP: 000000000\",\"Buy\",\"2\",\"$50.00\",\"($100.00)\"
\"3/2/2021\",\"3/2/2021\",\"3/4/2021\",\"ABC\",\"Abc Corp
CUSIP: 000000000\",\"Buy\",\"2\",\"$50.00\",\"($100.00)\"
\"6/1/2021\",\"6/1/2021\",\"6/1/2021\",\"ABC\",\"Cash Div: R/D 2021-05-20 P/D 2021-06-01 - 4 shares at 0.25\",\"CDIV\",\"\",\"\",\"$1.00\"
\"7/1/2021\",\"7/1/2021\",\"7/3/2021\",\"ABC\",\"Abc Corp
CUSIP: 000000000\",\"Sell\",\"1\",\"$1,060.00\",\"$1,060.00\"
\"7/2/2021\",\"7/2/2021\",\"7/2/2021\",\"\",\"ACH Deposit\",\"ACH\",\"\",\"\",\"$500.00\"
\"\"
\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"The data provided is for informational purposes only.\"
";

    #[test]
    fn reads_buys_sells_and_dividends_in_plaids_shape() {
        let (rows, skipped) = parse(REPORT).unwrap();
        assert_eq!(rows.len(), 4);
        let buy = &rows[0];
        assert_eq!((buy.kind, buy.quantity, buy.amount, buy.price), (Kind::Buy, 2.0, 100.0, Some(50.0)));
        assert!(buy.name.starts_with("buy "));
        // Two identical buys the same day are two rows, with two ids.
        assert_ne!(rows[0].id, rows[1].id);
        let div = &rows[2];
        assert_eq!((div.kind, div.amount), (Kind::Dividend, -1.0));
        assert!(div.name.starts_with("Cash dividend of $1.00 from ABC - DIVIDEND"));
        let sell = &rows[3];
        assert_eq!((sell.quantity, sell.amount), (-1.0, -1060.0));
        assert_eq!(skipped.get("ACH"), Some(&1));
    }

    /// Invented: 3 shares bought, a 2-for-1 split (3 added), then a 1-for-10
    /// reverse split (6 out, 0.6 in), newest first as the report lists them.
    #[test]
    fn splits_are_ratios_of_the_shares_held() {
        let d = |m, day| NaiveDate::from_ymd_opt(2024, m, day).unwrap();
        let s = |x: &str| x.to_string();
        let moves = vec![
            (d(9, 1), s("ABC"), s("SPR"), 6.0, true),
            (d(9, 1), s("ABC"), s("SPR"), 0.6, false),
            (d(6, 1), s("ABC"), s("SPL"), 3.0, false),
            (d(1, 2), s("ABC"), s("Buy"), 3.0, false),
        ];
        let rows = splits(&moves);
        let ratios: Vec<(NaiveDate, f64)> = rows.iter().map(|r| (r.date, r.quantity)).collect();
        assert_eq!(ratios.len(), 2);
        assert_eq!(ratios[0], (d(6, 1), 2.0));
        assert!((ratios[1].1 - 0.1).abs() < 1e-12 && ratios[1].0 == d(9, 1));
        assert!(rows.iter().all(|r| r.kind == Kind::Split));
    }

    /// Shares from elsewhere make a ratio inexact: 20-for-1 credited in two
    /// parts, on 2 held of which only 1.9 are in the report (invented).
    #[test]
    fn inexact_splits_are_made_simple() {
        let d = |day| NaiveDate::from_ymd_opt(2022, 6, day).unwrap();
        let s = |x: &str| x.to_string();
        let moves = vec![
            (d(6), s("ABC"), s("SPL"), 10.0, false),
            (d(3), s("ABC"), s("SPL"), 28.0, false),
            (d(1), s("ABC"), s("Buy"), 1.9, false),
        ];
        let rows = splits(&moves);
        assert_eq!(rows.len(), 1);
        assert_eq!((rows[0].date, rows[0].quantity), (d(3), 20.0));
        assert_eq!(simple(5.33), 5.0);
        assert_eq!(simple(3.41), 3.0);
        assert_eq!(simple(21.55), 20.0);
        // Nothing usual near: as it is.
        assert_eq!(simple(100.0), 100.0);
    }

    #[test]
    fn money_reads_the_reports_amounts() {
        assert_eq!(money("$1,234.56"), Some(1234.56));
        assert_eq!(money("($10.00)"), Some(-10.0));
        assert_eq!(money(""), None);
    }
}
