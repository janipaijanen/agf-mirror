use agov_core::Finding;
use regex::Regex;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
struct Registry {
    gates: Vec<Gate>,
}

#[derive(Debug, Deserialize)]
struct Gate {
    id: String,
    status: GateStatus,
    surfaces: Vec<String>,
    tests: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum GateStatus {
    Active,
    Retired,
}

/// Enforce test accompaniment, active-path resolution, and permanent gate IDs.
///
/// This deliberately makes no claim about CI job reachability or test quality.
pub fn change_check(
    repo_root: &Path,
    base_ref: &str,
    registry_path: &Path,
) -> Result<(), Vec<Finding>> {
    let registry_relative = repository_relative(repo_root, registry_path)?;
    let current_path = repo_root.join(&registry_relative);
    let data = crate::read_yaml_file(&current_path)?;
    let mut findings = crate::validate_schema(&data, "gate-registry.schema.json");
    if !findings.is_empty() {
        return Err(findings);
    }

    let current: Registry = match serde_json::from_value(data) {
        Ok(registry) => registry,
        Err(error) => {
            return Err(vec![Finding::blocking_at(
                format!("registry decode failed after schema validation: {error}"),
                registry_relative.display().to_string(),
            )]);
        }
    };
    let current_by_id = collect_unique(&current, "current", &mut findings);

    git(repo_root, &["rev-parse", "--verify", base_ref]).map_err(|message| {
        vec![Finding::blocking(format!(
            "cannot resolve comparison base '{base_ref}': {message}"
        ))]
    })?;

    let base = load_base_registry(repo_root, base_ref, &registry_relative, &mut findings);
    let base_by_id = collect_unique(&base, "base", &mut findings);
    for id in base_by_id.keys() {
        if !current_by_id.contains_key(id) {
            findings.push(Finding::blocking_at(
                format!("gate entry '{id}' was deleted; gate IDs are permanent, including retired entries"),
                registry_relative.display().to_string(),
            ));
        }
    }

    let tracked = match repository_paths(repo_root) {
        Ok(paths) => paths,
        Err(message) => {
            findings.push(Finding::blocking(format!(
                "cannot enumerate repository paths: {message}"
            )));
            BTreeSet::new()
        }
    };
    let changed = match changed_paths(repo_root, base_ref) {
        Ok(paths) => paths,
        Err(message) => {
            findings.push(Finding::blocking(format!(
                "cannot enumerate paths changed from '{base_ref}': {message}"
            )));
            BTreeSet::new()
        }
    };

    for gate in current_by_id
        .values()
        .filter(|gate| gate.status == GateStatus::Active)
    {
        let surface_patterns =
            compile_patterns(&gate.id, "surfaces", &gate.surfaces, &mut findings);
        let test_patterns = compile_patterns(&gate.id, "tests", &gate.tests, &mut findings);

        for (kind, patterns) in [("surface", &surface_patterns), ("test", &test_patterns)] {
            for (source, pattern) in patterns {
                if !tracked.iter().any(|path| pattern.is_match(path)) {
                    findings.push(Finding::blocking_at(
                        format!(
                            "active gate '{}' {kind} path '{source}' does not resolve",
                            gate.id
                        ),
                        format!(
                            "gates.{}.{}",
                            gate.id,
                            if kind == "surface" {
                                "surfaces"
                            } else {
                                "tests"
                            }
                        ),
                    ));
                }
            }
        }

        let surface_changed = surface_patterns
            .iter()
            .any(|(_, pattern)| changed.iter().any(|path| pattern.is_match(path)));
        let test_changed = test_patterns
            .iter()
            .any(|(_, pattern)| changed.iter().any(|path| pattern.is_match(path)));
        if surface_changed && !test_changed {
            findings.push(Finding::blocking_at(
                format!(
                    "active gate '{}' has a changed surface but no changed declared test path",
                    gate.id
                ),
                format!("gates.{}.tests", gate.id),
            ));
        }
    }

    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn repository_relative(repo_root: &Path, path: &Path) -> Result<PathBuf, Vec<Finding>> {
    let relative = if path.is_absolute() {
        path.strip_prefix(repo_root)
            .map(Path::to_path_buf)
            .map_err(|_| {
                vec![Finding::blocking(format!(
                    "registry path '{}' is outside repository root '{}'",
                    path.display(),
                    repo_root.display()
                ))]
            })?
    } else {
        path.to_path_buf()
    };
    if relative.components().any(|part| {
        matches!(
            part,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(vec![Finding::blocking(format!(
            "registry path '{}' must stay within the repository",
            path.display()
        ))]);
    }
    Ok(relative)
}

fn collect_unique<'a>(
    registry: &'a Registry,
    label: &str,
    findings: &mut Vec<Finding>,
) -> BTreeMap<&'a str, &'a Gate> {
    let mut by_id = BTreeMap::new();
    for gate in &registry.gates {
        if by_id.insert(gate.id.as_str(), gate).is_some() {
            findings.push(Finding::blocking_at(
                format!("duplicate gate id '{}' in {label} registry", gate.id),
                format!("gates.{}.id", gate.id),
            ));
        }
    }
    by_id
}

fn load_base_registry(
    repo_root: &Path,
    base_ref: &str,
    registry_path: &Path,
    findings: &mut Vec<Finding>,
) -> Registry {
    let object = format!("{base_ref}:{}", registry_path.to_string_lossy());
    let exists = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["cat-file", "-e", &object])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if !exists {
        return Registry { gates: Vec::new() };
    }
    match git(repo_root, &["show", &object]).and_then(|text| {
        serde_norway::from_str::<Registry>(&text).map_err(|error| error.to_string())
    }) {
        Ok(registry) => registry,
        Err(message) => {
            findings.push(Finding::blocking_at(
                format!("cannot read base registry: {message}"),
                object,
            ));
            Registry { gates: Vec::new() }
        }
    }
}

fn repository_paths(repo_root: &Path) -> Result<BTreeSet<String>, String> {
    let output = git_bytes(
        repo_root,
        &[
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ],
    )?;
    Ok(nul_paths(&output))
}

fn changed_paths(repo_root: &Path, base_ref: &str) -> Result<BTreeSet<String>, String> {
    let range = format!("{base_ref}...HEAD");
    let output = git_bytes(
        repo_root,
        &[
            "diff",
            "--name-only",
            "-z",
            "--diff-filter=ACDMRTUXB",
            &range,
            "--",
        ],
    )?;
    Ok(nul_paths(&output))
}

fn nul_paths(output: &[u8]) -> BTreeSet<String> {
    output
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
        .map(|part| String::from_utf8_lossy(part).replace('\\', "/"))
        .collect()
}

fn compile_patterns(
    gate_id: &str,
    field: &str,
    sources: &[String],
    findings: &mut Vec<Finding>,
) -> Vec<(String, Regex)> {
    sources
        .iter()
        .filter_map(|source| match glob_regex(source) {
            Ok(pattern) => Some((source.clone(), pattern)),
            Err(message) => {
                findings.push(Finding::blocking_at(
                    format!("gate '{gate_id}' has invalid {field} path '{source}': {message}"),
                    format!("gates.{gate_id}.{field}"),
                ));
                None
            }
        })
        .collect()
}

fn glob_regex(source: &str) -> Result<Regex, String> {
    if source.is_empty() || source.starts_with('/') || source.split('/').any(|part| part == "..") {
        return Err(
            "path must be non-empty, repository-relative, and contain no '..' segment".into(),
        );
    }
    let mut expression = String::from("^");
    let chars: Vec<char> = source.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        match chars[index] {
            '*' if chars.get(index + 1) == Some(&'*') => {
                index += 1;
                if chars.get(index + 1) == Some(&'/') {
                    index += 1;
                    expression.push_str("(?:.*/)?");
                } else {
                    expression.push_str(".*");
                }
            }
            '*' => expression.push_str("[^/]*"),
            '?' => expression.push_str("[^/]"),
            character => expression.push_str(&regex::escape(&character.to_string())),
        }
        index += 1;
    }
    expression.push('$');
    Regex::new(&expression).map_err(|error| error.to_string())
}

fn git(repo_root: &Path, args: &[&str]) -> Result<String, String> {
    String::from_utf8(git_bytes(repo_root, args)?).map_err(|error| error.to_string())
}

fn git_bytes(repo_root: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::glob_regex;

    #[test]
    fn repository_globs_match_expected_paths() {
        let recursive = glob_regex("tests/fixtures/**").unwrap();
        assert!(recursive.is_match("tests/fixtures/a/b.yaml"));
        assert!(recursive.is_match("tests/fixtures/file.yaml"));
        assert!(!recursive.is_match("tests/other/file.yaml"));

        let rust = glob_regex("tooling/*/src/?.rs").unwrap();
        assert!(rust.is_match("tooling/agov/src/a.rs"));
        assert!(!rust.is_match("tooling/agov/src/main.rs"));
    }
}
