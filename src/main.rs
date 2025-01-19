use calm_io::*;
use bindiff_rs::BinDiff;

#[pipefail]
fn main() -> std::io::Result<()> {
    let input_path = std::env::args()
        .nth(1)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Please provide a path to a BinDiff file"))?;
    
    let bd = BinDiff::open(&input_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Read file
    let file = bd.read_file()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    stdoutln!("{}", file)?;

    // Read metadata
    let metadata = bd.read_metadata()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    stdoutln!("{}", metadata)?;

    // Read function matches
    let func_matches = bd.read_function_matches()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    for func_match in func_matches {
        stdoutln!("{}", func_match)?;
    }

    bd.close()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}
