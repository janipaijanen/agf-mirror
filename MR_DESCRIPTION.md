# coordination: converge with Codex on untracking MR_DESCRIPTION.md

Branch: `coord/mr-description-converged-recommendation`

## Status

- **Pending:** human governor decision on !239/!241/this MR.
- **Blocking Claude:** nothing.

## Summary

Codex's response to the two-proposal handoff (`20260720T162959Z-coder-tester-hqhp.md`)
found a real gap in Claude's Option A: `MR_DESCRIPTION.md` is git-tracked, so
"keep it as scratch" still leaves every branch's edit as a tracked diff that
must be manually reverted before merge — which is exactly what caused the
repeated `--ours`/`--theirs` merge conflicts on this file during every
branch rebase earlier in this session.

This converges on a stronger fix than either original proposal alone: move
`MR_DESCRIPTION.md` to a gitignored path (`tmp/MR_DESCRIPTION.md`; `tmp/` is
already gitignored in this repo), removing the drift/conflict problem
structurally instead of disciplining around it. Both Claude and Codex agree
on deferring the pre-push-hook idea (Option B) in favor of, if ever needed,
an explicit `agov mr description sync` command instead of an implicit
auto-syncing hook.

## Files changed

| File | Change |
|---|---|
| `docs/coordination/20260720T163751Z-planner-architect-hkrr.md` | Converged recommendation, routed to governor-reviewer for decision |

## Review checklist

- [x] `agov coord validate` passes
- [x] No `Co-authored-by` trailers in commit
