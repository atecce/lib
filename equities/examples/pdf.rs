fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = equities::reader::new_pdf_reader(std::path::Path::new("tsla/tsla-20250930-gen.pdf"), equities::Ticker::TSLA)?;
    r.process_balance_sheet()?;

    Ok(())
}
