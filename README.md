<p align="center">
  <a href="https://github.com/blacktop/bindiff-rs"><img alt="Logo" src="https://github.com/blacktop/bindiff-rs/raw/main/logo.png" height="400" /></a>
  <h1 align="center">bindiff-rs</h1>
  <h4><p align="center">Rust library to interface with BinDiff</p></h4>
  <p align="center">
    <a href="https://github.com/blacktop/bindiff-rs/actions" alt="Actions">
          <img src="https://github.com/blacktop/bindiff-rs/actions/workflows/rust.yml/badge.svg" /></a>
    <a href="https://crates.io/crates/bindiff-rs" alt="Downloads">
          <img src="https://img.shields.io/crates/d/bindiff-rs" /></a>
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

## Install `bindiff-rs` binary

Via [homebrew](https://brew.sh)

```sh
brew install blacktop/tap/bindiff-rs
```

Via `cargo`

```sh
cargo install bindiff-rs
```

## Run `bindiff-rs`

Dump the BinDiff file metadata

```bash
❯ bindiff-rs path/to/BinDiff --info
```
```bash
FILE:
  id:               1
  filename:         kernel.release
  exe_filename:     kernel.release.t6020
  hash:             8a9f5d9305cec0a85bbaae3288ef3a5732bda406edcf20aa6edd9352fb555de0
  functions:        22140
  lib_functions:    0
  calls:            229914
  basic_blocks:     1037258
  lib_basic_blocks: 0
  edges:            1546378
  lib_edges:        0
  instructions:     4958934
  lib_instructions: 0

METADATA:
  version:      BinDiff 8 (@568181968, Sep 25 2023)
  file1:        1
  file2:        2
  description:
  created:      2025-01-12 23:51:57
  modified:     2025-01-12 23:51:57
  similarity:   0.99
  confidence:   0.99
```

Dump the BinDiff file function matches

```bash
❯ bindiff-rs path/to/BinDiff
```

```bash
memset_s:       similarity: 1.00, confidence: 0.99
timingsafe_bcmp:        similarity: 1.00, confidence: 0.99
cc_clear:       similarity: 1.00, confidence: 0.99
cc_disable_dit: similarity: 1.00, confidence: 0.97
ccdigest_init:  similarity: 1.00, confidence: 0.99
ccdigest_update:        similarity: 1.00, confidence: 0.99
cchmac: similarity: 1.00, confidence: 0.99
cchmac_init:    similarity: 1.00, confidence: 0.99
cchmac_update:  similarity: 1.00, confidence: 0.99
cchmac_final:   similarity: 1.00, confidence: 0.99
ccdigest_final_64be:    similarity: 1.00, confidence: 0.99
_ovbcopy:       similarity: 1.00, confidence: 0.99
_memmove:       similarity: 1.00, confidence: 0.99
_bzero: similarity: 1.00, confidence: 0.99
_memset:        similarity: 1.00, confidence: 0.99
<SNIP>
```

Dump the BinDiff file function matches in JSON format

```bash
❯ bindiff-rs path/to/BinDiff --json
```

```json
[
  {
    "id": 1,
    "address1": -2198902980608,
    "name1": "memset_s",
    "address2": -2198902980608,
    "name2": "memset_s",
    "similarity": 1.0,
    "confidence": 0.9933071490757153,
    "flags": 0,
    "algorithm": "NameHashMatching",
    "evaluate": false,
    "comment_supported": false,
    "basic_blocks": 6,
    "edges": 7,
    "instructions": 21
  },
  <SNIP>
]
```

## License

MIT Copyright (c) 2025 **blacktop**
