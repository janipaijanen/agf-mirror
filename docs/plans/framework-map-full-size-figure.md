---
id: public-mermaid-full-size-figures
status: pending-gate1
topic: public-mermaid-full-size-figures
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: "the governor expanded the direct usability finding from the framework map to the complete five-diagram Mermaid inventory and chose standalone openable figures over a wider global content column; the plan freezes that inventory and excludes unrelated existing figures"
  - role: infosec-sentinel
    status: not-applicable
    reason: "scope is public documentation, DOT/SVG figure sources, removal of the now-unused client-side Mermaid integration, and one learning record; it removes an external browser-side CDN import and introduces no secrets, personal data, authentication boundary, schema, or executable service"
---

# Plan: replace all public Mermaid diagrams with openable SVG figures

## Revision and supersession

This plan supersedes its initial one-diagram version at commit
`13fa639da651a471f093588e6896ac80679e368a` and Gate 1 handoff
`20260725T071805Z-planner-architect-6r3t`. No Gate 1 approval was recorded and
no implementation began under that version.

The governor broadened issue #64 on 2026-07-25 after confirming the same defect
on `docs/concepts/position/`: every public Mermaid diagram is constrained by
the article viewport and lacks a standalone open/zoom path. The requested
outcome is now the complete Mermaid inventory, not only the framework map.

## Problem and chosen remedy

The site currently renders five Mermaid diagrams client-side. Wide diagrams
either shrink labels to fit the article width or require horizontal scrolling;
neither presentation gives a clear full-size inspection path. A global wider
center column was considered, but it would lengthen prose lines across the
whole site and still would not give mobile or zoom users a standalone vector
asset.

The chosen remedy is to replace all five Mermaid blocks with governed,
committed DOT/SVG figures. Each page provides a linked image and an explicit
“Open the full-size … (SVG)” text link. The browser's native SVG view preserves
vector zoom without a lightbox, runtime JavaScript, or forced new tab.

This is a public-documentation usability correction. It changes no AGF role,
gate, command, schema, maturity level, or adopter contract.

## Closed inventory

The inventory is exact at `origin/main` commit `82bf13f`:

| Page | Current Mermaid | Governed figure target |
|---|---|---|
| `agf/docs/site/index.md` | Governed change flow | `governed-change-flow.dot` / `.svg` |
| `agf/docs/site/docs/concepts/position.md` | Runtime/governance boundary | `runtime-governance-boundary.dot` / `.svg` |
| `agf/docs/site/docs/concepts/position.md` | AGF relative to adjacent governance categories | `governance-scope-position.dot` / `.svg` |
| `agf/docs/site/docs/concepts/coordination-protocol.md` | Gate 1-to-Gate 2 handoff sequence | `gate-handoff-sequence.dot` / `.svg` |
| `agf/docs/site/docs/start/framework-map.md` | AGF concept map | `framework-map.dot` / `.svg` |

Acceptance re-runs `rg -n '^```mermaid' agf/docs/site --glob '*.md'` and expects
zero results. Any Mermaid block added after this baseline must be reconciled by
plan amendment rather than silently omitted or converted without review.

## Frozen implementation scope

### 1. Convert the five diagrams

For each inventory row:

- add editable Graphviz source under `agf/docs/figures/src/`;
- generate and commit the matching SVG under
  `agf/docs/site/reference/figures/`;
- preserve every existing node, edge relationship, label, and claim boundary
  unless the page's surrounding prose already provides an explicitly reviewed
  equivalent simplification;
- use the governed AGF palette, shape redundancy, normally 11pt node labels,
  normally 10pt edge labels, embedded legend where semantics need one, and a
  white standalone background;
- add a useful SVG `<title>` and `<desc>` after generation through the existing
  governed figure workflow, without making hand edits that cannot be
  reproduced from source.

The sequence diagram may be re-expressed as a Graphviz handoff timeline or
swimlane, but must preserve participants, message order, activation/review
steps, and completion. It must not imply parallelism or new protocol semantics.

### 2. Give every page an open/full-size path

Replace each Mermaid fence with:

- meaningful preceding prose carrying the essential relationship;
- a Markdown image linked to its standalone SVG;
- an adjacent, explicit “Open the full-size [figure name] (SVG)” link to the
  same asset; and
- existing page-specific explanation, accessible prose, captions, and guides
  retained or corrected rather than deleted merely because a figure exists.

Do not force a new tab. The image and text link must be keyboard reachable and
must use visible focus supplied by the existing site theme.

### 3. Retire the unused Mermaid runtime

After the inventory reaches zero:

- remove the Mermaid custom fence and JavaScript entry from
  `agf/docs/build/mkdocs.yml`;
- remove `agf/docs/site/javascripts/mermaid.mjs`;
- remove Mermaid-only presentation CSS from
  `agf/docs/site/stylesheets/agf.css`;
- revise Mermaid-specific wording in
  `agf/docs/site/reference/palette.md` so the canonical current figure workflow
  is DOT-to-SVG and no public page claims a live Mermaid integration.

This removal is part of the usability fix: it eliminates a browser-side CDN
dependency that has no remaining consumer. It must not alter unrelated
JavaScript, CSS, Markdown extensions, or palette rules.

### 4. Update the figure gallery and learning record

- Add all five new figures to `agf/docs/site/reference/figures.md`, naming their
  source files and page purpose. Renumber later gallery figures mechanically if
  needed; do not rewrite unrelated descriptions.
- Append one learning to `agf/learnings/foundry.yaml` using a timestamp ID. The
  bounded lesson is that responsive shrinking, horizontal scrolling, and a
  wider article column are not substitutes for a discoverable full-size path
  when a diagram carries dense orientation or relationship information.
- Record the evidence as direct foundry user feedback across two pages and a
  five-diagram inventory—not as multi-adopter validation or a universal law.

## Presentation and accessibility requirements

- Direct SVGs remain readable without site CSS, JavaScript, Mermaid, hover,
  color alone, or pointer-only interaction.
- At 390px page width, the explicit full-size link appears with the figure and
  the document itself introduces no horizontal overflow.
- At desktop article width, labels should remain useful as orientation; the
  full-size view is an enhancement, not the only way to discover what the
  figure contains.
- At 200% browser zoom, surrounding prose and links remain operable. The direct
  SVG stays sharp under further browser zoom.
- Meaningful image alt text identifies the figure. Essential relationships
  remain available in surrounding prose; external SVG internals are not
  treated as the sole accessible description.
- Link purpose is clear out of context and repeated links on the two-diagram
  position page have distinct names.

## Acceptance evidence

- Closed five-diagram before/after inventory with zero remaining Mermaid fences.
- Each DOT source regenerates its committed SVG; repeated generation is
  byte-stable or generator metadata variation is explained.
- Claim-preservation matrix compares every removed Mermaid node/edge/message
  with the replacement SVG and surrounding prose.
- Strict MkDocs build passes without the Mermaid extension or JavaScript.
- Built HTML confirms each image is linked and each explicit full-size link
  resolves to the same SVG.
- Every direct SVG URL/path renders independently with complete labels, white
  background, title, and description.
- Desktop and 390px rendered checks cover all four pages and five figures,
  recording discoverability, article overflow, page-scale orientation, and
  guide/prose readability.
- Keyboard checks cover every explicit link; 200% zoom checks cover one page of
  each layout type (landing, single-diagram concept, two-diagram concept, dense
  map).
- Network inspection confirms public pages no longer request the Mermaid CDN.
- `agov validate adopter-docs`, umbrella `agov validate`, learning validation,
  strict link checks, `git diff --check`, and applicable CI jobs pass.
- Fact-checker reviews the exact replacement labels and prose for lost,
  strengthened, or newly implied product/governance claims before Gate 2.

## Commit and review structure

One implementation MR with four reviewable commits:

1. `docs: add governed SVG sources for Mermaid replacements` — five DOT/SVG
   pairs and reproducibility evidence.
2. `docs: replace public Mermaid blocks with openable figures` — four pages and
   figure-gallery entries.
3. `docs: retire unused Mermaid site runtime` — MkDocs, JavaScript, CSS, and
   palette-reference cleanup after the inventory reaches zero.
4. `learning: record full-size path for dense figures` — one append-only,
   bounded learning record.

Gate 2 reviews the exact final SHA, fact-check record, claim matrix, generated
assets, and rendered evidence together.

## Explicit exclusions

- No change to the current v2.4.2 release MR, SHA, tag, or publication scope.
  This work follows in a later patch/release decision.
- No global widening of the MkDocs center content column. That alternative
  harms prose measure and does not solve standalone inspection on mobile.
- No site-wide lightbox, zoom library, new JavaScript, new MkDocs plugin, or
  forced new-tab behavior.
- No retrofit of the site's existing non-Mermaid figures in this MR. Their
  page-scale/open-link usability may be inventoried separately if evidence
  warrants it.
- No redesign or semantic expansion of the five diagrams under the guise of
  conversion.
- No formal WCAG-conformance or universal assistive-technology claim.

## Stop conditions

Stop and return to planning if a replacement cannot preserve the current
relationships, if an open/zoom path requires a new runtime dependency, if a
new Mermaid block appears outside the closed inventory, or if review discovers
a factual contradiction rather than a presentation defect.

## Gate requests

- **Gate 1:** governor-reviewer approves or blocks this revised complete-
  inventory plan before any implementation begins.
- **Gate 2:** governor-reviewer reviews the exact implementation SHA and all
  generated/rendered/fact-check evidence before merge.
