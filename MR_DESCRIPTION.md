# coord: ask coder-tester to collaborate on student-audience flyer redraft

Branch: `coord/nutshell-flyer-redraft-codex-collab`

## Status

- **Codex response filed:** `docs/coordination/20260720T152811Z-coder-tester-rqn4.md`.
- **Blocking Claude:** nothing. Claude can use the response to redraft the
  artifact.
- **Codex collaboration:** branch rebased on current `main` after docs MR !231
  merged, so the coordination MR no longer appears to revert recent docs work.

## Summary

Not part of the AGF gate pipeline — this coordinates a redraft of the
"AGF in a Nutshell" A3 flyer, a Claude Artifact (not a repo file) shared
directly with the human governor, not part of the documentation site.

The human governor reviewed the first draft and flagged that it targets the
wrong audience (a practitioner/adopter, not a 3rd-year engineering
bachelor's student) and misrepresents the command model (lists `agov`
commands as a human cheat sheet, when the framework's own
audience-boundary work — `agf/docs/site/docs/index.md`, MR !229 — says most
of that procedure is AI-executed, not a human to-do list).

This handoff asks Codex to propose content and visual direction for the
corrected draft before Claude re-renders it.

## Files changed

| File | Change |
|---|---|
| `docs/coordination/20260720T152054Z-planner-architect-q4dj.md` | New handoff to coder-tester |
| `docs/coordination/20260720T152811Z-coder-tester-rqn4.md` | Codex response with student-facing content and visual direction |

## Review checklist

- [x] `agov coord validate` passes
- [x] No `Co-authored-by` trailers in commit
- [x] Rebased after !231 to avoid stale-branch doc reversions
