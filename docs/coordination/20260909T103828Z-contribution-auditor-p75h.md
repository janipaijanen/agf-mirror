---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260909T103828Z-contribution-auditor-p75h
timestamp: 2026-09-09T10:38:28Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T051907Z-coder-tester-ebgh.md
  - 20260909T074906Z-governor-reviewer-33dn.md
  - 20260909T102151Z-governor-reviewer-rqqh.md
topic: propeller-stage4-mr2597-ea9924ef-contribution-review
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Contribution review — MR !2597 (Propeller Stage 4 activation) @ `ea9924ef`

**Author disclosure.** Claude session, directed by `33dn` / `ebgh` to run
the non-author contribution audit of this Codex-authored implementation.

Reviewed exact head: `ea9924ef32bb05b5c9889b174046ed79bcb87f51` (MR !2597,
merge-base `d0674e3f`, three-dot diff).
Coverage: COMPLETE. **Recommendation: CLEAR.** Blocking: 0. Advisory: 0.
Open accepted risks: 0. Authority: advisory review only; the governor
decides Gate 2.

## Faithful implementation of approved plan `20260909T015237Z-propeller-stage4-activation`

| Plan requirement | `ea9924ef` |
|---|---|
| `agf/agents/MANIFEST.yaml` `improvement-propeller.status: planned → active`; reason names player, schedule, state path, baselines, plan | Present. `status: active`, `played_by: "Codex, under governor decision …djh1"`; reason names Codex, Monday 09:00 UTC, `tmp/codex-improvement-propeller-state.json`, learning baseline 62 through `…ev5m`, coord lower bound `…dt4s`, release baseline `v2.9.0` peeled `ee27978b…`, and the plan. Matches the plan's confirmed parameters exactly. |
| `agf/agents/improvement-propeller.yaml` `0.1.0 → 0.2.0`; phase stays 4; `framework_version` stays `2.10.0` | Present and exact: `id`/`version` → `0.2.0`, `phase 4` unchanged, `framework_version: "2.10.0"` unchanged. |
| `AGENTS.md` stale "Planned/inactive" paragraph → truthful active foundry status, preserving adopter separate-activation | Present. New paragraph states "Active for foundry … Level 4 Validated pending a genuine unattended scheduled-wake proof" and "each adopter requires its own governor decision …". Adopter-nonactivation preserved. |
| `docs/roadmap.md` + `agf/docs/site/roadmap.md` atomically → Level 4 Validated (active), Level 5 pending genuine wake; no split-update fallback | Both files updated in this one MR. #131 rows now read "Stage 4 Gate 1 approved; tracked activation in progress … Only its first genuine unattended conforming wake can supply separate Level 5 evidence. Adopters remain inactive …". Consistent across both. |
| `agf/CHANGELOG.md` — the `rqqh`-authorized `Adopter docs: not applicable` waiver; no version bump; no feature entry | Present verbatim in `## [Unreleased]`: foundry-only, adopters inactive. No `VERSION` change, no feature changelog line. |
| Projection evidence `docs/reviews/rule-projection/20260909T015237Z-propeller-stage4-activation.yaml` | Already on `main` (merged with the approved plan), unchanged by this MR — correct per the plan's advisory note. |
| No rule / schema / validator / CI / release / tag / template / other-agent-definition change | Confirmed. Three-dot diff touches only: `AGENTS.md`, `agf/CHANGELOG.md`, `agf/agents/MANIFEST.yaml`, `agf/agents/improvement-propeller.yaml`, `docs/roadmap.md`, `agf/docs/site/roadmap.md`, and three coordination handoff files (`ebgh`/`sjk2`/`4ms4`, pure additions requesting this MR's reviews). |

## Authority-boundary and maturity claims

- **No Level 5 claim anywhere.** Every surface (MANIFEST reason, AGENTS.md,
  both roadmaps, CHANGELOG) states Level 4 Validated (active) and defers
  Level 5 explicitly to a genuine unattended scheduled wake. Matches
  `improvement-propeller-rule.md` ("Level 5 Deployable only after the
  supported real wake") and the plan's Level 5 boundary section.
- **No runtime state, no heartbeat, no automation** in the tracked diff —
  all deferred to the post-merge sequence, as the plan requires. The MR
  description's "Post-merge boundary" reproduces the plan's 5-step
  create/view-back/store sequence faithfully.
- **No `gate-self-test-rule` claim.** The MR description states the change
  "modifies no mechanical gate"; the plan forbids claiming a validator that
  does not exist. Confirmed — the table-top is presented as table-top
  acceptance evidence, not a gate-self-test.
- Table-top in the MR description covers all canonical
  `improvement-propeller-rule.md` cases plus planned-inert/active-visible,
  busy-wake→latch→one-cycle, month+release→one budget,
  malformed/symlink/wrong-mode/stale state→fail-closed+retain-cursors,
  view-back mismatch→pause+Level 4, and deactivation→pause-by-ID.

## No adversarial steering / no authority regression

The change activates an existing role via the human-governor red-list path
(Gate 1 `djh1` + this exact-head Gate 2). It grants the Propeller no new
authority; `improvement-propeller-rule.md`'s MUST-NOT list
(assign/prioritize/implement/merge/approve/self-review/URL-deref) is
untouched. Adopter projects remain inactive by explicit statement on three
surfaces.

`agov plan validate` (approved plan), `agov validate agents`, and the
umbrella `agov validate` are reported OK in the MR description; exact-head
pipeline `ea9924ef` has every governance/agov job green (see the build
review `q5pk` for the pipeline detail). Governor decides Gate 2.
