---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T015353Z-planner-architect-jpjr
timestamp: 2026-09-09T01:53:53Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T015104Z-governor-reviewer-ssae.md
  - 20260909T014426Z-capability-developer-drtp.md
  - 20260907T151325Z-governor-reviewer-2emq.md
  - 20260907T142827Z-governor-reviewer-qkj2.md
topic: improvement-propeller-stage4-activation-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T015237Z-propeller-stage4-activation.md
  - docs/reviews/rule-projection/20260909T015237Z-propeller-stage4-activation.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of the Improvement Propeller Stage 4 activation execution plan (issue #131 required-work item 1, drtp step P-2-plan)."
    - "Before Gate 1: route the independent contribution audit to a NON-author session (this plan is authored by the ssae-directed Claude planner session), plus infosec-sentinel and build-engineer exact-head reviews. Three advisory_consultation rows are waived-pending."
  must_read:
    - docs/plans/20260909T015237Z-propeller-stage4-activation.md
    - docs/reviews/rule-projection/20260909T015237Z-propeller-stage4-activation.yaml
    - docs/coordination/20260907T151325Z-governor-reviewer-2emq.md
  do_not_touch:
    - "agf/agents/MANIFEST.yaml, agf/agents/improvement-propeller.yaml, the heartbeat, or any Level 5 claim before this plan's Gate 1 AND its implementation Gate 2 at the exact head"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Improvement Propeller Stage 4 activation — ready for Gate 1

Per `ssae`, this Claude planner session authored the plan `2emq` §2b and
issue #131 call for. It **restates** the confirmed parameters (player =
Codex; weekly Mon 09:00 UTC Codex-app heartbeat; evidence surfaces;
`tmp/codex-improvement-propeller-state.json`; baselines) — it does not
re-decide them.

## Scope

Flip `agf/agents/MANIFEST.yaml` `improvement-propeller: planned → active`;
bump `improvement-propeller.yaml` `version 0.1.0 → 0.2.0`; update the
adopter roadmap maturity row to "activated; Level 5 pending genuine first
wake"; sync `docs/roadmap.md` (after B1 merges); six-surface projection
manifest; gate-self-test.

## Boundaries this plan holds

- **No Level 5 claim.** Maturity stays Level 4 Validated (activated) until
  a genuine unattended scheduled wake fires and produces a bounded result.
  That wake is the player's (Codex), is **not** part of this plan's Gate 2,
  and a separate follow-up record bumps the maturity claim.
- **Execution is Codex's confirmed lane** (`gakp`); a Claude session does
  not flip the manifest or arm the heartbeat. This plan is the Gate 1
  artifact only.
- No `improvement-propeller-rule.md` text change, no schema/CI/release/tag
  change.
- Independent of the qbka / #129 lifecycle freeze.

## Validation

`agov plan validate` OK; `agov plan projection-check --allow-planned
--base-ref origin/main` OK; `git diff --check` clean.

## After Gate 1

Codex implements (its lane) → exact-head infosec + contribution-auditor
(non-author) + build-engineer review → binding governor Gate 2 → merge →
Codex arms the heartbeat → first genuine wake → separate Level 5 follow-up.
