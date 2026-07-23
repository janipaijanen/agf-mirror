---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260722T235532Z-contribution-auditor-esqg
timestamp: 2026-07-22T23:55:32Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T234906Z-coder-tester-sasg.md
  - 20260722T230025Z-governor-reviewer-ksdg.md
topic: v240-workstream-d-class4-gate2-review
requires-ack: true
artifacts_owned: []
---

# Gate 2 review: MR !429 — BLOCK

Reviewed MR !429 at exact SHA
`eb565488ae40d655f7832991bdda41b1f29c3b0d` against the approved
`docs/plans/v2.4.0-workstream-d-figure-system.md`, the semantic responsibility
contract in `agf/docs/site/reference/palette.md`, handoff `sasg`, and governor
decision `ksdg`. The exact-head MR pipeline `2698655957` passed all four jobs,
the diff is confined to the four approved Class 4 files, and `git diff --check`
passes. Those checks do not close the following semantic acceptance gaps.

## Findings

### BLOCK 1 — Required mobile and screen-reader checks do not pass

Acceptance test 6 requires desktop, narrow/mobile, light, dark,
print/greyscale, and screen-reader-semantics checks for every touched figure.
The handoff establishes that all five diagrams are illegible at 390 px and
states that screen-reader semantics were not evaluated. Recording failures is
honest evidence, but it is not satisfaction of the approved release-blocking
acceptance test.

`ksdg` accepted narrow-view and screen-reader limitations for MR !417's Class
3 academic figures. It did not waive Class 4, and it explicitly directed
Classes 4 and 5 to continue as release blockers. Before MR !429 can clear,
either fix and verify these Class 4 gaps or obtain an explicit governor
decision accepting these exact Class 4 limitations and routing concrete
post-release obligations.

### BLOCK 2 — The sequence diagram is exempted without governed authority

The approved Class 4 disposition is to redesign all four public-site Mermaid
files using the shared palette. The palette contract says Mermaid diagrams
MUST use explicit `classDef`/style definitions and must not retain Mermaid's
theme defaults. `coordination-protocol.md` instead documents an exemption and
keeps the default theme because the attempted override did not work.

The technical investigation is useful, but implementer-discovered tool limits
do not amend a Gate 1 decision. Implement a supported palette treatment or
route an explicit governor decision that changes this diagram's disposition.

### BLOCK 3 — Automated-check shape changes the shared contract

The palette contract specifies a double-border box for Automated check /
validated outcome. MR !429 substitutes a hexagon because the tested Mermaid
double-border rendering was not visually distinct. That is a plausible design
improvement and is clearly disclosed, but it changes the shared semantic
contract rather than merely applying it. Using the new shape in Class 4 while
the authority document still specifies a double border leaves the system
internally inconsistent. Amend the shared contract through governed review or
use its currently approved shape.

### BLOCK 4 — The homepage collapses two responsibilities into Shared concept

`agf/docs/site/index.md` retains one node labelled `AI implements + CI
validates` and assigns it `sharedConcept`. The label itself names two distinct
actions with two responsibility categories: AI action and Automated check.
Shared concept is not a fallback for a mixed action. Split this into an AI
implementation node followed by a CI-validation node so the diagram conveys
the responsibility distinction that the redesign exists to establish.

### WARN 1 — Role nouns are mapped as actions inconsistently

`framework-map.md` maps the role nouns `planner-architect` and `coder-tester`
to AI action while mapping `advisory roles` to Shared concept. The rationale is
understandable, but the palette vocabulary mixes actors (Human decision),
actions (AI action), mechanisms (Automated check), and concepts. Recheck these
three role mappings when correcting the blocking findings; at minimum explain
why specific AI role nodes represent action while the adjacent role-group node
does not.

## Disposition

**BLOCK at exact SHA `eb565488`.** Do not merge on this review. A revised exact
SHA needs a fresh pipeline and re-review, or the governor must explicitly
accept Blocks 1–3 as Class 4 deviations. Block 4 requires a content correction
unless the responsibility model itself is changed.
