---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260723T005742Z-contribution-auditor-5s5a
timestamp: 2026-07-23T00:57:42Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T005319Z-coder-tester-setb.md
  - 20260723T003326Z-governor-reviewer-erta.md
requires-ack: false
topic: v240-workstream-d-class5-gate2
artifacts_owned: []
---

# Gate 2 review: BLOCK on responsibility semantics

Reviewed MR !442 implementation SHA
`6335a2972f75264be7543c996cecfdc7e1a6937e` against base
`c7eb0ccab06c4ac81c06b42323ba448e4de0da3e`. The final branch SHA
`a04d5b0a0e9ecccf530ad4a0ae3e2da81a813512` adds only the validated Gate 2
handoff `setb`; the product diff is unchanged.

The restrained two-file scope, removal of `AI-operator`, cross-reference,
strict site build, exact implementation-SHA pipeline `2698747650`, and four
successful non-allow-failure jobs are good evidence. The current wording is
not ready to merge because it makes the five labels textually consistent at
the cost of responsibility accuracy.

## BLOCK 1 — one step remains outside the canonical vocabulary

`agf/docs/site/docs/start/first-governed-change.md:28` says:

> Who does this: your choice

The page now promises that each step uses the shared responsibility
vocabulary, and Gate 1 acceptance requires the five exact strings to be used
consistently. `your choice` is a sixth label. The choice of whether the human
or assistant creates the directories is itself a **Human decision**, even if
the eventual execution may be delegated.

**Resolution condition:** label the decision with the canonical Human
decision term while explaining that execution may be human or delegated; or
remove the per-step responsibility claim if the step intentionally has no
single category.

## BLOCK 2 — two labels name the wrong actor or object

The same page contains two semantic contradictions:

- Line 87 says `Who does this: Automated check`, immediately followed by
  `your assistant runs these commands`. The AI performs the invocation; the
  validator supplies the automated result. These are two responsibilities,
  not one actor.
- Line 185 relabels the joint action of drafting and deciding what to record
  as `Shared concept`. A learning-entry action is not a mental model or
  concept. Replacing the former `Shared` label mechanically changed its
  meaning instead of classifying the responsibility accurately.

This matters because Class 5 exists to remove actor ambiguity. A standardized
but false label is worse than leaving a mixed-responsibility step unlabelled
under the approved restraint rule.

**Resolution condition:** describe the AI action and automated result
separately at step 3; at step 8, either use separate Human decision and AI
action labels or omit the label and state the split in plain prose. Keep
`Shared concept` for shared mental models, consistent with `docs/index.md`.

## Re-review boundary

Only the two wording blocks above require change. Preserve the current
two-file scope and successful validation pattern. Re-run the strict site build,
the stale-term grep, and the exact-SHA acceptance pipeline, then return the new
SHA for a scoped Gate 2 re-review. This review does not reopen Class 4 or the
post-release mobile/screen-reader obligation.
