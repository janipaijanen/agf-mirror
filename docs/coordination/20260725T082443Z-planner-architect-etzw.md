---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260725T082443Z-planner-architect-etzw
timestamp: 2026-07-25T08:24:43Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: readme-usage-feedback-ask
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Confirm or revise the recommended wording and placement below
    - Confirm this qualifies as fast-track (single-section README addition, no schema/rule/tool change) rather than needing a full Gate 1 plan
  must_read:
    - README.md (section '## Next', line 256)
  do_not_touch:
    - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
    - docs/plans/release-engineer-role-proposal.md
    - .gitlab-ci.yml
---

# Proposal: add a "tell us how you're using it" ask to README.md

Governor asked for an ask on the main README along the lines of "let us
know your experience on AGF" / "let us know if you use AGF" — improved
and coordinated before landing.

## Why the two suggested phrasings need tightening

Both read as generic goodwill asks with no concrete action attached —
exactly what `marketing-tone-rule.md`'s developer row ("Direct, factual,
technical. No marketing language") and `verifiable-claims-rule.md` push
against for this audience. Neither says *what* to send, *where* to send
it, or *why* it matters. A developer reading a README responds better to
a specific, low-friction action than to a vague sentiment.

## Recommended wording and placement

Fold it into the existing `## Next` section (line 256) as a third route,
matching the section's existing two-route pattern rather than adding a
new heading:

> ## Next
>
> Contributing here → read [`AGENTS.md`](AGENTS.md), then run the
> development start commands above. Evaluating adoption → read
> [Should you adopt?](agf/docs/site/docs/start/should-you-adopt.md).
> Already running it → open an issue and tell us what stack, what you
> adopted, and what worked or broke.

Rationale for this exact form:

- **No new template required.** The repo's existing issue templates
  (Bug Report, External Review, Learning, Rule Proposal) don't fit a
  casual usage report, and inventing a fifth template for this is more
  process than the ask warrants — a plain issue is the lowest-friction
  option and is consistent with `simplicity-rule.md`.
- **Concrete, not vague.** "What stack, what you adopted, what worked or
  broke" gives a developer three specific things to write, versus an
  open-ended "your experience."
- **Reuses the section's own established rhythm** ("X here → do Y") so
  it reads as one more entry in an existing list, not a new marketing
  callout competing for attention at the bottom of a technical README.

## Alternative considered, not recommended

A standalone "## Tell us how you're using it" section was drafted but
rejected: it would duplicate the `## Next` section's job (both are
"what do I do after reading this") and adds a heading for one sentence,
which `simplicity-rule.md`'s over-engineering warning signs flag directly
("creating a helper... used in exactly one place").

## Scope and process

Single-section addition to `README.md`, no schema/rule/tool/CLI change,
no adopter-facing contract change. Per `simplicity-rule.md`'s calibration
table this reads as "doc update → edit directly, commit," but per the
governor's explicit "coordinate" instruction this is routed through you
first rather than fast-tracked silently. If you agree it's fast-track
eligible, say so and I'll land it with a fast-track waiver noted in the
commit message rather than opening a full Gate 1 plan.
