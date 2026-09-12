---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260912T160245Z-planner-architect-s6qj
timestamp: 2026-09-12T16:02:45Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T160141Z-planner-architect-jj88
  - 20260912T155648Z-v2101-ruw-generic-artifact-validation
requires-ack: true
topic: v2101-implementation-assignment
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement docs/plans/20260912T155648Z-v2101-ruw-generic-artifact-validation.md exactly as approved. Gate 1 approved, version confirmed v2.10.1 (jj88).
    - Own the CLI/schema/crate design decisions the plan leaves open (exact command surface, crate placement) within the plan's named boundary; do not expand into A2/A4-domain/A5/A6/A7.
    - Produce the Gate 2 evidence items the plan lists (fixtures, negative cases, path-containment tests, deterministic-output test, regression checks).
    - Request a real infosec-sentinel pass on path/glob containment before presenting Gate 2 — the plan's infosec entry is a self-assessment, not a substitute.
  must_read:
    - docs/plans/20260912T155648Z-v2101-ruw-generic-artifact-validation.md
    - docs/proposals/labdns-customer-agf-enhancement-distillation.md
  do_not_touch:
    - v2.11/RUW work (stays paused per 31pr)
    - agf/VERSION, release branches, tags (release-via-merge-request-rule applies after this scope merges)
    - A2, A4's domain/governance-authority rules, A5, A6, A7
---

# v2.10.1 implementation assignment

Gate 1 approved and version confirmed v2.10.1 per Jani's direct
instruction, transcribed in `jj88`. Per Jani's separate instruction
("plan 2.10.1... and then implement with codex"), implementation is
yours. I retain planning and build-review (CI-mechanics correctness
once your implementation lands), per our existing role split.

Scope is exactly what the plan names: `artifact_sets` config (A1),
`agov artifact validate`/`index` (A3), and A4's structural subset
(reference existence, acyclicity, ID uniqueness) — nothing else from
the distillation. The plan leaves exact CLI shape and crate placement
(new crate vs. extending `agov-validators`) to your implementation
judgment within that boundary.

I'm available for build/CI-mechanics consultation as you go, same
pattern as the CPB1/cargo-deny work. No coordination-only MR needed for
routine progress notes — batch per the existing batching guidance
unless something is independently blocking.
