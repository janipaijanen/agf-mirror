# plan: Gate 1 plans for misc small-fixes cluster (issues #17, #19, #30, #31, #38)

Branch: `plan/misc-small-fixes-batch`

## Summary

Five independent, small-to-medium issues planned together as one batch
rather than five separate heavyweight plans, per the simplicity rule's
task-size calibration. Two turned out to already be resolved on `main`.

## What's in this branch

| Issue | Plan file | Outcome |
|---|---|---|
| #30 (bug) | `plan-validator-advisory-role-allowlist.md` | **Root cause found and reproduced**: `allowed_roles()` in `tooling/rust/crates/agov-coord/src/plan_validator.rs` only admits `status: active` roles from `agf/agents/MANIFEST.yaml`; `build-engineer` is real but marked `status: suspended`. Fix widens the filter to `active`/`suspended`. This unblocks the `build-engineer` workaround every other cluster planned this session had to use. |
| #31 (templates) | `gitlab-issue-templates-improvement-bug.md` | Short fast-track note; confirmed no collision with the four existing `.gitlab/issue_templates/` files |
| #19 (learning) | `learning-validator-external-review-status.md` | **Already resolved** on `main` (commits `98e9fcf`, `22e9e24`, 2026-06-25) — recommends closing the issue with a reference, no new work needed |
| #17 (docs review) | `external-reviewer-fit-assessment-proposals.md` | 7 of 8 proposals already implemented on `main` (verified by reading each target section); only proposal 8 (config-drift review step) remains, planned as a small addition |
| #38 (experiment) | `multi-agent-coordination-experiment.md` | Full Gate 1 weight: pre-registered protocol, task decomposition, Condition A/B definitions, metrics table |

## Why this grouping

None of these five are coupled to each other — they were batched because
they're each too small to warrant a dedicated cluster, not because they
share a design surface.

## Cross-cluster impact

#30's fix removes the need for every other cluster planned this session
to avoid `build-engineer` in `advisory_consultation` — see
`docs/coordination/20260720T080808Z-governor-reviewer-mrp3.md` Finding 4.

## Review checklist

- [x] All plan files pass `agov plan validate`
- [ ] #30's root-cause diagnosis confirmed by an independent read of
      `plan_validator.rs`
- [ ] #19 and #17(7/8) closure-by-reference recommendation reviewed
      against the cited commits
- [ ] No `Co-authored-by` trailers in commit
