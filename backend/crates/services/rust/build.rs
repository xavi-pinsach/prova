fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In Docker: ./proto, locally: ../../../contracts/protobuf/v1
    let proto_path = if std::path::Path::new("./proto/verifier.proto").exists() {
        "./proto"
    } else {
        "../../../contracts/protobuf/v1"
    };

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("verifier_descriptor.bin"))
        .compile_protos(
            &[format!("{}/verifier.proto", proto_path)],
            &[proto_path.to_string()],
        )?;
    Ok(())
}
