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
                match r.items(BalanceSheet) {
                    Ok(items) => {
                        println!("{:#?}", path);
                        println!("{:#?}", items);
                    },
                    Err(e) => eprintln!("failed to process balance sheet for path: {:?}; {}", path, e),
                }
            },
            Err(e) => eprintln!("failed to construst new reader from path: {:?}; {}", path, e),
        }
    }
    Ok(())
}
