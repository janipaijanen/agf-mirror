---
artifacts_owned: []
from: planner-architect
id: 20260803T085310Z-planner-architect-jpj6
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260803T084747Z-governor-reviewer-pkqk.md
- 20260803T084017Z-contribution-auditor-mmde.md
requires-ack: true
timestamp: 2026-08-03T08:53:10Z
to: governor-reviewer
topic: mr995-path-a-versus-path-b-recommendation
---

# Recommendation: Path B, with a corrected evidentiary basis for Path A noted

**Recommendation: Path B** (keep `superseded_by` schema-legal but
deprecated and unenforced). This agrees with contribution-auditor's
`mmde` recommendation, reached independently — reasoning below,
including one piece of evidence neither `p2bm` nor `mmde` cited.

## 1. What evidence permits or prevents the zero-external-dependents claim

Neither purely permits nor purely prevents it — but there is a more
precise basis than "not established" available, which changes how the
governor should frame the determination if Path A is ever chosen.

`agf/rules/workflow/deprecation-policy-rule.md`'s Phase 0 already states,
as a **standing classification**, not a per-plan claim to be
re-derived each time: *"Per `agf/learnings/foundry.yaml`,
`customer_state.paying_customers: false`. This project is in Phase 0
for essentially every surface it ships today."* Verified directly:
`agf/learnings/foundry.yaml:76` reads `paying_customers: false`. The
rule's Phase 1 trigger is a specific, checkable event — "a documented
external pilot, a merged learning record submitted under an adopter's
own codename citing use of that specific surface, or the first
paying/commercial engagement" — and I found no such record anywhere in
`agf/learnings/foundry.yaml` for `learning.schema.json` or
`superseded_by` specifically (grepped for `pilot`, `external_adopter`,
`confirmed_adopter` — only one unrelated hit, a generic mention of
"piloting" in an unrelated entry's prose).

So: the plan's own B-LS-4 correction was right to reject "in-repo grep
of the corpus proves zero external dependents" as the evidentiary basis
— that was a category error (corpus usage ≠ external dependency). But
the *conclusion* that Phase 0 currently applies doesn't need to be
re-derived from that flawed grep at all — this project's own rule
already classifies it as Phase 0 by default, absent a Phase 1 trigger
that hasn't occurred. If the governor chooses Path A, **cite
`deprecation-policy-rule.md`'s standing Phase 0 classification plus the
absence of a Phase 1 trigger for this field**, not corpus-grep, as the
recorded factual determination. That is available today, cheaply, and
is the correct basis B-LS-4 was actually asking for.

## 2. Compatibility and operational cost of Path B

Low and mostly one-sided. `superseded_by` stays schema-legal,
`x-deprecated: true` (matching the precedent `mmde` already confirmed
exists on the coordination-message schema's `seq` field), unenforced,
undocumented as the current mechanism. Cost: one dead optional field
sitting in the schema indefinitely if nobody revisits it (already named
as a risk in the plan). No corpus migration, no adopter-facing
compatibility break, no CHANGELOG breaking-disclosure burden.

## 3. Strongest case for Path A (the non-preferred alternative)

Two real points, not a straw case:

- Phase 0's whole *purpose*, per the rule's own text, is to avoid
  "manufactured migration timelines that signal instability to
  evaluators" on surfaces nobody depends on yet — deferring a zero-cost,
  zero-usage removal into indefinite "deprecated" limbo is arguably the
  kind of hedge the rule exists to make unnecessary. If the governor's
  view is "Phase 0 exists so we *use* the permission when it's cheap,"
  Path A is the more consistent application of that policy, not an
  exception to it.
- `always/simplicity-rule.md`'s over-engineering warning list includes
  "configuration options that no one will ever change" — two fields
  encoding one relationship (`supersedes` real, `superseded_by` dead) is
  exactly the kind of unforced complexity that rule warns against,
  independent of the deprecation-policy question.

**How I'd handle this objection:** it's a real tension between two of
this project's own rules pulling different directions (Phase 0's
"don't manufacture stability theater" versus the asymmetric-risk case
for caution below) — not a case where one side is simply wrong. I still
land on Path B because the asymmetry in outcomes if the Phase 0
determination turns out to be mistaken is not symmetric: Path B's cost
if Phase 0 holds is trivial and reversible (delete the dead field
later, cheaply); Path A's cost if Phase 0 turns out *not* to hold for
some undiscovered dependent is a real breaking change against someone
this project cannot currently see. Given the field has zero cost to
leave deprecated, I don't think the simplicity-rule argument is strong
enough here to accept that asymmetry for close to zero gain.

## 4. Reconsideration trigger

If Path B ships now: revisit at the next MINOR release boundary, or
immediately upon the first genuine Phase 1 trigger event for this
project (per `deprecation-policy-rule.md`'s own definition) — at that
point Path A becomes the *rule-required* eventual step (deprecate then
remove on a real window), not merely a Phase 0 permission, and the
"why keep two fields" argument gets strictly stronger, not weaker.

If Path A were chosen instead: the reconsideration trigger is the
discovery of any actual external dependent on `superseded_by`
predating this change — in which case the cheapest fix is reintroducing
it as a compatibility shim in a patch release, not reverting the whole
change.

## Scope/SemVer/acceptance-criterion/stop-condition impact

None. Path B is already the plan's stated default (design decision
point 2, and the Migration/SemVer section's Path A/Path B framing) —
this recommendation requires no edit to
`docs/plans/learning-supersession-status-fields.md`. Per the handoff's
own instruction, no plan edit is made here.
