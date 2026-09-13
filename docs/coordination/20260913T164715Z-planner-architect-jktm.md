---
kind: decision-record
from: planner-architect
to: all
id: 20260913T164715Z-planner-architect-jktm
timestamp: 2026-09-13T16:47:15Z
immutable: true
references-to:
  - 20260910T065635Z-coordination-lifecycle-semantics-work-order
  - 20260909T043242Z-build-engineer-dgb2
requires-ack: false
topic: temporary-coder-tester-coverage-four-item-batch
artifacts_owned:
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov-coord/src/
  - agf/schemas/gate1-plan.schema.json
  - docs/plans/
---

# Temporary coder-tester coverage: four-item batch while Codex is asleep

Jani: "take items 1, 2, 3, 4 while codex is sleeping. proceed independently"
(in response to a "what's next" survey listing: (1) Row 5 S4 CP-A
ack-pending display, (2) qbka Slice B legacy-plan migration, (3) the
unresolved `dgb2` OTGR fast-path CI-failure review, (4) Codex's draft
`!2679` CI-performance proposal).

Scope of this coverage, item by item:

1. **Row 5 S4** — implement per the approved work order
   (`20260910T065635Z-...`), own Gate 1 plan → Gate 2, `agov`-only,
   display-only ack-pending marker.
2. **qbka Slice B** — the 12-file legacy-plan migration deferred by qbka
   Slice A (now merged, `superseded` shape frozen). Own Gate 1 plan →
   Gate 2.
3. **`dgb2`** — investigation and closure only. MR !2587's commit-routing
   issue `dgb2` flagged appears already resolved (single commit, green
   pipeline, mergeable) as of this check; confirming and closing the
   pending-ack, not unparking OTGR relay implementation itself (stays
   parked per `otgr-relay-sequencing` until v2.10.1 + v2.11 both ship).
4. **`!2679`** — Codex's own CI-performance proposal, already independently
   reviewed CLEAR (`ssms`) as pre-Gate-1. Next legitimate step per that
   review is a concrete per-slice Gate 1 plan. I will draft the first
   slice's Gate 1 plan as planner-architect (normal division of labor:
   proposal authorship and Gate 1 plan authorship are not required to be
   the same role/session) — not alter Codex's own proposal document or
   claim its authorship.

This is temporary, single-session coverage, not a role reassignment.
Codex remains `coder-tester`; when it returns, ownership of any
in-flight branch from this batch transfers back to normal allocation and
Codex may review or continue any piece independently reviewed rather than
self-reviewed, per the pattern already used for qbka Slice A.
