use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use calamine::{open_workbook_auto, Data, DataType, Reader, Range};
use chrono::NaiveDate;

#[derive(Clone, Debug)]
enum Item {
    CurrentAssets,
    CashAndCashEquivalents,
    MarketableSecurities,
    AccountsReceivableNet,
    Inventories,
    PrepaidExpensesAndOtherCurrentAssets,
    TotalCurrentAssets,
    PropertyAndEquipmentNet,
    OperatingLeaseAssets,
    Goodwill,
    IntangibleAssetsNet,
    DeferredIncomeTaxAssets,
    NonMarketableEquitySecurities,
    OtherAssets,
    TotalAssets,

    CurrentLiabilities,
    AccountsPayable,
    AccruedAndOtherCurrentLiabilities,
    ShortTermDebt,
    TotalCurrentLiabilities,
    LongTermDebt,
    LongTermOperatingLeaseLiabilities,
    OtherLongTermLiabilities,
    TotalLiabilities,

    Revenue,
    CostOfRevenue,
    GrossProfit,

    OperatingExpenses,
    ResearchAndDevelopment,
    SalesGeneralAndAdministrative,
    TotalOperatingExpenses,
    OperatingIncome,

    InterestIncome,
    InterestExpense,
    OtherIncomeNet,
    TotalOtherIncomeNet,
    IncomeBeforeIncomeTax,

    IncomeTaxExpense,
    NetIncome,
}

impl Item {
    fn from(s: &str) -> Option<Self> {
        match s.trim() {
            "Cash and cash equivalents" => Some(Item::CashAndCashEquivalents),
            "Marketable securities" => Some(Item::MarketableSecurities),
            "Accounts receivable, net" => Some(Item::AccountsReceivableNet),
            "Inventories" => Some(Item::Inventories),
            "Prepaid expenses and other current assets" => Some(Item::PrepaidExpensesAndOtherCurrentAssets),
            "Total current assets" => Some(Item::TotalCurrentAssets),
            "Property and equipment, net" => Some(Item::PropertyAndEquipmentNet),
            "Operating lease assets" => Some(Item::OperatingLeaseAssets),
            "Goodwill" => Some(Item::Goodwill),
            "Intangible assets, net" => Some(Item::IntangibleAssetsNet),
            "Deferred income tax assets" => Some(Item::DeferredIncomeTaxAssets),
            "Non-marketable equity securities" => Some(Item::NonMarketableEquitySecurities),
            "Other assets" => Some(Item::OtherAssets),
            "Total assets" => Some(Item::TotalAssets),

            "Accounts payable" => Some(Item::AccountsPayable),
            "Accrued and other current liabilities" => Some(Item::AccruedAndOtherCurrentLiabilities),
            "Short-term debt" => Some(Item::ShortTermDebt),
            "Total current liabilities" => Some(Item::TotalCurrentLiabilities),
            "Long-term debt" => Some(Item::LongTermDebt),
            "Long-term operating lease liabilities" => Some(Item::LongTermOperatingLeaseLiabilities),
            "Other long-term liabilities" => Some(Item::OtherLongTermLiabilities),
            "Total liabilities" => Some(Item::TotalLiabilities),

            "Revenue" => Some(Item::Revenue),
            "Cost of revenue" => Some(Item::CostOfRevenue),
            "Gross profit" => Some(Item::GrossProfit),
            "Research and development" => Some(Item::ResearchAndDevelopment),
            "Sales, general and administrative" => Some(Item::SalesGeneralAndAdministrative),
            "Total operating expenses" => Some(Item::TotalOperatingExpenses),
            "Operating income" => Some(Item::OperatingIncome),
            "Interest income" => Some(Item::InterestIncome),
            "Interest expense" => Some(Item::InterestExpense),
            "Other income, net" => Some(Item::OtherIncomeNet),
            "Total other income, net" => Some(Item::TotalOtherIncomeNet),
            "Income before income tax" => Some(Item::IncomeBeforeIncomeTax),
            "Income tax expense" => Some(Item::IncomeTaxExpense),
            "Net income" => Some(Item::NetIncome),
            &_ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Item::CurrentAssets => "CurrentAssets",
            Item::CashAndCashEquivalents => "CashAndCashEquivalents",
            Item::MarketableSecurities => "MarketableSecurities",
            Item::AccountsReceivableNet => "AccountsReceivableNet",
            Item::Inventories => "Inventories",
            Item::PrepaidExpensesAndOtherCurrentAssets => "PrepaidExpensesAndOtherCurrentAssets",
            Item::TotalCurrentAssets => "TotalCurrentAssets",
            Item::PropertyAndEquipmentNet => "PropertyAndEquipmentNet",
            Item::OperatingLeaseAssets => "OperatingLeaseAssets",
            Item::Goodwill => "Goodwill",
            Item::IntangibleAssetsNet => "IntangibleAssetsNet",
            Item::DeferredIncomeTaxAssets => "DeferredIncomeTaxAssets",
            Item::NonMarketableEquitySecurities => "NonMarketableEquitySecurities",
            Item::OtherAssets => "OtherAssets",
            Item::TotalAssets => "TotalAssets",
            Item::CurrentLiabilities => "CurrentLiabilities",
            Item::AccountsPayable => "AccountsPayable",
            Item::AccruedAndOtherCurrentLiabilities => "AccruedAndOtherCurrentLiabilities",
            Item::ShortTermDebt => "ShortTermDebt",
            Item::TotalCurrentLiabilities => "TotalCurrentLiabilities",
            Item::LongTermDebt => "LongTermDebt",
            Item::LongTermOperatingLeaseLiabilities => "LongTermOperatingLeaseLiabilities",
            Item::OtherLongTermLiabilities => "OtherLongTermLiabilities",
            Item::TotalLiabilities => "TotalLiabilities",
            Item::Revenue => "Revenue",
            Item::CostOfRevenue => "CostOfRevenue",
            Item::GrossProfit => "GrossProfit",
            Item::OperatingExpenses => "OperatingExpenses",
            Item::ResearchAndDevelopment => "ResearchAndDevelopment",
            Item::SalesGeneralAndAdministrative => "SalesGeneralAndAdministrative",
            Item::TotalOperatingExpenses => "TotalOperatingExpenses",
            Item::OperatingIncome => "OperatingIncome",
            Item::InterestIncome => "InterestIncome",
            Item::InterestExpense => "InterestExpense",
            Item::OtherIncomeNet => "OtherIncomeNet",
            Item::TotalOtherIncomeNet => "TotalOtherIncomeNet",
            Item::IncomeBeforeIncomeTax => "IncomeBeforeIncomeTax",
            Item::IncomeTaxExpense => "IncomeTaxExpense",
            Item::NetIncome => "NetIncome",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Period {
    ThreeMonths,
    SixMonths,
    NineMonths,
    TwelveMonths,
    PointInTime,
}

impl Period {
    fn as_str(&self) -> &'static str {
        match self {
            Period::ThreeMonths => "3m",
            Period::SixMonths => "6m",
            Period::NineMonths => "9m",
            Period::TwelveMonths => "12m",
            Period::PointInTime => "pit",
        }
    }
}

#[derive(Debug)]
struct ReportedItem {
    t: NaiveDate,
    p: Period,
    item: Item,
    val: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut items = Vec::new();

    let paths: Vec<_> = fs::read_dir("./nvda")?
        .filter_map(|f| f.ok())
        .filter(|f| {
             let path = f.path();
             let ext = path.extension().and_then(|ext| ext.to_str());
             ext == Some("xlsx")
        })
        .map(|f| f.path())
        .collect();

    for path in paths {
        if let Err(e) = process_file(&path, &mut items) {
            eprintln!("Failed to process {:?}: {}", path, e);
        }
    }

    if !items.is_empty() {
        push_to_influx(&items)?;
        println!("Successfully pushed {} items to InfluxDB", items.len());
    }

    Ok(())
}

fn process_file(path: &Path, items: &mut Vec<ReportedItem>) -> Result<(), Box<dyn Error>> {
    let mut workbook = open_workbook_auto(path)?;

    if let Ok(range) = workbook.worksheet_range("BALANCE_SHEET") {
        process_balance_sheet(&range, items);
    }
    if let Ok(range) = workbook.worksheet_range("INCOME_STATEMENT") {
        process_income_statement(&range, items);
    }

    Ok(())
}

fn process_balance_sheet(range: &Range<Data>, items: &mut Vec<ReportedItem>) {
    let mut dates = HashMap::new();
    let rows: Vec<_> = range.rows().take(25).collect();

    for (r, row) in rows.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if let Some(date) = parse_date(cell) {
                dates.entry(c).or_insert(date);
            } else if let Some(month_day) = cell.get_string() {
                if month_day.trim().ends_with(',') || month_day.trim().split_whitespace().count() >= 2 {
                     if let Some(next_row) = rows.get(r + 1) {
                         if let Some(year_cell) = next_row.get(c) {
                             let year = match year_cell {
                                 Data::Float(f) => *f as i32,
                                 Data::Int(i) => *i as i32,
                                 _ => 0,
                             };
                             if year > 1900 && year < 2100 {
                                 let date_str = format!("{} {}", month_day, year);
                                 if let Some(date) = parse_date_str(&date_str) {
                                     dates.entry(c).or_insert(date);
                                 }
                             }
                         }
                     }
                }
            }
        }
    }

    if dates.is_empty() { return; }

    let multiplier = find_multiplier(range);

    for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
        let label = match row.get(1) {
            Some(l) if !l.is_empty() => l.get_string().unwrap_or(""),
            _ => continue,
        };

        if let Some(item) = Item::from(label) {
            for (col, date) in &dates {
                if let Some(v) = row.get(*col) {
                    let val = match v {
                        Data::Float(f) => *f,
                        Data::Int(i) => *i as f64,
                        _ => f64::NAN,
                    };
                    if !val.is_nan() {
                        items.push(ReportedItem { t: *date, p: Period::PointInTime, item: item.clone(), val: val * multiplier });
                    }
                }
            }
        }
    }
}

fn process_income_statement(range: &Range<Data>, items: &mut Vec<ReportedItem>) {
    let mut is_10k = false;
    let rows: Vec<_> = range.rows().take(25).collect();
    for row in rows.iter().take(10) {
        for cell in row.iter() {
            if let Some(s) = cell.get_string() {
                if s.to_lowercase().contains("form type: 10-k") {
                    is_10k = true;
                }
            }
        }
    }

    let mut col_periods: HashMap<usize, Period> = HashMap::new();
    for row in &rows {
        for (c, cell) in row.iter().enumerate() {
            if let Some(s) = cell.get_string() {
                let s = s.to_lowercase();
                let p = if s.contains("three months") { Some(Period::ThreeMonths) }
                    else if s.contains("six months") { Some(Period::SixMonths) }
                    else if s.contains("nine months") { Some(Period::NineMonths) }
                    else if s.contains("year ended") || s.contains("annual") || s.contains("twelve months") { Some(Period::TwelveMonths) }
                    else { None };
                
                if let Some(period) = p {
                    col_periods.insert(c, period);
                    col_periods.insert(c + 1, period);
                    col_periods.insert(c + 2, period);
                }
            }
        }
    }

    let mut col_info = HashMap::new();
    for (r, row) in rows.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if let Some(date) = parse_date(cell) {
                let p = col_periods.get(&c).cloned().unwrap_or_else(|| {
                    if is_10k { Period::TwelveMonths } else { Period::ThreeMonths }
                });
                col_info.entry(c).or_insert((date, p));
            } else if let Some(month_day) = cell.get_string() {
                if month_day.trim().ends_with(',') || month_day.trim().split_whitespace().count() >= 2 {
                     if let Some(next_row) = rows.get(r + 1) {
                         if let Some(year_cell) = next_row.get(c) {
                             let year = match year_cell {
                                 Data::Float(f) => *f as i32,
                                 Data::Int(i) => *i as i32,
                                 _ => 0,
                             };
                             if year > 1900 && year < 2100 {
                                 let date_str = format!("{} {}", month_day, year);
                                 if let Some(date) = parse_date_str(&date_str) {
                                     let p = col_periods.get(&c).cloned().unwrap_or_else(|| {
                                         if is_10k { Period::TwelveMonths } else { Period::ThreeMonths }
                                     });
                                     col_info.entry(c).or_insert((date, p));
                                 }
                             }
                         }
                     }
                }
            }
        }
    }

    if col_info.is_empty() { return; }

    let multiplier = find_multiplier(range);

    for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
        let label = match row.get(1) {
            Some(l) if !l.is_empty() => l.get_string().unwrap_or(""),
            _ => continue,
        };

        if let Some(item) = Item::from(label) {
            for (col, (date, period)) in &col_info {
                if let Some(v) = row.get(*col) {
                    let val = match v {
                        Data::Float(f) => *f,
                        Data::Int(i) => *i as f64,
                        _ => f64::NAN,
                    };
                    if !val.is_nan() {
                        items.push(ReportedItem { t: *date, p: *period, item: item.clone(), val: val * multiplier });
                    }
                }
            }
        }
    }
}

fn find_multiplier(range: &Range<Data>) -> f64 {
    for row in range.rows().take(20) {
        for cell in row.iter() {
            if let Some(s) = cell.get_string() {
                let s = s.to_lowercase();
                if s.contains("in millions") {
                    return 1_000_000.0;
                }
                if s.contains("in thousands") {
                    return 1_000.0;
                }
            }
        }
    }
    1.0
}

fn parse_date(cell: &Data) -> Option<NaiveDate> {
    match cell {
        Data::String(s) => parse_date_str(s),
        _ => None,
    }
}

fn parse_date_str(s: &str) -> Option<NaiveDate> {
    let s = s.trim().replace(",", "");
    let formats = ["%B %d %Y", "%b %d %Y", "%m/%d/%Y", "%Y-%m-%d"];
    for fmt in formats {
        if let Ok(date) = NaiveDate::parse_from_str(&s, fmt) {
            return Some(date);
        }
    }
    None
}

fn push_to_influx(items: &[ReportedItem]) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let auth_token = std::env::var("INFLUXDB3_AUTH_TOKEN")
        .map_err(|_| "INFLUXDB3_AUTH_TOKEN environment variable not set")?;

    for chunk in items.chunks(1000) {
        let mut payload = String::new();
        for item in chunk {
            let ts = item.t.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_nanos_opt().unwrap();
            payload.push_str(&format!(
                "reported_item_v3,item={},period={} value={} {}\n",
                item.item.as_str(),
                item.p.as_str(),
                item.val,
                ts
            ));
        }

        let res = client.post("http://localhost:8181/write?db=financials")
            .bearer_auth(&auth_token)
            .body(payload)
            .send()?;

        if !res.status().is_success() {
            return Err(format!("InfluxDB write failed: {}", res.text()?).into());
        }
    }

    Ok(())
}
