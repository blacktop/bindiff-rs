# bindiff-rs
<p align="center">
  <a href="https://github.com/blacktop/bindiff-rs"><img alt="Logo" src="https://github.com/blacktop/bindiff-rs/raw/main/logo.png" height="300" /></a>
  <h1 align="center">bindiff-rs</h1>
  <h4><p align="center">Rust library to interface with BinDiff</p></h4>
  <p align="center">
    <a href="https://github.com/blacktop/bindiff-rs/actions" alt="Actions">
          <img src="https://github.com/blacktop/bindiff-rs/actions/workflows/rust.yml/badge.svg" /></a>
    <a href="https://crates.io/crates/bindiff-rs" alt="Docs">
          <img src="https://img.shields.io/crates/v/bindiff-rs" /></a>
    <a href="http://doge.mit-license.org" alt="LICENSE">
          <img src="https://img.shields.io/:license-mit-blue.svg" /></a>
</p>
<br>

## Usage

```rust
use bindiff::BinDiff;

fn main() -> anyhow::Result<()> {
    let input_path = std::env::args().nth(1)
        .ok_or_else(|| anyhow::anyhow!("Please provide a path to a BinDiff file"))?;
    let bd = BinDiff::open(&input_path)?;

    // Read function matches
    let func_matches = bd.read_function_matches()?;
    for func_match in func_matches {
        println!("{:#?}", func_match);
    }

    Ok(())
}
```

## License

MIT Copyright (c) 2025 **blacktop**