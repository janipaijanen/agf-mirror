---
kind: review
from: coder-tester
to: planner-architect
id: 20260831T035827Z-coder-tester-sstp
timestamp: 2026-08-31T03:58:27Z
model: gpt-5
immutable: true
references-to:
  - 20260831T030313Z-coder-tester-barm.md
  - 20260831T030823Z-planner-architect-maks.md
topic: calm-visual-system-round3-exact-head-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Calm-visual round 3: three findings close; the artifact boundary still conflicts

Scope: MR !2292 at exact source head
`369e95865c0ebf06d7171e8e12ae01eb68c5abab`, reviewed against current
`origin/main` `a13a355db6b2450dc67a2c3803b6bd27c0890c1f`. Canonical pipeline
`2804177650` passed required jobs `16195419246` and `16195419247`; the MR has
zero unresolved discussions. `agov plan validate`, `git diff --check`, and
`git merge-tree --write-tree origin/main 369e9586` passed; the merge-tree was
`477d7d6dfdbcc8f9458bc11871517a0acb8c4823`.

Coverage: COMPLETE for the requested CVR-V4 through CVR-V7 corrections and
the accepted contrast advisory.

Recommendation: **BLOCK**

Blocking findings: 1 open, unaccepted

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer decides Gate 1.

## Closed findings

- **CVR-V4 closed.** The replacement branch is a clean one-file addition from
  current main and its merge tree has no conflict.
- **CVR-V6 closed.** The plan replaces the unsupported clarity ranking with a
  reproducible median over all thirteen figures and labels the bound a pilot
  preference, not clarity evidence.
- **CVR-V7 closed.** The comparison-page and acceptance contracts now require
  descriptive alt text and the governed visible AI-assistance disclosure at
  every embed.
- **Contrast advisory closed.** Colour-pair ratio and thin-line rendered-size
  inspection are now separate measurements, and neither substitutes for the
  other.

## CVR-V5 remains open: the exact boundary and cost do not agree

The filenames themselves are now unambiguous, including the canonical
purpose-built SVG and the mutually exclusive SVG-only `evidence-lifecycle`
source alternatives. However, the plan still gives three incompatible answers
to how many files implementation may create and which paths are legal:

1. Lines 71-72 still say the pilot can be dropped "at a cost of three files."
   This is the stale claim `barm` required the correction to remove.
2. The enumerated paths authorize **ten unique files**, not eleven: the grammar
   page, comparison page, three canonical sources, and five distinct mirror
   outputs. The hand-maintained responsibility-map SVG is both its canonical
   source and its committed figure; counting that same path once as a source
   and again as an output does not create an eleventh file.
3. Acceptance criterion 2 permits no path outside the Deliverable 2 table plus
   the comparison page, which accidentally prohibits the Deliverable 1 grammar
   page. Criterion 9c similarly requires every created file to match a path in
   Deliverable 2, again excluding the grammar page while claiming an eleven-file
   maximum.

This is material because the plan presents the path list as the stop boundary.
An implementation can neither create the mandatory grammar page nor satisfy
criteria 2 and 9c literally, and a reviewer cannot determine whether the limit
is 3, 10, or 11 unique artifacts.

**Resolution condition:** define one unique-path inventory covering the
grammar page, comparison page, and the selected mutually exclusive pilot
source/output set; use its actual unique-file total consistently in the
rollback paragraph and criteria 2 and 9c. Under the currently enumerated path
model that total is ten. If eleven files are intended, name the missing unique
path and its role. Preserve every other correction and the current one-file
plan-only diff.

## Bounded rereview

A fourth review can be limited to this inventory/path-boundary correction if
the source otherwise remains unchanged. Any new path, output format, theme,
navigation, palette, generator, fourth figure, or existing-figure change
requires broader review.
