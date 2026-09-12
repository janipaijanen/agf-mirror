//! Deterministic, project-schema-owned artifact inventory and structural graph.
//! This module does not decide governance gates, mint IDs, or inspect branches.
use crate::artifact_config::ArtifactConfig;
use crate::artifact_files::RepositoryFiles;
use agov_core::schema;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Coverage {
    Complete,
    Incomplete,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Problem {
    pub code: String,
    pub path: String,
    pub pointer: String,
    pub message: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Entry {
    pub id: String,
    pub kind: String,
    pub key: Option<String>,
    pub version: Option<Value>,
    pub set: String,
    pub path: String,
    pub pointer: String,
    pub digest: String,
    pub status: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub dependency: bool,
    pub path: String,
    pub pointer: String,
}
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Index {
    pub schema_version: u8,
    pub coverage: Coverage,
    pub files: Vec<String>,
    pub entries: Vec<Entry>,
    pub edges: Vec<Edge>,
    pub problems: Vec<Problem>,
}
impl Index {
    fn new() -> Self {
        Self {
            schema_version: 1,
            coverage: Coverage::Complete,
            files: vec![],
            entries: vec![],
            edges: vec![],
            problems: vec![],
        }
    }
    pub fn exit_status(&self) -> i32 {
        if self.coverage == Coverage::Incomplete {
            2
        } else if self.problems.is_empty() {
            0
        } else {
            1
        }
    }
    pub fn require_selected_paths(&mut self, paths: &[String]) {
        for path in paths {
            if !crate::artifact_config::contained_relative_path(path) || !self.files.contains(path)
            {
                self.problem(
                    "unregistered-path",
                    path,
                    "",
                    "requested path is not a member of the configured inventory".into(),
                );
            }
        }
        self.finish();
    }
    fn problem(&mut self, code: &str, path: &str, pointer: &str, message: String) {
        self.problems.push(Problem {
            code: code.into(),
            path: path.into(),
            pointer: pointer.into(),
            message,
        });
    }
    fn unavailable(&mut self, path: &str, message: String) {
        self.coverage = Coverage::Incomplete;
        self.problem("input-unavailable", path, "", message);
    }
    fn finish(&mut self) {
        self.files.sort();
        self.files.dedup();
        self.entries.sort_by(|a, b| {
            (&a.id, &a.path, &a.pointer, &a.set).cmp(&(&b.id, &b.path, &b.pointer, &b.set))
        });
        self.edges.sort();
        self.edges.dedup();
        self.problems.sort();
        self.problems.dedup();
        for entry in &mut self.entries {
            if self.coverage == Coverage::Incomplete
                || self
                    .problems
                    .iter()
                    .any(|p| p.path == entry.path || p.path.is_empty())
            {
                entry.status = "invalid".into();
            }
        }
    }
}
/// Read the explicit config through the same containment boundary as artifacts.
/// All sets are inventoried: selecting files for validation must never hide a
/// duplicate ID or a reference target in another configured set.
pub fn inspect(root: &Path, config_path: &str) -> Index {
    let mut result = Index::new();
    let files = match RepositoryFiles::new(root) {
        Ok(f) => f,
        Err(e) => {
            result.unavailable("", e);
            return result;
        }
    };
    let bytes = match files.read(config_path) {
        Ok(b) => b,
        Err(e) => {
            result.unavailable(config_path, e);
            return result;
        }
    };
    let config = match ArtifactConfig::parse(&bytes) {
        Ok(c) => c,
        Err(errors) => {
            for error in errors {
                result.problem("invalid-config", config_path, "", error);
            }
            result.finish();
            return result;
        }
    };
    // Reuse exact bytes for an overlapping file selection; no set may see a
    // second version of the same path during this one inventory.
    let mut cache: BTreeMap<String, Result<Vec<u8>, String>> = BTreeMap::new();
    for set in &config.artifact_sets {
        let schema_bytes = cache
            .entry(set.schema.clone())
            .or_insert_with(|| files.read(&set.schema))
            .clone();
        let compiled = match schema_bytes.and_then(|bytes| compile_local_schema(&bytes)) {
            Ok(schema) => schema,
            Err(e) => {
                result.unavailable(&set.schema, e);
                continue;
            }
        };
        let paths = match files.discover(&set.paths) {
            Ok(p) => p,
            Err(errors) => {
                for error in errors {
                    result.unavailable("", format!("{}: {error}", set.name));
                }
                continue;
            }
        };
        for path in paths {
            result.files.push(path.clone());
            let bytes = match cache
                .entry(path.clone())
                .or_insert_with(|| files.read(&path))
                .clone()
            {
                Ok(b) => b,
                Err(e) => {
                    result.unavailable(&path, e);
                    continue;
                }
            };
            let value: Value = match serde_norway::from_slice(&bytes) {
                Ok(v) => v,
                Err(e) => {
                    result.problem("invalid-document", &path, "", e.to_string());
                    continue;
                }
            };
            for error in schema::validate(&compiled, &value) {
                result.problem("schema-invalid", &path, &error.pointer, error.message);
            }
            let digest = format!("{:x}", Sha256::digest(&bytes));
            let mut owners = Vec::new();
            for pointer in &set.id_pointers {
                let Some(id) = value
                    .pointer(pointer)
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                else {
                    result.problem(
                        "invalid-id",
                        &path,
                        pointer,
                        "ID pointer must resolve to a nonempty string".into(),
                    );
                    continue;
                };
                let parent = pointer.rsplit_once('/').map(|(p, _)| p).unwrap_or("");
                let object = value.pointer(parent).unwrap_or(&value);
                owners.push((parent.to_owned(), id.to_owned()));
                result.entries.push(Entry {
                    id: id.into(),
                    kind: object
                        .get("kind")
                        .and_then(Value::as_str)
                        .unwrap_or(&set.name)
                        .into(),
                    key: object.get("key").and_then(Value::as_str).map(str::to_owned),
                    version: object.get("version").cloned(),
                    set: set.name.clone(),
                    path: path.clone(),
                    pointer: pointer.clone(),
                    digest: digest.clone(),
                    status: "valid".into(),
                });
            }
            for pointer in &set.reference_pointers {
                let Some(targets) = value.pointer(pointer) else {
                    result.problem("invalid-reference",&path,pointer,"declared reference pointer is missing; use an empty array for no references".into());
                    continue;
                };
                let mut candidates: Vec<_> = owners
                    .iter()
                    .filter(|(owner, _)| {
                        owner.is_empty()
                            || pointer
                                .strip_prefix(owner)
                                .is_some_and(|rest| rest.starts_with('/'))
                    })
                    .collect();
                candidates.sort_by_key(|(owner, _)| std::cmp::Reverse(owner.len()));
                let source = match candidates.first() {
                    Some((owner, id))
                        if candidates
                            .get(1)
                            .is_none_or(|(next, _)| next.len() != owner.len()) =>
                    {
                        id.as_str()
                    }
                    _ => {
                        result.problem(
                            "ambiguous-reference-owner",
                            &path,
                            pointer,
                            "reference must have one nearest enclosing object with a declared ID"
                                .into(),
                        );
                        continue;
                    }
                };
                let refs: Vec<(String, &Value)> = match targets {
                    Value::Array(a) => a
                        .iter()
                        .enumerate()
                        .map(|(i, v)| (format!("{pointer}/{i}"), v))
                        .collect(),
                    Value::String(_) => vec![(pointer.clone(), targets)],
                    _ => {
                        result.problem(
                            "invalid-reference",
                            &path,
                            pointer,
                            "reference must be an ID string or an array of ID strings".into(),
                        );
                        continue;
                    }
                };
                for (location, target) in refs {
                    if let Some(id) = target.as_str().filter(|s| !s.is_empty()) {
                        result.edges.push(Edge {
                            source: source.into(),
                            target: id.into(),
                            dependency: set.dependency_pointers.contains(pointer),
                            path: path.clone(),
                            pointer: location,
                        });
                    } else {
                        result.problem(
                            "invalid-reference",
                            &path,
                            &location,
                            "reference target must be a nonempty ID string".into(),
                        );
                    }
                }
            }
        }
    }
    validate_graph(&mut result);
    result.finish();
    result
}
fn compile_local_schema(bytes: &[u8]) -> Result<schema::CompiledSchema, String> {
    let value: Value = serde_json::from_slice(bytes).map_err(|e| format!("schema JSON: {e}"))?;
    let mut stack = vec![&value];
    while let Some(value) = stack.pop() {
        match value {
            Value::Object(map) => {
                if let Some(reference) = map.get("$ref") {
                    if !reference.as_str().is_some_and(|r| r.starts_with('#')) {
                        return Err(
                            "artifact schemas must be self-contained; external $ref is forbidden"
                                .into(),
                        );
                    }
                }
                stack.extend(map.values());
            }
            Value::Array(a) => stack.extend(a),
            _ => {}
        }
    }
    schema::compile_str(&serde_json::to_string(&value).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn validate_graph(result: &mut Index) {
    let mut ids: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for e in &result.entries {
        ids.entry(e.id.clone())
            .or_default()
            .push((e.path.clone(), e.pointer.clone()));
    }
    for (id, locations) in &ids {
        if locations.len() > 1 {
            for (path, pointer) in locations {
                result.problem(
                    "duplicate-id",
                    path,
                    pointer,
                    format!(
                        "ID {id:?} occurs {} times across the configured registry",
                        locations.len()
                    ),
                );
            }
        }
    }
    for edge in result.edges.clone() {
        match ids.get(&edge.target) {
            None => result.problem(
                "dangling-reference",
                &edge.path,
                &edge.pointer,
                format!("unknown target ID {:?}", edge.target),
            ),
            Some(v) if v.len() > 1 => result.problem(
                "ambiguous-reference",
                &edge.path,
                &edge.pointer,
                format!("target ID {:?} is not unique", edge.target),
            ),
            _ => {}
        }
    }
    // Kahn's algorithm proves acyclicity without recursive call-stack growth.
    // Remaining nodes include cycles and their dependents; do not claim every
    // remaining node lies on a cycle. Emit one graph-level diagnostic instead.
    let mut incoming: BTreeMap<&str, usize> = ids.keys().map(|id| (id.as_str(), 0)).collect();
    let mut adjacency: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for edge in &result.edges {
        if edge.dependency
            && ids.contains_key(&edge.source)
            && ids.contains_key(&edge.target)
            && adjacency
                .entry(&edge.source)
                .or_default()
                .insert(&edge.target)
        {
            *incoming.get_mut(edge.target.as_str()).unwrap() += 1;
        }
    }
    let mut ready: BTreeSet<_> = incoming
        .iter()
        .filter(|(_, n)| **n == 0)
        .map(|(id, _)| *id)
        .collect();
    let mut removed = 0;
    while let Some(id) = ready.pop_first() {
        removed += 1;
        for target in adjacency.get(id).into_iter().flatten() {
            let n = incoming.get_mut(target).unwrap();
            *n -= 1;
            if *n == 0 {
                ready.insert(target);
            }
        }
    }
    if removed != incoming.len() {
        result.problem(
            "dependency-cycle",
            "",
            "",
            "declared dependency graph contains a cycle".into(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    fn fixture() -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        fs::create_dir(d.path().join("items")).unwrap();
        fs::write(d.path().join("config.yaml"),"artifact_sets:\n- name: tasks\n  paths: ['items/*.json']\n  schema: schema.json\n  id_pointers: ['/id']\n  reference_pointers: ['/refs', '/deps']\n  dependency_pointers: ['/deps']\n").unwrap();
        fs::write(d.path().join("schema.json"),r#"{"type":"object","required":["id","refs","deps"],"properties":{"id":{"type":"string"},"refs":{"type":"array","items":{"type":"string"}},"deps":{"type":"array","items":{"type":"string"}}}}"#).unwrap();
        d
    }
    fn item(d: &Path, name: &str, id: &str, refs: &[&str], deps: &[&str]) {
        fs::write(
            d.join(format!("items/{name}.json")),
            serde_json::to_vec(&serde_json::json!({"id":id,"refs":refs,"deps":deps})).unwrap(),
        )
        .unwrap();
    }
    #[test]
    fn valid_index_is_byte_deterministic_and_ordinary_backreferences_are_not_cycles() {
        let d = fixture();
        item(d.path(), "a", "a", &["b"], &["b"]);
        item(d.path(), "b", "b", &["a"], &[]);
        let a = inspect(d.path(), "config.yaml");
        assert_eq!(a.exit_status(), 0, "{:?}", a.problems);
        assert_eq!(a.entries.len(), 2);
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&inspect(d.path(), "config.yaml")).unwrap()
        );
        assert_eq!(a.entries[0].digest.len(), 64);
    }
    #[test]
    fn duplicates_dangling_references_and_cycles_are_all_reported() {
        let d = fixture();
        item(d.path(), "a", "a", &["missing"], &["b"]);
        item(d.path(), "b", "b", &[], &["a"]);
        item(d.path(), "copy", "a", &[], &[]);
        let result = inspect(d.path(), "config.yaml");
        assert_eq!(result.exit_status(), 1);
        for code in ["duplicate-id", "dangling-reference", "dependency-cycle"] {
            assert!(
                result.problems.iter().any(|p| p.code == code),
                "{code}: {:?}",
                result.problems
            );
        }
        assert!(result.entries.iter().all(|e| e.status == "invalid"));
    }
    #[test]
    fn unavailable_schema_and_external_refs_never_produce_a_clean_index() {
        let d = fixture();
        item(d.path(), "a", "a", &[], &[]);
        fs::write(
            d.path().join("schema.json"),
            r#"{"$ref":"file:///etc/passwd"}"#,
        )
        .unwrap();
        let result = inspect(d.path(), "config.yaml");
        assert_eq!(result.exit_status(), 2);
        assert!(result.entries.is_empty());
        assert!(result.problems[0].message.contains("external $ref"));
        assert_eq!(inspect(d.path(), "../config.yaml").exit_status(), 2);
    }
    #[test]
    fn malformed_document_and_missing_pointer_are_not_silently_dropped() {
        let d = fixture();
        fs::write(d.path().join("items/bad.json"), "{broken").unwrap();
        fs::write(d.path().join("items/missing.json"), r#"{"id":"a"}"#).unwrap();
        let result = inspect(d.path(), "config.yaml");
        assert_eq!(result.exit_status(), 1);
        assert!(result.problems.iter().any(|p| p.code == "invalid-document"));
        assert!(result
            .problems
            .iter()
            .any(|p| p.code == "invalid-reference"));
        assert_eq!(result.entries[0].status, "invalid");
    }
}
