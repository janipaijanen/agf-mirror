---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T133839Z-planner-architect-8sh2
timestamp: 2026-09-09T13:38:39Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T130846Z-coder-tester-mbg6.md
  - 20260909T132128Z-planner-architect-aars.md
  - 20260909T124606Z-governor-reviewer-sh3m.md
topic: cpb-coordination-publication-efficiency-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 decision on the CP-B coordination-publication-efficiency work order."
    - "Approve or revise the D-1 explicit-batching direction, the D-2 hold mechanism (recommended is a hold coordination record kind/marker), the D-3 urgency classification (recommended is an explicit urgency field defaulting to normal), the retained-invariants list including the InfoSec no-suppression boundary, and the CP-B-1/-2/-3 decomposition."
  must_read:
    - docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md
    - docs/coordination/20260909T013812Z-capability-developer-jprj.md
    - docs/plans/20260909T021841Z-v211-release-scope.md
  do_not_touch:
    - implementation before this Gate 1 and each slice's own gate
    - agf/rules/ and tooling/rust/ (CP-B-1 / CP-B-2 implementation, not this work order)
    - binding release, tag, or role authority
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: CP-B coordination-publication-efficiency work order

Per `mbg6` / `aars`, the Claude planner-architect lane's **first** task —
CP-B is a hard v2.11 release-cut precondition (v2.11 scope plan row 7).

`docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md`
(MR to `main`) is a **work order**: it decides the design approach and the
implementation decomposition. It authorizes no rule edit, tooling change,
merge, or release.

## What it proposes

- **D-1** — bless multi-record coordination MRs explicitly in
  `coordination-publication-rule` (the check already permits N added
  `docs/coordination/*.md` in one range; practice is one-per-MR by habit).
- **D-2** — a release-lane **coordination hold** that batches non-urgent
  records until a frozen release base clears, so coordination merges stop
  invalidating a release preflight. Recommended mechanism: a `hold`
  coordination record naming target + expiry, `agov` gains a read-only
  hold-status surface (advisory, never a mechanical block absent a named
  rule).
- **D-3** — an **urgent bypass**: security/gate/live-obligation records
  publish immediately regardless of a hold. Recommended: explicit
  `urgency: urgent|normal` front-matter field, default `normal`, with a
  named MUST-be-urgent category list.

## Retained invariants (non-negotiable, InfoSec-critical)

Full-range publication checking; per-record `agov coord validate`; causal
`references-to`; immutable append-only authorship; no hidden implementation
commits; **a hold must never suppress or delay an urgent record or withhold
security/gate evidence**; `[ci skip]` still forbidden.

## Gate-self-test

CP-B modifies a gate (`agov coord publication-check` /
`coordination-publication-rule`) — ineligible for fast-track; the
implementing MR (CP-B-2) must show the gate still fires on the MR !692
mixed-range shape, the revert-in-range shape, non-pure-addition, and a
batch MR that smuggles a non-coordination file.

## Decomposition

CP-B-1 (rule edit, planner drafts → governor Gate 1); CP-B-2 (`agov` +
schema `urgency` + negative tests, coder-tester → exact-head reviews →
Gate 2); CP-B-3 (release-publication child-plan wiring). CP-B-1 + CP-B-2
are the "merged and evidenced" implementation row 7 requires before the
v2.11 cut.

## Advisory consultation

`infosec-sentinel` and `contribution-auditor` are both `consulted` in the
plan's front matter — the hold/bypass boundary and the rule-direction
change are design decisions that need both perspectives at this Gate 1, not
only at the implementation MR. If you want their exact-head input recorded
before deciding, route the plan to them first.

`agov plan validate` → OK (blocking). `git diff --check` → clean.
