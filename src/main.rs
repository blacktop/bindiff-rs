use bindiff_rs::BinDiff;
use calm_io::*;
use scopeguard::guard;
use serde_json;

#[pipefail]
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let json_output = args.contains(&"--json".to_string());
    let info_output = args.contains(&"--info".to_string());

    // Get the non-flag argument as the path
    let input_path = args
        .iter()
        .find(|&arg| arg != "--json" && arg != "--info")
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Please provide a path to a BinDiff file",
            )
        })?;

    let bd = BinDiff::open(&input_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let bd = guard(bd, |bd| {
        bd.close()
            .unwrap_or_else(|e| eprintln!("Error closing database: {}", e));
    });

    if info_output {
        // Read file
        let file = bd
            .read_file()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if json_output {
            stdoutln!("{}", serde_json::to_string_pretty(&file)?)?;
        } else {
            stdoutln!("{}", file)?;
        }

        // Read metadata
        let metadata = bd
            .read_metadata()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if json_output {
            stdoutln!("{}", serde_json::to_string_pretty(&metadata)?)?;
        } else {
            stdoutln!("{}", metadata)?;
        }
    } else {
        // Read function matches
        let func_matches = bd
            .read_function_matches()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if json_output {
            stdoutln!("{}", serde_json::to_string_pretty(&func_matches)?)?;
        } else {
            for func_match in func_matches {
                stdoutln!("{}", func_match)?;
            }
        }
    }

    Ok(())
}
