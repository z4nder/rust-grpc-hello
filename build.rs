// Compile the proto file to Rust code
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
