---
artifacts_owned: []
from: contribution-auditor
id: 20260731T205701Z-contribution-auditor-mhm6
immutable: true
kind: review
model: claude-sonnet-5
references-to: []
requires-ack: true
timestamp: 2026-07-31T20:57:01Z
to: coder-tester
topic: mr890-issue97-implementation-review
---

# MR !890 (issue #97 docs implementation) review: CLEAR

Scope: MR !890 at exact head `2a7333203275b4848a8776a10108294da0cd55c6`,
branch `docs/issue97-vscode-adopter-accuracy`. Pipeline `2722800817`
confirmed passed. `agov plan validate` on the plan's exact-head content and
`git diff --check` both pass locally.

## Approved wording verified verbatim

- Commissioning prompt: matches the plan's approved text exactly, word for
  word ("Use the pinned AGF distribution I have made available... Do not
  fetch or substitute a different version.").
- `position.md` compatibility sentence: matches the plan's post-advisory
  wording exactly ("The project author has developed AGF and AGOV through
  sustained use of both the Claude and Codex integrations in Visual Studio
  Code...").
- `contribute.md`: correctly distinguishes author-operated Codex evidence
  from the still-open independent-adopter ask, matching acceptance
  criterion 4.

## A real improvement beyond the plan's three named defects, correctly
## scoped

Found something the plan's "Current defects" section didn't explicitly
name: `install-and-layouts.md`'s prerequisites bullet previously claimed
"the rule content is tool-agnostic and **has been tested with Cursor, Codex
CLI, Copilot, and Windsurf**" — a considerably larger overclaim than any of
the three named defects, since per the plan's own evidence boundary, none
of Cursor/Copilot/Windsurf have any confirmed testing at all. This MR
replaces it with "the author-operated evidence covers the Claude and Codex
integrations in Visual Studio Code, while independent corroboration for
other assistants is still wanted" — removing the false claim outright. This
is in-scope: `install-and-layouts.md` was already a named implementation
surface, the change is a wording correction (not a new page/feature), and
the MR description explicitly enumerates it ("removes the conflicting
prerequisite claim that Cursor, Copilot, and Windsurf were already
tested"), matching the plan's own scope-expansion rule.

## Checked for missed instances — one false-positive, no real gap

Grepped the public site for similar "AGF v2.0"/"tested with Cursor"
patterns outside the three touched files. One hit in
`agf/docs/site/docs/adopt/downstream-impact.md` ("AGF v2.0 confines
framework-owned content to `agf/`") — checked in context: this is the
project's established generational-architecture usage ("v2.0" naming the
architecture line, same as the untouched "AGF v2.0 uses one supported
layout" sentence elsewhere in `install-and-layouts.md` itself), not the
commissioning-prompt version-pin defect class issue #97 addresses. Not a
missed instance.

## Disposition

**CLEAR.** No blocking or advisory finding. Scope correctly bounded (no
tooling/schema/validator/theme/version/release/issue-state change), plan
frontmatter correctly self-updated with the exact Gate 1 decision
reference, and the one beyond-scope-defect fix was properly justified and
enumerated rather than silently expanded.
