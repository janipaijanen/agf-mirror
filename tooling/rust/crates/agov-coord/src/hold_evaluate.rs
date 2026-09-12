//! Assemble the pinned evidence envelope. This evaluates declarations only;
//! publication and governor authority are always external checks.
use crate::{
    hold_classify, hold_evidence, hold_git, hold_result::*, hold_status,
    publication_hold::utc_timestamp,
};
use agov_core::schema::CompiledSchema;
use std::collections::BTreeMap;
use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};

fn timestamp(value: &str) -> Option<String> {
    OffsetDateTime::parse(value, &Rfc3339)
        .ok()?
        .to_offset(UtcOffset::UTC)
        .format(&Rfc3339)
        .ok()
}
fn key_ids(keys: &[ProblemKey], ids: &BTreeMap<ProblemKey, String>) -> Vec<String> {
    let mut out: Vec<_> = keys
        .iter()
        .filter_map(|key| ids.get(key).cloned())
        .collect();
    out.sort();
    out.dedup();
    out
}
fn provenance_order(p: &[Provenance]) -> (&str, &str) {
    p.first()
        .map(|p| (p.path.as_str(), p.source_id.as_str()))
        .unwrap_or(("", ""))
}

pub fn evaluate(
    inventory: hold_git::Inventory,
    channel: &str,
    schema: &CompiledSchema,
    now: OffsetDateTime,
) -> ResultV1 {
    evaluate_enabled(inventory, channel, schema, now, true)
}

pub(crate) fn evaluate_enabled(
    inventory: hold_git::Inventory,
    channel: &str,
    schema: &CompiledSchema,
    now: OffsetDateTime,
    enabled: bool,
) -> ResultV1 {
    let now = now.to_offset(UtcOffset::UTC);
    let mut problems = Problems::default();
    let mut source_keys: BTreeMap<String, Vec<ProblemKey>> = BTreeMap::new();
    for problem in &inventory.problems {
        let code = match problem.code {
            "invalid-input" => Code::InvalidInput,
            "nonregular-entry" => Code::NonregularEntry,
            _ => Code::GitReadFailed,
        };
        let operation = match problem.operation {
            "resolve-target" => Operation::ResolveTarget,
            "resolve-candidate" => Operation::ResolveCandidate,
            "read-blob" => Operation::ReadBlob,
            _ => Operation::ReadTree,
        };
        let key = problems.insert(
            ProblemKey {
                code,
                severity: Severity::Unavailable,
                operation,
                subject: Subject {
                    requested_ref: problem.requested_ref.clone(),
                    sha: problem.sha.clone(),
                    path: problem.path.clone(),
                    record_id: None,
                },
                source_ids: vec![problem.source_id.clone()],
                paths: problem.path.iter().cloned().collect(),
                record_ids: vec![],
            },
            problem.message.clone(),
        );
        source_keys
            .entry(problem.source_id.clone())
            .or_default()
            .push(key);
    }
    let evidence = hold_evidence::inventory(&inventory.blobs, schema, &mut problems);
    let mut evidence_keys: Vec<_> = evidence.iter().map(|e| e.problem_keys.clone()).collect();
    let mut holds: Vec<(usize, Hold, hold_status::Interval)> = Vec::new();
    for (i, item) in evidence.iter().enumerate() {
        if !item.valid || !item.problem_keys.is_empty() {
            continue;
        }
        let Some(control) = item.control() else {
            continue;
        };
        if control["action"] != "start" {
            continue;
        }
        // All fields below have passed the coordination schema and cross-field
        // validation. No malformed control is promoted to a valid hold row.
        let start = utc_timestamp(item.string("timestamp").unwrap()).unwrap();
        let expires = utc_timestamp(control["expires_at"].as_str().unwrap()).unwrap();
        let interval = hold_status::evaluate_interval(start, expires, &[], now).unwrap();
        let branch = control["target_branch"].as_str().unwrap();
        let sha = control["target_sha"].as_str().unwrap();
        let target_state = if Some(branch) != inventory.target_branch.as_deref() {
            TargetState::OtherTarget
        } else if Some(sha) != inventory.target_sha.as_deref() {
            TargetState::Stale
        } else {
            TargetState::Matching
        };
        let mut references: Vec<_> = item.raw.as_ref().unwrap()["references-to"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r.as_str().unwrap().to_owned())
            .collect();
        references.sort();
        references.dedup();
        holds.push((
            i,
            Hold {
                id: item.id.clone().unwrap(),
                timestamp: start.format(&Rfc3339).unwrap(),
                target_branch: branch.into(),
                target_sha: sha.into(),
                expires_at: expires.format(&Rfc3339).unwrap(),
                effective_end: interval.effective_end.format(&Rfc3339).unwrap(),
                ends: vec![],
                declared_release_references: references,
                temporal_state: temporal(interval.state),
                target_state,
                conflict: false,
                provenance: item.provenance.clone(),
                problem_ids: vec![],
                publication_verified: Unverified,
                authority_verified: Unverified,
                required_external_checks: external_checks(),
            },
            interval,
        ));
    }
    let mut end_keys: BTreeMap<String, Vec<ProblemKey>> = BTreeMap::new();
    for (i, item) in evidence.iter().enumerate() {
        if !item.valid || !item.problem_keys.is_empty() {
            continue;
        }
        let Some(control) = item.control() else {
            continue;
        };
        if control["action"] != "end" {
            continue;
        }
        let hold_id = control["hold_id"].as_str().unwrap();
        let branch = control["target_branch"].as_str().unwrap();
        let end_time = utc_timestamp(item.string("timestamp").unwrap()).unwrap();
        let matched = holds.iter_mut().find(|(_, h, _)| h.id == hold_id);
        let failure = match matched {
            None => Some((Code::MissingStart, "end has no unique valid start")),
            Some((_, hold, interval))
                if hold.target_branch != branch || end_time < interval.start =>
            {
                Some((
                    Code::InvalidControl,
                    "end target differs from start or timestamp precedes start",
                ))
            }
            Some((_, hold, _)) => {
                hold.ends.push(End {
                    id: item.id.clone().unwrap(),
                    timestamp: end_time.format(&Rfc3339).unwrap(),
                    hold_id: hold_id.into(),
                    target_branch: branch.into(),
                    provenance: item.provenance.clone(),
                    problem_ids: vec![],
                    publication_verified: Unverified,
                    authority_verified: Unverified,
                    required_external_checks: external_checks(),
                });
                end_keys.insert(item.id.clone().unwrap(), item.problem_keys.clone());
                None
            }
        };
        if let Some((code, message)) = failure {
            let mut key = hold_evidence::key_for(code, std::slice::from_ref(item));
            key.operation = Operation::ResolveHold;
            let key = problems.insert(key, message.into());
            evidence_keys[i].push(key);
        }
    }
    for (_, hold, interval) in &mut holds {
        hold.ends.sort_by(|a, b| {
            utc_timestamp(&a.timestamp)
                .cmp(&utc_timestamp(&b.timestamp))
                .then_with(|| a.id.cmp(&b.id))
                .then_with(|| provenance_order(&a.provenance).cmp(&provenance_order(&b.provenance)))
        });
        let ends: Vec<_> = hold
            .ends
            .iter()
            .map(|e| utc_timestamp(&e.timestamp).unwrap())
            .collect();
        *interval = hold_status::evaluate_interval(
            interval.start,
            utc_timestamp(&hold.expires_at).unwrap(),
            &ends,
            now,
        )
        .unwrap();
        hold.effective_end = interval.effective_end.format(&Rfc3339).unwrap();
        hold.temporal_state = temporal(interval.state);
    }
    for a in 0..holds.len() {
        for b in a + 1..holds.len() {
            if holds[a].1.target_branch != holds[b].1.target_branch {
                continue;
            }
            let Some(overlap) = hold_status::interval_overlap(&holds[a].2, &holds[b].2, now) else {
                continue;
            };
            let selected =
                Some(holds[a].1.target_branch.as_str()) == inventory.target_branch.as_deref();
            let (code, severity) = match overlap {
                hold_status::Overlap::Current if selected => {
                    (Code::ActiveOverlap, Severity::Semantic)
                }
                hold_status::Overlap::Current => continue,
                hold_status::Overlap::Historical => (Code::HistoricalOverlap, Severity::Advisory),
                hold_status::Overlap::Future => (Code::FutureOverlap, Severity::Advisory),
            };
            if overlap == hold_status::Overlap::Current {
                holds[a].1.conflict = true;
                holds[b].1.conflict = true;
            }
            let ia = holds[a].0;
            let ib = holds[b].0;
            let mut key = hold_evidence::key_for(code, &[]);
            key.operation = Operation::ResolveHold;
            key.severity = severity;
            for i in [ia, ib] {
                key.record_ids.extend(evidence[i].id.iter().cloned());
                for p in &evidence[i].provenance {
                    key.source_ids.push(p.source_id.clone());
                    key.paths.push(p.path.clone());
                }
            }
            let key = problems.insert(
                key,
                format!(
                    "{} and {} have overlapping declaration intervals",
                    holds[a].1.id, holds[b].1.id
                ),
            );
            evidence_keys[ia].push(key.clone());
            evidence_keys[ib].push(key);
        }
    }
    if !enabled {
        problems.insert(
            ProblemKey {
                code: Code::HoldBehaviorDisabled,
                severity: Severity::Advisory,
                operation: Operation::DisabledFallback,
                subject: Subject::default(),
                source_ids: vec![],
                paths: vec![],
                record_ids: vec![],
            },
            "publication hold behavior disabled; publish every action candidate immediately".into(),
        );
    }
    let (problems, ids) = problems.finish();
    let coverage = if inventory.sources.iter().any(|s| !s.available)
        || problems.iter().any(|p| p.severity == Severity::Unavailable)
    {
        Coverage::Incomplete
    } else {
        Coverage::Complete
    };
    // Cross-record errors are malformed evidence even if their individual
    // fields passed schema validation. A different target label cannot make
    // such evidence safe to ignore. Valid other-target controls have no error.
    let semantic_problem = evidence_keys
        .iter()
        .flatten()
        .any(|k| k.severity == Severity::Semantic);
    let mut records = Vec::new();
    for (i, item) in evidence
        .iter()
        .enumerate()
        .filter(|(_, e)| e.action_candidate)
    {
        let urgency = match item.raw.as_ref().and_then(|r| r.get("urgency")) {
            None if item.raw.is_some() => Urgency::Normal,
            None => Urgency::Invalid,
            Some(v) if v.as_str() == Some("normal") => Urgency::Normal,
            Some(v) if v.as_str() == Some("urgent") => Urgency::Urgent,
            _ => Urgency::Invalid,
        };
        let deferred_by = item.string("deferred_by");
        let hold =
            deferred_by.and_then(|id| holds.iter().find(|(_, h, _)| h.id == id).map(|(_, h, _)| h));
        let invalid_association = deferred_by.is_some_and(|id| {
            hold.is_none() && evidence.iter().any(|e| e.id.as_deref() == Some(id))
        });
        let classification = hold_classify::classify_enabled(
            hold_classify::Input {
                urgency,
                control: item.control().is_some(),
                requires_ack: item.raw.as_ref().is_some_and(|r| r["requires-ack"] == true),
                kind: item.string("kind"),
                coverage,
                semantic_problem,
                deferred_by,
                hold,
                invalid_association,
            },
            enabled,
        );
        let mut keys = evidence_keys[i].clone();
        // Global failure reasons remain attributable from every affected row.
        keys.extend(
            problems
                .iter()
                .filter(|p| {
                    p.severity == Severity::Unavailable
                        || (semantic_problem && p.severity == Severity::Semantic)
                })
                .filter_map(|p| {
                    ids.iter()
                        .find(|(_, id)| *id == &p.problem_id)
                        .map(|(key, _)| key.clone())
                }),
        );
        if !enabled {
            keys.extend(
                ids.keys()
                    .filter(|key| key.code == Code::HoldBehaviorDisabled)
                    .cloned(),
            );
        }
        records.push(Record {
            id: item.id.clone(),
            timestamp: item.string("timestamp").and_then(timestamp),
            provenance: item.provenance.clone(),
            urgency,
            deferred_by: deferred_by.map(str::to_owned),
            hold_id: classification.hold_id,
            state: classification.state,
            reasons: classification.reasons,
            problem_ids: key_ids(&keys, &ids),
            publication_verified: Unverified,
            authority_verified: Unverified,
            required_external_checks: external_checks(),
        });
    }
    records.sort_by(|a, b| {
        a.timestamp
            .as_deref()
            .and_then(utc_timestamp)
            .cmp(&b.timestamp.as_deref().and_then(utc_timestamp))
            .then_with(|| a.id.cmp(&b.id))
            .then_with(|| provenance_order(&a.provenance).cmp(&provenance_order(&b.provenance)))
    });
    let mut holds: Vec<_> = holds
        .into_iter()
        .map(|(i, mut h, _)| {
            h.problem_ids = key_ids(&evidence_keys[i], &ids);
            for end in &mut h.ends {
                end.problem_ids = key_ids(
                    end_keys.get(&end.id).map(Vec::as_slice).unwrap_or(&[]),
                    &ids,
                );
            }
            h
        })
        .collect();
    holds.sort_by(|a, b| {
        utc_timestamp(&a.timestamp)
            .cmp(&utc_timestamp(&b.timestamp))
            .then_with(|| a.id.cmp(&b.id))
            .then_with(|| provenance_order(&a.provenance).cmp(&provenance_order(&b.provenance)))
    });
    let mut sources: Vec<_> = inventory
        .sources
        .into_iter()
        .map(|s| {
            let mut requested_refs = s.requested_refs;
            requested_refs.sort();
            requested_refs.dedup();
            Source {
                problem_ids: {
                    let mut relevant = source_keys.get(&s.source_id).cloned().unwrap_or_default();
                    relevant.extend(
                        ids.keys()
                            .filter(|key| key.source_ids.contains(&s.source_id))
                            .cloned(),
                    );
                    key_ids(&relevant, &ids)
                },
                source_id: s.source_id,
                kind: if s.is_target {
                    SourceKind::Target
                } else {
                    SourceKind::Candidate
                },
                requested_refs,
                sha: s.sha,
                channel: channel.into(),
                status: if s.available {
                    SourceStatus::Read
                } else {
                    SourceStatus::Unavailable
                },
            }
        })
        .collect();
    sources.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    ResultV1 {
        schema_version: 1,
        target_ref: inventory.target_ref,
        target_branch: inventory.target_branch,
        target_remote: inventory.target_remote,
        target_sha: inventory.target_sha,
        now: now.format(&Rfc3339).expect("UTC clock must fit RFC3339"),
        publication_verified: Unverified,
        authority_verified: Unverified,
        required_external_checks: external_checks(),
        scope: Scope::ExplicitSources,
        sources,
        coverage,
        holds,
        records,
        problems,
    }
}
fn temporal(state: hold_status::TemporalState) -> TemporalState {
    match state {
        hold_status::TemporalState::Future => TemporalState::Future,
        hold_status::TemporalState::Active => TemporalState::Active,
        hold_status::TemporalState::Ended => TemporalState::Ended,
        hold_status::TemporalState::Expired => TemporalState::Expired,
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
    fn now() -> OffsetDateTime {
        utc_timestamp("2026-09-12T12:00:00Z").unwrap()
    }
    fn empty() -> hold_git::Inventory {
        hold_git::Inventory {
            target_ref: Some("refs/heads/main".into()),
            target_branch: Some("refs/heads/main".into()),
            target_remote: None,
            target_sha: Some("a".repeat(40)),
            sources: vec![
                hold_git::Source {
                    source_id: "candidate:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                    requested_refs: vec!["HEAD".into()],
                    sha: Some("a".repeat(40)),
                    is_target: false,
                    available: true,
                },
                hold_git::Source {
                    source_id: "target".into(),
                    requested_refs: vec!["main".into()],
                    sha: Some("a".repeat(40)),
                    is_target: true,
                    available: true,
                },
            ],
            blobs: vec![],
            problems: vec![],
        }
    }
    fn blob(
        source: &str,
        id: &str,
        timestamp: &str,
        control: Option<&str>,
        fields: &str,
        oid: char,
    ) -> hold_git::Blob {
        let (kind, from) = if control.is_some() {
            ("decision-record", "governor-reviewer")
        } else {
            ("note", "coder-tester")
        };
        let extra = control
            .map(|s| format!("urgency: urgent\npublication_hold:\n{s}"))
            .unwrap_or_default();
        hold_git::Blob { source_id: source.into(), path: format!("docs/coordination/{id}.md"), oid: oid.to_string().repeat(40),
            bytes: format!("---\nid: {id}\nkind: {kind}\nfrom: {from}\nto: all\ntimestamp: {timestamp}\nimmutable: true\nreferences-to: [hold-start]\ntopic: fixture\nrequires-ack: false\n{extra}{fields}---\n").into_bytes() }
    }
    fn active() -> hold_git::Inventory {
        let mut inv = empty();
        inv.sources[0].source_id = format!("candidate:{}", "d".repeat(40));
        inv.sources[0].sha = Some("d".repeat(40));
        inv.blobs.push(blob("target", "hold-start", "2026-09-12T08:00:00Z", Some(&format!("  action: start\n  target_branch: refs/heads/main\n  target_sha: {}\n  expires_at: 2026-09-12T18:00:00Z\n", "a".repeat(40))), "", 'b'));
        inv.blobs.push(blob(
            &inv.sources[0].source_id,
            "normal-note",
            "2026-09-12T11:00:00Z",
            None,
            "deferred_by: hold-start\n",
            'c',
        ));
        inv
    }
    fn checked(inv: hold_git::Inventory) -> ResultV1 {
        let result = evaluate(inv, "docs/coordination", &schema(), now());
        let contract = agov_core::schema::compile_str(include_str!(
            "../../../../../agf/schemas/coordination-hold-status.schema.json"
        ))
        .unwrap();
        let errors =
            agov_core::schema::validate(&contract, &serde_json::to_value(&result).unwrap());
        assert!(errors.is_empty(), "{errors:?}");
        result
    }
    #[test]
    fn empty_and_active_match_independent_complete_envelopes() {
        for (inv, expected) in [
            (
                empty(),
                include_str!("../tests/fixtures/hold-status/empty.json"),
            ),
            (
                active(),
                include_str!("../tests/fixtures/hold-status/active.json"),
            ),
        ] {
            assert_eq!(
                serde_json::to_value(checked(inv)).unwrap(),
                serde_json::from_str::<serde_json::Value>(expected).unwrap()
            );
        }
    }
    #[test]
    fn target_only_ends_keep_their_own_provenance_and_effective_boundary() {
        let mut inv = active();
        for (id, stamp, source, oid) in [
            ("late-end", "2026-09-12T11:00:00Z", "target", 'e'),
            ("early-end", "2026-09-12T10:00:00Z", "target", 'f'),
        ] {
            inv.blobs.push(blob(
                source,
                id,
                stamp,
                Some("  action: end\n  target_branch: refs/heads/main\n  hold_id: hold-start\n"),
                "",
                oid,
            ));
        }
        let result = checked(inv);
        assert_eq!(result.exit_status(), 0);
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.holds[0].temporal_state, TemporalState::Ended);
        assert_eq!(result.holds[0].effective_end, "2026-09-12T10:00:00Z");
        assert_eq!(
            result.holds[0]
                .ends
                .iter()
                .map(|e| e.id.as_str())
                .collect::<Vec<_>>(),
            ["early-end", "late-end"]
        );
        assert_eq!(result.holds[0].provenance.len(), 1);
        assert_eq!(
            result.holds[0].ends[0].provenance[0].blob_oid,
            "f".repeat(40)
        );
        assert_eq!(result.records[0].reasons, [Reason::EndedHold]);
    }
    #[test]
    fn invalid_end_cannot_silently_terminate_the_hold() {
        let mut inv = active();
        inv.blobs.push(blob(
            "target",
            "bad-end",
            "2026-09-12T07:00:00Z",
            Some("  action: end\n  target_branch: refs/heads/main\n  hold_id: hold-start\n"),
            "",
            'f',
        ));
        let result = checked(inv);
        assert_eq!(result.exit_status(), 1);
        assert!(result.holds[0].ends.is_empty());
        assert_eq!(result.holds[0].temporal_state, TemporalState::Active);
        assert_eq!(result.records[0].state, RecordState::PublishNow);
        assert!(result.records[0].reasons.contains(&Reason::SemanticProblem));
        assert_eq!(result.problems[0].code, Code::InvalidControl);
    }
    #[test]
    fn malformed_candidate_and_unavailable_source_override_every_action() {
        let mut inv = active();
        inv.blobs.push(hold_git::Blob {
            source_id: inv.sources[0].source_id.clone(),
            path: "docs/coordination/bad.md".into(),
            oid: "e".repeat(40),
            bytes: b"bad".to_vec(),
        });
        inv.sources.push(hold_git::Source {
            source_id: "unresolved:missing".into(),
            requested_refs: vec!["missing".into()],
            sha: None,
            is_target: false,
            available: false,
        });
        inv.problems.push(hold_git::ReadProblem {
            code: "git-read-failed",
            operation: "resolve-candidate",
            requested_ref: Some("missing".into()),
            sha: None,
            path: None,
            source_id: "unresolved:missing".into(),
            message: "platform diagnostic".into(),
        });
        let result = checked(inv);
        assert_eq!(result.exit_status(), 2);
        assert_eq!(result.records.len(), 2);
        assert_eq!(result.records[0].id, None);
        assert_eq!(result.records[0].urgency, Urgency::Invalid);
        assert!(result
            .records
            .iter()
            .all(|r| r.state == RecordState::PublishNow
                && r.reasons.contains(&Reason::IncompleteSource)
                && r.reasons.contains(&Reason::SemanticProblem)));
        assert!(result.records.iter().all(|r| r.problem_ids.len() == 2));
    }
    #[test]
    fn migration_fallback_keeps_errors_and_fixed_verification_fields() {
        let result = evaluate_enabled(active(), "docs/coordination", &schema(), now(), false);
        assert_eq!(result.exit_status(), 0);
        assert_eq!(result.records[0].state, RecordState::PublishNow);
        assert_eq!(result.records[0].reasons, [Reason::HoldBehaviorDisabled]);
        assert_eq!(result.problems[0].code, Code::HoldBehaviorDisabled);
        assert_eq!(result.publication_verified, Unverified);
    }
    #[test]
    fn independent_temporal_source_and_action_goldens() {
        for (name, expected) in [
            (
                "expired",
                include_str!("../tests/fixtures/hold-status/expired.json"),
            ),
            (
                "future",
                include_str!("../tests/fixtures/hold-status/future.json"),
            ),
            (
                "stale",
                include_str!("../tests/fixtures/hold-status/stale.json"),
            ),
            (
                "ended",
                include_str!("../tests/fixtures/hold-status/ended.json"),
            ),
            (
                "candidate-control",
                include_str!("../tests/fixtures/hold-status/candidate-control.json"),
            ),
            (
                "base-identical",
                include_str!("../tests/fixtures/hold-status/base-identical.json"),
            ),
        ] {
            let mut inv = active();
            match name {
                "expired" | "future" | "stale" => {
                    let text = String::from_utf8(inv.blobs[0].bytes.clone()).unwrap();
                    let (old, new) = match name {
                        "expired" => ("2026-09-12T18:00:00Z".into(), "2026-09-12T12:00:00Z".into()),
                        "future" => ("2026-09-12T08:00:00Z".into(), "2026-09-12T13:00:00Z".into()),
                        _ => ("a".repeat(40), "e".repeat(40)),
                    };
                    inv.blobs[0].bytes = text.replace(&old, &new).into_bytes();
                }
                "ended" => inv.blobs.push(blob(
                    "target",
                    "hold-end",
                    "2026-09-12T10:00:00Z",
                    Some(
                        "  action: end\n  target_branch: refs/heads/main\n  hold_id: hold-start\n",
                    ),
                    "",
                    'e',
                )),
                "candidate-control" => inv.blobs[0].source_id = inv.sources[0].source_id.clone(),
                "base-identical" => {
                    let mut b = inv.blobs[1].clone();
                    b.source_id = "target".into();
                    inv.blobs.push(b);
                }
                _ => unreachable!(),
            }
            assert_eq!(
                serde_json::to_value(checked(inv)).unwrap(),
                serde_json::from_str::<serde_json::Value>(expected).unwrap(),
                "{name}"
            );
        }
    }

    #[test]
    fn independent_error_goldens_pin_every_reference_and_exit_class() {
        for (name, expected, status) in [
            (
                "unavailable",
                include_str!("../tests/fixtures/hold-status/unavailable.json"),
                2,
            ),
            (
                "conflict",
                include_str!("../tests/fixtures/hold-status/conflict.json"),
                1,
            ),
            (
                "malformed",
                include_str!("../tests/fixtures/hold-status/malformed.json"),
                1,
            ),
        ] {
            let mut inv = active();
            match name {
                "unavailable" => {
                    inv.sources.push(hold_git::Source {
                        source_id: "unresolved:missing".into(),
                        requested_refs: vec!["missing".into()],
                        sha: None,
                        is_target: false,
                        available: false,
                    });
                    inv.problems.push(hold_git::ReadProblem {
                        code: "git-read-failed",
                        operation: "resolve-candidate",
                        requested_ref: Some("missing".into()),
                        sha: None,
                        path: None,
                        source_id: "unresolved:missing".into(),
                        message: "platform diagnostic".into(),
                    });
                }
                "conflict" => {
                    let mut other = inv.blobs[0].clone();
                    other.path = "docs/coordination/other-start.md".into();
                    other.oid = "e".repeat(40);
                    other.bytes = String::from_utf8(other.bytes)
                        .unwrap()
                        .replace("id: hold-start", "id: other-start")
                        .into_bytes();
                    inv.blobs.push(other);
                }
                _ => inv.blobs.push(hold_git::Blob {
                    source_id: inv.sources[0].source_id.clone(),
                    path: "docs/coordination/bad.md".into(),
                    oid: "e".repeat(40),
                    bytes: b"bad".to_vec(),
                }),
            }
            let result = checked(inv);
            assert_eq!(result.exit_status(), status);
            assert_eq!(
                serde_json::to_value(result).unwrap(),
                serde_json::from_str::<serde_json::Value>(expected).unwrap(),
                "{name}"
            );
        }
    }

    #[test]
    fn independent_end_provenance_goldens() {
        for (name, expected) in [
            (
                "candidate-end",
                include_str!("../tests/fixtures/hold-status/candidate-end.json"),
            ),
            (
                "identical-end",
                include_str!("../tests/fixtures/hold-status/identical-end.json"),
            ),
            (
                "candidate-start",
                include_str!("../tests/fixtures/hold-status/candidate-start.json"),
            ),
        ] {
            let mut inv = active();
            let mut end = blob(
                "target",
                "hold-end",
                "2026-09-12T10:00:00Z",
                Some("  action: end\n  target_branch: refs/heads/main\n  hold_id: hold-start\n"),
                "",
                'e',
            );
            if name == "identical-end" {
                inv.blobs.push(end.clone());
            }
            if name == "candidate-start" {
                inv.blobs[0].source_id = inv.sources[0].source_id.clone();
            } else {
                end.source_id = inv.sources[0].source_id.clone();
            }
            inv.blobs.push(end);
            assert_eq!(
                serde_json::to_value(checked(inv)).unwrap(),
                serde_json::from_str::<serde_json::Value>(expected).unwrap(),
                "{name}"
            );
        }
    }
    #[test]
    fn platform_diagnostics_do_not_renumber_final_rows() {
        let reports: Vec<_> = ["git diagnostic on platform A", "different platform B diagnostic"].into_iter().map(|message| {
            let mut inv=active();
            inv.sources[0].available=false;
            inv.problems.push(hold_git::ReadProblem {
                code:"git-read-failed",operation:"read-blob",requested_ref:None,
                sha:Some("d".repeat(40)),path:Some("docs/coordination/unreadable.md".into()),
                source_id:inv.sources[0].source_id.clone(),message:message.into(),
            });
            let report=checked(inv);
            assert_eq!(report.exit_status(),2);
            assert_eq!(report.records[0].problem_ids,vec!["p000001"]);
            assert_eq!(report.records[0].state,RecordState::PublishNow);
            let mut json=serde_json::to_value(report).unwrap();
            for problem in json["problems"].as_array_mut().unwrap() {problem["message"]=serde_json::Value::Null;}
            json
        }).collect();
        assert_eq!(reports[0],reports[1]);
    }

}
