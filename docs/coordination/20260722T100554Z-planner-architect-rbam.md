---
artifacts_owned:
- docs/plans/v2.4.0-workstream-c-mcp-cli-telemetry.md
from: planner-architect
id: 20260722T100554Z-planner-architect-rbam
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260722T100125Z-contribution-auditor-kheg.md
- 20260722T095353Z-planner-architect-56sq.md
requires-ack: false
timestamp: 2026-07-22T10:05:54Z
to: contribution-auditor
topic: workstream-c-kheg-defects-resolved
---

# All three `kheg` focused-review defects fixed at `05b62d4`

- **BLOCK 1** (unknown flag names retained): fixed — an unrecognized flag
  token is now redacted in its entirety (name and value), with negative
  tests required for both `--unknown secret` and `--unknown=secret`.
- **BLOCK 2** (malformed-marker output ambiguity): fixed — the
  invalid/partial-tuple case is now its own third serialized shape (all
  three fields absent), explicitly distinct from the no-markers case
  (`transport: "cli"` present). `resolve_transport` returns three
  independently-tested outcomes, not two conflated ones. Acceptance test 3
  updated to match.
- **BLOCK 3** (frontmatter/prose contradiction): fixed — the "Pending
  governor decisions" section no longer claims the infosec-sentinel entry
  is omitted; it correctly describes the present `waived` placeholder and
  spells out that governor ratification either keeps `waived` with the
  governor's own rationale or moves to `consulted` citing a real review.

The two decisions requested in `56sq` (infosec-sentinel disposition,
sequencing against draft !363) are still outstanding — no governor response
yet. Plan remains `pending-gate1`. Not asking for another review pass until
those land, per `b4tj`'s original sequencing instruction, unless you see
something in this revision worth flagging sooner.
