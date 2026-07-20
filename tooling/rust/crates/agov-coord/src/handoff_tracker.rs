use crate::coord_validator::CoordMessage;
use std::collections::{BTreeMap, BTreeSet};

/// A handoff message that has not yet been acknowledged or resolved.
#[derive(Debug, Clone)]
pub struct PendingHandoff {
    pub id: String,
    pub file: String,
    pub from: String,
    pub to: String,
    pub topic: String,
    pub pending_since: String,
}

/// A compact descriptor of one coordination message for graph-index use.
#[derive(Debug, Clone)]
pub struct MessageDescriptor {
    pub file: String,
    pub sort_key: String,
    pub refs: Vec<String>,
    pub msg: CoordMessage,
}

/// Find all pending (unresolved) handoffs in the channel.
///
/// A handoff is resolved when a later `ack`, `completion`, or `decision-record`
/// exists in its transitive descendant subgraph. `note` and other kinds are not
/// treated as resolving kinds.
///
/// `descriptors` must be sorted by filename (chronological order).
pub fn find_pending(descriptors: &[MessageDescriptor]) -> Vec<PendingHandoff> {
    let descendants = build_descendants(descriptors);
    let by_file: BTreeMap<&str, &MessageDescriptor> =
        descriptors.iter().map(|d| (d.file.as_str(), d)).collect();

    descriptors
        .iter()
        .filter(|d| d.msg.kind() == Some("handoff"))
        .filter(|d| !is_resolved(d, &descendants, &by_file))
        .map(|d| PendingHandoff {
            id: d.msg.id().unwrap_or("").to_string(),
            file: d.file.clone(),
            from: d.msg.from().unwrap_or("").to_string(),
            to: d.msg.to().unwrap_or("").to_string(),
            topic: d.msg.topic().unwrap_or("").to_string(),
            pending_since: d.msg.timestamp().unwrap_or("").to_string(),
        })
        .collect()
}

fn is_resolved(
    handoff: &MessageDescriptor,
    descendants: &BTreeMap<String, Vec<String>>,
    by_file: &BTreeMap<&str, &MessageDescriptor>,
) -> bool {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut stack = vec![handoff.file.clone()];

    while let Some(file) = stack.pop() {
        if !seen.insert(file.clone()) {
            continue;
        }
        for descendant in descendants.get(&file).into_iter().flatten() {
            let Some(d) = by_file.get(descendant.as_str()) else {
                continue;
            };
            if d.sort_key > handoff.sort_key
                && matches!(d.msg.kind(), Some("ack" | "completion" | "decision-record"))
            {
                return true;
            }
            stack.push(descendant.clone());
        }
    }

    false
}

/// Build a map from each file to the files that reference it (descendants).
fn build_descendants(descriptors: &[MessageDescriptor]) -> BTreeMap<String, Vec<String>> {
    let by_file: BTreeMap<&str, &MessageDescriptor> =
        descriptors.iter().map(|d| (d.file.as_str(), d)).collect();
    let mut descendants: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for d in descriptors {
        for referenced in &d.refs {
            let Some(referenced_d) = by_file.get(referenced.as_str()) else {
                continue;
            };
            if d.sort_key <= referenced_d.sort_key {
                continue;
            }
            descendants
                .entry(referenced.clone())
                .or_default()
                .push(d.file.clone());
        }
    }

    descendants
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor(file: &str, kind: &str, timestamp: &str, refs: &[&str]) -> MessageDescriptor {
        let raw = serde_json::json!({
            "kind": kind,
            "from": "coder-tester",
            "to": "governor-reviewer",
            "id": file.trim_end_matches(".md"),
            "timestamp": timestamp,
            "topic": "test-topic",
            "requires-ack": true,
            "references-to": refs.iter().map(|r| serde_json::Value::String(r.to_string())).collect::<Vec<_>>()
        });
        let msg = CoordMessage { raw };
        MessageDescriptor {
            file: file.to_string(),
            sort_key: format!("{timestamp}_{file}"),
            refs: refs.iter().map(|r| r.to_string()).collect(),
            msg,
        }
    }

    #[test]
    fn handoff_with_no_descendants_is_pending() {
        let handoff = descriptor(
            "20260623T100000Z-coder-tester-aaaa.md",
            "handoff",
            "2026-06-23T10:00:00Z",
            &[],
        );
        let pending = find_pending(&[handoff]);
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn handoff_resolved_by_ack() {
        let handoff = descriptor(
            "20260623T100000Z-coder-tester-aaaa.md",
            "handoff",
            "2026-06-23T10:00:00Z",
            &[],
        );
        let ack = descriptor(
            "20260623T110000Z-governor-reviewer-bbbb.md",
            "ack",
            "2026-06-23T11:00:00Z",
            &["20260623T100000Z-coder-tester-aaaa.md"],
        );
        let pending = find_pending(&[handoff, ack]);
        assert!(pending.is_empty());
    }

    #[test]
    fn handoff_resolved_transitively_through_note() {
        let handoff = descriptor(
            "20260623T100000Z-coder-tester-aaaa.md",
            "handoff",
            "2026-06-23T10:00:00Z",
            &[],
        );
        let note = descriptor(
            "20260623T110000Z-planner-architect-bbbb.md",
            "note",
            "2026-06-23T11:00:00Z",
            &["20260623T100000Z-coder-tester-aaaa.md"],
        );
        let completion = descriptor(
            "20260623T120000Z-coder-tester-cccc.md",
            "completion",
            "2026-06-23T12:00:00Z",
            &["20260623T110000Z-planner-architect-bbbb.md"],
        );
        let pending = find_pending(&[handoff, note, completion]);
        assert!(pending.is_empty());
    }

    #[test]
    fn note_kind_is_never_pending() {
        let note = descriptor(
            "20260623T100000Z-coder-tester-aaaa.md",
            "note",
            "2026-06-23T10:00:00Z",
            &[],
        );
        let pending = find_pending(&[note]);
        assert!(pending.is_empty());
    }
}
