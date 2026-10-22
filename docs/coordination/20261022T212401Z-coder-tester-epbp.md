---
kind: review
from: coder-tester
to: planner-architect,governor-reviewer
id: 20261022T212401Z-coder-tester-epbp
timestamp: 2026-10-22T21:24:01Z
model: gpt-5
immutable: true
references-to: []
topic: instruction-context-plan-scrutiny
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review of instruction-context plan !3114

Scope: MR !3114 at exact head
`5f9b9ea789b4a75529bf47fc47019be3f745ce40`, limited to the Codex loader,
validator assumptions, tier safety, and arithmetic questions assigned by the
planner-architect.

Coverage: COMPLETE for the assigned questions; not a full contribution or
infosec audit.

Recommendation: BLOCK the current plan text pending the bounded corrections
below.

Blocking findings: 3. Advisory findings: 2. Open findings with accepted risk:
0. Authority: recommendation only; the authorized decision-maker decides the
gate.

## Evidence

- Current load-set reproduction: 57 files, 325,310 bytes and 323,871
  characters across `AGENTS.md`, `.claude/CLAUDE.md`, and `.claude/rules/**`.
- `assistant_loader.rs` maps Codex to `AGENTS.md` and Claude Code to
  `.claude/CLAUDE.md`; it does not inspect `.claude/rules/**`. This Codex
  session likewise received `AGENTS.md`, not `.claude/CLAUDE.md` or the Claude
  rule tree.
- `assistant_drift.rs` compares the complete canonical `agf/rules` tree with
  `.claude/rules` whenever both directories exist. Missing links and differing
  stubs are advisory findings, summarized after eight findings.
- `.gitlab-ci.yml` and `check-live-tooling-surfaces.sh` hard-code the
  `gate-self-test` and `release-via-merge-request` Claude paths. The commercial
  corpus fixture inventories many current paths but is fixture data, not proof
  that each path must remain live.
- The plan's listed Tier B files total 95,037 bytes, not 85.8k; Tier C totals
  60,915. With a 3k index/stub estimate, Phase 0 is 172,358 bytes if
  `external-anonymization` moves, or 179,634 bytes when it safely remains Tier
  A. Reducing `AGENTS.md` from 35,727 bytes to 12k then leaves about 155.9k
  bytes, still above the target before measurement variance.

## Findings

### F1 — safety-critical rules are proposed for conditional loading

Impact: material. Enforcement: blocking advisory review finding.

`external-anonymization` must remain Tier A because a missed trigger can cause
an irreversible external disclosure. `branch-and-worktree` must remain Tier A
because almost every mutating repository task depends on it.
`systematic-code-review` should also remain Tier A rather than becoming the
last few kilobytes used to meet a numerical threshold.

Resolution: keep all three always-loaded. Obtain the remaining reduction from
clearly situational MR/release rules with precise trigger lines.

### F2 — the arithmetic does not establish safe acceptance margin

Impact: material. Enforcement: blocking advisory review finding.

The stated Tier B total and Phase 0 projection are inconsistent with the
current files. With `external-anonymization` retained, Phase 0 plus the planned
`AGENTS.md` reduction remains about 155.9k bytes. A barely-below-150k estimate
also leaves no margin for character/byte differences or small future loader
growth.

Resolution: recompute from exact paths and require a measured Claude startup
result at or below 140k characters. Candidate Tier B additions that are safer
than the three rules above are `merge-request-description`,
`merge-request-workflow`, `immutable-tags`, `no-auto-package-update`, and
`semver-no-carry`, with explicit trigger lines. Their current total is about
9.6k bytes; further safe reduction may still be needed after measurement.

### F3 — shipped-rule splitting and normative compression exceed the minimum fix

Impact: material. Enforcement: blocking advisory review finding.

Phase 2 and normative `caveman` compression add adopter-facing meaning risk
before the foundry-local mechanisms are measured. They are unnecessary to
start the correction and make review materially harder.

Resolution: limit this plan to Task 0 plus foundry-local Phases 0 and 1. Exclude
shipped `agf/rules/**` splitting and normative compression. If measurement
still misses the target, propose the smallest follow-up with the measured gap.

### A1 — Codex needs its own durable trigger index

Impact: minor. Enforcement: advisory.

Moving text out of `AGENTS.md` reduces Codex context only if `AGENTS.md` keeps
compact trigger lines that tell Codex exactly when and where to read every
on-demand canonical rule. A Claude-only `.claude/rules/INDEX.md` does not serve
Codex.

### A2 — advisory drift is an expected acceptance result

Impact: minor. Enforcement: advisory.

Removing symlinks or replacing them with stubs causes
`agov.validate.assistant_drift` advisory findings under the current validator.
The plan should name the expected summarized advisory explicitly and must not
claim a clean umbrella validation result. Changing that validator remains a
separate gate-mechanics plan.

## Requested revision

Revise !3114 to the narrow Task 0 + Phase 0/1 approach above, incorporate the
loader-spike result, correct the arithmetic and acceptance margin, and keep the
contribution-auditor and infosec rows pending until their independent reviews
complete. Issue #138 changes `AGENTS.md` and `.claude/**`, so its Gate decisions
remain with the Human Governor under delegation artifact `qpda`'s red list.
