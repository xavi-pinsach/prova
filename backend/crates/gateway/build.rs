fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In Docker: ./proto, locally: ../../contracts/protobuf/v1
    let proto_path = if std::path::Path::new("./proto/verifier.proto").exists() {
        "./proto"
    } else {
        "../../contracts/protobuf/v1"
    };

    tonic_prost_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(
            &[format!("{}/verifier.proto", proto_path)],
            &[proto_path.to_string()],
        )?;
    Ok(())
}
