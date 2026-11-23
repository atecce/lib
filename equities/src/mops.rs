//! Reading MOPS's quarterly reports (Taiwan; the `mops` crate's cache) into
//! reported items. See equities/TAIWAN.md.
//!
//! A report is one page, Big5-encoded HTML, with the balance sheet (資產負債表,
//! zīchǎn fùzhài biǎo), the statement of comprehensive income (綜合損益表,
//! zònghé sǔnyì biǎo) and the cash flow statement (現金流量表, xiànjīn liúliàng
//! biǎo), in one of two layouts: to 2018, Chinese labels only; from 2019, each
//! line with its account code (代號, dàihào) and Chinese and English labels.
//! The Chinese labels are the same in both, so lines are mapped by label.
//!
//! Each table starts with a header naming its columns' periods:
//!
//! | header | period |
//! |---|---|
//! | 2025年6月30日 (…2025/6/30) | at that date |
//! | 2017年第2季 (dì èr jì, second quarter) | the quarter |
//! | 2025年4月1日至6月30日 (zhì, "to") | from, to |
//! | 2017年度 (niándù, fiscal year) | the year |
//!
//! Amounts are in thousands of New Taiwan dollars (新臺幣仟元, Xīntáibì qiān
//! yuán), earnings per share in dollars. Outflows are in parentheses.

use std::collections::HashMap;
use std::error::Error;

use chrono::NaiveDate;
use scraper::{Html, Selector};

use crate::item::{Filed, Item, Reported, Source, Unit};
use crate::{Currency, Period, Ticker};

/// Chinese label (the part before any English) → item, and a rank: where two
/// lines give the same item, the lower rank wins. Amounts of lines marked
/// `sum` are added (depreciation and amortization are two lines).
const LABELS: &[(&str, Item, usize)] = &[
    // 資產負債表 (balance sheet)
    ("現金及約當現金", Item::CashAndCashEquivalents, 0),              // xiànjīn jí yuēdāng xiànjīn
    ("現金及約當現金總額", Item::CashAndCashEquivalents, 0),
    ("應收帳款淨額", Item::AccountsReceivableNet, 0),                 // yìngshōu zhàngkuǎn jìng'é
    ("存貨", Item::Inventories, 0),                                   // cúnhuò
    ("存貨合計", Item::Inventories, 0),
    ("流動資產合計", Item::TotalCurrentAssets, 0),                    // liúdòng zīchǎn héjì
    ("不動產、廠房及設備", Item::PropertyAndEquipmentNet, 0),         // bùdòngchǎn, chǎngfáng jí shèbèi
    ("不動產、廠房及設備合計", Item::PropertyAndEquipmentNet, 0),
    ("無形資產", Item::IntangibleAssetsNet, 0),                       // wúxíng zīchǎn
    ("無形資產合計", Item::IntangibleAssetsNet, 0),
    ("資產總計", Item::TotalAssets, 0),                               // zīchǎn zǒngjì
    ("資產總額", Item::TotalAssets, 0),
    ("流動負債合計", Item::TotalCurrentLiabilities, 0),               // liúdòng fùzhài héjì
    ("負債總計", Item::TotalLiabilities, 0),                          // fùzhài zǒngjì
    ("負債總額", Item::TotalLiabilities, 0),
    ("歸屬於母公司業主之權益合計", Item::StockholdersEquity, 0),      // guīshǔ yú mǔgōngsī yèzhǔ zhī quányì héjì
    // 綜合損益表 (statement of comprehensive income)
    ("營業收入合計", Item::Revenue, 0),                               // yíngyè shōurù héjì
    ("營業成本合計", Item::CostOfRevenue, 0),                         // yíngyè chéngběn héjì
    ("營業毛利（毛損）淨額", Item::GrossProfit, 0),                   // yíngyè máolì (máosǔn) jìng'é
    ("營業毛利（毛損）", Item::GrossProfit, 1),
    ("研究發展費用", Item::ResearchAndDevelopment, 0),                // yánjiū fāzhǎn fèiyòng
    ("研究發展費用合計", Item::ResearchAndDevelopment, 0),
    ("營業利益（損失）", Item::OperatingIncome, 0),                   // yíngyè lìyì (sǔnshī)
    ("繼續營業單位稅前淨利（淨損）", Item::IncomeBeforeIncomeTax, 0), // jìxù yíngyè dānwèi shuìqián jìnglì
    ("稅前淨利（淨損）", Item::IncomeBeforeIncomeTax, 1),
    ("所得稅費用（利益）合計", Item::IncomeTaxExpense, 0),            // suǒdéshuì fèiyòng (lìyì) héjì
    ("母公司業主（淨利／損）", Item::NetIncome, 0),                   // mǔgōngsī yèzhǔ (jìnglì/sǔn): the parent's owners'
    ("本期淨利（淨損）", Item::NetIncome, 1),                         // běnqī jìnglì (jìngsǔn): including minorities
    ("基本每股盈餘", Item::EarningsPerShareBasic, 1),                 // jīběn měigǔ yíngyú
    ("基本每股盈餘合計", Item::EarningsPerShareBasic, 0),
    ("稀釋每股盈餘", Item::EarningsPerShareDiluted, 1),               // xīshì měigǔ yíngyú
    ("稀釋每股盈餘合計", Item::EarningsPerShareDiluted, 0),
    // 現金流量表 (cash flow statement)
    ("營業活動之淨現金流入（流出）", Item::NetCashProvidedByOperatingActivities, 0), // yíngyè huódòng zhī jìng xiànjīn liúrù (liúchū)
    ("投資活動之淨現金流入（流出）", Item::NetCashUsedInInvestingActivities, 0),     // tóuzī huódòng
    ("籌資活動之淨現金流入（流出）", Item::NetCashUsedInFinancingActivities, 0),     // chóuzī huódòng
    ("取得不動產、廠房及設備", Item::PurchasesRelatedToPropertyAndEquipmentAndIntangibleAssets, 0), // qǔdé
    ("發放現金股利", Item::DividendsPaid, 0),                         // fāfàng xiànjīn gǔlì
    ("庫藏股票買回成本", Item::PaymentsRelatedToRepurchasesOfCommonStock, 0), // kùcáng gǔpiào mǎihuí chéngběn
    ("本期現金及約當現金增加（減少）數", Item::ChangeInCashAndCashEquivalents, 0),
];

/// Lines added together into one item: depreciation (折舊費用, zhéjiù
/// fèiyòng) and amortization (攤銷費用, tānxiāo fèiyòng).
const SUMS: &[(&str, Item)] = &[
    ("折舊費用", Item::DepreciationAndAmoritization),
    ("攤銷費用", Item::DepreciationAndAmoritization),
];

/// A company's cached reports, read: (items, reports read, reports that
/// failed to read).
pub fn read_company(cache: &mops::Cache, ticker: Ticker) -> Result<(Vec<Reported>, usize, usize), Box<dyn Error>> {
    let mut items = Vec::new();
    let (mut read, mut failed) = (0, 0);
    for report in cache.reports(ticker.as_str())? {
        match read_report(&report, ticker) {
            Ok(mut got) => { items.append(&mut got); read += 1 },
            Err(e) => { eprintln!("{} {}: {}", ticker, report.id(), e); failed += 1 },
        }
    }
    Ok((items, read, failed))
}

pub fn read_report(report: &mops::Report, ticker: Ticker) -> Result<Vec<Reported>, Box<dyn Error>> {
    let bytes = std::fs::read(&report.path)?;
    let (text, _, _) = encoding_rs::BIG5.decode(&bytes);
    let filed = Filed {
        accession: report.id(),
        filed: report.due(),
        source: Source::Mops,
        form: if report.quarter == 4 { "annual".into() } else { format!("Q{}", report.quarter) },
        period_end: Some(report.period_end()),
    };
    Ok(read_html(&text, ticker, &filed))
}

/// The items in a report's HTML (decoded).
pub fn read_html(html: &str, ticker: Ticker, filed: &Filed) -> Vec<Reported> {
    let doc = Html::parse_document(html);
    let tr = Selector::parse("tr").unwrap();
    let cell = Selector::parse("td, th").unwrap();
    let th = Selector::parse("th").unwrap();

    // (item, date, period) → (value, rank); sums add up.
    let mut best: HashMap<(Item, NaiveDate, Period), (f64, usize)> = HashMap::new();
    let mut sums: HashMap<(Item, NaiveDate, Period), f64> = HashMap::new();
    let mut columns: Vec<(usize, NaiveDate, Period)> = Vec::new();

    for row in doc.select(&tr) {
        let cells: Vec<String> = row.select(&cell)
            .map(|c| c.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        let headers: Vec<(usize, NaiveDate, Period)> = cells.iter().enumerate()
            .filter_map(|(i, c)| column(c).map(|(d, p)| (i, d, p))).collect();
        if !headers.is_empty() {
            columns = headers;
            continue;
        }
        // A header without dates (the changes in equity's account codes)
        // ends the columns until the next statement's header.
        if row.select(&th).next().is_some() && row.select(&th).count() == cells.len() && cells.len() > 2 {
            columns.clear();
            continue;
        }
        if columns.is_empty() {
            continue;
        }
        let Some(label) = cells.iter().find(|c| c.chars().any(is_cjk)).map(|c| chinese(c)) else { continue };
        let mapped = LABELS.iter().find(|(l, _, _)| *l == label).map(|(_, i, r)| (*i, *r, false))
            .or_else(|| SUMS.iter().find(|(l, _)| *l == label).map(|(_, i)| (*i, 0, true)));
        let Some((item, rank, sum)) = mapped else { continue };
        for &(i, date, period) in &columns {
            let Some(v) = cells.get(i).and_then(|c| amount(c)) else { continue };
            let v = if item.unit() == Unit::Money { v * 1000.0 } else { v };
            let key = (item, date, period);
            if sum {
                *sums.entry(key).or_default() += v;
            } else if best.get(&key).is_none_or(|&(_, r)| rank < r) {
                best.insert(key, (v, rank));
            }
        }
    }
    for (key, v) in sums {
        best.entry(key).or_insert((v, 0));
    }
    let mut out: Vec<Reported> = best.into_iter().map(|((item, date, p), (val, _))| Reported {
        ticker, date, p, item, currency: Currency::TWD, val, filing: Some(filed.clone()), derived: false,
    }).collect();
    out.sort_by_key(|r| (r.date, r.item, r.p));
    out
}

fn is_cjk(c: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&c)
}

/// The Chinese part of a label: before the English, without the full-width
/// spaces that indent it.
fn chinese(label: &str) -> String {
    let s: String = label.chars().map(|c| if c == '\u{3000}' { ' ' } else { c }).collect();
    let end = s.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(s.len());
    s[..end].trim().to_string()
}

/// "1,234", "(1,234)" or "-1,234"; empty or a dash is no value.
fn amount(cell: &str) -> Option<f64> {
    let t: String = cell.chars().filter(|c| !matches!(c, ',' | ' ')).collect();
    let (neg, digits) = match t.strip_prefix('(').and_then(|t| t.strip_suffix(')')) {
        Some(inner) => (true, inner.to_string()),
        None => match t.strip_prefix('-') {
            Some(rest) if !rest.is_empty() => (true, rest.to_string()),
            _ => (false, t.clone()),
        },
    };
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return None;
    }
    let n: f64 = digits.parse().ok()?;
    Some(if neg { -n } else { n })
}

/// A column header's date and period, in either layout.
fn column(header: &str) -> Option<(NaiveDate, Period)> {
    let h: String = header.split_whitespace().collect();
    let num = |s: &str| s.parse::<u32>().ok();
    // A range: 2025年4月1日至6月30日 or 2017年01月01日至2017年06月30日.
    if let Some((from, to)) = h.split_once('至') {
        let (fy, rest) = from.split_once('年')?;
        let (fm, rest) = rest.split_once('月')?;
        let fd = rest.split_once('日')?.0;
        let start = NaiveDate::from_ymd_opt(fy.parse().ok()?, num(fm)?, num(fd)?)?;
        let (ty, to) = match to.split_once('年') { Some((y, r)) if y.len() == 4 => (y.parse().ok()?, r), _ => (start.year_ce().1 as i32, to) };
        let (tm, rest) = to.split_once('月')?;
        let td = rest.split_once('日')?.0;
        let end = NaiveDate::from_ymd_opt(ty, num(tm)?, num(td)?)?;
        let months = (end.year_ce().1 as i32 - start.year_ce().1 as i32) * 12 + end.month0() as i32 - start.month0() as i32 + 1;
        let p = match months { 3 => Period::ThreeMonths, 6 => Period::SixMonths, 9 => Period::NineMonths, 12 => Period::TwelveMonths, _ => return None };
        return Some((end, p));
    }
    // A quarter: 2017年第2季.
    if let Some((y, rest)) = h.split_once("年第") {
        let q = num(rest.strip_suffix('季')?)?;
        return Some((mops::quarter_end(y.parse().ok()?, q), Period::ThreeMonths));
    }
    // A year: 2017年度.
    if let Some(y) = h.strip_suffix("年度") {
        return Some((mops::quarter_end(y.parse().ok()?, 4), Period::TwelveMonths));
    }
    // A date: 2025年6月30日 (followed in the new layout by 2025/6/30).
    let (y, rest) = h.split_once('年')?;
    let (m, rest) = rest.split_once('月')?;
    let (d, _) = rest.split_once('日')?;
    if y.len() != 4 {
        return None;
    }
    Some((NaiveDate::from_ymd_opt(y.parse().ok()?, num(m)?, num(d)?)?, Period::PointInTime))
}

use chrono::Datelike;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers() {
        let d = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        assert_eq!(column("2025年6月30日 2025/6/30"), Some((d("2025-06-30"), Period::PointInTime)));
        assert_eq!(column("2017年06月30日"), Some((d("2017-06-30"), Period::PointInTime)));
        assert_eq!(column("2025年4月1日至6月30日 2025/4/1To6/30"), Some((d("2025-06-30"), Period::ThreeMonths)));
        assert_eq!(column("2025年1月1日至6月30日2025/1/1To6/30"), Some((d("2025-06-30"), Period::SixMonths)));
        assert_eq!(column("2017年01月01日至2017年06月30日"), Some((d("2017-06-30"), Period::SixMonths)));
        assert_eq!(column("2017年第2季"), Some((d("2017-06-30"), Period::ThreeMonths)));
        assert_eq!(column("2017年度"), Some((d("2017-12-31"), Period::TwelveMonths)));
        assert_eq!(column("會計項目"), None);
    }

    #[test]
    fn labels_and_amounts() {
        assert_eq!(chinese("營業收入合計　Total operating revenue"), "營業收入合計");
        assert_eq!(chinese("　現金及約當現金"), "現金及約當現金");
        assert_eq!(amount("2,364,524,340"), Some(2364524340.0));
        assert_eq!(amount("(1,234)"), Some(-1234.0));
        assert_eq!(amount("-1,234"), Some(-1234.0));
        assert_eq!(amount(""), None);
        assert_eq!(amount("-"), None);
    }
}
