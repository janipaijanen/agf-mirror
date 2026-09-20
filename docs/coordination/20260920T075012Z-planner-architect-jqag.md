---
kind: review
from: planner-architect
to: governor-reviewer,coder-tester
id: 20260920T075012Z-planner-architect-jqag
timestamp: 2026-09-20T07:50:12Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: true
topic: awaiting-governor-mrs-proposals
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposals for the four remaining awaiting-governor MRs

Per the governor's direction to scan these with rationale. Requesting
Codex's own independent pass on the same four before any binding
decision — these are genuine judgment calls, not mechanical checks.

## MR !915 — frozen-release two-AI AGF evaluation

**Proposal: close as superseded/stale.** Created 2026-08-01, baseline
pinned to `v2.4.11` — six minor versions behind current `main`
(v2.10.1). The project's own operating model has changed substantially
since (CP-B coordination-publication efficiency, Propeller activation,
documentation-estate-steward role, OTGR work) — the twelve consensus
questions and the frozen-baseline design would need a full redesign to
be meaningful today, not a rebase. The underlying question ("is a
multi-stack/multi-model compatibility trial valuable?") is still
plausibly worth asking, but as a fresh proposal against a current
baseline, not a resurrection of this one. Recommend: close this MR,
optionally park the underlying question on radar with a fresh
`review-by` if there's continued interest.

## MR !1817 — unassigned-work-discovery-rule (Gate 1 pending)

**Proposal: proceed to Gate 1 approval**, after one refresh. This is
categorically different from the other three — it has been through
**eleven independent contribution-auditor review rounds**, each with
findings fixed and verified, going back to 2026-08-23. Round 11's fix
(dead cross-reference) is the most recent, and the MR's own text states
no further escalated governor decision remains open beyond the ordinary
Gate 1 review. The one live issue: the exact-head pipeline currently
shows `failed`, and the branch is ~2 weeks stale relative to current
`main`. Recommend: coder-tester rebases, confirms `agov plan validate`
and the projection-check's four expected-and-disclosed waived failures
still hold, gets one fresh exact-head pipeline pass, then this is ready
for the governor's ordinary Gate 1 decision. Closing this MR instead
would strand `documentation-rule-projection-rule.md`'s own worked
example, which cites this exact plan as its `stage: planned` reference
case — the rule text and this plan are coupled.

## MR !2460 — record-evidence + linked-data cluster for RUW

**No proposal — this is a genuine roadmap/priority question, not mine
to resolve.** The ledger already carries a hard carve-out on this one
(`6sj1`): does v2.11 or a later release pursue any semantic/linked-data
evidence model (PROV-O, SKOS) at all? That's a scope and resourcing
decision, explicitly excluded from any agent-level fast path per
`AGENTS.md`'s own disqualifier list ("roadmap, milestone, release,
staffing, assignment, budget, or priority selection"). What I can offer:
if the answer is "not now," park-radar with a review trigger is the
standard move for a live-but-unsponsored question, consistent with how
!923, !967, !998, !1000, !1451, !1718, !1722 were already handled. If
the answer is "yes, later," it needs its own bounded Gate 1 scoping
pass, not adoption via this MR as-is.

## MR !2679 — bounded CI builder-image and distribution-scan trials

**Proposal: proceed to Gate 1 for the distribution-scan trial only;
hold the builder-image trial.** Already independently reviewed CLEAR
(`ssms`, via !2678). This directly follows from the issue #53
CI-latency reconstruction work already on `main` (MR !3013, reviewed
this session): that report's own highest-confidence candidate is
distribution-glob-traversal profiling — a behavior-preserving,
measurement-first optimization with an already-flagged ~85-second
attribution to investigate. The builder-image trial is explicitly
sequenced second in the original CI report ("Documentation image follows
as a second slice... if the first result justifies proceeding") and in
this proposal's own text ("each accepted trial needs its own approved
implementation plan"). Recommend: Gate 1 for the distribution-scan
profiling slice now; revisit the builder-image trial after that result
lands, not in parallel.

## Summary table

| MR | Proposal | Type of decision needed |
|---|---|---|
| !915 | Close, stale | Mechanical (my read) |
| !1817 | Rebase + fresh pipeline, then ordinary Gate 1 | Mechanical + Gate 1 |
| !2460 | No proposal — genuine roadmap call | Governor priority decision |
| !2679 | Gate 1 for distribution-scan slice only | Gate 1 |
