---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260724T152038Z-governor-reviewer-jgeg
timestamp: 2026-07-24T15:20:38Z
model: human-governor
immutable: true
references-to:
  - 20260724T151451Z-governor-reviewer-h737
  - 20260724T151121Z-contribution-auditor-s36p
topic: v242-plain-language-progressive-disclosure-requirement
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
---

# Governor amendment: plain-language progressive disclosure in v2.4.2

Add an explicit communication requirement to the revised v2.4.2 Gate 1 plan:
evaluator-facing public content must provide excellent plain-language answers
before introducing AGF-specific terminology or technical detail.

The intended standard is **ELI5 clarity without infantilizing the reader or
removing necessary precision**:

1. begin with the concrete reader problem or question;
2. explain one concept at a time in ordinary language;
3. use a short concrete example where it materially improves understanding;
4. introduce the exact AGF term only after its meaning is clear;
5. progressively disclose commands, schemas, maturity vocabulary, and
   implementation detail through links or later sections;
6. preserve bounded evidence language and technical accuracy—simple wording
   must not become an absolute promise or erase an important limitation.

Apply this review to the evaluator-facing homepage, root `README.md`,
`should-you-adopt.md`, adopter FAQ, use cases, and the nutshell orientation
surface. Do not rewrite operator/reference pages merely to make them
beginner-level; those pages should remain precise for their stated audience.

The revised plan must add:

- an exact inventory of the evaluator-facing files included in this pass;
- a plain-language-before-terminology acceptance check for each included
  concept;
- a check that each page has a clear reader question and next action;
- fact-checker review ensuring simplification preserves feature, maturity,
  evidence, cost, privacy, and enforcement boundaries;
- representative rendered review at narrow and desktop widths;
- explicit non-goals preventing a site-wide rewrite or changes to technical
  contracts.

This amendment joins the revision required by `h737`. It does not restore Gate
1 authority: the complete revised plan still requires contribution-auditor
review and a fresh governor exact-SHA approval before implementation.
