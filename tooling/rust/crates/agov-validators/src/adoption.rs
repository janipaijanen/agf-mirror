use agov_core::Finding;
use std::path::Path;

/// Validate the adoption structure under `root`.
///
/// Mirrors `Agov.ValidateAdoption.validate/1`. Checks:
///   - `agents/` directory exists
///   - `agents/` contains at least one YAML file
///   - mandatory agents (infosec-sentinel, contribution-auditor) are present
///   - `agents/MANIFEST.yaml` exists
///   - `GOVERNANCE.md` or `framework.md` exists
pub fn validate(root: &Path) -> Result<Vec<Finding>, Vec<Finding>> {
    todo!()
}
