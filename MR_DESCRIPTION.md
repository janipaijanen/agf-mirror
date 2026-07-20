# coord: reconcile seven parallel Gate 1 plan clusters (issues #17-#49)

Branch: `coord/plan-cluster-reconciliation-2026-07-20`

## Summary

23 open issues (#17-#49) were split into seven independent clusters, each
Gate-1-planned by a separate isolated agent instance. This MR adds the
cross-cluster reconciliation note: consistency findings against each
other, against existing implementation, and against the framework's own
core principles.

## Files changed

| File | Change |
|---|---|
| `docs/coordination/20260720T080808Z-governor-reviewer-mrp3.md` | New `kind: review` coordination message — five findings across the seven plan-cluster branches, a recommended merge order, and two open questions flagged for governor decision |

## Findings summary

- **Finding 1** (resolved in a follow-up commit on each affected branch):
  naming conflict between `accountable_human`, `owner`, and
  `proposed_by`/`approved_by` across three clusters — collapsed to one
  canonical field name.
- **Finding 2** (open, needs governor decision): the `provisional →
  recommended → MUST` ladder vs. the actual two-field
  `rule.schema.json` shape; SPDX vs. CycloneDX vocabulary for
  input-provenance.
- **Finding 3**: several cross-cluster assumptions confirmed compatible,
  no action needed.
- **Finding 4**: issue #30's root cause found and fixed by the
  misc-small-fixes cluster — unblocks the `build-engineer` workaround
  every other cluster used.
- **Finding 5**: branch-basis divergence across clusters, diffed and
  confirmed low-risk (version bump only, no schema/code substance).

## Review checklist

- [ ] `agov coord validate` passes (already run: OK, one advisory
      warning for the optional `model` field)
- [ ] Recommended merge order is sound given the stated dependencies
- [ ] No `Co-authored-by` trailers in commit
