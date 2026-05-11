use std::error::Error;

use calamine::{open_workbook_auto, Data, DataType, Error as CalamineError, Xls, Reader, RangeDeserializer, RangeDeserializerBuilder, Sheets};
use chrono::{DateTime, NaiveDate, Utc};
use prometheus::{register_gauge_vec, GaugeVec};

// type Record = (String, f32, f32, f32, f32);
//
// fn read_csv() -> Result<Vec<Record>, Box<dyn Error>> {
//     let mut r = csv::Reader::from_reader(
//         File::open("./MacroTrends_Data_Download_BRK.A.trimmed.csv").expect("can't open file"),
//     );
//     let mut records = Vec::new();
//
//     for res in r.deserialize() {
//         let record: Record = res?;
//         records.push(record);
//     }
//
//     Ok(records)
// }

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
    // TODO(atec): start tracking share counts and these stats
    // Commitments and contingencies - see Note 12 | 2026: 0 | 2025: 0
    // Preferred stock, $0.001 par value; 2 shares authorized; none issued | 2026: 0 | 2025: 0
    // outstanding as of January 26, 2025 | 2026: 24 | 2025: 24
    // Additional paid-in capital | 2026: 10118 | 2025: 11237
    // Accumulated other comprehensive income | 2026: 178 | 2025: 28
    // Retained earnings | 2026: 146973 | 2025: 68038
    // Total shareholders' equity | 2026: 157293 | 2025: 79327
    // Total liabilities and shareholders' equity | 2026: 206803 | 2025: 111601

    // Basic | 2026: 4.93 | 2025: 2.97
    // Diluted | 2026: 4.9 | 2025: 2.94
    // Basic | 2026: 24359 | 2025: 24555
    // Diluted | 2026: 24514 | 2025: 24804
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
            "Interese expense" => Some(Item::InterestExpense),
            "Other income, net" => Some(Item::OtherIncomeNet),
            "Total other income, net" => Some(Item::TotalOtherIncomeNet),
            "Income before income tax" => Some(Item::IncomeBeforeIncomeTax),
            "Income tax expense" => Some(Item::IncomeTaxExpense),
            "Net income" => Some(Item::NetIncome),
            &_ => None,
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut workbook = open_workbook_auto("2-25-26.xlsx")?;

    let gauge = register_gauge_vec!("reported_item", "Reported financial items", &["item", "date"])?;

    process_items(&workbook.worksheet_range("BALANCE_SHEET"), &gauge);
    process_items(&workbook.worksheet_range("INCOME_STATEMENT"), &gauge);

    prometheus::push_metrics(
        "nvda",
        std::collections::HashMap::new(),
        "localhost:9091",
        prometheus::gather(),
        None,
    )?;

    Ok(())
}

fn process_items(range: &Result<calamine::Range<Data>, calamine::Error>, gauge: &GaugeVec) {
    if let Ok(range) = range {
        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            let label = match row.get(1) {
                Some(l) if !l.is_empty() => l,
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

            if let Some(item) = Item::from(label.get_string().expect("failed to cast label to &str")) {
                if !val_2025.is_nan() {
                    gauge.with_label_values(&[label.get_string().expect("failed to get label as &str"), &DATE_2025.to_string()]).set(val_2025);
                }
                if !val_2026.is_nan() {
                    gauge.with_label_values(&[label.get_string().expect("failed to get label as &str"), &DATE_2026.to_string()]).set(val_2026);
                }
                println!("{:<40} | 2026: {:<10} | 2025: {:<10}", label, val_2026, val_2025);
            }
        }
    }
}
