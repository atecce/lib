/// Usage: dump_table <R page>  — print the grid filing::table_rows builds.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: dump_table <R page>")?;
    for row in edgar::filing::table_rows(&std::fs::read_to_string(path)?)? {
        println!("{:?}", row);
    }
    Ok(())
}
