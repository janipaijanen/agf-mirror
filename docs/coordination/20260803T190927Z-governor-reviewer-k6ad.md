---
kind: decision-record
from: governor-reviewer
to: all
id: 20260803T190927Z-governor-reviewer-k6ad
timestamp: 2026-08-03T19:09:27Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T190255Z-contribution-auditor-a8gg
  - 20260803T184309Z-coder-tester-t883
  - 20260803T183732Z-governor-reviewer-rntt
  - 20260803T173905Z-governor-reviewer-p1mm
topic: v250-rc1-release-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: v2.5.0-rc.1 release MR !1067

Codex-scribed governor-reviewer decision, transcribing the human governor's
explicit approval in the live session on 2026-08-03.

## Binding decision

Gate 2 is approved for MR !1067 only at exact head
`8feb0560178cc171b5447e4e6fdf50e921f9bcf8`.

The governor conditioned approval on suitability for everyone involved. That
condition is satisfied by the published chain:

- coder-tester completed the release-only change set and requested renewed
  review in `20260803T184309Z-coder-tester-t883`;
- contribution-auditor issued an exact-head round-2 CLEAR with zero blockers
  or advisories in `20260803T190255Z-contribution-auditor-a8gg`;
- exact-head pipeline `2728134784` passed every automatic job; and
- the governor's baseline amendment
  `20260803T183732Z-governor-reviewer-rntt` is incorporated and verified.

## Authorized sequence

Merge exact head `8feb0560178cc171b5447e4e6fdf50e921f9bcf8` to `main` with
`should_remove_source_branch: false`, verify the remote release branch remains
and `origin/main` is the resulting merge commit, then immediately create and
push the single annotated immutable tag `v2.5.0-rc.1` on that exact merge
commit. Continue through the approved tag pipeline and post-publication
verification in `docs/plans/v250-rc1-release-publication.md`.

If the MR head changes before merge, this approval is void and renewed exact-
head review is required. Do not move, delete, or recreate the tag after push.
