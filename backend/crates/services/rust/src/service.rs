use std::io::Write;
use tempfile::NamedTempFile;
use tokio::process::Command;
use tonic::{Request, Response, Status};

use crate::manifest::{self as manifest, Manifest};
use crate::verifier::{
    HealthRequest, HealthResponse, VerifyRequest, VerifyResponse, verifier_server::Verifier,
};

/// Generic Rust-based verifier service.
/// Loads verifier binary from ARTIFACTS_DIR based on manifest.yaml.
#[derive(Debug)]
pub struct RustVerifierService {
    artifacts_dir: String,
    manifest: Manifest,
}

impl RustVerifierService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let artifacts_dir =
            std::env::var("ARTIFACTS_DIR").unwrap_or_else(|_| "/artifacts".to_string());

        let manifest_path = format!("{}/manifest.yaml", artifacts_dir);
        let manifest = Manifest::load(&manifest_path)?;

        // Validate active version binary: exists, executable, checksum (always required)
        if let Some(version) = manifest.versions.iter().find(|v| v.active) {
            manifest::validate_binary(&artifacts_dir, version)?;
        }

        tracing::info!(
            prover = %manifest.prover,
            artifacts_dir = %artifacts_dir,
            "loaded and validated manifest"
        );

        Ok(Self {
            artifacts_dir,
            manifest,
        })
    }

    /// Get the artifacts directory
    pub fn artifacts_dir(&self) -> &str {
        &self.artifacts_dir
    }

    /// Get the manifest
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    fn get_active_version(&self) -> Option<&manifest::Version> {
        self.manifest.versions.iter().find(|v| v.active)
    }

    async fn execute_verifier(
        &self,
        proof: &[u8],
        public_inputs: &[String],
    ) -> Result<(bool, Option<String>), Status> {
        let version = self
            .get_active_version()
            .ok_or_else(|| Status::internal("No active version configured"))?;

        let interface = &version.interface;

        if interface.interface_type != "cli" {
            return Err(Status::unimplemented(format!(
                "Interface type '{}' not supported",
                interface.interface_type
            )));
        }

        // Create secure temp files with automatic cleanup
        let mut proof_file = NamedTempFile::new()
            .map_err(|e| Status::internal(format!("Failed to create temp file: {}", e)))?;

        let mut inputs_file = NamedTempFile::new()
            .map_err(|e| Status::internal(format!("Failed to create temp file: {}", e)))?;

        proof_file
            .write_all(proof)
            .map_err(|e| Status::internal(format!("Failed to write proof: {}", e)))?;

        let inputs_json = serde_json::to_vec(public_inputs)
            .map_err(|e| Status::internal(format!("Failed to serialize inputs: {}", e)))?;

        inputs_file
            .write_all(&inputs_json)
            .map_err(|e| Status::internal(format!("Failed to write inputs: {}", e)))?;

        // Build command
        let bin_path = format!("{}/{}", self.artifacts_dir, version.bin_path);
        let mut cmd = Command::new(&bin_path);

        if let Some(verify_cmd) = &interface.verify_command {
            cmd.arg(verify_cmd);
        }

        // Add configured arguments
        if let Some(args) = &interface.args {
            if let Some(proof_arg) = &args.proof {
                let arg =
                    proof_arg.replace("{proof_file}", proof_file.path().to_str().unwrap_or(""));
                for part in arg.split_whitespace() {
                    cmd.arg(part);
                }
            }
            if let Some(inputs_arg) = &args.public_inputs {
                let arg =
                    inputs_arg.replace("{inputs_file}", inputs_file.path().to_str().unwrap_or(""));
                for part in arg.split_whitespace() {
                    cmd.arg(part);
                }
            }
            if let Some(vk_arg) = &args.vk {
                if let Some(vk_path) = &version.vk_path {
                    let full_vk_path = format!("{}/{}", self.artifacts_dir, vk_path);
                    let arg = vk_arg.replace("{vk_file}", &full_vk_path);
                    for part in arg.split_whitespace() {
                        cmd.arg(part);
                    }
                }
            }
        }

        tracing::debug!(command = ?cmd, "executing verifier");

        // Execute asynchronously
        let output = cmd
            .output()
            .await
            .map_err(|e| Status::internal(format!("Failed to execute verifier: {}", e)))?;

        tracing::debug!(
            exit_code = ?output.status.code(),
            stdout_len = output.stdout.len(),
            stderr_len = output.stderr.len(),
            "verifier execution complete"
        );

        // Temp files are automatically cleaned up when they go out of scope

        // Parse result based on output_format
        let success_code = interface.success_exit_code.unwrap_or(0);

        match interface.output_format.as_deref() {
            Some("json") => {
                if let Ok(result) = serde_json::from_slice::<VerifierOutput>(&output.stdout) {
                    Ok((result.valid, result.error))
                } else {
                    let valid = output.status.code() == Some(success_code);
                    let error = if valid {
                        None
                    } else {
                        Some(String::from_utf8_lossy(&output.stderr).to_string())
                    };
                    Ok((valid, error))
                }
            }
            _ => {
                let valid = output.status.code() == Some(success_code);
                let error = if valid {
                    None
                } else {
                    Some(String::from_utf8_lossy(&output.stderr).to_string())
                };
                Ok((valid, error))
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct VerifierOutput {
    valid: bool,
    error: Option<String>,
}

#[tonic::async_trait]
impl Verifier for RustVerifierService {
    async fn verify(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyResponse>, Status> {
        let req = request.into_inner();

        let version = self
            .get_active_version()
            .map(|v| v.version.clone())
            .unwrap_or_else(|| "unknown".to_string());

        tracing::info!(
            prover = %self.manifest.prover,
            proof_system = %req.proof_system,
            proof_len = req.proof.len(),
            "verification request"
        );

        let (valid, error) = self
            .execute_verifier(&req.proof, &req.public_inputs)
            .await?;

        tracing::info!(
            prover = %self.manifest.prover,
            valid = valid,
            "verification complete"
        );

        Ok(Response::new(VerifyResponse {
            valid,
            prover_version: version,
            error,
        }))
    }

    async fn health(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let version = self
            .get_active_version()
            .map(|v| v.version.clone())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(Response::new(HealthResponse {
            healthy: true,
            version,
        }))
    }
}
