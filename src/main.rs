use bindiff_rs::BinDiff;
fn main() -> anyhow::Result<()> {
    let input_path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Please provide a path to a BinDiff file"))?;
    let bd = BinDiff::open(&input_path)?;

    // Read file
    let file = bd.read_file()?;
    println!("{}", file);

    // Read metadata
    let metadata = bd.read_metadata()?;
    println!("{}", metadata);

    // Read function matches
    let func_matches = bd.read_function_matches()?;
    for func_match in func_matches {
        println!("{}", func_match);
    }

    // // Read basic blocks
    // let basic_block_matches = bd.read_basic_block_matches()?;
    // for basic_block_match in basic_block_matches {
    //     println!("{:#?}", basic_block_match);
    // }

    // // Read instruction matches
    // let instruction_matches = bd.read_instruction_matches()?;
    // for instruction_match in instruction_matches {
    //     println!("{:#?}", instruction_match);
    // }

    bd.close()?;

    Ok(())
}
