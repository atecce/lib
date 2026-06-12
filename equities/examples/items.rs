use std::error::Error;
use std::fs;

use calamine::DataType;

use equities::item::Item;
use equities::xlsx::new_reader;
use equities::sheet_info::new_sheet_info;
use equities::sheet_info::SheetType::BalanceSheet;


fn main() -> Result<(), Box<dyn Error>>{
    for path in fs::read_dir("nvda")?
        .filter_map(|f| f.ok())
        .filter(|f| {
             let path = f.path();
             let ext = path.extension().and_then(|ext| ext.to_str());
             ext == Some("xlsx")
        })
        .map(|f| f.path()) {

        match new_reader(&path, equities::Ticker::NVDA) {
            Ok(mut r) => {
                match r.rows("BALANCE_SHEET") {
                    Ok(rows) => {
                        let tmp1 = rows.clone();
                        let tmp2: Vec<&[calamine::Data]> = tmp1.iter().map(|row| row.as_slice()).collect();

                        match new_sheet_info(&tmp2, BalanceSheet) {
                            Ok(sheet_info) => {
                                let mut items = Vec::new();
                                for row in rows {
                                    if let Some(label) = row.get(sheet_info.labels).and_then(|c| c.get_string()) {
                                        if let Ok(item) = label.parse::<Item>() {
                                            items.push(item);
                                        }
                                    }
                                }
                                println!("{:#?}", path);
                                println!("{:#?}", items);
                            },
                            Err(e) => eprintln!("failed to construct sheet info for path: {:?}; {}", path, e),
                        }
                    },
                    Err(e) => eprintln!("failed to process balance sheet for path: {:?}; {}", path, e),
                }
            },
            Err(e) => eprintln!("failed to construst new reader from path: {:?}; {}", path, e),
        }
    }
    Ok(())
}
