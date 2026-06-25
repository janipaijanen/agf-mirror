---
kind: note
from: planner-architect
to: coder-tester
id: 20260625T060132Z-planner-architect-k82k
timestamp: 2026-06-25T06:01:32Z
immutable: true
references-to: []
topic: v1x-docs-review-findings
requires-ack: false
artifacts_owned: []
---

# v1.x docs review: tone assessment and content gap analysis

Governor asked for a full review of the v1.x public documentation site
(https://janipaijanen.gitlab.io/agent-governance/, v1.1.50) to assess
(a) tone parity and (b) v1.x concepts or figures that should be
incorporated into v2.x docs.

This note summarises findings for Codex's review and response.

---

## Tone: v1.x vs v2.x

v1.x tone is more academic and process-heavy: passive voice is common,
sections are formally structured, and the framework describes itself in
the third person. The "advanced-prototype" status framing is explicit and
prominent throughout.

v2.x tone has improved significantly: shorter sentences, active voice,
direct framing ("No new accounts. No hosted service."), developer-facing
pragmatism. The "should you adopt / wait / skip" structure is clearer
than v1.x's "quick decision tree."

**Assessment: v2.x tone is generally better. No broad adjustment needed.**

Two specific v1.x phrasings are worth preserving or echoing in v2.x:

1. **"Deliberately ordinary — no dramatic incident"** — from the v1.x
   walkthrough. This is the honest framing for what governance looks like
   in practice: repeatable, unremarkable routine. v2.x's
   `first-governed-change.md` does not yet carry this framing.

2. **The failure-modes framing** — v1.x "Why this exists" page listed
   five concrete failure modes (authority without boundaries, capability
   drift, accountability gaps, local-vs-global misalignment, knowledge
   decay). v2.x index.md has "AI coding agents can produce changes you
   cannot reconstruct later" but no named failure-mode taxonomy. Named
   failure modes help evaluators identify whether their situation applies.

---

## Content present in v1.x but missing from v2.x

### 1. Position / "how it relates to other tools" page

v1.x had a full "Position" section:
- vs agent orchestrators (LangGraph, CrewAI, AutoGen, Claude Code, Cursor)
- Per-tool adoption notes for each
- vs AI policy/ethics frameworks
- vs SDLC governance, vs CI/CD, vs ML governance
- Five-layer verifiability model
- The dual-surface model

v2.x has fragments in `evaluate.md` ("This project is not...") and
`use-cases.md`, but these are reviewer-facing and adopter-facing
respectively, not a coherent comparison page.

**Recommendation: add `docs/adopt/position.md` or
`docs/concepts/position.md` with the per-tool relationship notes.**
Codex CLI users specifically are a real audience gap — v1.x had specific
notes for Codex/non-Claude tools that v2.x doesn't.

### 2. Contribute / upstream guide

v1.x had a structured contribution guide with three paths:
- Submit a learning (gaps, validations, tensions, adaptations, overkill)
- Propose a rule (provisional by default; evidence requirements)
- Propose an agent (higher evidence threshold: 2+ adopters)

v2.x has the mechanics in rules and `evidence-and-learnings.md` but no
user-facing guide for adopters who want to contribute back. The
`evidence-and-learnings.md` page mentions anonymization and upstream
contribution but does not walk through the mechanics.

**Recommendation: add `docs/contribute.md` (or surface as a section in
`evidence-and-learnings.md`) covering the three paths with evidence
thresholds.**

### 3. Bibliography

v2.x rules cite Lehman (1980), Swanson (1976), Parnas (1972), Šimon &
Buhnova (2024) — but there is no bibliography page for external readers
who want to follow the theoretical grounding. v1.x had this as a
distinct nav section.

**Recommendation: add `reference/bibliography.md` listing academic
citations used in rules and docs.**

### 4. "Why this exists" / failure modes

v1.x had a dedicated "Why this exists" entry point. v2.x has this
spread across index.md and should-you-adopt.md. A dedicated short page
or section could improve discoverability for academic/evaluator readers
who want the motivation without the adoption decision framing.

**This is lower priority** — most of the content exists; it is a
structural question.

### 5. Status / transparency page

v1.x "Status" page listed live metrics (projects, learnings, rules,
agents, skills counts), candidates (provisional rules awaiting promotion),
and honest maturity framing. v2.x has `reference/maturity-vocabulary.md`
and the `evaluate.md` page but no "current snapshot of where the
framework stands" page.

**Recommendation: consider whether a generated or manually maintained
status page belongs in v2.x. This may not be worth the maintenance cost
unless generation is automated.**

---

## Figures: v1.x vs v2.x assessment

v2.x has 8 figures covering the key visual concepts:
governance-loop, gate-pipeline, coordination-channel,
operating-responsibility-map, evidence-lifecycle, learning-loop,
adoption-exit-flow, coordination-map-example.

v1.x had fewer figures — the concept section had a combined
"roles + pipeline" flowchart and a sequence diagram emphasizing
handovers. Both of those concepts are now better covered in v2.x.

**v1.x figure content not currently covered in v2.x:**

The 12-role taxonomy (4 tiers: Production, Governance, Cross-cutting,
Meta) is described in the v1.x framework reference but v2.x only surfaces
the three core pipeline roles (PA, CT, GR) in figures. Supporting roles
(build engineer, security sentinel, etc.) exist in `agf/agents/` YAML
but are not visualized.

**Recommendation: one optional figure — a "full role taxonomy" figure
showing all 12 roles in their tier structure, linked from
`roles-and-gates.md`. Low priority; the YAML files are the authoritative
source and the quick-reference sheet already lists them.**

---

## Summary recommendation for Codex

Three concrete additions are worth implementing:

| Priority | Item | Target file |
|---|---|---|
| High | Position / per-tool relationship page | `docs/adopt/position.md` |
| High | Contribute / upstream guide | `docs/contribute.md` or extend `evidence-and-learnings.md` |
| Medium | Bibliography | `reference/bibliography.md` |
| Low | Full role taxonomy figure | new DOT source + SVG |
| Low | "Why this exists" standalone page | optional restructure |

Requested of Codex: please review the v1.x site at
https://janipaijanen.gitlab.io/agent-governance/ (position, contribute,
and evidence sections specifically) and the current v2.x docs tree, then:

1. Confirm or revise the priority ordering above based on what you observe
   in v2.x coverage vs the v1.x site.
2. For the position page: draft a structure (headings only) that would
   work within v2.x docs conventions, covering the per-tool notes for
   Claude Code, Codex CLI, Cursor/Windsurf, and LangGraph/CrewAI.
3. For the contribute page: assess whether extending
   `evidence-and-learnings.md` is sufficient or a dedicated page is
   warranted.

No implementation yet — Gate 1 plan pending governor decision on scope.
