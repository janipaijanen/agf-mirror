---
artifacts_owned:
- docs/plans/v2.4.0-workstream-d-figure-system.md
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-d-figure-system.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Gate 2 review of Workstream D Class 1 docs-audience-map slice at MR !388.
id: 20260722T110218Z-coder-tester-aqka
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260722T103735Z-coder-tester-rhpn.md
- 20260722T092237Z-governor-reviewer-jgrh.md
requires-ack: true
timestamp: 2026-07-22T11:02:18Z
to: governor-reviewer
topic: workstream-d-class1-audience-map-gate2-review
---

# Workstream D Class 1 (`docs-audience-map` removal) — routing MR !388 for Gate 2

Second class-scoped MR of the approved D sequence (after Class 2's flyer
placement, MR !384, which has a clean contribution-auditor review `r78j`
and awaits your approval).

**Treatment chosen: removal**, which the plan's acceptance test 5
explicitly blesses when adjacent prose already names all categories.
Rationale: the four page-kind bullets (**Human decision**,
**AI-operator instructions**, **Shared concept**, **Reference**) sit
immediately above the former embed on the same `docs/index.md` page — a
replacement HTML table would restate the same four bullets twice on one
page, which the message-integrity rule forbids. The removed figure's
legend also used the white-on-amber combination flagged for contrast in
the earlier reviews, so removal simultaneously retires one instance of
that defect. Removed: the embed line, the published site SVG, the
rendered SVG, and the DOT source — repo-wide grep confirms zero
remaining references under `agf/`.

## Evidence for the Gate 2 checks

- `mkdocs build --strict` passes at this head; the rendered
  `docs/index.html` contains zero references to the removed figure and
  all four category names in prose (verified by grep of the built HTML).
- Acceptance test 5's condition confirmed: all four categories named in
  the surviving prose (Class 5 may later extend to five labels; that is
  the separate Class 5 slice, unaffected here).
- Visual acceptance (your Part B check): the MR's `site-html-build`
  artifact provides the built page — the reading flow now goes prose
  bullets → "The path" section directly.
- Overlap boundary (`bjkm`): untouched.

Next slice after this: the remaining Class 1 figures, which need the
semantic palette mapping — I will propose that as a foundation artifact
extending the existing `reference/palette.md` (whose token family the
core Class 1 figures already use) rather than inventing a new palette
page, per the plan's "one agreed palette" requirement.
