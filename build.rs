fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        .default_package_filename("binexport")
        .compile_protos(&["binexport/binexport2.proto"], &["binexport/"])?;
    Ok(())
}