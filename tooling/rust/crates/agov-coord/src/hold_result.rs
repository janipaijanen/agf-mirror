//! Publication-hold JSON v1. Declarations never authenticate authority.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

macro_rules! vocabulary {
    ($name:ident { $($variant:ident => $wire:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name { $(#[serde(rename = $wire)] $variant),+ }
        impl $name { pub fn as_str(self) -> &'static str { match self { $(Self::$variant => $wire),+ } } }
        impl Ord for $name { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.as_str().cmp(other.as_str()) } }
        impl PartialOrd for $name { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
    };
}
// A boolean with only one representable value, including during deserialization.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Unverified;
impl Serialize for Unverified {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(false)
    }
}
impl<'de> Deserialize<'de> for Unverified {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if bool::deserialize(deserializer)? {
            Err(serde::de::Error::custom("verification must remain false"))
        } else {
            Ok(Self)
        }
    }
}
vocabulary!(Check {
    Publication => "publication", GovernorAuthority => "governor_authority",
    RecordUrgencyAndObligations => "record_urgency_and_obligations", PublicationTarget => "publication_target",
    CurrentTargetAndExpiry => "current_target_and_expiry", ReleaseHoldAuthorization => "release_hold_authorization"
});
pub fn external_checks() -> Vec<Check> {
    vec![
        Check::CurrentTargetAndExpiry,
        Check::GovernorAuthority,
        Check::Publication,
        Check::PublicationTarget,
        Check::RecordUrgencyAndObligations,
        Check::ReleaseHoldAuthorization,
    ]
}
vocabulary!(Scope { ExplicitSources => "explicit-sources" });
vocabulary!(Coverage { Complete => "COMPLETE", Incomplete => "INCOMPLETE" });
vocabulary!(SourceKind { Target => "target", Candidate => "candidate" });
vocabulary!(SourceStatus { Read => "read", Unavailable => "unavailable" });
vocabulary!(TemporalState { Future => "future", Active => "active", Ended => "ended", Expired => "expired" });
vocabulary!(TargetState { Matching => "matching", Stale => "stale", OtherTarget => "other-target" });
vocabulary!(Urgency { Normal => "normal", Urgent => "urgent", Invalid => "invalid" });
vocabulary!(RecordState { PublishNow => "publish-now", Unassociated => "unassociated", ReviewRequired => "review-required" });
vocabulary!(Reason {
    Urgent => "urgent", HoldControl => "hold-control", RequiresAck => "requires-ack", Handoff => "handoff", Review => "review",
    IncompleteSource => "incomplete-source", SemanticProblem => "semantic-problem", Unassociated => "unassociated",
    MissingHold => "missing-hold", InvalidHold => "invalid-hold", FutureHold => "future-hold", EndedHold => "ended-hold",
    ExpiredHold => "expired-hold", StaleHold => "stale-hold", ConflictingHold => "conflicting-hold",
    ExternalReviewRequired => "external-review-required", HoldBehaviorDisabled => "hold-behavior-disabled"
});
vocabulary!(Code {
    InvalidInput => "invalid-input", GitReadFailed => "git-read-failed", NonregularEntry => "nonregular-entry",
    InvalidRecord => "invalid-record", PathConflict => "path-conflict", IdConflict => "id-conflict",
    InvalidControl => "invalid-control", MissingStart => "missing-start", ActiveOverlap => "active-overlap",
    HistoricalOverlap => "historical-overlap", FutureOverlap => "future-overlap", HoldBehaviorDisabled => "hold-behavior-disabled"
});
vocabulary!(Severity { Unavailable => "unavailable", Semantic => "semantic", Advisory => "advisory" });
vocabulary!(Operation {
    ResolveTarget => "resolve-target", ResolveCandidate => "resolve-candidate", ReadTree => "read-tree", ReadBlob => "read-blob",
    ValidateRecord => "validate-record", ResolveHold => "resolve-hold", ClassifyRecord => "classify-record", DisabledFallback => "disabled-fallback"
});
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultV1 {
    pub schema_version: u8,
    pub target_ref: Option<String>,
    pub target_branch: Option<String>,
    pub target_remote: Option<String>,
    pub target_sha: Option<String>,
    pub now: String,
    pub publication_verified: Unverified,
    pub authority_verified: Unverified,
    pub required_external_checks: Vec<Check>,
    pub scope: Scope,
    pub sources: Vec<Source>,
    pub coverage: Coverage,
    pub holds: Vec<Hold>,
    pub records: Vec<Record>,
    pub problems: Vec<Problem>,
}
impl ResultV1 {
    pub fn exit_status(&self) -> i32 {
        if self.coverage == Coverage::Incomplete
            || self
                .problems
                .iter()
                .any(|p| p.severity == Severity::Unavailable)
        {
            2
        } else if self
            .problems
            .iter()
            .any(|p| p.severity == Severity::Semantic)
        {
            1
        } else {
            0
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub source_id: String,
    pub kind: SourceKind,
    pub requested_refs: Vec<String>,
    pub sha: Option<String>,
    pub channel: String,
    pub status: SourceStatus,
    pub problem_ids: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub source_id: String,
    pub path: String,
    pub blob_oid: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hold {
    pub id: String,
    pub timestamp: String,
    pub target_branch: String,
    pub target_sha: String,
    pub expires_at: String,
    pub effective_end: String,
    pub ends: Vec<End>,
    pub declared_release_references: Vec<String>,
    pub temporal_state: TemporalState,
    pub target_state: TargetState,
    pub conflict: bool,
    pub provenance: Vec<Provenance>,
    pub problem_ids: Vec<String>,
    pub publication_verified: Unverified,
    pub authority_verified: Unverified,
    pub required_external_checks: Vec<Check>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct End {
    pub id: String,
    pub timestamp: String,
    pub hold_id: String,
    pub target_branch: String,
    pub provenance: Vec<Provenance>,
    pub problem_ids: Vec<String>,
    pub publication_verified: Unverified,
    pub authority_verified: Unverified,
    pub required_external_checks: Vec<Check>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub id: Option<String>,
    pub timestamp: Option<String>,
    pub provenance: Vec<Provenance>,
    pub urgency: Urgency,
    pub deferred_by: Option<String>,
    pub hold_id: Option<String>,
    pub state: RecordState,
    pub reasons: Vec<Reason>,
    pub problem_ids: Vec<String>,
    pub publication_verified: Unverified,
    pub authority_verified: Unverified,
    pub required_external_checks: Vec<Check>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    #[serde(rename = "ref")]
    pub requested_ref: Option<String>,
    pub sha: Option<String>,
    pub path: Option<String>,
    pub record_id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Problem {
    pub problem_id: String,
    pub code: Code,
    pub severity: Severity,
    pub operation: Operation,
    pub subject: Subject,
    pub message: String,
    pub source_ids: Vec<String>,
    pub paths: Vec<String>,
    pub record_ids: Vec<String>,
}
/// Opaque stable identity used by rows until the complete inventory is known.
/// Diagnostic prose is deliberately absent from the identity and ordering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProblemKey {
    pub code: Code,
    pub severity: Severity,
    pub operation: Operation,
    pub subject: Subject,
    pub source_ids: Vec<String>,
    pub paths: Vec<String>,
    pub record_ids: Vec<String>,
}
#[derive(Default)]
pub struct Problems(BTreeMap<ProblemKey, BTreeSet<String>>);
impl Problems {
    pub fn insert(&mut self, mut key: ProblemKey, message: String) -> ProblemKey {
        for values in [&mut key.source_ids, &mut key.paths, &mut key.record_ids] {
            values.sort();
            values.dedup();
        }
        self.0.entry(key.clone()).or_default().insert(message);
        key
    }
    pub fn finish(self) -> (Vec<Problem>, BTreeMap<ProblemKey, String>) {
        let mut ids = BTreeMap::new();
        let problems = self
            .0
            .into_iter()
            .enumerate()
            .map(|(i, (key, messages))| {
                let problem_id = format!("p{:06}", i + 1);
                ids.insert(key.clone(), problem_id.clone());
                Problem {
                    problem_id,
                    code: key.code,
                    severity: key.severity,
                    operation: key.operation,
                    subject: key.subject,
                    source_ids: key.source_ids,
                    paths: key.paths,
                    record_ids: key.record_ids,
                    message: messages.into_iter().collect::<Vec<_>>().join("\n"),
                }
            })
            .collect();
        (problems, ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verification_cannot_be_deserialized_as_authority() {
        assert!(serde_json::from_str::<Unverified>("true").is_err());
        assert!(serde_json::from_str::<Unverified>("null").is_err());
        assert_eq!(serde_json::to_string(&Unverified).unwrap(), "false");
        assert_eq!(
            serde_json::from_str::<Unverified>("false").unwrap(),
            Unverified
        );
    }
    #[test]
    fn diagnostic_wording_and_encounter_order_do_not_change_identity() {
        let key = ProblemKey {
            code: Code::GitReadFailed,
            severity: Severity::Unavailable,
            operation: Operation::ReadTree,
            subject: Subject {
                sha: Some("a".repeat(40)),
                ..Subject::default()
            },
            source_ids: vec!["target".into(), "target".into()],
            paths: vec![],
            record_ids: vec![],
        };
        let mut a = Problems::default();
        let canonical = a.insert(key.clone(), "platform z".into());
        a.insert(key.clone(), "platform a".into());
        a.insert(key.clone(), "platform z".into());
        let (problems, ids) = a.finish();
        let mut b = Problems::default();
        b.insert(key, "different diagnostic".into());
        let (other, other_ids) = b.finish();
        assert_eq!(ids, other_ids);
        assert_eq!(ids[&canonical], "p000001");
        assert_eq!(problems[0].message, "platform a\nplatform z");
        assert_eq!(problems[0].problem_id, other[0].problem_id);
        assert_eq!(problems[0].source_ids, vec!["target"]);
    }
    #[test]
    fn vocabularies_sort_by_wire_spelling_and_checks_are_complete() {
        let checks = external_checks();
        assert_eq!(checks.len(), 6);
        assert!(checks.windows(2).all(|p| p[0] < p[1]));
        assert!(Code::ActiveOverlap < Code::InvalidInput);
        assert!(Severity::Advisory < Severity::Unavailable);
    }
}

#[cfg(test)]
mod contract_tests {
    use super::*;
    use agov_core::schema;
    fn contract() -> schema::CompiledSchema {
        schema::compile_str(include_str!(
            "../../../../../agf/schemas/coordination-hold-status.schema.json"
        ))
        .unwrap()
    }
    #[test]
    fn independent_empty_golden_round_trips_and_pins_required_nulls() {
        let value: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/hold-status/empty.json")).unwrap();
        let result: ResultV1 = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(result.exit_status(), 0);
        assert_eq!(serde_json::to_value(result).unwrap(), value);
        let schema = contract();
        assert!(schema::validate(&schema, &value).is_empty());
        for field in value.as_object().unwrap().keys() {
            let mut missing = value.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(
                !schema::validate(&schema, &missing).is_empty(),
                "missing key {field} was accepted"
            );
        }
        for pointer in ["/publication_verified", "/authority_verified"] {
            let mut elevated = value.clone();
            *elevated.pointer_mut(pointer).unwrap() = true.into();
            assert!(!schema::validate(&schema, &elevated).is_empty());
            assert!(serde_json::from_value::<ResultV1>(elevated).is_err());
        }
        let mut extra = value.clone();
        extra["delay_approved"] = true.into();
        assert!(!schema::validate(&schema, &extra).is_empty());
        assert!(serde_json::from_value::<ResultV1>(extra).is_err());
        let mut wrong_checks = value;
        wrong_checks["required_external_checks"]
            .as_array_mut()
            .unwrap()
            .pop();
        assert!(!schema::validate(&schema, &wrong_checks).is_empty());
    }
    #[test]
    fn unavailable_has_precedence_over_semantic_and_advisory_findings() {
        let mut result: ResultV1 =
            serde_json::from_str(include_str!("../tests/fixtures/hold-status/empty.json")).unwrap();
        let mut problems = Problems::default();
        problems.insert(
            ProblemKey {
                code: Code::InvalidRecord,
                severity: Severity::Semantic,
                operation: Operation::ValidateRecord,
                subject: Subject::default(),
                source_ids: vec![],
                paths: vec![],
                record_ids: vec![],
            },
            "invalid".into(),
        );
        result.problems = problems.finish().0;
        assert_eq!(result.exit_status(), 1);
        result.coverage = Coverage::Incomplete;
        assert_eq!(result.exit_status(), 2);
        result.coverage = Coverage::Complete;
        result.problems[0].severity = Severity::Unavailable;
        assert_eq!(result.exit_status(), 2);
        result.problems[0].severity = Severity::Advisory;
        assert_eq!(result.exit_status(), 0);
    }
}
