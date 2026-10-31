<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 1 ("Who does what?"): sentence-preservation report and anchor map

- **Status:** advisory review evidence for the slice 1 candidate. It is not a complete semantic inventory and not a blocking check. Review unflagged safeguards too.
- **Plan:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 4. Claim: `20261031T055947Z-planner-architect-m2xq`.
- **Base:** main `705114cc`. Files: `roles-and-gates.md` and `position.md`.
- **Method:** sentences in the base files that contain MUST, must, never, only, approve, approval or authority (tables, code, comments and headings removed, whitespace normalized) are checked for verbatim presence in the final files. The script is the inline Python in the author's session; rerun by pasting the same regular expression.

## `roles-and-gates.md`

- Candidate sentences in the base: 37. Present verbatim in the final: 37. Not found verbatim: 0.
- Headings before: 10. After: 13. Removed anchors: none. Added anchors: ['ask-for-a-specialist-during-a-change', 'roles-in-plain-terms', 'which-roles-does-my-project-need'].

## `position.md`

- Candidate sentences in the base: 4. Present verbatim in the final: 4. Not found verbatim: 0.
- Headings before: 11. After: 11. Removed anchors: none. Added anchors: none.

## Result

All 41 candidate sentences are present verbatim in the final files, so no disposition entry is needed. The slice adds
text and removes none. This check cannot see a meaning change caused by new text placed next to an old sentence, so
the independent factual review still reads the new sections against the owners.

## What the slice adds

- `roles-and-gates.md`: sections "Roles in plain terms", "Which roles does my project need?" and "Ask for a specialist
  during a change". Existing headings, anchors and the `#approval-authority-and-optional-deputies` anchor are kept.
- `position.md`: three lines in the opening paragraph that define an agent as a role definition and link to
  `roles-and-gates.md#roles-in-plain-terms`.

## What the slice does not do

It does not publish the proposal paragraph "Planning, building and approving are separate". That paragraph needs a
governor decision before public use. It does not change the existing sentence "No agent both implements and approves
its own work.", the gate descriptions, the figures, or any canonical owner. It makes no policy change.

