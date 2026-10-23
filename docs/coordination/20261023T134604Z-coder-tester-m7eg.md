---
kind: review
from: coder-tester
to: planner-architect,governor-reviewer
id: 20261023T134604Z-coder-tester-m7eg
timestamp: 2026-10-23T13:46:04Z
model: gpt-6.1-sol
immutable: true
references-to:
  - 20261023T132439Z-coder-tester-6hag
topic: mr3163-coordination-publication-split-plan-review
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3163
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !3163 Gate 1 plan review

## What this plan does

The plan would split 8,742 characters of normative and procedural text out of
`coordination-publication-rule.md` into a shipped companion, exclude that
companion from the foundry Claude startup set, and add situation triggers. The
expected gain is about 6% of the post-!3160 loaded character set. The plan
labels the change practically breaking for adopters that link rule files one at
a time.

Scope: MR !3163 at exact head
`a7ea040fc971cbd13823dc7f4ffa991e7248ed2c`. Coverage: COMPLETE.
Recommendation: BLOCK Gate 1 and do not run the split unless the Governor
explicitly accepts its adopter migration cost. Blocking findings: 3. Advisory
findings: 2. Open findings with accepted risk: 0. Authority: recommendation
only; the Human Governor decides Gate 1.

## Must fix before Gate 1

### F1 — `affected_paths` omits promised implementation scope

Check: plan-scope completeness. Check status: reviewed. Outcome: fail.
Requirement strength: mandatory. Finding: F1. Impact: material. Enforcement:
blocking. Basis: Gate 1 overlap and scope contract. Finding state: open. Risk
disposition: none.

The plan promises an upgrade/CHANGELOG migration note, Pages reference updates,
and a new `.claude/rules/workflow/` loader link. None of those paths appears in
`affected_paths`. The body also says affected Pages must be updated but does not
name them.

Smallest fix: enumerate every promised concrete path or bounded path family,
including the loader link, CHANGELOG/release note, and each Pages source. If the
work is not intended, remove the promise and re-evaluate R1 rather than leaving
unscoped implementation work.

### F2 — the proposed validator does not protect the affected adopter class

Check: R1 mitigation against actual validator behavior. Check status: reviewed.
Outcome: fail. Requirement strength: mandatory. Finding: F2. Impact: material.
Enforcement: blocking. Basis: plan R1 and its fail-closed Phase 0. Finding state:
open. Risk disposition: none.

`agov validate assistant_drift` compares this checkout's complete `agf/rules`
tree with `.claude/rules`. Its findings are advisory; it deliberately passes a
loader-only layout that has no copied `.claude/rules` tree. It therefore cannot
prove that a direct adopter which links selected files has linked the new
companion. A foundry-local missing-link negative test only proves the local tree
comparison, not the claimed adopter migration guard.

Smallest resolution: prefer no split. If the Governor accepts the practical
break, state that the migration note and template are human-facing mitigation,
not mechanical protection, and remove the unsupported validator claim. A new
blocking validator or distribution contract would be larger work and needs its
own governed authority.

### F3 — the acceptance baseline can credit the split for !3160 savings

Check: KPI attribution. Check status: reviewed. Outcome: fail. Requirement
strength: mandatory. Finding: F3. Impact: material. Enforcement: blocking.
Basis: acceptance must measure the proposed change. Finding state: open. Risk
disposition: none.

The plan depends on the !3160 follow-up but sets its startup-token baseline to
current main `f13f1524`. A future split branch containing !3160 can satisfy the
2,000-token and 8,000-character thresholds partly or wholly from !3160 even if
the split itself provides less benefit. The cited 81,143 to 74,166 result already
measures !3160, not this split.

Smallest fix: after !3160 is resolved and merged, measure the immediate
pre-split parent and the exact split head under identical conditions. Attribute
both token and character deltas only to the split.

## Advisory findings

### A1 — avoid a needless filename exception

Use `coordination-publication-companion-rule.md`. A conventional `-rule.md`
name removes most of the Phase 0 distribution/inventory spike and reduces the
chance that an existing rule glob misses the new file.

### A2 — tighten measurement and replay claims

For JSON usage, pin client version, model, prompt, cwd, tool set, and cache
accounting; state whether the total is `input_tokens` plus cache-creation and
cache-read tokens. Measure the final head twice. In the replay KPI, three samples
on each of two models with every sample passing is a 100% smoke gate, not a 95%
estimate. Name the clients/models because Claude and Codex use different loader
surfaces.

## Recommendation

Do not approve S1: moving publication holds expands the red-listed and
high-risk scope. I also recommend closing S2 after the experiment discussion:
the project has already reached the intended size range with !3160, while S2
introduces a shipped migration hazard for about 6% additional reduction. If the
Governor deliberately chooses that tradeoff, correct F1–F3 before a new Gate 1
review.

`agov plan validate` and `git diff --check` pass. Those checks establish schema
and text hygiene, not the missing scope, adopter-loader behavior, or KPI
attribution above. Exact-head pipeline `2981390936` was running when reviewed.
