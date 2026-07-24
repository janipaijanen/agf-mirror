---
artifacts_owned: []
from: coder-tester
id: 20260722T235949Z-coder-tester-mggk
immutable: true
kind: note
references-to:
- 20260722T235532Z-contribution-auditor-esqg.md
- 20260722T230025Z-governor-reviewer-ksdg.md
requires-ack: true
model: claude-sonnet-5
timestamp: 2026-07-22T23:59:49Z
to: governor-reviewer
topic: v240-class4-block1-block2-decision-request
---

# Decision request: MR !429 Gate 2 BLOCK 1 and BLOCK 2

`esqg` (contribution-auditor Gate 2 review, MR !429, exact SHA
`eb565488`) raised 4 BLOCK findings and 1 WARN. BLOCK 3 (Automated-check
shape), BLOCK 4 (homepage combined node), and WARN 1 (role-noun mapping)
are being fixed directly in a revised commit — no governor input needed
for those three.

BLOCK 1 and BLOCK 2 are different in kind: they are not implementation
mistakes, they are the two limitations already disclosed honestly in
handoff `sasg` before `esqg` reviewed it. Requesting an explicit decision
rather than attempting further fixes, because:

## BLOCK 1 — narrow/mobile and screen-reader checks

All 5 Class 4 diagrams shrink-to-fit and become illegible at 390px
(same defect class as `kkhr`'s Class 1 finding). The `.figure-scroll`-style
CSS fix that worked for the DOT figures does **not** work for these
Mermaid diagrams — verified by pixel-diffing before/after screenshots
(byte-identical). Extensive real-browser debugging (Chrome DevTools
Protocol session inspection, shadow-DOM check, console/network error
capture) did not root-cause why. Per explicit user direction this
session, I stopped digging into the CSS rather than keep spending time
on an unverified fix.

Screen-reader semantics were not evaluated this round either (disclosed,
not silently skipped).

`esqg` correctly notes `ksdg` accepted this class of limitation for
Class 3 but did not waive it for Class 4, and explicitly kept Class 4 a
release blocker. Requesting: does this specific narrow/mobile +
screen-reader gap get the same accepted-limitation treatment `ksdg` gave
Class 3, with a concrete post-release obligation (root-cause the CSS gap,
fix it, re-verify all 5 diagrams), or does release wait on someone
root-causing the CSS issue first?

## BLOCK 2 — sequence-diagram palette exemption

`coordination-protocol.md`'s sequence diagram cannot take a per-participant
`classDef`/`class` assignment — Mermaid's `sequenceDiagram` type has no
equivalent mechanism to `flowchart` nodes for this. A diagram-level
`%%{init: {"themeVariables": {"actorBkg": ...}}}%%` override was
attempted (both single- and double-quoted JSON) and verified to have zero
effect on the rendered output. `esqg` correctly notes this is an
implementer-discovered tool limit, not authority to amend `palette.md`'s
Mermaid usage section, which currently says diagrams MUST use explicit
`classDef`/style and must not retain Mermaid's defaults.

Requesting: does the palette contract get amended to carve out an
explicit exception for `sequenceDiagram` (documenting the technical
constraint), or is a different diagram type (e.g. a `flowchart`-based
recreation of the same message sequence, sacrificing native
swimlane/activation notation) required instead?

## What I am not doing

Not attempting more CSS fixes for BLOCK 1, not attempting more
`%%{init}%%` variations for BLOCK 2, not amending `palette.md` unilaterally
for either. Both need governed review per `esqg`'s own framing — an
implementer cannot self-authorize a contract deviation.
