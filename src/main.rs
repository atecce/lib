use std::error::Error;

use calamine::{open_workbook_auto, Data, DataType, Error as CalamineError, Xls, Reader, RangeDeserializer, RangeDeserializerBuilder, Sheets};

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
    IntanglibleAssetsNet,
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

struct ReportedItem {
    t: std::time::SystemTime,
    item: Item,
    val: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut workbook = open_workbook_auto("2-25-26.xlsx")?;

    print_items(&workbook.worksheet_range("BALANCE_SHEET"));
    println!();
    print_items(&workbook.worksheet_range("INCOME_STATEMENT"));

    Ok(())
}

fn print_items(range: &Result<calamine::Range<Data>, calamine::Error>) {
    if let Ok(range) = range {
        let zipped = range.rows()
            .filter(|row| !row.iter().all(|c| c.is_empty()))
            .filter_map(|row| {
                let label = row.get(1)?;
                let val_2026 = row.get(2)?;
                let val_2025 = row.get(3)?;

                if !label.is_empty() && (val_2026.is_float() || val_2025.is_float()) {
                    Some((label, val_2026, val_2025))
                } else {
                    None
                }
            });

        for (label, val26, val25) in zipped {
            println!("{:<40} | 2026: {:<10} | 2025: {:<10}", label, val26, val25);
        }
    }
}
