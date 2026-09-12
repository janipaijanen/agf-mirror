pub mod artifact_config;
pub mod artifact_files;
pub mod artifact_index;
pub mod adopter_docs;
pub mod adoption;
pub mod agents;
pub mod assistant_drift;
pub mod assistant_loader;
pub mod ci_rules;
pub mod contributions;
pub mod framework_config;
pub mod gate_change;
pub mod infosec;
pub mod learnings;
pub mod readme_stats;

use agov_core::Finding;
use std::path::{Path, PathBuf};

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

pub(crate) fn yaml_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|ext| ext == "yaml" || ext == "yml")
        })
        .collect();
    files.sort();
    files
}

pub(crate) fn read_yaml_file(path: &Path) -> Result<serde_json::Value, Vec<Finding>> {
    agov_core::yaml::read_yaml_file_json(path).map_err(|err| {
        vec![Finding::blocking_at(
            match err {
                agov_core::yaml::YamlError::Io(message) if message == "file not found" => {
                    "file not found".to_string()
                }
                other => format!("YAML parse/read error: {other:?}"),
            },
            path.display().to_string(),
        )]
    })
}

pub(crate) fn validate_schema(data: &serde_json::Value, schema_name: &str) -> Vec<Finding> {
    let schema_path = agov_core::schema_dir().join(schema_name);
    match agov_core::schema::load_and_compile(&schema_path) {
        Ok(compiled) => agov_core::schema::validate(&compiled, data)
            .into_iter()
            .map(|finding| Finding::blocking_at(finding.message, finding.pointer))
            .collect(),
        Err(err) => vec![Finding::blocking_at(
            format!("schema load error: {err:?}"),
            schema_path.display().to_string(),
        )],
    }
}

pub(crate) fn freetext(data: &serde_json::Value) -> Vec<String> {
    let mut values = Vec::new();
    collect_freetext(data, &mut values);
    values
}

fn collect_freetext(data: &serde_json::Value, values: &mut Vec<String>) {
    match data {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if matches!(
                    key.as_str(),
                    "description" | "text" | "summary" | "recommendation"
                ) && value.is_string()
                {
                    values.push(value.as_str().unwrap_or_default().to_string());
                } else if value.is_object() || value.is_array() {
                    collect_freetext(value, values);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_freetext(item, values);
            }
        }
        _ => {}
    }
}

pub(crate) fn split_results(
    results: Vec<(PathBuf, Result<serde_json::Value, Vec<Finding>>)>,
) -> CollectionResult {
    let mut ok_files = Vec::new();
    let mut file_errors = Vec::new();

    for (file, result) in results {
        match result {
            Ok(_data) => ok_files.push(path_label(&file)),
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

pub(crate) fn path_label(path: &Path) -> String {
    std::env::current_dir()
        .ok()
        .and_then(|cwd| path.strip_prefix(cwd).ok().map(Path::to_path_buf))
        .unwrap_or_else(|| path.to_path_buf())
        .display()
        .to_string()
}
