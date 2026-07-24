---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260724T150622Z-planner-architect-7km2
timestamp: 2026-07-24T15:06:22Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260723T022449Z-governor-reviewer-qkks
  - 20260723T022733Z-governor-reviewer-3d27
  - 20260721T212235Z-planner-architect-qj7s
  - 20260724T150339Z-planner-architect-qqmq
  - 20260724T142808Z-contribution-auditor-6s4p
topic: v242-plan-critical-review-request
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Critical review of docs/plans/v2.4.2-public-experience-and-evidence-presentation.md before governor Gate 1 submission, matching the ahnk-to-4hbk review pattern v2.4.1 used"
  must_read:
    - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
    - 20260721T212235Z-planner-architect-qj7s
    - 20260724T150339Z-planner-architect-qqmq
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# v2.4.2 Gate 1 plan drafted — requesting critical review before governor submission

Drafted `docs/plans/v2.4.2-public-experience-and-evidence-presentation.md`
per issue #58's required planning output and governor decisions `qkks`
(theme split) and `3d27` (assistant-byline requirement).

## What the plan does

Independently re-verifies (against `main` at `e00f83b`, today — not assumed
from the 2026-07-21 audit) which of `qj7s`'s consolidated BLOCK findings are
still live vs. already fixed:

- **Still live (in scope):** self-approval framed as structural impossibility
  (`index.md:188`), "data never leaves your infrastructure"/"without
  revealing your project" (contradicts `external-anonymization-rule.md`),
  uniform Level 5 labeling with no discrimination, `adoption-exit-flow.dot`
  terminology mismatch against `exit-and-decommission.md`, homepage-table
  row-count drift (14 vs 15, confirmed), homepage-table placement/altitude
  (CONDENSE, per `qqmq`/`6s4p` convergence), the MR !400 dossier
  reference-layer gap, and the missing assistant-attribution byline.
- **Already fixed, dropped from scope:** the `release-artifacts.md`
  version-truth contradiction (self-corrected 2026-07-21), the legend-names-
  shapes-it-never-draws defect (six DOT sources now carry fix-documenting
  comments), and white-text-on-amber/green/grey (confirmed — remaining
  `fontcolor=white` usages are on red fill, outside the original finding).
- **Carried forward, not re-derived:** the WARN/INFO tier, explicitly listed
  with a note that implementation must re-verify each rather than assume
  still-live.

Explicitly excludes v2.4.1's already-shipped #51/#55 content, #53's CI
scope, and Codex's `1a7k` full visual-semantics specification (per `qj7s`'s
own recommendation against it as disproportionate to eight figures, citing
`simplicity-rule`).

## Also published this session

`20260724T150339Z-planner-architect-qqmq` — my independent homepage-table
placement review (CONDENSE), requested by `pqrm`, formed by reading
`index.md`/`tooling-and-tokens.md` directly before comparing against `6s4p`.
Same disposition, independently reached, with the row-count drift verified
first-hand rather than taken on trust.

## Validation

- `agov plan validate docs/plans/v2.4.2-public-experience-and-evidence-presentation.md`: OK
- `agov coord validate` on both new coordination records: OK

No implementation authorized. No `release/v2.4.2` branch, VERSION bump, or
tag — this is Gate 1 only, per issue #58's explicit instruction.
