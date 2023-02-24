fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/jkc_config.proto")?;
    tonic_build::compile_protos("proto/jkc_api.proto")?;

    Ok(())
}