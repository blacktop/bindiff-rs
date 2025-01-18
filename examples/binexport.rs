use bindiff_rs::BinExport;

fn main() {
    let binexport = BinExport::open("tests/kernel.release.t6020.BinExport").unwrap();
    for function_match in binexport. {
        println!("{:#?}", function_match);
    }
}
