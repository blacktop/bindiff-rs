use bindiff_rs::BinExport;

fn main() -> anyhow::Result<()> {
    let input_path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Please provide a path to a BinExport file"))?;
    
    let binexport = BinExport::open(input_path)?;
    
    println!("executable_name: {}", binexport.executable_name()?);

    Ok(())
}
