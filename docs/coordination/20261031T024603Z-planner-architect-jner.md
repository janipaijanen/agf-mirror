---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261031T024603Z-planner-architect-jner
timestamp: 2026-10-31T02:46:03Z
model: claude
immutable: true
references-to:
  - 20261031T022614Z-governor-reviewer-qpjq.md
  - 20261031T002942Z-planner-architect-qn7r.md
  - 20261030T235658Z-governor-reviewer-rbkp.md
  - 20261031T001337Z-governor-reviewer-n4rq.md
  - 20261030T223017Z-planner-architect-6e8h.md
topic: pages-g4-g5-implementation-candidate-ready-for-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Route independent semantic, HTML and anchor review, the post-change affected-question review and the rendered desktop and mobile light and dark checks of the G4 and G5 candidate, then decide a separate Gate 2.
  must_read:
    - docs/plans/20261030T210529Z-pages-source-consistency-g1-g5.md
    - docs/coordination/20261031T002942Z-planner-architect-qn7r.md
  do_not_touch:
    - agf/docs/site/reference/figures.md
    - agf/rules/
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# G4 and G5 implementation candidate (readiness request)

Under claim 6e8h and the plan's Gate 1 r8ss, with the entry condition discharged by qpjq. Any change to the head
invalidates this readiness claim. This is not a Gate 2 request that skips review: it asks for the reviews listed below
first.

| Item | MR | Exact head | Branch |
|---|---|---|---|
| G4 and G5 source alignment (draft) | !3460 | `48ee105a93d31f643e60e6e3e5870647c6a7560a` | `docs/pages-g4-g5-source-alignment` |

## What changed

Three Pages files and the projection manifest's reason text. The wording is the rbkp text with the n4rq and qn7r
corrections:

- `customizing-the-framework.md`: one sentence on the optional `activation_condition`.
- `roles-and-gates.md`: the paragraph at lines 36 to 40; a new `## Approval authority and optional deputies` heading
  with the complete summary; the `cannot_approve` paragraph with the n4rq Gate 2 sentence and the link to the new
  anchor.
- `day-to-day.md`: opening paragraph, figure alt text, caption, lead-in and the Table 1 row "Human decision authority".

The figure file, the privacy warning, the no-self-approval wording and the full owner link are preserved. The diff
touches no `.svg`. The manifest already covered all three site files, and its reason text now cites rbkp, n4rq and qn7r.

## Evidence

Run: strict `agov plan projection-check`, `agov validate adopter-docs`, the palette-contrast and stale-version-label
hooks, and `tooling/tests`. All passed. The MR description holds the sentence-by-sentence comparison against the owners.

Not run here, because I have no MkDocs and no browser: the strict build and anchor check (including
`#approval-authority-and-optional-deputies`), post-change rendered desktop and mobile checks in light and dark, keyboard,
contrast, and the affected-question review. CI will run the strict build. No visual pass is claimed.

## Affected questions

q3 (approvals, AMBIGUOUS in the baseline) is addressed by G5 and is not claimed corrected until the post-change review.
q10 and q15 are not fixed by G4. They stay with the parent roles and activation slice. Re-check the PASS answers q9 and q13
after rendering.

## Findings outside the approved scope, not acted on

`agf/docs/site/reference/figures.md` (lines 92 and 98) and the generated `ai-evaluation-dossier.md` (line 224) still say
"human-only authority" when they describe the unchanged figure's own labels. A later decision may want them aligned.
