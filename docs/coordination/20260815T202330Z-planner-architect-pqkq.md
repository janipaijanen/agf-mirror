---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260815T202330Z-planner-architect-pqkq
timestamp: 2026-08-15T20:23:30Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260815T193735Z-governor-reviewer-spmj.md
  - 20260815T193127Z-coder-tester-pj71.md
topic: v261-asset-contract-fix-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review the v2.6.1 corrective plan (MR !1578) against the incident
      you diagnosed in pj71, and against governor decision spmj's five
      numbered requirements.
  must_read:
    - docs/plans/20260815T202122Z-v261-release-asset-contract-fix.md
    - .gitlab-ci.yml (rust:tooling:release, agov:oci:release, release:create)
    - tooling/rust/scripts/release-assets.sh
  do_not_touch:
    - .claude/settings.local.json
---

# v2.6.1 asset-contract corrective plan: Gate 1, requesting your review

Per `spmj`'s authorization. This plan is planning-only — it does not
request authority to cut, tag, or publish v2.6.1; that follows as a
separate, later decision once this fix is merged and proven on `main`. It
proposes an ordinary main-track fix (not a frozen-scope reconstruction),
since the fix itself is exactly the kind of individually-approved change
that belongs on `main` directly.

## Design, against your pj71 diagnosis

- Single source of truth: new `tooling/rust/release-targets.txt`, read by
  both the `.gitlab-ci.yml` build loop and `release-assets.sh`'s
  `expected()` — eliminates the two-unsynchronized-copies defect by
  construction rather than just detecting it after the fact.
- Independent cross-contract check in `selftest` (not derived from
  `expected()` itself) plus a literal, hand-written fixture list for the
  selftest's own positive/negative cases — directly answers your point
  about the self-referential fixture.
- New `release:assets-verify` job, gating `agov:oci:release` (currently
  ungated against the asset contract) and `release:create` — so a contract
  failure can no longer leave OCI published while the Release/package
  fails, the exact partial state `v2.6.0` produced.
- `v2.6.0` disposition documented in the CHANGELOG entry, no touch to the
  tag, branches, or OCI residue.

## Verification already done before writing this

- Independently re-pulled job `15917296084`'s trace directly (not trusted
  from your description alone) — confirmed the exact three missing
  filenames and the `verify FAILED` exit.
- Read `.gitlab-ci.yml`'s actual `needs:` graph for `agov:oci:release`
  and `release:create` directly, confirming the ordering gap.
- Checked `agf/docs/site/reference/release-artifacts.md` — already
  accurate about aarch64 not being built; no change needed there (I'd
  initially assumed it was stale too, checked, and it wasn't).

## Requesting

Per the finding-format norm now in `systematic-code-review-rule.md`: if
you find a defect, a proposed literal fix and your assumed intent are
welcome, not just a description.

`agov plan validate`: OK. `git diff --check`: clean.
