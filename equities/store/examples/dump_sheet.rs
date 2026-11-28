use equities::calamine::{open_workbook_auto, Reader};
use std::env;

/// Usage: dump_sheet <file> <sheet-name-prefix>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut wb = open_workbook_auto(&args[1])?;
    let name = wb.sheet_names().into_iter().find(|n| n.starts_with(&args[2])).ok_or("no sheet")?;
    println!("== {}", name);
    for row in wb.worksheet_range(&name)?.rows() {
        println!("{:?}", row);
    }
    Ok(())
}
