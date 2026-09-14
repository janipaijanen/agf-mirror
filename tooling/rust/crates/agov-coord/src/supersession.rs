//! Row 5 S2: self-announcing coordination-record supersession.
//!
//! A later coordination record may declare `supersedes: [id, ...]` to
//! retire an earlier record's live-obligation status for the same
//! audience. Coordination records are immutable and append-only, so the
//! earlier record never acquires a field after the fact -- the reverse
//! relationship is always derived here, at read time, never written back.
//!
//! An edge is honored only after all six checks in [`is_valid_direct_edge`]
//! pass; resolution is a chain walk ([`valid_superseder`]) to the current
//! leaf, not a flat scan against the original record -- see that
//! function's own documentation for why the walk (not a flat scan) is
//! what keeps a governor-reviewer override from being silently displaced
//! by a lower-authority correction.
//!
//! See `docs/plans/20260913T195450Z-row5-s2-self-announcing-supersession.md`,
//! "Corrected design (round 2)" and "Correction (round 3)", for the full
//! rationale and the independent reviews that shaped each check.

use crate::handoff_tracker::MessageDescriptor;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

/// Parses a record's declared `timestamp` to a chronological instant.
/// `None` on any missing or unparseable value -- callers must fail safe
/// (treat the edge/ordering as invalid) rather than fall back to string
/// comparison. See S2-I1: RFC3339 lexical order is not chronological
/// order once valid but differing UTC offsets are allowed by the schema
/// (`2026-08-01T11:00:00+02:00` is chronologically `09:00Z`, earlier than
/// `2026-08-01T10:00:00Z`, despite sorting later as a raw string).
fn parsed_timestamp(descriptor: &MessageDescriptor) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(descriptor.msg.timestamp()?, &Rfc3339).ok()
}

/// Returns true iff `s` is a valid direct superseder of `target` (a single
/// hop). All six checks below must pass; any failure means the edge is not
/// honored and `target` is unaffected by `s`.
fn is_valid_direct_edge(s: &MessageDescriptor, target: &MessageDescriptor) -> bool {
    // 1. Existence + causal reference (S2-G1-R2-F1): `s` must name
    //    `target` in BOTH its own `supersedes` array AND its
    //    `references-to` array. `supersedes` alone, without the matching
    //    `references-to` citation, is not honored -- this is what
    //    restores the coordination-publication-rule's own "references
    //    and supersedes" wording.
    if !s.msg.supersedes().iter().any(|id| id == &target.file) {
        return false;
    }
    if !s.msg.references_to().iter().any(|id| id == &target.file) {
        return false;
    }

    // 2. Strict temporal order, compared as parsed instants, not raw
    //    strings (S2-I1 fix). A missing or unparseable timestamp on
    //    either side fails safe: the edge is not honored.
    let (Some(s_ts), Some(target_ts)) = (parsed_timestamp(s), parsed_timestamp(target)) else {
        return false;
    };
    if s_ts <= target_ts {
        return false;
    }

    // 3. No self-edge.
    if s.file == target.file {
        return false;
    }

    // 4. Same audience. Supersession replaces a record for the same
    //    intended audience; a record addressed to a different role
    //    cannot supersede this one.
    let (Some(s_to), Some(target_to)) = (s.msg.to(), target.msg.to()) else {
        return false;
    };
    if s_to != target_to {
        return false;
    }

    // 5. Sender authority (S2-G1-R2-F2): checked against `target` (the
    //    current chain leaf at this hop), not the original candidate.
    //    This is what preserves governor authority across hops -- once
    //    governor-reviewer becomes a leaf, only governor-reviewer (or a
    //    fresh governor-reviewer record) satisfies this check for the
    //    next hop. Matches the existing `reply_evidence` precedent
    //    (`next_agent` or `governor-reviewer`) already established in
    //    this codebase for an analogous "who may act on this record"
    //    question.
    let (Some(s_from), Some(target_from)) = (s.msg.from(), target.msg.from()) else {
        return false;
    };
    if s_from != target_from && s_from != "governor-reviewer" {
        return false;
    }

    true
}

/// Walks the chain of valid direct edges starting at `candidate`, hop by
/// hop, always re-targeting the *current leaf* (never the original
/// `candidate` once a valid hop has been taken). Returns the final leaf if
/// at least one valid hop exists; `None` if `candidate` has no valid
/// direct superseder at all, meaning it stays live.
///
/// A corrector must always name the *current leaf* -- not the original
/// record -- in both `supersedes` and `references-to` to take effect. A
/// record naming only the original root once a leaf already exists fails
/// the existence check at that hop and is not honored, exactly the
/// fail-safe behavior this design already guarantees for any other
/// invalid edge (S2-G1-R2-F2's fix): the original author cannot
/// retroactively displace a governor-reviewer's override by targeting the
/// original record, because after the override becomes the leaf, only an
/// edge targeting the override itself is evaluated, and only the
/// override's own author (`governor-reviewer`) or a fresh
/// governor-reviewer record passes the sender-authority check at that hop.
///
/// At any hop, if multiple candidates satisfy [`is_valid_direct_edge`],
/// selection is, in order:
///
/// 1. Earliest parsed instant wins (S2-I1 fix: chronological, not
///    lexical, comparison).
/// 2. At an exactly equal instant, a `governor-reviewer`-authored
///    candidate wins over a non-`governor-reviewer` one (S2-I2 fix).
///    Without this step, an ordinary author's equal-timestamp record
///    could out-sort a governor-reviewer's equal-timestamp record on id
///    alone, silently discarding a governor override that
///    [`is_valid_direct_edge`]'s own sender-authority check already
///    deemed valid at this hop -- exactly the authority guarantee
///    S2-G1-R2-F2 (and the chain-walk design generally) exists to
///    preserve.
/// 3. Lexicographically smallest `id` is the final, deterministic and
///    stable tie-break, used only when steps 1 and 2 do not distinguish
///    the candidates (e.g. two non-governor candidates at the same
///    instant, or two governor-reviewer candidates at the same instant).
///
/// This function is pure and advisory: it raises no error and blocks
/// nothing. An invalid or absent edge simply means the walk does not take
/// that hop.
pub fn valid_superseder<'a>(
    candidate: &'a MessageDescriptor,
    all: &'a [MessageDescriptor],
) -> Option<&'a MessageDescriptor> {
    let mut leaf: &'a MessageDescriptor = candidate;
    let mut took_hop = false;

    loop {
        let mut direct_successors: Vec<&'a MessageDescriptor> = all
            .iter()
            .filter(|s| is_valid_direct_edge(s, leaf))
            .collect();

        if direct_successors.is_empty() {
            break;
        }

        direct_successors.sort_by(|a, b| {
            // `is_valid_direct_edge` already required both to parse
            // successfully to reach this point.
            let a_ts = parsed_timestamp(a).expect("validated by is_valid_direct_edge");
            let b_ts = parsed_timestamp(b).expect("validated by is_valid_direct_edge");
            let a_is_governor = a.msg.from() == Some("governor-reviewer");
            let b_is_governor = b.msg.from() == Some("governor-reviewer");
            a_ts.cmp(&b_ts)
                .then_with(|| b_is_governor.cmp(&a_is_governor)) // governor (true) sorts first
                .then_with(|| a.msg.id().unwrap_or("").cmp(b.msg.id().unwrap_or("")))
        });

        leaf = direct_successors[0];
        took_hop = true;
    }

    if took_hop {
        Some(leaf)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord_validator::CoordMessage;

    fn descriptor(
        file: &str,
        from: &str,
        to: &str,
        timestamp: &str,
        supersedes: &[&str],
        references_to: &[&str],
    ) -> MessageDescriptor {
        let raw = serde_json::json!({
            "kind": "note",
            "from": from,
            "to": to,
            "id": file.trim_end_matches(".md"),
            "timestamp": timestamp,
            "topic": "test-topic",
            "requires-ack": true,
            "references-to": references_to.iter().map(|r| serde_json::Value::String(r.to_string())).collect::<Vec<_>>(),
            "supersedes": supersedes.iter().map(|r| serde_json::Value::String(r.to_string())).collect::<Vec<_>>(),
        });
        let msg = CoordMessage { raw };
        MessageDescriptor {
            file: file.to_string(),
            sort_key: format!("{timestamp}_{file}"),
            refs: references_to.iter().map(|r| r.to_string()).collect(),
            msg,
        }
    }

    /// `descriptor()` with no `supersedes` field at all (an ordinary,
    /// non-superseding record).
    fn plain(file: &str, from: &str, to: &str, timestamp: &str) -> MessageDescriptor {
        let raw = serde_json::json!({
            "kind": "note",
            "from": from,
            "to": to,
            "id": file.trim_end_matches(".md"),
            "timestamp": timestamp,
            "topic": "test-topic",
            "requires-ack": true,
            "references-to": [],
        });
        MessageDescriptor {
            file: file.to_string(),
            sort_key: format!("{timestamp}_{file}"),
            refs: vec![],
            msg: CoordMessage { raw },
        }
    }

    const X: &str = "20260801T100000Z-coder-tester-xxxx.md";

    #[test]
    fn no_superseder_stays_live() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        assert!(valid_superseder(&x, std::slice::from_ref(&x)).is_none());
    }

    #[test]
    fn valid_single_hop_resolves() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T110000Z-coder-tester-ssss.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let all = vec![x.clone(), s.clone()];
        let result = valid_superseder(&x, &all).expect("expected a superseder");
        assert_eq!(result.file, s.file);
    }

    #[test]
    fn supersedes_without_matching_references_to_is_invalid() {
        // S2-G1-R2-F1 regression: supersedes names X but references-to does not.
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T110000Z-coder-tester-ssss.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[], // missing causal reference
        );
        let all = vec![x.clone(), s];
        assert!(valid_superseder(&x, &all).is_none());
    }

    #[test]
    fn references_to_without_matching_supersedes_is_invalid() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T110000Z-coder-tester-ssss.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[], // missing supersedes entry
            &[X],
        );
        let all = vec![x.clone(), s];
        assert!(valid_superseder(&x, &all).is_none());
    }

    #[test]
    fn non_existent_target_is_invalid() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T110000Z-coder-tester-ssss.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &["20260801T090000Z-nonexistent.md"],
            &["20260801T090000Z-nonexistent.md"],
        );
        // `s` does not name X at all, so X has no superseder.
        let all = vec![x.clone(), s];
        assert!(valid_superseder(&x, &all).is_none());
    }

    #[test]
    fn equal_timestamp_is_invalid() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T100000Z-coder-tester-ssss.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z", // same timestamp, not later
            &[X],
            &[X],
        );
        let all = vec![x.clone(), s];
        assert!(valid_superseder(&x, &all).is_none());
    }

    #[test]
    fn earlier_timestamp_is_invalid() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T090000Z-coder-tester-ssss.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T09:00:00Z", // earlier
            &[X],
            &[X],
        );
        let all = vec![x.clone(), s];
        assert!(valid_superseder(&x, &all).is_none());
    }

    #[test]
    fn self_edge_is_invalid() {
        // A record cannot supersede itself: construct one that names its
        // own file in both supersedes and references-to.
        let x = descriptor(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
            &[X],
            &[X],
        );
        let all = vec![x.clone()];
        assert!(valid_superseder(&x, &all).is_none());
    }

    #[test]
    fn different_audience_is_invalid() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T110000Z-coder-tester-ssss.md",
            "coder-tester",
            "planner-architect", // different `to`
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let all = vec![x.clone(), s];
        assert!(valid_superseder(&x, &all).is_none());
    }

    #[test]
    fn unauthorized_sender_is_invalid() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T110000Z-planner-architect-ssss.md",
            "planner-architect", // neither X's own author nor governor-reviewer
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let all = vec![x.clone(), s];
        assert!(valid_superseder(&x, &all).is_none());
    }

    #[test]
    fn governor_override_is_authorized_regardless_of_original_author() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let g = descriptor(
            "20260801T110000Z-governor-reviewer-gggg.md",
            "governor-reviewer",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let all = vec![x.clone(), g.clone()];
        let result = valid_superseder(&x, &all).expect("governor override must resolve");
        assert_eq!(result.file, g.file);
    }

    #[test]
    fn multi_hop_chain_reaches_final_leaf() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let a = descriptor(
            "20260801T110000Z-coder-tester-aaaa.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let b = descriptor(
            "20260801T120000Z-coder-tester-bbbb.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T12:00:00Z",
            &[a.file.as_str()],
            &[a.file.as_str()],
        );
        let all = vec![x.clone(), a.clone(), b.clone()];
        let result = valid_superseder(&x, &all).expect("expected chain to resolve");
        assert_eq!(result.file, b.file);
    }

    #[test]
    fn governor_before_author_author_cannot_displace_override() {
        // S2-G1-R2-F2 regression, "governor-before-author": governor
        // publishes an override against X first; the original author's
        // later record also names X (not the governor's record) --
        // author's record must NOT displace governor's override.
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let g = descriptor(
            "20260801T110000Z-governor-reviewer-gggg.md",
            "governor-reviewer",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let a = descriptor(
            "20260801T120000Z-coder-tester-aaaa.md",
            "coder-tester", // original author of X
            "governor-reviewer",
            "2026-08-01T12:00:00Z", // later than g
            &[X],                   // targets X directly, not g -- invalid at hop against g
            &[X],
        );
        let all = vec![x.clone(), g.clone(), a];
        let result = valid_superseder(&x, &all).expect("expected governor override to resolve");
        assert_eq!(
            result.file, g.file,
            "governor's override must remain authoritative; the later author record targeting X (not g) must not win"
        );
    }

    #[test]
    fn author_before_governor_governor_must_target_leaf_to_win() {
        // S2-G1-R2-F2 regression, "author-before-governor": the original
        // author corrects X first; governor's later record also names X
        // (not the author's record) -- governor's record is invalid at
        // that hop (wrong leaf target) and does not resolve.
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let a = descriptor(
            "20260801T110000Z-coder-tester-aaaa.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let g = descriptor(
            "20260801T120000Z-governor-reviewer-gggg.md",
            "governor-reviewer",
            "governor-reviewer",
            "2026-08-01T12:00:00Z",
            &[X], // targets X directly, not a's leaf -- invalid at that hop
            &[X],
        );
        let all = vec![x.clone(), a.clone(), g];
        let result =
            valid_superseder(&x, &all).expect("expected author's record to remain the leaf");
        assert_eq!(
            result.file, a.file,
            "governor's record naming the original root (not the current leaf) must not resolve"
        );
    }

    #[test]
    fn a_fresh_governor_record_correctly_targeting_the_leaf_does_win() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let a = descriptor(
            "20260801T110000Z-coder-tester-aaaa.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let g = descriptor(
            "20260801T120000Z-governor-reviewer-gggg.md",
            "governor-reviewer",
            "governor-reviewer",
            "2026-08-01T12:00:00Z",
            &[a.file.as_str()], // correctly targets the current leaf, a
            &[a.file.as_str()],
        );
        let all = vec![x.clone(), a.clone(), g.clone()];
        let result =
            valid_superseder(&x, &all).expect("expected chain to extend to governor's record");
        assert_eq!(result.file, g.file);
    }

    #[test]
    fn equal_timestamp_direct_candidates_break_tie_by_smallest_id() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let a = descriptor(
            "20260801T110000Z-coder-tester-bbbb.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let b = descriptor(
            "20260801T110000Z-coder-tester-aaaa.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z", // identical timestamp to `a`
            &[X],
            &[X],
        );
        let all = vec![x.clone(), a, b.clone()];
        let result = valid_superseder(&x, &all).expect("expected a deterministic winner");
        assert_eq!(
            result.file, b.file,
            "equal-timestamp direct candidates must resolve to the lexicographically smallest id"
        );
    }

    /// S2-I1 regression (`20260914T063028Z-coder-tester-hgrg`): a
    /// schema-valid successor at a non-UTC offset that is actually
    /// EARLIER in real chronological time than the target must NOT be
    /// honored as a valid edge, even though its raw RFC3339 string sorts
    /// later. `2026-08-01T11:00:00+02:00` is `2026-08-01T09:00:00Z` --
    /// one hour before the target's `10:00:00Z`.
    #[test]
    fn mixed_offset_earlier_instant_is_invalid_despite_later_looking_string() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T110000Z-coder-tester-ssss.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00+02:00", // == 2026-08-01T09:00:00Z, earlier than target
            &[X],
            &[X],
        );
        let all = vec![x.clone(), s];
        assert!(
            valid_superseder(&x, &all).is_none(),
            "a chronologically earlier successor must not be honored merely because its \
             raw RFC3339 string sorts later"
        );
    }

    /// S2-I1 regression, positive counterpart: a schema-valid successor
    /// at a non-UTC offset that IS actually later in real chronological
    /// time must be honored, even though its raw string sorts earlier
    /// than the target's.
    #[test]
    fn mixed_offset_later_instant_is_valid_despite_earlier_looking_string() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let s = descriptor(
            "20260801T090000Z-coder-tester-ssss.md",
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T09:00:00-02:00", // == 2026-08-01T11:00:00Z, later than target
            &[X],
            &[X],
        );
        let all = vec![x.clone(), s.clone()];
        let result = valid_superseder(&x, &all)
            .expect("a chronologically later successor must be honored despite an earlier-looking raw string");
        assert_eq!(result.file, s.file);
    }

    /// S2-I2 regression (`20260914T063028Z-coder-tester-hgrg`): at an
    /// exactly equal instant, a governor-reviewer-authored direct
    /// successor must win over a non-governor one, even when the
    /// non-governor id would otherwise sort first alphabetically. This
    /// is the authority guarantee the chain walk exists to preserve
    /// (S2-G1-R2-F2) -- id-only tie-breaking silently discarded it.
    #[test]
    fn equal_instant_governor_candidate_wins_over_lower_sorting_author_id() {
        let x = plain(
            X,
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T10:00:00Z",
        );
        let author = descriptor(
            "20260801T110000Z-coder-tester-aaaa.md", // sorts before the governor id
            "coder-tester",
            "governor-reviewer",
            "2026-08-01T11:00:00Z",
            &[X],
            &[X],
        );
        let governor = descriptor(
            "20260801T110000Z-governor-reviewer-zzzz.md", // sorts after "aaaa" by id alone
            "governor-reviewer",
            "governor-reviewer",
            "2026-08-01T11:00:00Z", // identical instant to `author`
            &[X],
            &[X],
        );
        let all = vec![x.clone(), author, governor.clone()];
        let result = valid_superseder(&x, &all).expect("expected a winner");
        assert_eq!(
            result.file, governor.file,
            "a governor-reviewer-authored candidate must win an equal-instant tie over a \
             non-governor candidate, regardless of id ordering"
        );
    }
}
