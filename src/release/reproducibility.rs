// Release artifact manifest generation for deterministic builds.

use serde::{Deserialize, Serialize};
use std::io;
use std::process::Command;

/// Release manifest with build metadata for reproducibility verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub version: String,
    pub git_tag: String,
    pub git_commit: String,
    pub rust_toolchain: String,
    pub target: String,
    pub build_timestamp: String,
    pub binary_sha256: String,
}

impl ReleaseManifest {
    /// Generate a manifest from the current environment.
    pub fn generate(
        version: &str,
        target: &str,
        binary_sha256: &str,
    ) -> Result<Self, io::Error> {
        let git_tag = run_cmd("git", &["describe", "--tags", "--always"])
            .unwrap_or_else(|_| "unknown".to_string());
        let git_commit = run_cmd("git", &["rev-parse", "HEAD"])
            .unwrap_or_else(|_| "unknown".to_string());
        let rust_toolchain = run_cmd("rustc", &["--version"])
            .unwrap_or_else(|_| "unknown".to_string());

        Ok(Self {
            version: version.to_string(),
            git_tag: git_tag.trim().to_string(),
            git_commit: git_commit.trim().to_string(),
            rust_toolchain: rust_toolchain.trim().to_string(),
            target: target.to_string(),
            build_timestamp: chrono::Utc::now().to_rfc3339(),
            binary_sha256: binary_sha256.to_string(),
        })
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

fn run_cmd(program: &str, args: &[&str]) -> Result<String, io::Error> {
    let output = Command::new(program).args(args).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
