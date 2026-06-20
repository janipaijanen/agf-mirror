use crate::{path_label, yaml_files, CollectionResult, FileError};
use agov_core::Finding;
use std::path::Path;

/// Scan learning record files in `dir` for InfoSec findings.
///
/// Mirrors `Agov.ValidateInfosec.validate_all/1`. Scans freetext fields
/// (description, text, summary, recommendation) for secret patterns.
pub fn validate_all(dir: &Path) -> CollectionResult {
    let mut ok_files = Vec::new();
    let mut file_errors = Vec::new();

    for file in yaml_files(dir) {
        match validate_file(&file) {
            Ok(()) => ok_files.push(path_label(&file)),
            Err(findings) => file_errors.push(FileError {
                file: path_label(&file),
                findings,
            }),
        }
    }

    CollectionResult {
        ok_files,
        file_errors,
    }
}

fn validate_file(path: &Path) -> Result<(), Vec<Finding>> {
    let label = path_label(path);
    let content = std::fs::read_to_string(path).map_err(|err| {
        vec![Finding::blocking_at(
            format!("file read error: {err:?}"),
            label.clone(),
        )]
    })?;

    let findings: Vec<Finding> = agov_core::infosec::scan_content(&content)
        .into_iter()
        .map(|hit| Finding::blocking_at(format!("{} detected", hit.pattern_name), label.clone()))
        .collect();

    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}
