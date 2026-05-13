use std::error::Error;
use std::fs;

use calamine::{open_workbook_auto, Data, DataType, Reader};
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

#[derive(Debug)]
struct ReportedItem {
    t: NaiveDate,
    item: Item,
    val: f64,
}

const DATE_2026: NaiveDate = NaiveDate::from_ymd_opt(2026, 1, 25).unwrap();
const DATE_2025: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 26).unwrap();
const DATE_2024: NaiveDate = NaiveDate::from_ymd_opt(2024, 1, 28).unwrap();

fn main() -> Result<(), Box<dyn Error>> {

    let files: Vec<_> = fs::read_dir("./nvda")?
        .filter_map(|f| f.ok())
        .filter(|f| {
             f.path().extension()
                .and_then(|ext| ext.to_str()) == Some("xlsx")
        })
        .map(|f| f.path().file_stem()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
        )
        .collect();

    println!("{:#?}", files);

    let mut workbook = open_workbook_auto("2-25-26.xlsx")?;

    let mut items = Vec::new();

    process_balance_sheet(&workbook.worksheet_range("BALANCE_SHEET"), &mut items);
    process_income_statement(&workbook.worksheet_range("INCOME_STATEMENT"), &mut items);

    push_to_influx(&items)?;

    Ok(())
}

fn push_to_influx(items: &[ReportedItem]) -> Result<(), Box<dyn Error>> {
    let mut payload = String::new();
    for item in items {
        let ts = item.t.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_nanos_opt().unwrap();
        payload.push_str(&format!(
            "reported_item,item={} value={} {}\n",
            item.item.as_str(),
            item.val,
            ts
        ));
    }

    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:8181/write?db=financials")
        .body(payload)
        .send()?;

    if !res.status().is_success() {
        return Err(format!("InfluxDB write failed: {}", res.text()?).into());
    }

    Ok(())
}

fn process_balance_sheet(range: &Result<calamine::Range<Data>, calamine::Error>, items: &mut Vec<ReportedItem>) {
    if let Ok(range) = range {
        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            let label = match row.get(1) {
                Some(l) if !l.is_empty() => l.get_string().unwrap_or(""),
                _ => continue,
            };
            let val_2026 = match row.get(2) {
                Some(v) if v.is_float() => v.get_float().unwrap(),
                _ => f64::NAN,
            };
            let val_2025 = match row.get(3) {
                Some(v) if v.is_float() => v.get_float().unwrap(),
                _ => f64::NAN,
            };

            if let Some(item) = Item::from(label) {
                if !val_2026.is_nan() {
                    items.push(ReportedItem { t: DATE_2026, item: item.clone(), val: val_2026 });
                }
                if !val_2025.is_nan() {
                    items.push(ReportedItem { t: DATE_2025, item, val: val_2025 });
                }
                println!("{:<40} | 2026: {:<10} | 2025: {:<10}", label, val_2026, val_2025);
            }
        }
    }
}

fn process_income_statement(range: &Result<calamine::Range<Data>, calamine::Error>, items: &mut Vec<ReportedItem>) {
    if let Ok(range) = range {
        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            let label = match row.get(1) {
                Some(l) if !l.is_empty() => l.get_string().unwrap_or(""),
                _ => continue,
            };
            let val_2026 = match row.get(2) {
                Some(v) if v.is_float() => v.get_float().unwrap(),
                _ => f64::NAN,
            };
            let val_2025 = match row.get(3) {
                Some(v) if v.is_float() => v.get_float().unwrap(),
                _ => f64::NAN,
            };
            let val_2024 = match row.get(4) {
                Some(v) if v.is_float() => v.get_float().unwrap(),
                _ => f64::NAN,
            };

            if let Some(item) = Item::from(label) {
                if !val_2026.is_nan() {
                    items.push(ReportedItem { t: DATE_2026, item: item.clone(), val: val_2026 });
                }
                if !val_2025.is_nan() {
                    items.push(ReportedItem { t: DATE_2025, item: item.clone(), val: val_2025 });
                }
                if !val_2024.is_nan() {
                    items.push(ReportedItem { t: DATE_2024, item, val: val_2024 });
                }
                println!("{:<40} | 2026: {:<10} | 2025: {:<10} | 2024: {:<10}", label, val_2026, val_2025, val_2024);
            }
        }
    }
}
