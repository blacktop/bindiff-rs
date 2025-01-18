fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        .default_package_filename("binexport")
        .compile_protos(&["src/binexport/binexport2.proto"], &["src/binexport/"])?;
    Ok(())
}