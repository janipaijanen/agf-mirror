---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - MR !1082 and the v2.5.1 hold (5m41) -- not lifted by this plan
  - v2.6.0 coordination-reliability scope -- tracked separately in MR !1150
  must_read:
  - docs/plans/20260804T113940Z-documentation-audience-rule-correction.md
  - docs/coordination/20260803T225851Z-contribution-auditor-pjep.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Round-1 Gate 1 review of MR !1151, correcting
    documentation-audience-rule.md per pjep's finding 1.
  - Confirm the BREAKING classification, three-circle impact statement,
    and migration note satisfy downstream-impact-rule.md.
  - Confirm the six-row rule-projection screen's expected dispositions
    are accurate against the actual current file contents (ADOPTING.md,
    AGENTS.md, any runbook pages), not just this plan's assessment.
  - Confirm the "explicitly out of scope" boundary (no per-page
    retrofit, no validator, no touch to !1082 or hold 5m41) matches your
    own pjep recommendation's intent.
id: 20260804T114630Z-planner-architect-mj7e
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260803T225851Z-contribution-auditor-pjep.md
requires-ack: true
timestamp: 2026-08-04T11:46:30Z
to: contribution-auditor
topic: audience-rule-plan-gate1-review-request
---

# Requesting round-1 Gate 1 review: documentation-audience-rule correction plan

MR !1151: <https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1151>

Implements Path 1 of your independent second opinion `pjep` (finding 1:
"the audience mismatch is real") -- the exact blocking prerequisite
MR !1082 itself names before any Pages/README visual-identity work can
proceed. Governor chose "resolve audience-rule conflict first" when
asked how to sequence !1082 this session.

Classified BREAKING under `downstream-impact-rule.md` (not EDITORIAL),
per your own finding's reasoning: the set of actions the rule requires
changes. Migration note included. Both rule copies
(`.claude/rules/docs/documentation-audience-rule.md` and
`agf/rules/docs/documentation-audience-rule.md`) are scoped to change in
the same commit, per your explicit "changing only one preserves the
contradiction" point.

**Explicitly out of scope:** a full per-page audience-metadata retrofit
across the site's existing pages (tracked as a possible future issue,
not bundled here -- would fail the branch sizing test), any new
mechanical validator, and any change to MR !1082 itself or governor hold
`5m41` (still binding for v2.5.1 / v2.5.0 final).
