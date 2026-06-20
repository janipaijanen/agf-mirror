pub mod infosec;
pub mod output;
pub mod roi;
pub mod schema;
pub mod ulid_util;
pub mod yaml;

use std::path::{Path, PathBuf};

// ── Shared finding types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Blocking,
    Advisory,
}

/// A single validation finding.
///
/// `location` is a JSON-pointer string (e.g. `"#/advisory_consultation"`) for
/// schema findings, or a human-readable path token for semantic findings.
#[derive(Debug, Clone)]
pub struct Finding {
    pub severity: Severity,
    pub message: String,
    pub location: Option<String>,
}

impl Finding {
    pub fn blocking(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Blocking,
            message: message.into(),
            location: None,
        }
    }

    pub fn blocking_at(message: impl Into<String>, location: impl Into<String>) -> Self {
        Self {
            severity: Severity::Blocking,
            message: message.into(),
            location: Some(location.into()),
        }
    }

    pub fn advisory(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Advisory,
            message: message.into(),
            location: None,
        }
    }
}

/// Find the repository root by walking upward from the current directory.
pub fn repo_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    find_repo_root_from(&cwd).unwrap_or(cwd)
}

pub fn find_repo_root_from(start: &Path) -> Option<PathBuf> {
    for dir in start.ancestors() {
        if dir.join("agf").join("schemas").is_dir() && dir.join("tooling").is_dir() {
            return Some(dir.to_path_buf());
        }
    }
    None
}

/// Resolve the AGF schema directory, honoring `AGOV_SCHEMA_DIR`.
pub fn schema_dir() -> PathBuf {
    std::env::var_os("AGOV_SCHEMA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("agf").join("schemas"))
}

/// Resolve the AGF agent manifest path, honoring `AGOV_MANIFEST_PATH`.
pub fn manifest_path() -> PathBuf {
    std::env::var_os("AGOV_MANIFEST_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("agf").join("agents").join("MANIFEST.yaml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_repo_root_from_nested_path() {
        let root = repo_root();
        let nested = root.join("tooling").join("rust");
        assert_eq!(find_repo_root_from(&nested), Some(root));
    }
}
