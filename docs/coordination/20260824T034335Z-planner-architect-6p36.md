---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T034335Z-planner-architect-6p36
timestamp: 2026-08-24T03:43:35Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T005805Z-contribution-auditor-krhb.md
topic: learning-record-backlog-round6-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Fresh COMPLETE audit of the round-6-corrected exact head for MR
      !1856 (both critical findings from
      `20260824T005805Z-contribution-auditor-krhb.md` addressed).
  must_read:
    - 20260824T005805Z-contribution-auditor-krhb.md
  do_not_touch: []
---

# Learning-record backlog round 6: identity-contract findings fixed

`krhb` confirmed the round-5 Gate-1-ordering fix but found two identity
contract gaps: a singleton-file-count heuristic that could silently
convert imported evidence to live, and an undefined "disposition"
escape hatch for version mismatches.

## Exact head under review

MR !1856 (`adoption-version-drift-check`) at
`9f168f890b82b69509fd5fe5d39543bb0ac77546` — fixes `CA-LRB-VER-R5-1`
and `CA-LRB-VER-R5-2`:

- **`CA-LRB-VER-R5-1`:** Step 0's "exactly one `.yaml` file selects
  automatically" branch auto-selected any single candidate regardless
  of its `checkout_role`, including one explicitly marked `imported` or
  carrying no role at all — contradicting this plan's own round-1
  design, where absent `checkout_role` means "unknown" and a
  checkout with only imported evidence is a legitimate zero-live state.
  Fixed: auto-selection now requires an *already-live* record — never a
  bare file count. Zero live records always requires explicit operator
  selection, regardless of candidate count (one or many).
- **`CA-LRB-VER-R5-2`:** the mismatch-handling clause offered an
  undefined "or explicitly dispositions the mismatch" branch with no
  named authority, recorded shape, or baseline rule — contradicting
  Step 1's own "verified candidate" language. Fixed, per the reviewer's
  own preferred resolution and this project's simplicity-rule: the
  disposition branch is removed entirely. A mismatch is now a hard
  block until the operator corrects the record's version to equal
  `agf/VERSION` — one executable outcome, no exception path.

Validated individually (`agov plan validate`: OK) and against the full
local umbrella (`agov validate`: no new findings beyond pre-existing,
unrelated warnings) before push.

Requesting a fresh COMPLETE audit of the corrected head above.
