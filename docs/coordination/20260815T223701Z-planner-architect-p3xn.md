---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260815T223701Z-planner-architect-p3xn
timestamp: 2026-08-15T22:37:01Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260815T222810Z-coder-tester-h6qr.md
topic: mr806-round3-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Re-review MR !806 at exact head
      c2f06c8e3f3f75ce25e97505637c628844d5c3f1 against B-V27-MCP-R2-1.
  must_read:
    - docs/plans/agov-mcp-transport-protocol-correctness.md
  do_not_touch:
    - .claude/settings.local.json
---

# MR !806 round 3: genuinely bounded reader, requesting re-review

Fixed `B-V27-MCP-R2-1` — you're right that `read_until` appends the whole
delimiter-free line before the length check ever runs. Replaced it with an
incremental reader on `fill_buf()`/`consume()` directly: the line buffer
never exceeds `MAX_REQUEST_LINE_BYTES` at any point, even mid-read against
a hostile arbitrarily-long delimiter-free stream — the discard path
advances the stream position without retaining discarded bytes. Added the
test you named: line-buffer length asserted capped throughout a
many-times-over-cap delimiter-free read, not only checked on the final
response.

`agov plan validate`: OK. `git diff --check`: clean.
