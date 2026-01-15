//! Manifest parser for verifier artifacts.
//!
//! Each verifier artifact directory contains a manifest.yaml that describes
//! available versions and how to invoke the verifier binary.

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("Failed to read manifest file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse manifest YAML: {0}")]
    ParseError(#[from] serde_yml::Error),

    #[error("Manifest validation failed: {0}")]
    ValidationError(String),

    #[error("Binary integrity check failed: {0}")]
    IntegrityError(String),
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub prover: String,
    pub description: Option<String>,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub version: String,
    pub active: bool,
    pub bin_path: String,
    /// SHA256 checksum of the binary (required for security)
    pub sha256: String,
    /// Path to verification key, relative to ARTIFACTS_DIR
    pub vk_path: Option<String>,
    pub interface: VerifierInterface,
}

#[derive(Debug, Deserialize)]
pub struct VerifierInterface {
    #[serde(rename = "type")]
    pub interface_type: String,
    pub verify_command: Option<String>,
    pub args: Option<InterfaceArgs>,
    pub success_exit_code: Option<i32>,
    pub output_format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InterfaceArgs {
    pub proof: Option<String>,
    pub public_inputs: Option<String>,
    pub vk: Option<String>,
    pub vk_path: Option<String>,
}

impl Manifest {
    pub fn load(path: &str) -> Result<Self, ManifestError> {
        let content = std::fs::read_to_string(path)?;
        let manifest: Manifest = serde_yml::from_str(&content)?;
        manifest.validate()?;
        Ok(manifest)
    }

    fn validate(&self) -> Result<(), ManifestError> {
        if self.prover.is_empty() {
            return Err(ManifestError::ValidationError(
                "prover name cannot be empty".to_string(),
            ));
        }

        if self.versions.is_empty() {
            return Err(ManifestError::ValidationError(
                "at least one version must be defined".to_string(),
            ));
        }

        let active_count = self.versions.iter().filter(|v| v.active).count();
        if active_count == 0 {
            return Err(ManifestError::ValidationError(
                "at least one version must be active".to_string(),
            ));
        }
        if active_count > 1 {
            return Err(ManifestError::ValidationError(
                "only one version can be active at a time".to_string(),
            ));
        }

        for version in &self.versions {
            version.validate()?;
        }

        Ok(())
    }
}

impl Version {
    fn validate(&self) -> Result<(), ManifestError> {
        if self.version.is_empty() {
            return Err(ManifestError::ValidationError(
                "version string cannot be empty".to_string(),
            ));
        }

        if self.bin_path.is_empty() {
            return Err(ManifestError::ValidationError(format!(
                "bin_path cannot be empty for version {}",
                self.version
            )));
        }

        self.interface.validate(&self.version)?;

        Ok(())
    }
}

impl VerifierInterface {
    fn validate(&self, version: &str) -> Result<(), ManifestError> {
        let valid_types = ["cli"];
        if !valid_types.contains(&self.interface_type.as_str()) {
            return Err(ManifestError::ValidationError(format!(
                "unsupported interface type '{}' for version {} (supported: {:?})",
                self.interface_type, version, valid_types
            )));
        }

        let valid_formats = ["json", "exit_code_only"];
        if let Some(format) = &self.output_format {
            if !valid_formats.contains(&format.as_str()) {
                return Err(ManifestError::ValidationError(format!(
                    "unsupported output_format '{}' for version {} (supported: {:?})",
                    format, version, valid_formats
                )));
            }
        }

        Ok(())
    }
}

/// Validate that the binary exists, is executable, and matches checksum
pub fn validate_binary(artifacts_dir: &str, version: &Version) -> Result<(), ManifestError> {
    let full_path = Path::new(artifacts_dir).join(&version.bin_path);

    // Check file exists
    if !full_path.exists() {
        return Err(ManifestError::ValidationError(format!(
            "verifier binary not found at {}",
            full_path.display()
        )));
    }

    // Check it's a file
    if !full_path.is_file() {
        return Err(ManifestError::ValidationError(format!(
            "verifier binary path is not a file: {}",
            full_path.display()
        )));
    }

    // Check it's executable (Unix)
    let metadata = std::fs::metadata(&full_path)?;
    let permissions = metadata.permissions();
    if permissions.mode() & 0o111 == 0 {
        return Err(ManifestError::ValidationError(format!(
            "verifier binary is not executable: {}",
            full_path.display()
        )));
    }

    verify_binary_checksum(&full_path, &version.sha256)?;

    tracing::info!(
        version = %version.version,
        sha256 = %version.sha256,
        "binary integrity verified"
    );

    Ok(())
}

/// Compute SHA256 hash of a file and compare with expected value
fn verify_binary_checksum(path: &Path, expected_hex: &str) -> Result<(), ManifestError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let computed_hash = hex::encode(hasher.finalize());

    if computed_hash.eq_ignore_ascii_case(expected_hex) {
        Ok(())
    } else {
        Err(ManifestError::IntegrityError(format!(
            "checksum mismatch for {}: expected {}, got {}",
            path.display(),
            expected_hex,
            computed_hash
        )))
    }
}

/// Compute SHA256 hash of a binary file (utility for generating checksums)
pub fn compute_binary_checksum(path: &Path) -> Result<String, ManifestError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hex::encode(hasher.finalize()))
}
