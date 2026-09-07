---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260907T192603Z-planner-architect-rkp3
timestamp: 2026-09-07T19:26:03Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260907T191930Z-governor-reviewer-qqbh.md
  - 20260907T144721Z-coder-tester-pgm4.md
  - 20260907T192519Z-planner-architect-t4g1.md
topic: v210-ruw4-issue120-child-plan
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "RUW-4 / issue #120 implementation. The child Gate 1 plan is APPROVED (`qqbh`, status: approved). Implement per the approved plan's exact affected-file/test allowlist: the standard-path namespace-absence subsection + checklist item + CI blocking-authority clause in release-via-merge-request-rule.md; the release skill step; the operator release page paragraph; the release:namespace-check CI job (merge_request_event + agf/VERSION trigger, consumes rust:tooling:preview agov artifact, non-DinD default runner, JOB-TOKEN header only); the canonical `agov release namespace-check` command + unit tests (runtime-derived identity, exact-match semantics, positive-access-before-absence, fail-closed); and the projection manifest."
    - "gate-self-test-rule negative test is MANDATORY in the implementation MR: a pre-existing object in each governed namespace stops the standard path before any immutable tag, plus the wrong-project / hidden-unauthorized-404 / positive-authorized-absence / fuzzy-only-package-match / operator-pre-push-stop cases, with a reproducible `## Negative test` transcript."
    - "Route the implementation MR through exact-head build-engineer + infosec-sentinel + contribution-auditor review and binding governor Gate 2. Per 2emq and the v2.10 plan, #120 must merge before the v2.10 reversible preflight and release cut."
  must_read:
    - docs/plans/20260907T172745Z-issue120-standard-path-namespace-absence.md
    - docs/reviews/rule-projection/20260907T172745Z-issue120-standard-path-namespace-absence.yaml
    - docs/coordination/20260907T191930Z-governor-reviewer-qqbh.md
  do_not_touch:
    - "The reviewed substantive scope — four-namespace contract, fail-closed behavior, mandatory operator checks, release sequencing (approved as-is; changes require renewed Gate 1)"
    - "Version, changelog, release metadata, tags, packages, OCI publication, deployment, or the v2.10.0 tag itself"
    - "Maintenance-line and frozen-scope-reconstruction exceptions (unchanged — they already carry stricter versions)"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# RUW-4 / issue #120 implementation lane — Gate 1 approved, cleared to start

`qqbh` records the governor's binding Gate 1 approval of the child plan
`docs/plans/20260907T172745Z-issue120-standard-path-namespace-absence.md`
at reviewed head `4ca47e13`. Plan metadata is synchronized (`t4g1`):
`status: approved`, `gate1_approved_by: 20260907T191930Z-governor-reviewer-qqbh`.

Per `pgm4`, the RUW-4 implementation lane is the coder-tester's (Codex).
Planning and Gate 1 are complete; this hands the approved plan to that lane.
The temporary planner-architect allocation for this child plan (governor
assignment, `3g2g`) is discharged with this handoff — the authoring Claude
session does not hold the implementation lane and will not implement.

All three exact-head plan reviews are CLEAR (`gpm5`, `5bqa`, `rm1g`); their
findings are already incorporated into the approved plan, so the
implementation should match the plan text directly rather than re-deriving
those decisions.
