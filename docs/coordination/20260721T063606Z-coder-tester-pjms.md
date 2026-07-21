---
kind: note
from: coder-tester
to: planner-architect
id: 20260721T063606Z-coder-tester-pjms
timestamp: 2026-07-21T06:36:06Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T231159Z-coder-tester-ad63
topic: applicability-catalog-backfill-followup
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/adopt/applicability-catalog.md
---

# Tracked follow-up: full backfill of the applicability catalog

`docs/plans/adopter-applicability-catalog.md` (Gate 1 approved, MR !277/!284)
and its Gate 2 implementation (`agf/docs/site/docs/adopt/applicability-catalog.md`)
deliberately classify only a first representative slice: 8 of 57 governed
artifacts (45 rules + 12 agent roles). The remaining 49 (43 rules; 6 roles —
fact-checker, capability-developer, documentation-writer,
deployment-engineer, academic-peer-reviewer, academic-visual-reviewer) are
explicitly out of scope for this tranche (see the plan's "Explicitly out of
scope" section).

This note exists so that gap is a tracked, referenced item rather than an
implied, unscheduled intention — per the plan's own "Catalog page" design
section, which requires the follow-up to be named at Gate 1 approval time.

## What the follow-up plan needs to do

- Classify the remaining 43 rules: `activation_condition`,
  `safe_to_remove_when`, `removal_forfeits`, and (where applicable)
  `depends_on`, using `rule.schema.json`'s now-optional fields.
- Classify the remaining 6 agent roles: `activation_condition` and
  `deactivation.local_deactivation` / `deactivation.definition_removal`,
  using `agent.schema.json`'s now-optional fields.
- Size the work per this project's branch-sizing discipline — one
  reviewable batch per yes/no question, not one 43-file flag day. A
  reasonable split is per-scope batches (`always/`, `workflow/`, `docs/`,
  `ui/` for rules; remaining roles as a separate batch).
- Build the deferred `agov validate rules` command (or equivalent),
  named but not built in this tranche, so schema-consistency for these
  fields becomes enforced rather than manually verified.

No owner or timeline is assigned by this note — it is a tracking anchor for
whoever (human or agent) picks up the next batch, not a commitment to
immediate execution.
