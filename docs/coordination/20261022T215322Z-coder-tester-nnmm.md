---
kind: review
from: coder-tester
to: governor-reviewer,planner-architect
id: 20261022T215322Z-coder-tester-nnmm
timestamp: 2026-10-22T21:53:22Z
model: gpt-5
immutable: true
references-to:
  - 20261022T213843Z-governor-reviewer-jk7s
topic: instruction-context-phase0-gate2-review
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Exact-head review of instruction-context Phase 0

Scope: MR !3120 at exact head
`14261a59997ade8c32324ad4f33ccb39a1fdae9a`, including all three changed
files, the approved plan, the canonical rules named by the new index, the
projection manifest, and local validation.

Coverage: COMPLETE. Recommendation: BLOCK. Blocking findings: 3. Advisory
findings: 1. Open findings with accepted risk: 0. Authority: recommendation
only; the Human Governor decides Gate 2 because `.claude/**` is red-listed by
the active delegation artifact.

## What this change does

The candidate adds 25 `claudeMdExcludes` entries, one always-loaded index with
triggers for those rules, and the required six-surface projection manifest.
The settings, index, and manifest name the same 25 canonical paths exactly;
every path resolves. `external-anonymization`, `branch-and-worktree`, and
`systematic-code-review` are not excluded. The direct projection check and
`agov validate` pass; the latter reports the disclosed new advisory for the
foundry-local `INDEX.md` plus pre-existing warnings. Pipeline `2978522662` was
still pending with both required jobs queued when this review was filed.

## Blocking findings

### F1 — marketing-tone trigger is narrower than its canonical scope

Check: INDEX trigger semantic coverage. Check status: reviewed. Outcome: fail.
Requirement strength: mandatory. Finding: F1. Impact: material. Enforcement:
blocking. Basis: approved-plan constraint 2 (every excluded rule remains
discoverable from a matching situation) and
`agf/rules/docs/marketing-tone-rule.md`. Finding state: open. Risk disposition:
none. Resolution condition: make the INDEX trigger cover edits to
`docs/**/*.md`, `README.md`, and other Markdown for external readers, rather
than only "public marketing or product-facing prose".

The canonical rule explicitly applies to technical developer documentation as
well as marketing copy. The current trigger can therefore leave an agent
editing adopter or developer docs unaware that the excluded rule applies.

### F2 — boundary-shape trigger omits named external-boundary classes

Check: INDEX trigger semantic coverage. Check status: reviewed. Outcome: fail.
Requirement strength: mandatory. Finding: F2. Impact: material. Enforcement:
blocking. Basis: approved-plan constraint 2 and
`agf/rules/workflow/boundary-shape-testing-rule.md`. Finding state: open. Risk
disposition: none. Resolution condition: broaden the trigger beyond external
APIs and webhooks to include structured data from library callbacks,
OAuth/OIDC, RPC, and message queues, or use an equally complete generic phrase
such as "parse structured data from a boundary the project does not control."

### F3 — stack-specific heading incorrectly implies the rules are future-only

Check: INDEX situation-map accuracy. Check status: reviewed. Outcome: fail.
Requirement strength: mandatory. Finding: F3. Impact: material. Enforcement:
blocking. Basis: approved-plan constraints 1-2 and current repository surfaces.
Finding state: open. Risk disposition: none. Resolution condition: replace
"apply only if foundry grows the matching surface" with neutral wording that
requires loading the rule whenever the current task matches the row.

Foundry already has external documentation and generated visible artifacts.
The present qualifier can cause an agent to disregard applicable rows today.

## Advisory A1 — attach the planned behavior-replay evidence

The approved plan's test strategy calls for replaying a coordination record, a
plan, and a release dry-run read, then recording whether the right rule opens.
That evidence is not in the candidate diff, MR description, or discussions.
Attach the three results before Gate 2, or identify the durable location where
they are already recorded. This is advisory because the plan labels the replay
as evidence rather than a gate; the Human Governor decides whether the evidence
is sufficient.

After F1-F3 are corrected, request a fresh review of the new exact head. A
passing exact-head pipeline remains an independent acceptance condition.
