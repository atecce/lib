//! The API's types from proto/equities/v1/equities.proto: prost's messages,
//! and pbjson's serde for their JSON form (the Connect protocol's
//! application/json). protox compiles the file in Rust, so the build needs
//! no protoc.

use prost::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto = "proto/equities/v1/equities.proto";
    println!("cargo:rerun-if-changed={}", proto);
    let descriptors = protox::compile([proto], ["proto"])?;
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    prost_build::Config::new().compile_fds(descriptors.clone())?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptors.encode_to_vec())?
        .out_dir(&out)
        .build(&[".equities.v1"])?;
    Ok(())
}
