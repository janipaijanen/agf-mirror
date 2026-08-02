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

/// A pending (unresolved) `requires-ack: true` record of any kind other than
/// `handoff` or `ack`.
#[derive(Debug, Clone)]
pub struct PendingAck {
    pub id: String,
    pub file: String,
    pub kind: String,
    pub from: String,
    pub to: String,
    pub topic: String,
    pub pending_since: String,
}

/// Find all pending (unresolved) `requires-ack: true` records of any kind
/// other than `handoff` and `ack`, published at or after `cutoff`.
///
/// This is a distinct, additive surface from [`find_pending`] -- it does not
/// change `handoff next`/`take`/`complete`'s existing behavior in any way.
/// `handoff` records keep using [`find_pending`]'s own tracking exclusively.
/// `ack` records are excluded because the schema forbids `kind: ack` from
/// setting `requires-ack: true` in the first place (an acknowledgement
/// requiring its own acknowledgement is not a meaningful obligation) --
/// excluding them here is defensive, not load-bearing.
///
/// Resolution is direct reference, not [`find_pending`]'s transitive
/// descendant-chain semantics: a record is resolved the moment any later
/// record's `references-to` lists its file, full stop. Per
/// `docs/plans/v250-rc1-contract-core.md` work package 2: "a requires-ack:
/// true record is resolved when any later record's references-to includes
/// its id" -- generalizing this project's existing practiced norm ("review
/// alone does not clear pending; ack/completion/decision-record resolves")
/// to the mechanism, not to a fixed set of resolving kinds.
///
/// `cutoff` grandfathers pre-existing records (an RFC 3339 timestamp,
/// typically this work package's own merge commit date): only descriptors
/// whose own `timestamp` is `>= cutoff` are considered at all, pending or
/// resolved. A resolving reference's own timestamp is not cutoff-gated --
/// only whether the *candidate* record is in scope.
///
/// `descriptors` must be sorted by filename (chronological order), same
/// contract as [`find_pending`].
pub fn find_pending_all_kinds(descriptors: &[MessageDescriptor], cutoff: &str) -> Vec<PendingAck> {
    descriptors
        .iter()
        .filter(|d| d.msg.requires_ack())
        .filter(|d| !matches!(d.msg.kind(), Some("handoff") | Some("ack")))
        .filter(|d| d.msg.timestamp().is_some_and(|ts| ts >= cutoff))
        .filter(|d| !is_directly_referenced_later(d, descriptors))
        .map(|d| PendingAck {
            id: d.msg.id().unwrap_or("").to_string(),
            file: d.file.clone(),
            kind: d.msg.kind().unwrap_or("").to_string(),
            from: d.msg.from().unwrap_or("").to_string(),
            to: d.msg.to().unwrap_or("").to_string(),
            topic: d.msg.topic().unwrap_or("").to_string(),
            pending_since: d.msg.timestamp().unwrap_or("").to_string(),
        })
        .collect()
}

fn is_directly_referenced_later(target: &MessageDescriptor, descriptors: &[MessageDescriptor]) -> bool {
    descriptors
        .iter()
        .any(|d| d.sort_key > target.sort_key && d.refs.iter().any(|r| r == &target.file))
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

    /// Like `descriptor()` but with an explicit `requires-ack`, since
    /// `descriptor()` hardcodes `true` (fine for `find_pending`, which
    /// ignores the field entirely, but load-bearing for
    /// `find_pending_all_kinds`).
    fn ack_descriptor(
        file: &str,
        kind: &str,
        timestamp: &str,
        requires_ack: bool,
        refs: &[&str],
    ) -> MessageDescriptor {
        let raw = serde_json::json!({
            "kind": kind,
            "from": "coder-tester",
            "to": "governor-reviewer",
            "id": file.trim_end_matches(".md"),
            "timestamp": timestamp,
            "topic": "test-topic",
            "requires-ack": requires_ack,
            "references-to": refs.iter().map(|r| serde_json::Value::String(r.to_string())).collect::<Vec<_>>()
        });
        MessageDescriptor {
            file: file.to_string(),
            sort_key: format!("{timestamp}_{file}"),
            refs: refs.iter().map(|r| r.to_string()).collect(),
            msg: CoordMessage { raw },
        }
    }

    const CUTOFF: &str = "2026-08-01T00:00:00Z";

    #[test]
    fn post_cutoff_review_with_requires_ack_is_pending() {
        let review = descriptor(
            "20260802T100000Z-contribution-auditor-aaaa.md",
            "review",
            "2026-08-02T10:00:00Z",
            &[],
        );
        let pending = find_pending_all_kinds(&[review], CUTOFF);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].kind, "review");
    }

    #[test]
    fn post_cutoff_review_resolved_by_direct_reference() {
        let review = ack_descriptor(
            "20260802T100000Z-contribution-auditor-aaaa.md",
            "review",
            "2026-08-02T10:00:00Z",
            true,
            &[],
        );
        let decision = ack_descriptor(
            "20260802T110000Z-governor-reviewer-bbbb.md",
            "decision-record",
            "2026-08-02T11:00:00Z",
            false,
            &["20260802T100000Z-contribution-auditor-aaaa.md"],
        );
        let pending = find_pending_all_kinds(&[review, decision], CUTOFF);
        assert!(pending.is_empty());
    }

    #[test]
    fn resolution_is_direct_not_transitive() {
        // A is referenced by B; C references B, not A. Under find_pending's
        // (handoff) transitive descendant-chain semantics this would also
        // resolve A. Under this surface's direct-only semantics it must
        // NOT -- only a reference naming A's own file resolves A.
        let a = ack_descriptor(
            "20260802T100000Z-contribution-auditor-aaaa.md",
            "review",
            "2026-08-02T10:00:00Z",
            true,
            &[],
        );
        let b = ack_descriptor(
            "20260802T110000Z-planner-architect-bbbb.md",
            "note",
            "2026-08-02T11:00:00Z",
            false,
            &[], // does not reference A
        );
        let c = ack_descriptor(
            "20260802T120000Z-governor-reviewer-cccc.md",
            "decision-record",
            "2026-08-02T12:00:00Z",
            false,
            &["20260802T110000Z-planner-architect-bbbb.md"], // references B, not A
        );
        let pending = find_pending_all_kinds(&[a, b, c], CUTOFF);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].file, "20260802T100000Z-contribution-auditor-aaaa.md");
    }

    #[test]
    fn pre_cutoff_record_never_appears_even_if_unresolved() {
        let old_review = descriptor(
            "20260615T100000Z-contribution-auditor-aaaa.md",
            "review",
            "2026-06-15T10:00:00Z",
            &[],
        );
        let pending = find_pending_all_kinds(&[old_review], CUTOFF);
        assert!(
            pending.is_empty(),
            "pre-cutoff record must be grandfathered, never appear"
        );
    }

    #[test]
    fn handoff_kind_never_appears_in_all_kinds_surface() {
        // handoff next/take/complete own this kind exclusively; this
        // surface must not duplicate or interfere with it.
        let handoff = descriptor(
            "20260802T100000Z-planner-architect-aaaa.md",
            "handoff",
            "2026-08-02T10:00:00Z",
            &[],
        );
        let pending = find_pending_all_kinds(&[handoff], CUTOFF);
        assert!(pending.is_empty());
    }

    #[test]
    fn ack_kind_never_appears_in_all_kinds_surface() {
        // Defensive: the schema already forbids kind: ack from setting
        // requires-ack: true, but this surface excludes it explicitly too.
        let ack = descriptor(
            "20260802T100000Z-governor-reviewer-aaaa.md",
            "ack",
            "2026-08-02T10:00:00Z",
            &[],
        );
        let pending = find_pending_all_kinds(&[ack], CUTOFF);
        assert!(pending.is_empty());
    }

    #[test]
    fn requires_ack_false_never_appears() {
        let raw = serde_json::json!({
            "kind": "note",
            "from": "coder-tester",
            "to": "governor-reviewer",
            "id": "20260802T100000Z-coder-tester-aaaa",
            "timestamp": "2026-08-02T10:00:00Z",
            "topic": "test-topic",
            "requires-ack": false,
            "references-to": []
        });
        let d = MessageDescriptor {
            file: "20260802T100000Z-coder-tester-aaaa.md".to_string(),
            sort_key: "2026-08-02T10:00:00Z_20260802T100000Z-coder-tester-aaaa.md".to_string(),
            refs: vec![],
            msg: CoordMessage { raw },
        };
        let pending = find_pending_all_kinds(&[d], CUTOFF);
        assert!(pending.is_empty());
    }
}
