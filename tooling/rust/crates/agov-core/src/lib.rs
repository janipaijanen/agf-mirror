pub mod infosec;
pub mod output;
pub mod roi;
pub mod schema;
pub mod ulid_util;
pub mod yaml;

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
        Self { severity: Severity::Blocking, message: message.into(), location: None }
    }

    pub fn blocking_at(message: impl Into<String>, location: impl Into<String>) -> Self {
        Self {
            severity: Severity::Blocking,
            message: message.into(),
            location: Some(location.into()),
        }
    }

    pub fn advisory(message: impl Into<String>) -> Self {
        Self { severity: Severity::Advisory, message: message.into(), location: None }
    }
}
