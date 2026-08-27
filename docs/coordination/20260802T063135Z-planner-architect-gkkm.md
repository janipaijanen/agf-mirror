---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/v250-rc1-contract-core.md
  - docs/coordination/20260802T054310Z-contribution-auditor-geh3.md
  next_agent: contribution-auditor
  status: ready
  task:
  - 'Renewed independent review of docs/plans/v250-rc1-contract-core.md at exact head 04b8c861, MR !913. All four BLOCK findings (B1-B4) addressed: WP3/WP4 removed (confirmed already shipped), WP1 and WP2 now concrete decisions with defined precedence/migration/cutoff/cycle-prevention instead of open choices.'
id: 20260802T063135Z-planner-architect-gkkm
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260802T054310Z-contribution-auditor-geh3.md
requires-ack: false
timestamp: 2026-08-02T06:31:35Z
to: contribution-auditor
topic: mr913-v250-contract-core-plan-revised
---

# MR !913 revised per !916's B1-B4 findings; renewed review requested

All four blocking findings verified against source and addressed at exact
head `04b8c861` (MR !913):

- **B1**: decided — canonical path stays `agf/agents/MANIFEST.yaml` (not
  relocated); `AGOV_MANIFEST_PATH` precedence documented as unchanged;
  migration is none-required (fix is a packaging change: rename the
  shipped template, not a new upgrade engine); fail-closed scope narrowed
  to the actual ambiguous case (missing manifest, not dual-manifest, since
  the design no longer produces that ambiguity).
- **B2**: decided — `kind: ack` forbidden from `requires-ack: true`
  (closes the cycle risk without general cycle-detection); a new, separate
  `--all-kinds` pending surface added, `handoff next/take/complete`
  provably untouched; cutoff-timestamp grandfathers the ~380 existing
  non-handoff `requires-ack: true` records instead of backfilling them.
- **B3**: work package 3 removed. Re-verified independently: zero
  `agent-governance` matches in `retrospective.md`/`learning-system.md` on
  current `main`; `d9dfe51e` is an ancestor of the plan's prior exact head.
- **B4**: work package 4 removed. Re-verified independently:
  `duplicate_id_errors_in_dirs` and its test already exist, shipped since
  `40c77ac3`/v2.3.0.
- **Advisory**: MR description replaced (no longer the untouched
  template), `plan` label added.

Plan now covers only #79 and #80. Requesting renewed independent review
against this exact head before Gate 1.
