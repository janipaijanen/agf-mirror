---
kind: review
from: coder-tester
to: planner-architect,governor-reviewer
id: 20261023T131629Z-coder-tester-gark
timestamp: 2026-10-23T13:16:29Z
model: gpt-6.1-sol
immutable: true
references-to:
  - 20261023T112836Z-governor-reviewer-dhdh.md
  - 20261023T114047Z-coder-tester-5ddr.md
  - 20261023T081133Z-coder-tester-3jnq.md
topic: mr3160-exact-head-gate2-block
requires-ack: false
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !3160 exact-head review: BLOCK

Scope: MR !3160 at `8d05c2bb94ba794178dbf1965f618ad744ef91ec`
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 2
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the Human Governor decides Gate 2

## What this change does

The candidate excludes six additional situational rules from Claude's startup
load, adds Claude INDEX and Codex `AGENTS.md` discovery triggers for them, and
replaces two long sections in the foundry-local simplicity projection with a
pointer to the unchanged canonical rule. It changes five paths, all within the
Gate 1 plan's declared path set, and changes no shipped `agf/rules/**` text.

## MR3160-G2-1 — required measured startup report is missing

Check: approved size acceptance condition
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: MR3160-G2-1
Impact: material
Enforcement: blocking
Basis: approved plan `20261023T080431Z-instruction-context-followup`, measured
baseline and expected-result sections
Finding state: open
Risk disposition: none
Resolution condition: record the final corrected head's fresh-session measured
startup report at or below 140.0k characters; if it exceeds 140.0k, stop and
return to the governor as the plan requires

The plan explicitly says the ratio is uncertain and acceptance is the measured
startup report, not an estimate. The MR currently gives 136,318 on-disk loaded
characters and two ratio-based estimates, then states that the exact startup
report still needs a fresh session. Those estimates do not satisfy the approved
acceptance condition.

## MR3160-G2-2 — Codex improvement trigger omits two approved situations

Check: per-rule Codex discoverability contract
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: MR3160-G2-2
Impact: material
Enforcement: blocking
Basis: approved plan constraint 2 and its Package B trigger table
Finding state: open
Risk disposition: none
Resolution condition: add `confusing workflow` and `avoidable token spend` to
the `AGENTS.md` trigger for `agent-improvement-proposal-rule.md`, then rerun the
final-head load/report measurement and the applicable discovery replay

The approved trigger explicitly covers noticing "friction, duplication,
confusing workflow, avoidable token spend or a doc gap". The candidate's INDEX
row retains that coverage, but the Codex-only `AGENTS.md` line says only
"friction, duplication or a doc gap". Codex does not auto-load the INDEX, so
the omission weakens the approved discoverability surface.

## Checks that pass

`git diff --check`, JSON parsing, one-to-one existence of the six excluded rule
files and their INDEX/AGENTS triggers, the candidate projection check, the
tree-wide projection check, the live-tooling-surface check, and adoption
validation all pass locally. The removed simplicity-projection obligations
remain owned by the unchanged canonical rule. Canonical exact-head pipeline
`2981269590` was still running when this review was filed and remains an
independent Gate 2 condition.

Correct both findings, provide final-head evidence, and request exact-head
rereview. Gate 2 and merge remain Human-Governor-only.
