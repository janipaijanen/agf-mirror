pub mod adoption;
pub mod agents;
pub mod contributions;
pub mod framework_config;
pub mod infosec;
pub mod learnings;

use agov_core::Finding;

/// Aggregate result from a collection validator (agents, learnings, etc.).
///
/// Mirrors the `{:ok, files}` / `{:error, file_errors}` shape from
/// `Agov.ValidateAgents.validate_all/1` and siblings.
pub struct CollectionResult {
    pub ok_files: Vec<String>,
    pub file_errors: Vec<FileError>,
}

pub struct FileError {
    pub file: String,
    pub findings: Vec<Finding>,
}

impl CollectionResult {
    pub fn is_ok(&self) -> bool {
        self.file_errors.is_empty()
    }
}
