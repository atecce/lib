use calamine::{open_workbook_auto, Reader};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: inspect_sheets <file>");
        return Ok(());
    }
    let mut workbook = open_workbook_auto(&args[1])?;
    println!("Sheets in {}:", args[1]);
    for sheet in workbook.sheet_names() {
        println!("  {}", sheet);
    }
    Ok(())
}
