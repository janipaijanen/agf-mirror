---
kind: decision-record
from: governor-reviewer
to: all
id: 20260622T202034Z-governor-reviewer-8ma3
timestamp: 2026-06-22T20:20:34Z
model: claude-sonnet-4-6
immutable: true
references-to:
  - 20260622T200555Z-coder-tester-ttrp
  - 20260622T141109Z-coder-tester-e86d
  - 20260622T131002Z-coder-tester-b316
  - 20260622T111737Z-coder-tester-qm7j
  - 20260622T110457Z-coder-tester-qmpj
  - 20260622T103920Z-coder-tester-kddj
  - 20260622T101959Z-coder-tester-21b3
  - 20260622T073408Z-coder-tester-4tk1
  - 20260622T061255Z-coder-tester-3nrs
  - 20260622T054638Z-planner-architect-pj45
  - 20260622T052359Z-coder-tester-4pk4
  - 20260622T050749Z-coder-tester-nr2j
  - 20260622T044037Z-coder-tester-jnr5
  - 20260622T020409Z-coder-tester-rpp7
  - 20260622T015257Z-coder-tester-5kjq
  - 20260622T014608Z-coder-tester-4sek
  - 20260622T012539Z-coder-tester-q71j
  - 20260622T010607Z-coder-tester-ppjr
  - 20260622T005847Z-coder-tester-8t1k
  - 20260622T005408Z-coder-tester-mqjm
  - 20260622T004812Z-coder-tester-rs1q
  - 20260622T002642Z-coder-tester-hhgb
  - 20260622T001623Z-coder-tester-4tss
  - 20260621T221330Z-coder-tester-b65j
  - 20260621T220004Z-coder-tester-b6ej
  - 01KVK7J9N2AW8DJZQAG2
topic: stale-handoff-queue-cleanup
requires-ack: false
artifacts_owned:
  - docs/coordination/20260622T202034Z-governor-reviewer-8ma3.md
---

# Cleanup decision-record: stale handoff queue resolved

All 25 handoffs currently shown as pending by `agov coord status` are resolved
by prior merges. This record closes the queue so the scanner reflects actual
project state.

## Standing decision

A branch merge is sufficient resolution for a handoff. No explicit ack is
required after merge unless a finding from the Gate 2 review remains open.
This applies retroactively to all handoffs listed below.

## Resolved items

All work referenced by the following coordination files was completed and
merged to main. No further action is required on any of them.

- 20260622T141109Z-coder-tester-e86d.md — scenario-router-docs-gate2 (MR !119 merged)
- 20260622T131002Z-coder-tester-b316.md — scenario-router-docs-gate1 (approved, implemented)
- 20260622T111737Z-coder-tester-qm7j.md — coordination-push-audit-log-learning (MR !115 merged)
- 20260622T110457Z-coder-tester-qmpj.md — ci-artifact-retention-storage-reduction (merged 63c7b61)
- 20260622T103920Z-coder-tester-kddj.md — gitlab-issue-8-learning-recorded (issue #8 closed)
- 20260622T101959Z-coder-tester-21b3.md — feat-agov-version-flag (merged to main)
- 20260622T073408Z-coder-tester-4tk1.md — release-v2.1.6 (v2.1.6 released)
- 20260622T061255Z-coder-tester-3nrs.md — issue-6-upgrade-friction-gate2 (issue #6 closed)
- 20260622T054638Z-planner-architect-pj45.md — issue-6-upgrade-friction (closed)
- 20260622T052359Z-coder-tester-4pk4.md — release v2.1.5 Windows patch (released)
- 20260622T050749Z-coder-tester-nr2j.md — windows-release-artifacts (merged)
- 20260622T044037Z-coder-tester-jnr5.md — release 2.1.4 gate2 (released)
- 20260622T020409Z-coder-tester-rpp7.md — rust-source-dist-packaging (merged)
- 20260622T015257Z-coder-tester-5kjq.md — release artifacts implementation boundary (merged)
- 20260622T014608Z-coder-tester-4sek.md — delegated AI work roadmap (merged)
- 20260622T012539Z-coder-tester-q71j.md — external review failure-mode docs (merged)
- 20260622T010607Z-coder-tester-ppjr.md — release 2.1.3 gate2 (released)
- 20260622T005847Z-coder-tester-8t1k.md — three-minute entry docs (merged)
- 20260622T005408Z-coder-tester-mqjm.md — evaluate-lumo-followup-gate2 (merged)
- 20260622T004812Z-coder-tester-rs1q.md — ci-rust-release-docs-gate2 (merged)
- 20260622T002642Z-coder-tester-hhgb.md — release-2.1.2-gate2 (released)
- 20260622T001623Z-coder-tester-4tss.md — principal-delegation-v2x-docs-gate2 (merged)
- 20260621T221330Z-coder-tester-b65j.md — dependency-cve-scan-ci (merged e2b1f4b)
- 20260621T220004Z-coder-tester-b6ej.md — rust-session-resume-output (merged cb14e06)
- 01KVK7J9N2AW8DJZQAG2.md — readable-coordination-ids-implementation-cleared (implemented)

## Scanner note

The current `frontmatter_refs` implementation does not add `.md` to bare IDs
in `references-to:` fields, so this record also lists all filenames in body
text with `.md` suffixes for `body_refs` to pick up until the code fix lands.
See coordination note 20260622T202034Z-governor-reviewer-8ma3.md for the
tooling follow-up.
