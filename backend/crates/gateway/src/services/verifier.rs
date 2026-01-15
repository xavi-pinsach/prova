pub mod proto {
    tonic::include_proto!("prova.verifier.v1");
}

use crate::error::ApiError;
use proto::verifier_client::VerifierClient as GrpcClient;
use proto::{HealthRequest, VerifyRequest as GrpcVerifyRequest};
use tonic::transport::Channel;

#[derive(Clone)]
pub struct VerifierClient {
    zisk_client: GrpcClient<Channel>,
    zisk_url: String,
}

#[derive(Debug)]
pub struct VerifyResponse {
    pub valid: bool,
    pub prover_version: String,
    pub error: Option<String>,
}

impl VerifierClient {
    pub async fn new(zisk_url: String) -> Result<Self, ApiError> {
        let zisk_channel = Channel::from_shared(zisk_url.clone())
            .map_err(|e| ApiError::VerifierService(format!("Invalid zisk URL: {}", e)))?
            .connect_lazy();

        Ok(Self {
            zisk_client: GrpcClient::new(zisk_channel),
            zisk_url,
        })
    }

    /// Check if the verifier service is healthy.
    /// Returns Ok(version) if healthy, Err if not reachable.
    pub async fn health_check(&self) -> Result<String, ApiError> {
        let mut client = self.zisk_client.clone();

        let response = client
            .health(tonic::Request::new(HealthRequest {}))
            .await
            .map_err(|e| {
                ApiError::VerifierService(format!(
                    "Verifier health check failed ({}): {}",
                    self.zisk_url, e
                ))
            })?;

        let inner = response.into_inner();
        if inner.healthy {
            Ok(inner.version)
        } else {
            Err(ApiError::VerifierService(
                "Verifier reported unhealthy status".to_string(),
            ))
        }
    }

    pub async fn verify_zisk(
        &self,
        proof: Vec<u8>,
        public_inputs: Option<Vec<String>>,
    ) -> Result<VerifyResponse, ApiError> {
        let mut client = self.zisk_client.clone();

        let request = tonic::Request::new(GrpcVerifyRequest {
            proof,
            public_inputs: public_inputs.unwrap_or_default(),
            proof_system: "zisk".to_string(),
        });

        let response = client
            .verify(request)
            .await
            .map_err(|e| ApiError::VerifierService(format!("gRPC error: {}", e)))?;

        let inner = response.into_inner();
        Ok(VerifyResponse {
            valid: inner.valid,
            prover_version: inner.prover_version,
            error: inner.error,
        })
    }
}
