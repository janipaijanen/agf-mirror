---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-d-figure-system.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Gate 2 review of MR !429 (Workstream D Class 4, four inline Mermaid files).
model: claude-sonnet-5
id: 20260722T234906Z-coder-tester-sasg
immutable: true
kind: handoff
references-to:
- 20260722T230526Z-contribution-auditor-qhm2.md
- 20260722T230025Z-governor-reviewer-ksdg.md
requires-ack: true
timestamp: 2026-07-22T23:49:06Z
to: contribution-auditor
topic: v240-workstream-d-class4-gate2
---

# MR !429 — Class 4 Mermaid diagrams, ready for Gate 2

Branch `feat/v240-workstream-d-class4-mermaid`, exact SHA
`eb565488ae40d655f7832991bdda41b1f29c3b0d`, rebased on current `main`
(includes merged MR !417). Seed-then-MR sequence followed correctly this
time (draft MR opened before the real commit) — no duplicate branch
pipeline.

## Scope

The four approved inline Mermaid files, no more:

- `agf/docs/site/docs/concepts/position.md` (2 diagrams)
- `agf/docs/site/index.md` (1 diagram)
- `agf/docs/site/docs/concepts/coordination-protocol.md` (1 diagram)
- `agf/docs/site/docs/start/framework-map.md` (1 diagram)

## Palette application

All flowchart-based diagrams use explicit `classDef`/`class` per
`palette.md`'s Mermaid usage section — no node left on Mermaid's default
theme fill. Shape carries the distinction independent of colour, per the
palette's colour-independence requirement:

- Human decision → diamond (`{ }`)
- AI action → rounded box (`( )`)
- Automated check → hexagon (`{{ }}`) — chosen over the `[[ ]]` subroutine
  shape after real-browser inspection showed the subroutine's double-bar
  edges rendering indistinguishably from a plain box at this Mermaid
  version/theme; the hexagon's angled outline is unambiguous at any zoom,
  confirmed by pixel-zoomed screenshot.
- Shared concept → plain box, white fill, blue-mid border/text
- Reference/deferred → plain box, grey fill, dashed border

Per-diagram responsibility mappings and rationale are documented inline
in each file's palette note/comment (visible in the diff), not just in
this handoff — matching the Class 3 precedent of keeping rationale next
to the artifact.

## Sequence diagram limitation (coordination-protocol.md)

Verified in a real browser via pixel-sampling the rendered participant-box
fill (not assumed): `sequenceDiagram` participants have no `classDef`/
`class` mechanism the way `flowchart` nodes do, and an attempted
diagram-level `%%{init: {"themeVariables": {"actorBkg": ...}}}%%` override
(tried single- and double-quoted) produced a byte-identical render to no
override — this site's rendering pipeline doesn't honour it. Removed the
ineffective directive rather than ship dead code; documented in the
in-page palette note. This diagram keeps Mermaid's own default theme,
which already tracks the site's light/dark toggle automatically.

## Overlap check (coordination-protocol.md)

Checked the Mermaid sequence diagram against the page's existing
DOT-sourced `coordination-map-example` figure (same page, further down).
Not duplicative: the sequence diagram shows temporal message order for
one linear example thread; the DOT figure shows network topology and
message volume across the full participant set. Kept both.

## Accessibility evidence (desktop/narrow/light/dark/print/screen-reader)

- **Desktop**: real Chromium headless screenshots of all 4 pages,
  visually inspected — correct shapes, correct legend/rationale text,
  legible at full resolution.
- **Narrow/mobile (390px)**: real-browser screenshot confirmed all 5
  diagrams (framework-map's 30-node diagram worst-case) shrink-to-fit and
  become illegible at this width — the same defect class `kkhr` flagged
  for the DOT figures. I attempted the same `.figure-scroll`-style CSS
  fix used there; it measurably had **zero effect** on the rendered
  output (pixel-diffed before/after — byte-identical screenshots, both
  for the simple 5-node index.md diagram and the worst-case 30-node
  framework-map.md diagram). Extensive real-browser debugging (CDP
  session inspection, shadow-DOM check, console/network error capture)
  did not conclusively root-cause why the CSS had no effect, and I
  stopped chasing it rather than keep spending review-cycle time on an
  unverified fix per explicit user direction. **This is a real, disclosed,
  unfixed limitation** — not claimed as resolved. The CSS attempt was
  reverted (agf.css has zero diff versus pre-Class-4 `main`) rather than
  ship inert dead code.
- **Light**: default theme, the primary inspected render above.
- **Dark**: flowchart-based diagrams use fixed hex fills (not
  dark-adaptive) — the same explicit "fixed fill regardless of page
  theme" contract `palette.md`'s "Light/dark behaviour" section already
  documents as an accepted decision for DOT figures, extended here to
  Mermaid flowcharts for consistency. The sequence diagram (no fixed
  fill applied, see above) does track the page theme automatically.
- **Print/greyscale**: real-browser screenshots converted to 8-bit
  greyscale (Pillow), visually inspected for `index.md` and
  `framework-map.md` (widest category coverage: all 5 responsibility
  categories present). Diamond / rounded box / plain box / hexagon /
  dashed box remain shape-distinguishable with colour removed.
- **Screen-reader/semantic**: not separately evaluated this round: Mermaid
  emits diagram content as inline SVG with generated `<title>`/structure,
  not evaluated against a specific screen-reader this pass — flagging as
  unverified rather than silently assuming parity with Class 1's DOT-figure
  `alt=` text.

## Explicitly accepted / deferred limitations (proposed, for governor decision)

Following the `ksdg` precedent rather than blocking release on these:

1. Narrow/mobile shrink-to-fit illegibility on all 5 diagrams (above) —
   real fix needs someone who can dig into why the CSS override has no
   effect (shadow DOM was ruled out; root cause still unknown).
2. Screen-reader/semantic evaluation not performed this round.
3. framework-map.md's 30-node diagram is dense; no attempt made to
   simplify or split it in this slice — palette-only scope, per plan.

## Verification

- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` — clean, no
  broken-link errors.
- `python3 agf/docs/build/hooks/check_palette_contrast.py` (+
  `--self-test`) — pass (does not cover Mermaid `.md` sources, only DOT;
  Mermaid hex values were reused verbatim from `palette.md`'s own
  pre-verified contrast table, not independently re-checked here).

## Pipeline disclosure

Seed-then-MR sequence followed: empty `[ci skip]` commit pushed first,
draft MR !429 opened, then the real implementation commit pushed. Exact
SHA `eb565488ae40d655f7832991bdda41b1f29c3b0d`, pipeline `2698655957`
(source=merge_request_event) — **succeeded**, 4/4 jobs green, no retries:

| Job | Stage | Result |
|---|---|---|
| `rust:agov:validate` | test | success |
| `agf:dist:validate` | test | success |
| `agf:dist:preview` | build | success |
| `site-html-build` | build | success |

No duplicate branch-sourced pipeline for this commit (only the seed
commit had branch+MR pipelines, both correctly skipped/no-op per the CI
dedup rule).

## Status

Ready for Gate 2 review. Two real, disclosed limitations above (mobile
shrink, screen-reader) are proposed as post-release deferrals matching
`ksdg`'s precedent, not claimed as resolved.
