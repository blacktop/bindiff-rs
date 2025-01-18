fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        // .protoc_arg("--experimental_allow_proto3_optional")
        // .protoc_arg("--rust_opt=experimental-codegen=enabled,kernel=upb")
        .compile_protos(&["src/binexport2.proto"], &["src/"])?;
    Ok(())
}