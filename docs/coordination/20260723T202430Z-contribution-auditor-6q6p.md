---
kind: handoff
from: contribution-auditor
to: governor-reviewer
id: 20260723T202430Z-contribution-auditor-6q6p
timestamp: 2026-07-23T20:24:30Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T195440Z-governor-reviewer-rekk
  - 20260723T202047Z-planner-architect-6get
  - 20260723T202142Z-planner-architect-dr5h
topic: v241-accessibility-prose-hardening-gate2
requires-ack: true
artifacts_owned:
  - agf/docs/site/**
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - "Resolve the missing live browser, 390px viewport, accessibility-tree, and named screen-reader smoke-test evidence before Gate 2 approval, or record an explicit governed disposition of that unmet plan criterion"
  must_read:
    - docs/plans/v2.4.1-accessibility-prose-hardening.md
    - 20260723T202047Z-planner-architect-6get
    - 20260723T202142Z-planner-architect-dr5h
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# v2.4.1 #51/#55 implementation evidence — Gate 2 blocked on live AT test

MR !458 implements the approved v2.4.1 accessibility/prose-hardening plan.
Claude completed the first 18 static-figure introductions and all five Mermaid
source/CSS changes, then stopped and published handoff `6get`. Codex added the
four dossier-specific introductions. The exact content revision reviewed here
is `abfd415b81839f43234a0dda00f7a0b5ec02f7a0`.

This is an evidence handoff, not Gate 2 approval. The implementation remains
blocked because the approved plan requires live browser/390px and named
screen-reader evidence that neither assistant produced.

## Reproduced checks at `abfd415b81839f43234a0dda00f7a0b5ec02f7a0`

| Check | Result | Provenance |
|---|---|---|
| Static-image inventory command from the plan | PASS: 22 line-start embeds across 10 pages | Codex executed against the exact revision |
| Mermaid inventory across `agf/docs/site/docs` and `agf/docs/site/index.md` | PASS: 5 fences | Codex executed against the exact revision |
| `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml` | PASS with MkDocs 1.6.1; build completed in 0.80 seconds | Codex executed using the existing `tmp/mkdocs-venv` |
| `git diff --check` | PASS | Codex executed before the content commit |
| Version/release mutation | PASS: no VERSION bump, changelog, tag, or release operation in this MR | Diff/history inspection |

The untrusted worktree `mise.toml` prevented the PATH-resolved `mkdocs` shim
from executing. The successful build used the repository's existing pinned
MkDocs 1.6.1 virtual environment by absolute path; the documentation build
itself exited zero. The Material-for-MkDocs upstream advisory printed during
the build is informational and did not become a strict-build warning.

## Per-embed prose and metadata review (#51)

Each row was read in page order without relying on the SVG. “Adequate—preserved”
means the existing concise alt text still provides an appropriate accessible
name, while the newly preceding prose carries the essential relationship and
takeaway; it does not mean the image alone substitutes for the prose.

| Page | Figure | Prose-alone finding | Metadata finding |
|---|---|---|---|
| `ai-evaluation-dossier.md` | gate pipeline | Separates plan approval, implementation, evidence, and merge approval; validators support but do not decide. | Adequate—preserved |
| `ai-evaluation-dossier.md` | coordination channel | Explains role-to-role flow and why corrections extend rather than rewrite the chain. | Adequate—preserved |
| `ai-evaluation-dossier.md` | learning loop | Connects observations to governed promotion or retirement while retaining evidence strength. | Adequate—preserved |
| `ai-evaluation-dossier.md` | adoption and exit | Relates adoption, reassessment, freeze/replace/removal paths, and retained history. | Adequate—preserved |
| `reference/figures.md` | governance loop | States the self-application cycle and that learning feeds the next plan. | Adequate—preserved |
| `reference/figures.md` | gate pipeline | States the two decision points and the prohibition on self-approval. | Adequate—preserved |
| `reference/figures.md` | coordination channel | States the append-only lifecycle and reconstructible references. | Adequate—preserved |
| `reference/figures.md` | operating responsibility map | Distinguishes human-only authority, AI execution, and shared evidence work. | Adequate—preserved |
| `reference/figures.md` | evidence lifecycle | Explains how observations can become learnings and governed changes. | Adequate—preserved |
| `reference/figures.md` | learning loop | Explains evidence feedback into later rules, validators, docs, or retirement. | Adequate—preserved |
| `reference/figures.md` | adoption and exit | Explains entry, governed operation, reassessment, and exit choices. | Adequate—preserved |
| `reference/figures.md` | coordination map example | Identifies it as a dated foundry sample, not a universal workflow template. | Adequate—preserved |
| `adopt/downstream-impact.md` | adoption and exit | Connects adoption cost with reversible freeze, replace, and removal paths. | Adequate—preserved |
| `adopt/exit-and-decommission.md` | adoption and exit | Makes preservation of history explicit across decommission choices. | Adequate—preserved |
| `concepts/roles-and-gates.md` | gate pipeline | Explains separated planning, implementation, and human decision authority. | Adequate—preserved |
| `concepts/roles-and-gates.md` | coordination channel | Explains how the immutable chain exposes missing or improper authority. | Adequate—preserved |
| `start/first-governed-change.md` | gate pipeline | Places the reader's first change in prospective Gate 1 and pre-merge Gate 2 order. | Adequate—preserved |
| `concepts/coordination-protocol.md` | coordination map example | Explains the concrete sample's participants and dated, non-normative scope. | Adequate—preserved |
| `operate/day-to-day.md` | operating responsibility map | Separates recurring operating work by authority mode. | Adequate—preserved |
| `operate/day-to-day.md` | governance loop | Connects daily completion and learning back to the next governed cycle. | Adequate—preserved |
| `concepts/evidence-and-learnings.md` | learning loop | Explains that recorded evidence is assessed before changing framework behavior. | Adequate—preserved |
| `operate/review-and-release.md` | gate pipeline | Relates review evidence to the two human decisions and release boundary. | Adequate—preserved |

Result: all 22 embeds have distinguishable, substantive preceding prose. The
essential relationship is available without perceiving the image. No caption,
legend, cross-reference, or alt-text defect requiring correction was found in
this bounded review.

## Per-diagram evidence and limitations (#55)

| Page/diagram | Source-level accessible-name evidence | Equivalent surrounding prose | 390px/live DOM/AT status |
|---|---|---|---|
| `index.md` governed-change flow | `accTitle` and independently descriptive `accDescr` present | Explains AI work, human gates, and CI's supporting role | NOT TESTED live |
| `concepts/coordination-protocol.md` handoff sequence | `accTitle` and independently descriptive `accDescr` present; documented default-colour exception preserved | Explains Gate 1 handoff through Gate 2 decision and completion | NOT TESTED live |
| `concepts/position.md` runtime comparison | `accTitle` and independently descriptive `accDescr` present | Explains runtime execution versus AGF governance responsibilities | NOT TESTED live |
| `concepts/position.md` adjacent categories | `accTitle` and independently descriptive `accDescr` present | Explains overlap and separation from five adjacent categories | NOT TESTED live |
| `start/framework-map.md` concept map | `accTitle` and independently descriptive `accDescr` present | Enumerates the five concept groups and their relationships | NOT TESTED live |

Codex reproduced that all five Mermaid sources contain `accTitle` and
`accDescr`, and that shared CSS applies horizontal overflow while removing the
rendered SVG's shrink-to-fit `max-width`. The strict MkDocs build confirms the
source integrates into the site build. It does **not** execute the browser-side
Mermaid renderer or prove accessibility-tree exposure.

Handoff `6get` reports a separate Mermaid CLI experiment: default output lacked
an accessible name, while adding `accTitle`/`accDescr` produced title/description
elements and ARIA references. That is useful reported static-render evidence,
not first-person reproduction by Codex and not a live Material-theme result.

## Blocking acceptance gaps

The following approved-plan evidence remains absent:

1. desktop and 390px live rendered evidence for each diagram;
2. keyboard reachability and zoom/reflow observations;
3. browser accessibility-tree inspection for accessible name, semantics, and
   node/label exposure;
4. one named screen-reader/browser/OS smoke test across the shared rendering
   pattern, with concrete observed behavior and limitations.

Therefore no claim of WCAG conformance, universal assistive-technology
support, completed #55 acceptance, or Gate 2 readiness is made. The governor
must arrange the missing test evidence or explicitly govern a different
disposition; an assistant must not convert this gap into a PASS by inference.
