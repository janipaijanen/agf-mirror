//! Conservative action classification. A declaration is never permission.
use crate::hold_result::{
    Coverage, Hold, Reason, RecordState, TargetState, TemporalState, Urgency,
};

pub struct Input<'a> {
    pub urgency: Urgency,
    pub control: bool,
    pub requires_ack: bool,
    pub kind: Option<&'a str>,
    pub coverage: Coverage,
    pub semantic_problem: bool,
    pub deferred_by: Option<&'a str>,
    pub hold: Option<&'a Hold>,
    pub invalid_association: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Classification {
    pub state: RecordState,
    pub hold_id: Option<String>,
    pub reasons: Vec<Reason>,
}

pub fn classify(input: Input<'_>) -> Classification {
    classify_enabled(input, true)
}

/// Internal migration fixture only. There is deliberately no operator switch
/// that can declare the external checks completed or permit publication delay.
pub(crate) fn classify_enabled(input: Input<'_>, enabled: bool) -> Classification {
    let mut reasons = Vec::new();
    let hold_id = input.hold.map(|h| h.id.clone());
    if input.urgency == Urgency::Urgent {
        reasons.push(Reason::Urgent);
    }
    if input.control {
        reasons.push(Reason::HoldControl);
    }
    if input.requires_ack {
        reasons.push(Reason::RequiresAck);
    }
    match input.kind {
        Some("handoff") => reasons.push(Reason::Handoff),
        Some("review") => reasons.push(Reason::Review),
        _ => {}
    }
    if input.coverage == Coverage::Incomplete {
        reasons.push(Reason::IncompleteSource);
    }
    if input.semantic_problem || input.urgency == Urgency::Invalid {
        reasons.push(Reason::SemanticProblem);
    }
    if !enabled {
        reasons.push(Reason::HoldBehaviorDisabled);
    }
    let state = if !reasons.is_empty() {
        RecordState::PublishNow
    } else if input.deferred_by.is_none() {
        reasons.push(Reason::Unassociated);
        RecordState::Unassociated
    } else if input.invalid_association {
        reasons.push(Reason::InvalidHold);
        RecordState::PublishNow
    } else if let Some(hold) = input.hold {
        match hold.temporal_state {
            TemporalState::Future => reasons.push(Reason::FutureHold),
            TemporalState::Ended => reasons.push(Reason::EndedHold),
            TemporalState::Expired => reasons.push(Reason::ExpiredHold),
            TemporalState::Active => {}
        }
        match hold.target_state {
            TargetState::Stale => reasons.push(Reason::StaleHold),
            TargetState::OtherTarget => reasons.push(Reason::InvalidHold),
            TargetState::Matching => {}
        }
        if hold.conflict {
            reasons.push(Reason::ConflictingHold);
        }
        if reasons.is_empty() {
            reasons.push(Reason::ExternalReviewRequired);
            RecordState::ReviewRequired
        } else {
            RecordState::PublishNow
        }
    } else {
        reasons.push(Reason::MissingHold);
        RecordState::PublishNow
    };
    reasons.sort();
    reasons.dedup();
    Classification {
        state,
        hold_id,
        reasons,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hold_result::{external_checks, Unverified};
    fn hold() -> Hold {
        Hold {
            id: "start".into(),
            timestamp: "2026-09-12T00:00:00Z".into(),
            target_branch: "refs/heads/main".into(),
            target_sha: "a".repeat(40),
            expires_at: "2026-09-13T00:00:00Z".into(),
            effective_end: "2026-09-13T00:00:00Z".into(),
            ends: vec![],
            declared_release_references: vec![],
            temporal_state: TemporalState::Active,
            target_state: TargetState::Matching,
            conflict: false,
            provenance: vec![],
            problem_ids: vec![],
            publication_verified: Unverified,
            authority_verified: Unverified,
            required_external_checks: external_checks(),
        }
    }
    fn input(hold: Option<&Hold>) -> Input<'_> {
        Input {
            urgency: Urgency::Normal,
            control: false,
            requires_ack: false,
            kind: Some("note"),
            coverage: Coverage::Complete,
            semantic_problem: false,
            deferred_by: Some("start"),
            hold,
            invalid_association: false,
        }
    }
    #[test]
    fn exhaustive_structural_and_global_overrides_cannot_be_delayed() {
        let h = hold();
        for urgent in [false, true] {
            for control in [false, true] {
                for ack in [false, true] {
                    for kind in ["note", "handoff", "review"] {
                        for incomplete in [false, true] {
                            for semantic in [false, true] {
                                let mut i = input(Some(&h));
                                i.urgency = if urgent {
                                    Urgency::Urgent
                                } else {
                                    Urgency::Normal
                                };
                                i.control = control;
                                i.requires_ack = ack;
                                i.kind = Some(kind);
                                i.coverage = if incomplete {
                                    Coverage::Incomplete
                                } else {
                                    Coverage::Complete
                                };
                                i.semantic_problem = semantic;
                                let result = classify(i);
                                let overridden = urgent
                                    || control
                                    || ack
                                    || kind != "note"
                                    || incomplete
                                    || semantic;
                                assert_eq!(
                                    result.state,
                                    if overridden {
                                        RecordState::PublishNow
                                    } else {
                                        RecordState::ReviewRequired
                                    }
                                );
                                assert_eq!(result.hold_id.as_deref(), Some("start"));
                                assert_eq!(
                                    result.reasons.contains(&Reason::IncompleteSource),
                                    incomplete
                                );
                                assert_eq!(
                                    result.reasons.contains(&Reason::SemanticProblem),
                                    semantic
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn temporal_target_and_conflict_states_are_independent() {
        for temporal in [
            TemporalState::Future,
            TemporalState::Active,
            TemporalState::Ended,
            TemporalState::Expired,
        ] {
            for target in [
                TargetState::Matching,
                TargetState::Stale,
                TargetState::OtherTarget,
            ] {
                for conflict in [false, true] {
                    let mut h = hold();
                    h.temporal_state = temporal;
                    h.target_state = target;
                    h.conflict = conflict;
                    let result = classify(input(Some(&h)));
                    assert_eq!(
                        result.state == RecordState::ReviewRequired,
                        temporal == TemporalState::Active
                            && target == TargetState::Matching
                            && !conflict
                    );
                    assert_eq!(result.reasons.contains(&Reason::ConflictingHold), conflict);
                    assert_eq!(
                        result.reasons.contains(&Reason::StaleHold),
                        target == TargetState::Stale
                    );
                }
            }
        }
    }
    #[test]
    fn missing_invalid_and_unassociated_have_distinct_meanings() {
        let missing = classify(input(None));
        assert_eq!(missing.hold_id, None);
        assert_eq!(missing.reasons, [Reason::MissingHold]);
        let mut invalid = input(None);
        invalid.invalid_association = true;
        assert_eq!(classify(invalid).reasons, [Reason::InvalidHold]);
        let mut ordinary = input(None);
        ordinary.deferred_by = None;
        assert_eq!(classify(ordinary).state, RecordState::Unassociated);
        let mut bad = input(None);
        bad.deferred_by = None;
        bad.urgency = Urgency::Invalid;
        assert_eq!(classify(bad).state, RecordState::PublishNow);
    }
    #[test]
    fn disabled_behavior_preserves_failures_and_never_delays_any_population() {
        let h = hold();
        for association in [None, Some("start")] {
            for incomplete in [false, true] {
                let mut i = input(Some(&h));
                i.deferred_by = association;
                i.coverage = if incomplete {
                    Coverage::Incomplete
                } else {
                    Coverage::Complete
                };
                let result = classify_enabled(i, false);
                assert_eq!(result.state, RecordState::PublishNow);
                assert!(result.reasons.contains(&Reason::HoldBehaviorDisabled));
                assert_eq!(
                    result.reasons.contains(&Reason::IncompleteSource),
                    incomplete
                );
            }
        }
    }
}
