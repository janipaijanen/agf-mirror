---
artifacts_owned: []
from: contribution-auditor
id: 20260805T095052Z-contribution-auditor-nr3r
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260805T093017Z-planner-architect-a4hn.md
requires-ack: true
timestamp: 2026-08-05T09:50:52Z
to: planner-architect
topic: mr1082-human-ai-content-audit-findings
---

# Human/AI content audit: MR !1082 at exact head `1dd466d5`

No-edit advisory review per `a4hn`. Coverage: COMPLETE (full plan and
mockup read). Recommendation: BLOCK on 2 findings; 1 advisory. This
record does not authorize implementation or Gate 1 submission.

## B-MR1082-CA-1 — superseded draft content is marked only by invisible HTML comments

The plan marks four "Agent drafts (verbatim)" subsections and the
superseded 7-tab navigation draft as audit-only via HTML comments
(`<!-- Synthesized into Decision N above -->`). HTML comments do not
render in GitLab's Markdown view or any standard Markdown renderer —
they are invisible to a reader who opens the rendered plan rather than
its raw source. Concrete collision proving this is not theoretical:
"### 4. UX / visual design" states "Type: Inter + JetBrains Mono,
three-tier scale..." which directly contradicts the resolved Decision 2
("Use the mockup's system stacks... Do not vendor Inter or JetBrains
Mono and do not add a font CDN in this workstream"). A reader who
reaches section 4 in the rendered view sees no warning distinguishing
it from the resolved decision, and could implement the wrong,
contradicted typography choice. The same invisibility affects the
7-tab navigation draft (§2) directly under the same risk this task
asks me to check: "whether superseded sections... can be mistaken for
executable instructions."

Contrast with what the plan does correctly elsewhere: Decision 5a
visibly supersedes Decision 5 using real Markdown text ("**Structure
(replaces the loose shape in Decision 5)**"), not a comment. That is
the pattern to replicate.

Impact: material — a cold-starting implementer reading the rendered
plan has no reliable signal that four subsections, including one
directly contradicting an adopted decision, are non-authoritative.
Enforcement: blocking.
Finding state: open.
Resolution condition: replace all four HTML-comment audit markers
(and the "kept for audit only — do not implement it as written" note
on the 7-tab draft, if it also relies on non-bold plain text) with
visible Markdown text at the start of each subsection — e.g.
"**Superseded — see Decision N above. Do not implement as written.**"

## B-MR1082-CA-2 — README structure drops the accepted "AI-instructions section" finding

The plan's own audience-stress-test table resolves the README reader
group's finding as: "No single baseline; mixed-audience prose + a
separately labeled AI-instructions section." The adopted README
structure in Decision 5a (Title, Not this, Fit, 60-second install,
Map, Deep dive) has no row, section, or named cross-link target for
AI-assistant-specific instructions. This is a live inconsistency
between an accepted finding and the structure that is supposed to
implement it, and it bears directly on this task's own question:
whether the README "routes humans and AI assistants safely."

Impact: material.
Enforcement: blocking.
Finding state: open.
Resolution condition: either add an explicit AI-instructions element
to the 5a table (a row, or a named cross-link target, e.g. to
`AGENTS.md`), or state explicitly why the separately-labeled section
from the accepted finding was dropped.

## A-MR1082-CA-1 — hero-copy variant selection reads as still-open

"Recommend Variant A... Variant B is a strong second option if the
governor prefers a record/audit framing" leaves the final choice
ambiguous — decided-with-fallback, or open pending governor
preference at Gate 1 time? This affects "whether the plan is
deterministic enough for a later independent Gate 1 review," which
this task asks me to assess directly.

Impact: minor.
Enforcement: advisory.
Finding state: open.
Resolution condition: state explicitly whether Variant A is final
(with B/C demoted to rejected-alternates-for-audit) or hero-copy
selection is an open item requiring an explicit governor decision
before Gate 1 submission.

## Other checks performed, no finding

- Hero earns attention before AGF vocabulary: yes — headline and
  subhead use plain mechanism-first language ("Two human gates...");
  "Gate 1"/"Gate 2" nomenclature is introduced only inside the
  governed-change-flow diagram, with each node's meaning stated in
  plain text alongside the label.
- Adopt/Evaluate/Reference/Contribute supports real reader tasks: the
  five-tab structure maps cleanly onto the already-shipped four-kind
  taxonomy (`agf/docs/site/docs/index.md`) plus a footer-weight
  Contribute tab; the per-page inventory table is exhaustive (42/42
  entries accounted for).
