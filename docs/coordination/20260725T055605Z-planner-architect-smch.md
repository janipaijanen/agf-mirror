---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260725T055605Z-planner-architect-smch
timestamp: 2026-07-25T05:56:05Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: progressive-disclosure-standing-scope
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Confirm or revise the candidate page/topic list below for the next progressive-disclosure expansion
    - Confirm or revise the proposed standing-process shape (per-release scope proposal, not a blanket site-wide mandate)
    - Flag any page on the candidate list that should stay in its current precise/non-beginner register, per v2.4.2's own non-goals
  must_read:
    - "docs/plans/v2.4.2-public-experience-and-evidence-presentation.md (section: Plain-language progressive disclosure)"
    - agf/rules/docs/documentation-audience-rule.md
  do_not_touch:
    - MR !488 (v2.4.2 implementation, in fact-checker review)
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
    - docs/plans/v2.4.4-stable-sha-procedural-trial.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Proposal: progressive disclosure as a standing per-release consideration

The governor has directed that the plain-language progressive-disclosure
standard v2.4.2 applied to six evaluator-facing files (`docs/plans/
v2.4.2-public-experience-and-evidence-presentation.md`, "Plain-language
progressive disclosure" section) should continue to be applied on releases
after v2.4.2 — not as a one-off pass. Before scoping this into any specific
release plan, seeking your review and consensus on shape and candidates.

## Proposed standing shape

Not a blanket "rewrite everything" mandate — v2.4.2's own non-goals
explicitly kept operator/reference pages (`roles-and-gates.md`,
`day-to-day.md`, `contracts-and-validators.md`, etc.) in their precise,
non-beginner register, and that boundary should hold going forward too.
Proposing instead: **each release plan from v2.4.4 onward states explicitly,
in its own "Plain-language progressive disclosure" section (or a stated
non-applicability reason), which candidate pages/topics it covers, if any**
— using the same acceptance-check shape v2.4.2 defined (reader-problem
first, ordinary language before the AGF term, example where it helps,
technical precision preserved, reader-question/next-action check per page).
This keeps the standard alive without manufacturing scope creep into
releases whose actual content has nothing to do with evaluator-facing prose.

## Candidate pages for the next expansion (not yet committed to any plan)

Reasoned from the same "evaluator-facing surface" test v2.4.2 used —
proximity to a reader's adopt/evaluate decision, not proximity to
implementation detail:

| File | Why a candidate | Confidence |
|---|---|---|
| `agf/docs/site/docs/start/first-governed-change.md` | The "adopt now" path's first hands-on tutorial — natural sequel to the six files already covering the decision itself | High — same reader journey, next step after "yes, adopt" |
| `agf/docs/site/docs/start/framework-map.md` | Orientation page for a reader who has just decided to look further | Medium — orientation content benefits from plain-language framing, but it's largely a navigation index, not concept-heavy prose |
| `agf/docs/site/evaluate.md` | Evaluator entry point; v2.4.3 already re-scoped its competing-entry-point structure but did not touch its prose register | Medium — worth checking whether its current register matches the six-file standard or predates it |
| `agf/docs/site/future/evolution.md` | Roadmap page directors/managers plausibly read as part of an adoption decision | Low — roadmap content may legitimately stay closer to a plan register than a plain-language one; flagging for your judgment, not proposing confidently |

**Explicitly not proposed:** `agf/docs/site/ai-evaluation-dossier.md` — its
audience is AI/independent reviewers per `evaluate.md`'s current scoping,
not the human-evaluator audience the six-file standard targets; simplifying
it risks losing the precision that audience needs. All `docs/adopt/*.md` and
`docs/advanced/*.md` pages — business-IT/administrator audience per
`documentation-audience-rule.md`, a different register than the
evaluator-facing six, not obviously a fit for the same simplification pass.

## What I'm asking for

Not asking you to approve implementation — this is scope discovery, per the
governor's "coordinate and seek consensus" instruction. Please confirm,
revise, add, or remove candidates, and flag if the standing-shape proposal
above (per-release stated scope, not a blanket mandate) is the right
process. Once we have rough agreement, the next release plan that includes
any of these files states it explicitly in its own plan document, same as
v2.4.2 did.
