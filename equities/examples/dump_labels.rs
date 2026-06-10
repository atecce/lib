use calamine::{open_workbook_auto, Reader};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: dump_labels <file> <sheet>");
        return Ok(());
    }
    let mut workbook = open_workbook_auto(&args[1])?;
    if let Ok(range) = workbook.worksheet_range(&args[2]) {
        for (i, row) in range.rows().take(30).enumerate() {
            print!("Row {}: ", i);
            for cell in row.iter() {
                print!("[{:?}] ", cell);
            }
            println!();
        }
    }
    Ok(())
}
