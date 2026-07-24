---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260723T204829Z-planner-architect-n31b
timestamp: 2026-07-23T20:48:29Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260723T202430Z-contribution-auditor-6q6p
topic: v241-live-browser-evidence
requires-ack: true
artifacts_owned:
  - agf/docs/site/stylesheets/agf.css
---

# Live-browser evidence attempt for `6q6p`'s blocking gaps — partial, one real bug found and partially fixed, AT smoke test still not done

Attempted the four items `6q6p` flagged as blocking: desktop/390px live
rendering, keyboard/zoom-reflow, browser accessibility-tree inspection, and a
named screen-reader smoke test. Used Playwright + both Playwright's bundled
Chromium and the system's `/usr/bin/chromium`, driven against an isolated
Xvnc virtual display (`:99`, TigerVNC, `-SecurityTypes None -localhost`) —
**not** the user's live desktop session (`:0`, confirmed as an active logged-in
session before choosing this approach; deliberately not touched).

## What this surfaced first: Material's own mermaid auto-render does not fire under Playwright

Before any of the four items could be attempted, discovered that Material's
built-in `pre.mermaid → mermaid.run()` auto-detection never fires under
Playwright-driven navigation — reproduced identically in headless and headed
Chromium, on this MR's local build, on the **live production site**
(`janipaijanen.gitlab.io/agents-govern-framework`), and on the **upstream
mkdocs-material reference docs** (`squidfunk.github.io/mkdocs-material`). This
is not something introduced by this MR or specific to this repo — it
reproduces on Material's own docs. Root cause not fully diagnosed (Material's
bundle does locate `pre.mermaid` elements — confirmed via a wrapped-function
trace showing `mermaid.initialize()` is called — but `mermaid.run()` is never
subsequently invoked, and the `pre` gets replaced with an emptied `div`
regardless). Worked around it by reading each diagram's exact source
server-side from the built HTML and calling `window.mermaid.render()`
directly — same mermaid engine, same version, loaded from the same
`unpkg.com/mermaid@11.16.0` CDN path Material's bundle itself fetches.

## 1. Desktop + 390px live rendering — DONE, screenshots captured

All 5 diagrams rendered successfully via the workaround above, at both
1280×900 and 390×844 viewports. Screenshots (full page + isolated element,
both sizes, all 5 diagrams — 20 PNGs) are in
`/tmp/.../scratchpad/evidence/*.png` in this session's scratchpad — **not
committed to the repo** (ephemeral verification artifacts, same precedent as
the earlier `mmdc` CLI check in `gj5s`/`6get`). Happy to attach or describe
any specific one on request.

## 2. Browser accessibility-tree inspection — DONE

For each diagram, captured the live SVG's `role`, `aria-roledescription`,
`aria-labelledby`/`aria-describedby` wiring, and confirmed the referenced
`<title>`/`<desc>` element IDs actually match (not just present — wired).
All 5: `titleIdMatchesLabelledby: true`, `descIdMatchesDescribedby: true`,
title/desc text matches the `accTitle`/`accDescr` source exactly. Also
captured Playwright's `ariaSnapshot()` (Chromium's actual accessibility tree,
the same tree an OS AT-SPI bridge reads from) for each diagram — e.g. the
coordination-protocol sequence diagram's tree reads as a "document" named
"Gate 1 to Gate 2 coordination handoff sequence" containing the participant
and message-label text. Full JSON in `evidence/report.json`.

## 3. Keyboard reachability / zoom-reflow — DONE, one tooling bug caught and corrected before reporting

`focusableElementsInsideDiagram: 0` for all 5 — mermaid SVGs are
presentational, not interactive, so this is the *expected* correct state,
not a gap. A first automated keyboard-trap check falsely reported
`keyboardTrapDetected: true` on every page; on inspection this was a bug in
*my own* comparison logic (matched on tag+className, which collides across
several distinct anchor tags with no class), not a real trap. Re-ran a clean
manual 22-step Tab trace on `index.html`: skip-link → logo → version badge →
search → nav → TOC anchors, every stop distinct, no repetition, diagram
never receives focus. Correcting this here rather than letting the false
positive stand.

Zoom-equivalent check (640px viewport, simulating ~200% zoom on a 1280px
baseline): `document.body.scrollWidth === document.body.clientWidth` on all
5 pages — the horizontal-scroll reflow stays contained to the `.mermaid`
element, does not break the page's own layout.

## 4. Named screen-reader smoke test — NOT DONE, stopped here

Set up an isolated Xvnc display and confirmed AT-SPI/`speech-dispatcher` are
installed, but did not get Orca actually running against a real, speaking
session before being told to stop and save state. This item remains open.

## A real bug found and partially fixed: `agf.css`'s #55 CSS does not achieve native-width scroll for Mermaid

While capturing the 390px screenshots, `mobile390Reflow.horizontallyScrollable`
was `false` on all 5 diagrams — the intended "scroll instead of shrink"
behavior from commit `46080e2` was not actually happening, even though the
CSS rule was present. Root cause: mermaid's generated SVG carries an inline
`style="max-width: NNNpx"` attribute, and an inline style always wins over an
external stylesheet rule regardless of selector specificity *unless* that
rule uses `!important` — the original rule didn't. Added `!important`
(commit staged, see diff) — this fixed the `max-width` computed value
(`none` now, confirmed live), but did **not** fix the shrink: mermaid's SVG
*also* carries `width="100%"`, and per the CSS2.1 replaced-element sizing
algorithm, a block-level SVG with only an intrinsic aspect ratio (`viewBox`)
and no intrinsic absolute width still resolves `width: auto` to "fill the
containing block" — confirmed by testing `width: auto`, `width: max-content`,
and `display: inline-block` against the live-rendered diagrams; every
variant still shrank to the 343px container width instead of scrolling.
This differs from the DOT-generated figures elsewhere on the site, whose SVG
files have absolute pixel width/height (not `100%`), so `max-width: none`
alone is sufficient for those — it is not sufficient for mermaid's output.

**Current state of `agf.css`:** kept the `!important` fix (strictly correct
and verified) and documented the residual gap in a code comment rather than
claiming full resolution. A real fix needs mermaid's own `useMaxWidth: false`
render option, which requires a custom `extra_javascript` re-init hook —
that's new JS delivery infrastructure, out of this MR's CSS-only #55 scope
and not something to add without a governor call. Flagging rather than
silently shipping a partial fix as if it were complete.

## Disposition needed

1. Screen-reader smoke test (item 4) — still needs either continued
   Xvnc+Orca setup effort, or a human running it, or an explicit governed
   waiver.
2. The mermaid native-width-scroll gap — needs a decision: accept
   shrink-to-fit-with-legible-minimum-size as sufficient (would need
   re-verifying label legibility at 390px specifically, not yet done), defer
   a `useMaxWidth:false` JS fix to a follow-up MR, or authorize adding it
   here.

Everything above is committed to `feat/v241-accessibility-prose-hardening`
(this note + the `agf.css` `!important` fix). Stopping here per instruction.
