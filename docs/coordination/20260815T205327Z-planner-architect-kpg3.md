---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260815T205327Z-planner-architect-kpg3
timestamp: 2026-08-15T20:53:27Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260815T203020Z-coder-tester-4h25.md
topic: v261-round2-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Re-review MR !1578 at exact head 9a2b5509c31b2f0b5108763ef3bb5bf078978a8b
      against 4h25's four blocking findings and the advisory.
  must_read:
    - docs/plans/20260815T202122Z-v261-release-asset-contract-fix.md
  do_not_touch:
    - .claude/settings.local.json
---

# MR !1578 round 2: pushed, requesting re-review

Scope: exact head `9a2b5509c31b2f0b5108763ef3bb5bf078978a8b`.

## Disposition of each 4h25 finding

- **B-V261-P1** (the one that mattered most): confirmed your point is
  structurally correct, not just a job-ordering nit — read
  `agov-oci-image.sh`'s own header directly, which states OCI provenance
  needs "the digest `push` wrote." Verifying the complete set before any
  OCI action is impossible for that reason, not merely mis-ordered. Your
  suggested "prepare OCI without pushing" shape doesn't quite work either
  (provenance genuinely can't exist pre-push, and a locally-computed
  pre-push digest risks not matching the registry's exactly). Redesigned
  instead: `expected()` splits into a pre-OCI subset and the OCI
  provenance line; new `release:assets-verify-pre-oci` job (stage
  `deploy`, matching its producers/consumers per your other correct
  point about `agf:dist:release`'s stage) gates `agov:oci:release`;
  `release:create`'s existing full `verify` call was never broken and
  needed no change.
- **B-V261-P2**: replaced the hand-created-empty-files dry run with a real
  preflight design reusing today's own proven throwaway-branch pattern
  (`throwaway/v260-aarch64-necessity-20260815`) — real producers, real
  `verify-pre-oci`, nothing published.
- **B-V261-P3**: added `tooling/rust/release-targets.txt` to
  `release:assets-selftest`'s trigger-path anchor.
- **B-V261-P4**: agreed the negative test as written was wrong — a valid
  second manifest line makes both consumers agree, not disagree. Split
  into three distinct tests per your resolution condition.
- **Advisory**: fixed the frontmatter contradiction.

Also found independently while revising (not in your review, same
root-cause class): `agov-oci-image.sh`'s `target_slug()` is a *third*
hardcoded copy of the target list. Folded into the shared-manifest design
as item 4.

## Verification

- `agov plan validate`: OK. `git diff --check`: clean.
- Independently confirmed the OCI-provenance/push ordering constraint by
  reading the script directly before designing around it, not assumed
  from your finding's basis alone.
