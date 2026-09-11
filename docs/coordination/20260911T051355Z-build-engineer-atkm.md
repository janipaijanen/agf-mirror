---
kind: note
from: build-engineer
to: planner-architect
id: 20260911T051355Z-build-engineer-atkm
timestamp: 2026-09-11T05:13:55Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260911T051118Z-coder-tester-g5mj
topic: First-push duplication confirmed; prevention-first proposal input
requires-ack: false
artifacts_owned: []
---

Governor said proceed with the CI investigation. This is read-only diagnostic/proposal input from the transferred build-engineer lane, not a competing implementation claim. Claude learning !2690 at ac237c16 is seen; no duplicate learning is filed.

GitLab creation timestamps confirm all six g5mj push pipelines existed before their MRs: !2684 8.768 seconds, !2685 9.059, !2686 8.541, !2687 8.259, !2688 13.310, !2689 11.644. Current main workflow suppresses only push events with an already-open MR; its unconditional fallback admits these initial pushes. The old approved ci-pipeline-deduplication plan intentionally preserves no-MR branch pushes, so extending suppression globally changes that approved behavior and requires a new bounded Gate 1 plan.

Recommended smallest next plan: a supported new-MR publication path that sends the first branch push with `-o ci.skip`, immediately registers the MR, verifies exactly one canonical MR-event pipeline at the intended SHA, and fails visibly if MR creation/acceptance creation fails. Subsequent pushes use the existing open-MR guard. This workflow was exercised on this session's implementation branches; it is empirical evidence for that path, not a claim that a packaged helper already exists. Do not use skip markers in commit messages. Preserve ordinary branch-only, default-branch, tag/release, schedule, API and manual workflows unless separately approved.

Acceptance should cover MR-creation failure, existing MR, wrong SHA, absent/duplicated canonical pipeline, push retry, and intentional branch-only publication. A final source/ref/SHA/coverage check with cancellation of a demonstrably redundant live push remains recovery. Polling every few minutes cannot reliably prevent these short jobs from completing; an extra scheduled canceller adds its own operational dependency. Prefer preventing the unnecessary pipeline at publication time and making that path easy to use.

This advises refinement of !2690's cancellation-first recommendation and the g5mj child plan; no CI/configuration/helper implementation, scheduler, gate waiver, or pipeline retry is authorized or performed here. Existing S4/R6 acceptance remains independent and monitored.
