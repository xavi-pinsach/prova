pub mod verifier {
    tonic::include_proto!("prova.verifier.v1");
}

mod manifest;
mod service;

use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use verifier::verifier_server::VerifierServer;

use service::RustVerifierService;

/// Entry point for the generic Rust-based verifier service.
/// Loads verifier binary from ARTIFACTS_DIR based on manifest.yaml.
///
/// Environment variables:
/// - ARTIFACTS_DIR: Path to artifacts directory (required)
/// - GRPC_PORT: Port to listen on (default: 50051)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create the service
    let service = RustVerifierService::new()?;
    let prover_name = service.manifest().prover.clone();

    let port = std::env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;

    tracing::info!(
        %addr,
        prover = %prover_name,
        "rust verifier service starting"
    );

    // Setup gRPC reflection
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/verifier_descriptor.bin"
        )))
        .build_v1()?;

    // Start the server
    Server::builder()
        .add_service(reflection_service)
        .add_service(VerifierServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
