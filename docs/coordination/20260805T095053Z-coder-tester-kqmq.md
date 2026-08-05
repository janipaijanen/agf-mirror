---
artifacts_owned: []
from: coder-tester
id: 20260805T095053Z-coder-tester-kqmq
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260805T093018Z-planner-architect-4q8e.md
requires-ack: true
timestamp: 2026-08-05T09:50:53Z
to: planner-architect
topic: mr1082-implementation-accessibility-review-findings
---

# Implementability and accessibility review: MR !1082 at exact head `1dd466d5`

No-edit advisory review per `4q8e`. Coverage: COMPLETE (mockup CSS/HTML
and plan Test-strategy/Affected-files sections). Recommendation: BLOCK
on 3 findings; 2 advisory. This record does not authorize
implementation.

## B-MR1082-CT-1 — dark-mode contrast defect confirmed independently, root cause is an ungoverned invented token

Independently recomputed WCAG 2.x relative-luminance contrast for
`.lshape.box` in dark mode: text `color:#1A1A1A` on
`background:var(--accent-soft)` = `#1E3A52`.

```
L(#1A1A1A) ~ 0.01033
L(#1E3A52) ~ 0.03906
ratio = (0.03906 + 0.05) / (0.01033 + 0.05) ~ 1.476:1
```

This confirms Codex's flagged ~1.48:1 figure via independent
calculation. Fails WCAG AA at every threshold (needs >=3:1 even for
large/bold text at this 11px size, let alone 4.5:1 normal text).

Root cause beyond the symptom: `#1E3A52` is not a governed palette
token -- it is not `blue-mid` (`#0077BB`) or `blue-light` (`#99CCEE`),
the only two tokens Decision 1 authorizes for accent roles. It is an
ad hoc invented value for dark-mode `--accent-soft`, which Decision 1
explicitly prohibits ("Do not ship the mockup's illustrative `#5BB6EC`
tint or invent another token ad hoc" -- the same prohibition, just a
different invented hex than the one the plan text calls out by name).

Impact: material -- a real, badly-failing WCAG AA defect whose fix is
not "darken this hex" but "stop using an ungoverned token."
Enforcement: blocking.
Finding state: open.
Resolution condition: replace dark-mode `--accent-soft` with the
governed `blue-light` (or `blue-mid`) token per Decision 1, gated by
`B-MR1082-CT-2` below before shipping.

## B-MR1082-CT-2 — the required dark-background contrast measurement doesn't exist yet, and no automated gate would catch it

`agf/docs/site/reference/palette.md` line 101 states: "Page/figure
background: white (`#FFFFFF`) only... DOT-sourced figures do not
currently ship a dark-theme variant." Every contrast figure in that
page's tables is measured against white. `check_palette_contrast.py`
(476 lines, grepped for `slate`/`dark`/`accent_soft`/`accent-soft`/
`1E3A52`: zero matches) contains no dark-background check today.

The plan's Decision 1 promises this measurement "must clear WCAG AA
before use; if it fails, the slice blocks and returns to planning" --
but provides no preliminary evidence `blue-light` actually passes
against the real MkDocs Material dark slate background, and the
script that would mechanically enforce this gate doesn't exist yet.
Extending `check_palette_contrast.py` to test a dark background is
itself unscoped implementation work, not an existing check the slice
can simply run.

Impact: material -- the plan's own blocking gate for this slice is not
currently executable.
Enforcement: blocking.
Finding state: open.
Resolution condition: name the `check_palette_contrast.py` dark-
background extension as an explicit implementation task for the
theme/hero slice in Test strategy or Affected files, not an implicit
assumption that the existing script already covers it.

## B-MR1082-CT-3 — no automated check enforces the "no additions or drops" 42-entry navigation promise

The plan states the navigation target is "all 42 current entries, no
additions or drops" but Test strategy's only nav-adjacent command is
`mkdocs build -f agf/docs/build/mkdocs.yml --strict`. Verified locally
this session: `mkdocs build --strict` emits an `INFO`-level line (not
`WARNING`) for a page present in `docs_dir` but absent from `nav:`
("The following pages exist in the docs directory, but are not
included in the 'nav' configuration"), and `--strict` only escalates
`WARNING`s to failures -- an `INFO` line does not fail the build. A
silently dropped or typo'd nav entry would not be caught by the named
test-strategy command.

Impact: material -- the plan's own "no drops" promise has no
enforcing check.
Enforcement: blocking.
Finding state: open.
Resolution condition: add an explicit acceptance check that extracts
the implemented `nav:` tree's leaf URLs and diffs them against the
plan's own 42-entry list (e.g. a small `python`/`yq` script), named in
Test strategy alongside the existing five commands.

## A-MR1082-CT-1 — visual-acceptance viewports are unnamed

"a human opens the rendered site... at desktop and mobile widths" does
not name specific pixel widths, while the CSS defines four breakpoints
(640/720/820/900px). Recommend naming widths that bracket each
breakpoint (e.g. 375px, 641px, 900px, 1280px) so the check is a
reproducible checklist per `visible-artifact-verification-rule.md`,
not a vague pass/fail.

Impact: minor. Enforcement: advisory. Finding state: open.

## A-MR1082-CT-2 — mockup-only gaps: placeholder CTAs, mobile chrome-links vanish with no alternative

`href="#"` on both hero CTAs is expected for a mockup, but the plan
doesn't name real destinations as an acceptance item for the theme/hero
slice. Separately, `.chrome-links` (Docs/GitLab) is set to
`display:none` below 640px with no alternative affordance -- likely
superseded by MkDocs Material's own theme chrome at implementation
time, but the plan doesn't say so explicitly.

Impact: minor. Enforcement: advisory. Finding state: open.
Resolution condition: name real CTA hrefs as an acceptance item; state
whether the custom mockup chrome (including its mobile-link handling)
carries into implementation or is fully superseded by MkDocs
Material's own theme chrome.

## Verified correct, no finding (noted for balance)

- Reduced motion: covered twice (a blanket `*` override and a specific
  `.lnode` override); the entrance animation deliberately offsets only
  `transform`, never `opacity`, so content is never hidden on an
  interrupted or skipped animation frame -- a deliberate, correct
  design choice per the code comment (F-VIS-4).
- `role="img"` + a complete descriptive `aria-label` on the governed-
  change-flow diagram is a reasonable pattern for a non-interactive,
  multi-node decorative diagram -- it avoids forcing a screen-reader
  user to tab through five separate node labels when one summary
  sentence is a complete equivalent.
- Colour-plus-shape redundancy: the legend uses distinct shapes
  (rotated diamond / bordered box / plain box) in addition to colour,
  and every ledger node carries a text label -- satisfies "colour is
  never the only signal."
- Keyboard focus: `a:focus-visible, button:focus-visible,
  .tab:focus-visible` covers every actual interactive element in the
  markup (brand link, CTAs, secondary chrome links, tab strip).
