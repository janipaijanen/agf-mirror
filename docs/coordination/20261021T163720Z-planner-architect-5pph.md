---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261021T163720Z-planner-architect-5pph
timestamp: 2026-10-21T16:37:20Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: third-party-plugin-coexistence-doc
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/adopt/third-party-plugin-coexistence.md
  - agf/docs/site/docs/adopt/assistant-loader-configuration.md
  - docs/reviews/rule-projection/20261021T163241Z-third-party-plugin-coexistence.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 2 review and merge decision for branch
      docs/third-party-plugin-coexistence
  must_read:
    - agf/docs/site/docs/adopt/third-party-plugin-coexistence.md
    - docs/reviews/rule-projection/20261021T163241Z-third-party-plugin-coexistence.yaml
  do_not_touch:
    - .claude/settings.local.json
---

# New adopter-docs page: coexisting with third-party assistant plugins

The user asked whether a draft page on running AGF alongside a third-party
coding-style plugin (worked example: Ponytail,
https://github.com/DietrichGebert/ponytail, release v5.3.2) was sufficient,
then asked for the full revision.

## What changed

- New page: `agf/docs/site/docs/adopt/third-party-plugin-coexistence.md`
  (Level 1 Planned, Theoretical evidence status — guidance only, no
  tooling). Gives a loader precedence snippet (governor/AGF authority above
  plugin coding-style instructions), a coexistence checklist, and a
  Ponytail worked example.
- One-line "Next" pointer added to
  `agf/docs/site/docs/adopt/assistant-loader-configuration.md` so the new
  page is discoverable from the existing loader-setup page.
- Rule-projection evidence per `documentation-rule-projection-rule.md`:
  `docs/reviews/rule-projection/20261021T163241Z-third-party-plugin-coexistence.yaml`,
  validated with `agov plan projection-check --base-ref origin/main`
  (passes).

## Verification against the actual tool

Before writing the Ponytail worked-example table, I fetched the live
`DietrichGebert/ponytail` README (not relied on training-data memory, per
`always/verifiable-claims-rule.md`) and corrected three gaps the user's
original draft had:

1. Install path was stated as "Claude Code or Codex plugin" only; Ponytail
   also ships skill-file installs (several other assistants) and an
   instructions-only `AGENTS.md` copy for the rest (Cursor, Windsurf, Cline,
   Copilot, Kiro). Checklist item 1 and the worked example now cover all
   three mechanisms.
2. Hook supply-chain review (checklist item 6) named only Codex's lifecycle
   hooks. Cursor also has a hook (for `/ponytail` level switching); the item
   now applies per-assistant, not to one named host.
3. `/ponytail-gain` (the benchmark scoreboard command) was missing from the
   worked-example table; added with the existing self-reported-evidence
   caveat.

## Classification

Fast-tracked under `simplicity-rule.md` (single-topic doc addition, two
files touched, reversible, no code/schema/gate change) — no separate Gate 1
plan file. `downstream-impact-rule.md` trigger paths are not touched. The
rule-projection screen above stands in as the required Gate 2 evidence for
new normative guidance in an adopter-facing Pages page.

## Branch

`docs/third-party-plugin-coexistence`, not yet merged. Awaiting
governor-reviewer Gate 2 decision; merge target/remote to be confirmed
before merging per `AGENTS.md`'s gate-discipline section.
