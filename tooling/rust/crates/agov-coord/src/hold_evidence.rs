//! Logical evidence inventory over exact Git bytes. Target evidence is retained
//! for resolution but excluded from the action population, even after a rename.
use crate::coord_validator::validate_content_with_schema;
use crate::hold_git::Blob;
use crate::hold_result::{Code, Operation, ProblemKey, Problems, Provenance, Severity, Subject};
use agov_core::{schema::CompiledSchema, yaml};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct Evidence {
    pub raw: Option<Value>,
    pub valid: bool,
    pub id: Option<String>,
    pub provenance: Vec<Provenance>,
    pub action_candidate: bool,
    pub problem_keys: Vec<ProblemKey>,
}
impl Evidence {
    pub fn string(&self, field: &str) -> Option<&str> {
        self.raw.as_ref()?.get(field)?.as_str()
    }
    pub fn control(&self) -> Option<&Value> {
        self.raw.as_ref()?.get("publication_hold")
    }
}

/// Schema compilation happens once outside the inventory. No filesystem or
/// network is consulted by parsing and cross-source identity checks.
pub fn inventory(
    blobs: &[Blob],
    schema: &CompiledSchema,
    problems: &mut Problems,
) -> Vec<Evidence> {
    // Exact bytes, rather than parsed YAML or blob labels, define equality.
    let mut grouped: BTreeMap<(&[u8], Option<&str>), Vec<&Blob>> = BTreeMap::new();
    for blob in blobs {
        let has_id = std::str::from_utf8(&blob.bytes)
            .ok()
            .and_then(|text| yaml::parse_frontmatter_json(text).ok())
            .is_some_and(|raw| raw.get("id").and_then(Value::as_str).is_some());
        // Without a logical ID, malformed evidence remains path-addressed.
        let path_identity = (!has_id).then_some(blob.path.as_str());
        grouped
            .entry((&blob.bytes, path_identity))
            .or_default()
            .push(blob);
    }
    let mut evidence = Vec::new();
    for ((bytes, _), group) in grouped {
        let mut provenance: Vec<_> = group
            .iter()
            .map(|b| Provenance {
                source_id: b.source_id.clone(),
                path: b.path.clone(),
                blob_oid: b.oid.clone(),
            })
            .collect();
        provenance.sort();
        provenance.dedup();
        let text = std::str::from_utf8(bytes);
        let raw = text.ok().and_then(|s| yaml::parse_frontmatter_json(s).ok());
        let validation = text
            .map_err(|e| vec![(e.to_string(), String::new())])
            .and_then(|s| validate_content_with_schema(s, schema));
        let id = raw
            .as_ref()
            .and_then(|r| r.get("id"))
            .and_then(Value::as_str)
            .map(str::to_owned);
        let candidate = provenance.iter().any(|p| p.source_id != "target");
        let target = provenance.iter().any(|p| p.source_id == "target");
        let mut item = Evidence {
            raw,
            valid: validation.is_ok(),
            id,
            provenance,
            action_candidate: candidate && !target,
            problem_keys: Vec::new(),
        };
        if let Err(errors) = validation {
            let code = if item.control().is_some() {
                Code::InvalidControl
            } else {
                Code::InvalidRecord
            };
            let key = key_for(code, std::slice::from_ref(&item));
            let diagnostic = errors
                .into_iter()
                .map(|(message, pointer)| format!("{pointer}: {message}"))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join("\n");
            item.problem_keys.push(problems.insert(key, diagnostic));
        }
        evidence.push(item);
    }
    // Each entry now has distinct bytes. Sharing a path or ID therefore proves
    // a conflict; no normalized YAML comparison can conceal a body-only edit.
    let mut paths: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    let mut ids: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    for (i, item) in evidence.iter().enumerate() {
        for p in &item.provenance {
            paths.entry(p.path.clone()).or_default().insert(i);
        }
        if let Some(id) = &item.id {
            ids.entry(id.clone()).or_default().insert(i);
        }
    }
    for (code, groups) in [(Code::PathConflict, paths), (Code::IdConflict, ids)] {
        for (label, indices) in groups.into_iter().filter(|(_, v)| v.len() > 1) {
            let items: Vec<_> = indices.iter().map(|i| &evidence[*i]).collect();
            let mut key = key_for_refs(code, &items);
            if code == Code::PathConflict {
                key.subject.path = Some(label.clone());
            } else {
                key.subject.record_id = Some(label.clone());
            }
            let key = problems.insert(
                key,
                format!(
                    "distinct raw bytes share {} {label}",
                    if code == Code::PathConflict {
                        "path"
                    } else {
                        "ID"
                    }
                ),
            );
            for i in indices {
                evidence[i].problem_keys.push(key.clone());
            }
        }
    }
    for item in &mut evidence {
        item.problem_keys.sort();
        item.problem_keys.dedup();
    }
    // Stable evidence order is independent of blob/ref encounter order. Canonical
    // timestamp ordering is applied to final typed rows by the result assembler.
    evidence.sort_by(|a, b| {
        a.id.cmp(&b.id)
            .then_with(|| a.provenance.cmp(&b.provenance))
    });
    evidence
}

pub fn key_for(code: Code, items: &[Evidence]) -> ProblemKey {
    key_for_refs(code, &items.iter().collect::<Vec<_>>())
}
fn key_for_refs(code: Code, items: &[&Evidence]) -> ProblemKey {
    let mut source_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut record_ids = BTreeSet::new();
    for item in items {
        for p in &item.provenance {
            source_ids.insert(p.source_id.clone());
            paths.insert(p.path.clone());
        }
        if let Some(id) = &item.id {
            record_ids.insert(id.clone());
        }
    }
    ProblemKey {
        code,
        severity: Severity::Semantic,
        operation: Operation::ValidateRecord,
        subject: Subject {
            path: (paths.len() == 1).then(|| paths.first().unwrap().clone()),
            record_id: (record_ids.len() == 1).then(|| record_ids.first().unwrap().clone()),
            ..Subject::default()
        },
        source_ids: source_ids.into_iter().collect(),
        paths: paths.into_iter().collect(),
        record_ids: record_ids.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn schema() -> CompiledSchema {
        agov_core::schema::compile_str(include_str!(
            "../../../../../agf/schemas/coordination-message.schema.json"
        ))
        .unwrap()
    }
    fn blob(source: &str, path: &str, id: &str, body: &str) -> Blob {
        Blob { source_id: source.into(), path: path.into(), oid: "a".repeat(40),
            bytes: format!("---\nid: fixture-{id}\nkind: note\nfrom: coder-tester\nto: all\ntimestamp: 2026-09-12T12:00:00Z\nimmutable: true\nreferences-to: []\ntopic: fixture\nrequires-ack: false\n---\n{body}\n").into_bytes() }
    }
    #[test]
    fn target_identical_bytes_at_a_new_path_are_evidence_only() {
        let mut p = Problems::default();
        let items = inventory(
            &[
                blob("target", "old.md", "one", "body"),
                blob("candidate:a", "new.md", "one", "body"),
                blob("candidate:b", "third.md", "one", "body"),
            ],
            &schema(),
            &mut p,
        );
        assert_eq!(items.len(), 1);
        assert!(items[0].valid);
        assert!(!items[0].action_candidate);
        assert_eq!(items[0].provenance.len(), 3);
        assert!(p.finish().0.is_empty());
    }
    #[test]
    fn body_only_changes_and_path_collisions_retain_both_action_and_evidence() {
        let mut p = Problems::default();
        let items = inventory(
            &[
                blob("target", "same.md", "one", "before"),
                blob("candidate:a", "same.md", "one", "after"),
            ],
            &schema(),
            &mut p,
        );
        assert_eq!(items.len(), 2);
        assert_eq!(items.iter().filter(|i| i.action_candidate).count(), 1);
        let problems = p.finish().0;
        assert_eq!(
            problems.iter().map(|p| p.code).collect::<Vec<_>>(),
            [Code::IdConflict, Code::PathConflict]
        );
        assert!(items.iter().all(|i| i.problem_keys.len() == 2));
        let mut p = Problems::default();
        inventory(
            &[
                blob("candidate:a", "same.md", "one", "body"),
                blob("candidate:b", "same.md", "two", "body"),
            ],
            &schema(),
            &mut p,
        );
        assert_eq!(
            p.finish().0.iter().map(|p| p.code).collect::<Vec<_>>(),
            [Code::PathConflict]
        );
    }
    #[test]
    fn candidate_dedup_retains_all_provenance_and_malformed_rows() {
        let mut p = Problems::default();
        let invalid = Blob {
            source_id: "candidate:a".into(),
            path: "bad.md".into(),
            oid: "b".repeat(40),
            bytes: vec![255],
        };
        let items = inventory(
            &[
                blob("candidate:a", "one.md", "one", "body"),
                blob("candidate:b", "two.md", "one", "body"),
                invalid,
            ],
            &schema(),
            &mut p,
        );
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|i| i.action_candidate));
        assert_eq!(items[0].id, None);
        assert!(!items[0].valid);
        assert_eq!(items[0].provenance[0].path, "bad.md");
        assert_eq!(items[1].provenance.len(), 2);
        assert!(items[1].valid);
        assert_eq!(p.finish().0[0].code, Code::InvalidRecord);
    }
    #[test]
    fn malformed_no_id_does_not_merge_distinct_paths() {
        let first = Blob {
            source_id: "target".into(),
            path: "old.md".into(),
            oid: "b".repeat(40),
            bytes: b"bad".to_vec(),
        };
        let second = Blob {
            source_id: "candidate:a".into(),
            path: "new.md".into(),
            ..first.clone()
        };
        let mut problems = Problems::default();
        let items = inventory(&[first, second], &schema(), &mut problems);
        assert_eq!(items.len(), 2);
        assert_eq!(items.iter().filter(|i| i.action_candidate).count(), 1);
        assert!(items
            .iter()
            .all(|i| i.id.is_none() && i.provenance.len() == 1));
        assert_eq!(problems.finish().0.len(), 2);
    }
    #[test]
    fn input_order_cannot_change_conflict_ids_or_provenance() {
        let mut blobs = vec![
            blob("candidate:b", "x.md", "one", "B"),
            blob("target", "x.md", "one", "A"),
            blob("candidate:a", "y.md", "one", "B"),
        ];
        let mut a = Problems::default();
        let items_a = inventory(&blobs, &schema(), &mut a);
        blobs.reverse();
        let mut b = Problems::default();
        let items_b = inventory(&blobs, &schema(), &mut b);
        assert_eq!(a.finish(), b.finish());
        assert_eq!(
            items_a
                .iter()
                .map(|i| (&i.provenance, &i.problem_keys))
                .collect::<Vec<_>>(),
            items_b
                .iter()
                .map(|i| (&i.provenance, &i.problem_keys))
                .collect::<Vec<_>>()
        );
    }
}
