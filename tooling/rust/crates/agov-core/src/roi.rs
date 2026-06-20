use std::path::PathBuf;

/// A saved-token ROI record appended to `tmp/agov-token-roi.jsonl`.
///
/// Fields mirror the Elixir `Agov.TokenRoi` JSONL format (§8.2 of the plan).
/// Unknown workflows may omit the `saved_*` fields.
#[derive(Debug, serde::Serialize)]
pub struct RoiRecord {
    pub event: &'static str,
    pub timestamp: String,
    pub implementation: &'static str,
    pub command: String,
    pub args: Vec<String>,
    pub workflow: String,
    pub chars: u64,
    pub approx_tokens: u64,
    pub prompt_overhead_tokens: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approx_saved_min_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approx_saved_max_tokens: Option<u64>,
}

/// Resolve the ROI log path from environment or default.
///
/// Returns `None` when logging is disabled via `AGOV_TOKEN_ROI=0/false/off`.
pub fn log_path() -> Option<PathBuf> {
    todo!()
}

/// Append `record` to the ROI log as a JSONL line.
///
/// Failures are silently swallowed — logging must not affect stdout, stderr,
/// output ordering, or exit codes (plan §8.4).
pub fn try_append(record: &RoiRecord) {
    todo!()
}
