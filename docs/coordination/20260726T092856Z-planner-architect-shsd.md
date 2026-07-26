---
kind: note
from: planner-architect
to: all
id: 20260726T092856Z-planner-architect-shsd
timestamp: 2026-07-26T09:28:56Z
immutable: true
references-to:
  - 20260725T060849Z-fact-checker-3fk5
topic: readme-reposition-start-development-section
requires-ack: false
artifacts_owned:
  - README.md
---

# README: relocate "Start development in this checkout" to the last section

### Claude · planner-architect

The human governor flagged that `### Start development in this checkout`
was misplaced in `README.md` and asked for it to be moved to the last
element, or removed, with reasoning.

## Argument

The section was a `###` subsection nested under `## Repository map`,
sitting between the repository map table and `## Maturity and evidence
labels`. Both of those sections belong to this README's dominant audience
— an evaluator deciding whether to adopt AGF, per
`agf/rules/docs/documentation-audience-rule.md`'s README-targets-developers
scope, specifically developers evaluating the product. The "Start
development in this checkout" content is a different, narrower audience:
someone contributing to AGF's own codebase in this repository. Its old
position interrupted the adoption-evaluation reading path (`Should you
adopt?` -> `Repository map` -> [contributor detour] -> `Maturity and
evidence labels` -> `Adoption steps`).

The README's own closing `## Next` section already anticipated a
contributor exit distinct from the adoption-evaluation exit ("Contributing
here -> read AGENTS.md, then run the development start commands above.
Evaluating adoption -> ..."), but the phrase "commands above" pointed
several sections upward to a mid-document detour rather than to adjacent
content. Removing entirely was not the better option: the content is
genuine, fact-checked, working guidance (verified verbatim by
`20260725T060849Z-fact-checker-3fk5`) and the existing `## Next` text
already promises its presence — deleting it would break that promise and
lose real value for the minority of README readers who are contributors.

## Change

Moved the section verbatim (heading text unchanged, so any existing anchor
reference to `#start-development-in-this-checkout` still resolves) from
between `## Repository map` and `## Maturity and evidence labels` to
immediately before `## Next`, promoting it from `###` to `##` since it no
longer nests under Repository map. Added one clarifying sentence at the
top of the relocated section stating explicitly that it is for AGF's own
contributors, not adopters, and updated `## Next`'s "commands above" to
"commands immediately above" now that the phrase is accurate.

## Verification

- `python3 -m unittest discover -s agf/docs/build/hooks -p 'test_*.py'` —
  6 passed (includes the public-feature-summary byte-identical placement
  test, unaffected — that block was not moved).
- `git diff --check` — passed, no whitespace errors.
- No other file in the repository links to
  `#start-development-in-this-checkout` (checked via grep); the anchor
  text is unchanged so no link would have broken regardless.

This is a same-file, same-audience-boundary reordering with no content,
claim, or command change — fast-tracked per
`.claude/rules/workflow/simplicity-rule.md`'s doc-update tier rather than
a full Gate 1 plan; low risk, no schema/authority/security surface.
