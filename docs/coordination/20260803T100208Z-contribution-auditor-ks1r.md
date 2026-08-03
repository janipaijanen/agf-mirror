---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260803T100208Z-contribution-auditor-ks1r
timestamp: 2026-08-03T10:02:08Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T084017Z-contribution-auditor-mmde.md
  - 20260803T090605Z-planner-architect-7hbs.md
  - 20260803T094702Z-governor-reviewer-3q28.md
  - 20260803T095929Z-coder-tester-hdah.md
topic: mr995-approved-plan-metadata-gate2-review
requires-ack: true
artifacts_owned: []
---

# Gate 2 assessment: MR !995 approved-plan metadata

Scope: MR !995 exact head
`6913cebe13aa231d8b637ef3988f0102a36fb733`, limited to the delta from
the Gate 1-approved head `55bdd4d5caaca5623e8c3ca57d59a30913c64f62`.

Coverage: COMPLETE.

Recommendation: **BLOCK the conditional Gate 2 approval** pending one factual
metadata correction. The Path B contract itself is preserved.

Blocking findings: 1. Advisory findings: 1. Open accepted risks: 0.
Authority: recommendation only; the human governor's stated approval applies
if this assessment supports it, so it is not activated while this finding is
open.

## Verified

- The delta changes only
  `docs/plans/learning-supersession-status-fields.md`; `git diff --check`
  passes.
- `status: approved`, approval timestamp, Path B selection, checklist, stop
  conditions, and implementation scope match binding decision `3q28`.
- Path A remains unauthorized; `superseded_by` remains schema-legal and
  deprecated; optional `status`, single-ID `supersedes`, derived reverse
  relationships, validator checks, and negative tests remain intact.
- Exact-head pipeline 2726729917 succeeded and GitLab reports the MR
  mergeable with discussions resolved.

## B-LS-META-1 — approval provenance names the wrong scribe

The front matter says:

`gate1_approved_by: "governor-reviewer; Claude-scribed decision in ...3q28.md"`

`3q28` has `model: codex-gpt-5` and states that Codex operationalized the
human governor's approval. Claude's own `hdah` note also correctly says
"`3q28` (Codex...)." The plan metadata therefore contradicts both referenced
records on who scribed the decision.

Resolution condition: replace the value with unambiguous authority-first
metadata, for example:

`gate1_approved_by: "governor-reviewer (decision 20260803T094702Z-governor-reviewer-3q28)"`

Do not encode the AI scribe as the approver.

## Advisory A-LS-PATH-1 — make the adopted rule path explicit

The final Related Documents bullet names only
`workflow/deprecation-policy-rule.md`. The decision chain exists because
`jpj6` confused the generic `agf/rules/` template with this project's adopted
`.claude/rules/` rule. Spell out
`.claude/rules/workflow/deprecation-policy-rule.md` here so the merged plan
does not preserve the same ambiguity. This is advisory by itself but should be
fixed in the same bounded correction.

After those two textual edits, re-run plan validation and obtain an exact-head
pipeline. A bounded renewed review is sufficient; no implementation or broad
plan re-review is requested.
