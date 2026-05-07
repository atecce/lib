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
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut workbook = open_workbook_auto("2-25-26.xlsx")?;

    print_non_empties(workbook.worksheet_range("BALANCE_SHEET"));
    print_non_empties(workbook.worksheet_range("INCOME_STATEMENT"));

    Ok(())
}

fn print_non_empties(range: Result<calamine::Range<Data>, calamine::Error>) {
    if let Ok(range) = range {
        for row in range.rows().filter(|row| !row.iter().all(|c| c.is_empty())) {
            println!("\trow: {:?}", row);

            for cell in row.iter().filter(|cell| !cell.is_empty()) {
                println!("\t\tcell: {:?}", cell);
            }
        }
    }
}
