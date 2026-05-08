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
    let balance_sheet = workbook.worksheet_range("BALANCE_SHEET");
    let income_statement = workbook.worksheet_range("INCOME_STATEMENT");

    print_non_empties(&balance_sheet);
    println!();
    print_non_empties(&income_statement);

    print_col(&balance_sheet, 1);
    print_col(&balance_sheet, 2);
    print_col(&balance_sheet, 3);

    print_col(&income_statement, 1);
    print_col(&income_statement, 2);
    print_col(&income_statement, 3);

    Ok(())
}

fn print_non_empties(range: &Result<calamine::Range<Data>, calamine::Error>) {
    if let Ok(range) = range {
        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            println!("{:?}", row);
        }
    }
}

fn print_col(range: &Result<calamine::Range<Data>, calamine::Error>, col: usize) {
    if let Ok(range) = range {
        for row in range.rows() {
            if let Some(val) = row.get(col) {
                println!("{}", val);
            }
        }
    }
}
