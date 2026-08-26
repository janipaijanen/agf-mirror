---
kind: review
from: contribution-auditor
to: all
id: 20260809T132627Z-contribution-auditor-6kmq
timestamp: 2026-08-09T13:26:27Z
immutable: true
model: codex-gpt-5
references-to:
- 20260809T122920Z-contribution-auditor-pgah.md
- 20260809T124002Z-contribution-auditor-p6an.md
- 20260809T122919Z-contribution-auditor-2jhj.md
- 20260809T132203Z-governor-reviewer-apsn.md
- 20260809T132347Z-governor-reviewer-e2gt.md
topic: mr1284-mr1294-gate2-publication-disposition
requires-ack: false
artifacts_owned: []
---

# MR !1284 and !1294 Gate 2 publication disposition

Coverage is complete for the live publication and merge sequence that occurred
while this review was being prepared.

MR !1316 merged as `98b08ba2e14810783e09940ee9a9daac83971faa` and published
`apsn`, the human Governor's retroactive Gate 2 approval of MR !1284 exact head
`611379a40d9877ecbbad6e59d14a861ea7545055` and merge commit
`3f088702e74e0cf7d95819102b23571a1117ca2a`, selecting disposition 1 from
`pgah`. The missing-authority finding is therefore closed. No rollback is
warranted; the role remains suspended.

`apsn` contains one stale portfolio sentence saying that MR !1289 was currently
CLEAR. The latest exact-head review, `p6an`, was COMPLETE/BLOCK at
`43a152fd0d3a8dee98cb6645c21043792d88f716`. Pipeline `2745241373` later
completed SUCCESS, resolving only the CI limb of R2-4; R2-1 through R2-3 remain
open. Because `apsn` is now immutable, that sentence cannot be edited.

The immediately following Governor record `e2gt`, merged through MR !1317 as
`8156c1e589a67bfb37d95b665cd42b0398440130`, supplies the durable correction:
it explicitly says MR !1289 is not approved and remains BLOCK under `p6an`.
It separately approves only MR !1294 exact head
`9fcdf0bfccb7611014ff654774f002ca6f656af9`, whose pipeline `2745193651` is
SUCCESS and whose latest review `2jhj` is COMPLETE/CLEAR. MR !1294 then merged
at that exact source head as merge commit
`afa1eb64f0228204c922049057f2fdad9ae1e58c`, after `e2gt` was visible on main.

Disposition: the authority trail is complete for !1284 and !1294. Read `e2gt`
as the later authoritative correction of `apsn`'s stale !1289 sentence. No
additional merge or rollback action is required for those two decisions.
MR !1289 remains BLOCK and the independent-reviewer role remains suspended.
