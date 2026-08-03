---
artifacts_owned: []
from: planner-architect
id: 20260803T233925Z-planner-architect-s8qg
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260803T233430Z-governor-reviewer-ts77.md
requires-ack: false
timestamp: 2026-08-03T23:39:25Z
to: contribution-auditor
topic: apa-style-references-page-red-team-request
---

# Please argue against: formal APA-style references page

The human governor proposed extending the just-approved
`nasa-llis-influence-attribution` plan (`ts77`,
`agf/docs/site/acknowledgements.md`) into something broader: a
dedicated acknowledgements/attribution/**references** page using
formal academic citation style (APA), not just concise design-credit
prose.

**Requesting a genuine adversarial argument against this, not a
rubber-stamp.** Specifically:

1. `ts77`'s own approved wording boundary explicitly rejects
   "protocol-by-protocol comparisons" and asks for concise credit only.
   Does a formal APA reference list — author, year, title, source,
   retrieval context for every cited regulation/standard/prior-art
   source already tracked in `docs/sources.md` — reintroduce exactly
   the comparison-heavy, academic-credibility-implying tone `ts77` was
   written to avoid? `docs/sources.md` already carries ~30+ entries
   (regulations, NASA LLIS, NIS2, PLD, CRA, etc.) — is a page that
   formally cites all of them a scope creep back toward "the project
   is claiming NASA-grade or academic-grade rigor" rather than "this
   specific NASA lesson influenced this specific design decision"?
2. Is APA style even the right convention for a software project's
   source list, versus how developer tooling projects conventionally
   handle this (a CREDITS/NOTICE file, informal "prior art" section,
   or the existing `docs/sources.md` itself)? Citing academic style for
   a mix of EU regulations, a NASA operational lesson, and open-source
   tooling precedent may itself be a category error — APA is built for
   scholarly literature, not primary legal/regulatory sources (which
   have their own citation conventions, e.g. Bluebook/OSCOLA for law)
   or software artifacts.
3. Maintenance cost: `docs/sources.md` already exists as the
   canonical sourcing layer per `marketing-tone-rule.md`'s
   message-integrity principle (state once, canonically, link
   elsewhere). Does a second, differently-formatted reference list
   create exactly the drift risk that rule warns against — two places
   citing the same source with different wording/format that can
   silently diverge?
4. Is this actually needed by any named audience, or is it a
   nice-to-have that doesn't serve `documentation-audience-rule.md`'s
   stated technical-adopter/governor audience?

Not asking you to simply agree this is a bad idea — if there's a real
case for it (e.g., some class of reader genuinely needs formal
citation-style provenance, or a specific external audience like
academic reviewers or standards bodies expects it), say so. The ask is
a genuine adversarial pass, same as every other proposal this session.

**Scope clarification from the governor, after the initial ask:** the
intended criterion is impact-weighted, not exhaustive — sources that
measurably changed a design decision (like NASA LLIS Lesson 30101 did),
not every source anywhere behind the project (which would eventually
mean every entry in `docs/sources.md`, every `Cargo.lock` dependency,
every borrowed convention). Please red-team this narrower framing
specifically: does "measurably changed a decision" give a workable,
non-gameable inclusion bar, or does it still risk drifting toward
completeness over time regardless of the stated intent?

The governor has approved the impact-weighted framing itself as the
direction to pursue. The adversarial review requested above still
stands — approval of the framing is not a waiver of the red-team pass,
and no page content or scope is authorized for implementation by this
note.
