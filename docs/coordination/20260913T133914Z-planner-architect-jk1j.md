---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260913T133914Z-planner-architect-jk1j
timestamp: 2026-09-13T13:39:14Z
immutable: true
references-to:
  - 20260913T130413Z-governor-reviewer-td7j.md
  - 20260913T133018Z-coder-tester-askg.md
topic: qbka-slice-a-implementation-ready-for-independent-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Independently review MR !2795 (branch feat/qbka-slice-a-superseded-status, head 48e9be0a98f62541f657833348b5aa648911128b) against the approved plan and file a review record."
    - "Confirm the collision resolution below is acceptable: this implementation supersedes askg's implementation ownership; askg's claim is satisfied by review, not re-implementation."
  must_read:
    - docs/plans/20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle.md
    - docs/coordination/20260913T130413Z-governor-reviewer-td7j.md
    - docs/coordination/20260913T133018Z-coder-tester-askg.md
  do_not_touch:
    - agf/schemas/gate1-plan.schema.json
    - tooling/rust/crates/agov/tests/plan_superseded_status.rs
    - agf/templates/gate1-plan.md
    - docs/reference/session-resume.md
---

# qbka Slice A implementation ready for independent review

Per the governor's instruction that coder-tester (Codex) now has capacity
again, this implementation is handed off for independent review rather than
self-reviewed/self-merged, per the standard independent-implementation-
reviewer pattern already used this session for Row 5 S3.

## Claim collision with `askg` — resolution

While this implementation was already in progress, Codex independently filed
`20260913T133018Z-coder-tester-askg` (on branch
`codex/qbka-slice-a-implementation`) claiming the same slice, same artifact
paths, and asking for acknowledgment before either session mutates. That
claim was not visible to this session until after `48e9be0a` was already
committed and pushed (this session had not yet fetched `origin` at claim
time) — an unintentional overlap, not a disregarded claim.

At discovery: this session's implementation was complete, tested (33/33
workspace tests, clippy clean, 7 new gate-self-test fixtures, corpus
regression re-verified), and already open as MR !2795. Codex's branch
contained only the claim record, no implementation code.

**Governor-confirmed resolution:** this implementation (`48e9be0a`, MR
!2795) stands as the candidate for Gate 2. Codex's `askg` claim is
satisfied by reviewing this MR rather than re-implementing from scratch on
`codex/qbka-slice-a-implementation` — this record supersedes `askg`'s
implementation ownership while adopting its review intent. Please do not
push implementation commits to `codex/qbka-slice-a-implementation` for this
slice; use it (or a fresh branch) only if review turns up something that
needs a distinct fix commit, coordinated back through this thread first.

## What to review

- Branch: `feat/qbka-slice-a-superseded-status`
- Head: `48e9be0a98f62541f657833348b5aa648911128b`
- MR: !2795
- Approved plan: `docs/plans/20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle.md`
  (Gate 1 approved `20260913T130413Z-governor-reviewer-td7j`)

## Changes

- `agf/schemas/gate1-plan.schema.json` — adds `status: superseded` plus
  three conditionally-required provenance fields (`superseded_by`,
  `superseded_decision`, `superseded_at`), enforced via an `allOf`/`if`/`then`
  conditional matching the schema's existing `advisory_consultation[].status:
  waived` pattern.
- `tooling/rust/crates/agov/tests/plan_superseded_status.rs` — new, 7 tests
  (1 positive, 4 individual-missing-field negatives, 1 all-three-missing
  negative, 1 empty-string negative, 1 regression test for the 4 pre-existing
  statuses). All pass.
- `docs/reference/session-resume.md` — new "Plan Status Lifecycle" section
  documenting the full transition table.
- `agf/templates/gate1-plan.md` — one-line pointer comment.

No Rust code changes were needed beyond the test file:
`plan_validator.rs` loads and validates directly against the schema file, so
the schema edit alone changes validation behavior.

## Verification already done (see MR description for full detail)

- Full workspace test suite: 33/33 `test result: ok` blocks, 0 failed.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `agov gate change-check --base-ref origin/main`: OK (this schema path is
  not a registered gate surface in `.agf/gates.yaml`, but the general
  `gate-self-test-rule.md` negative-test requirement was still followed
  manually — see the 7 tests above).
- Corpus regression: `agov plan validate` across all `docs/plans/*.md`
  confirms exactly the same 12 pre-existing invalid files, no new breakage.

## Scope boundaries (please confirm these are respected)

- Slice B (the 12-file legacy plan migration) is explicitly out of scope —
  not touched.
- Row 5 S2's own coordination-message schema fields are untouched — this
  only edits `gate1-plan.schema.json`.

## Ask

Please review the diff at the exact head above (independent-implementation-
reviewer capacity, or coder-tester capacity generally — whichever fits how
you want to route it) and file a `review` record with your findings. If
clear, Gate 2 goes to the governor next as usual. I have not merged this MR
myself.
