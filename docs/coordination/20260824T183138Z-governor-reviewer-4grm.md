---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260824T183138Z-governor-reviewer-4grm
timestamp: 2026-08-24T18:31:38Z
model: human-via-codex
immutable: true
references-to:
  - 20260824T163113Z-governor-reviewer-36rr.md
  - 20260824T175134Z-governor-reviewer-q28m.md
  - 20260824T181925Z-planner-architect-m78s.md
  - 20260824T182750Z-contribution-auditor-5tms.md
topic: mr1847-dangling-symlink-final-correction-authorized
requires-ack: false
artifacts_owned: []
---

# MR !1847 final dangling-symlink correction authorized

The human governor directs the team to deliver rather than leave the plan in
another review loop. The still-open dangling-symlink half of
`CA-NNP-R14-1` is authorized for one final surgical correction at MR !1847.

The correction must follow `5tms`'s resolution condition: after
`symlink_metadata` identifies a symlink whose target cannot be canonicalized,
use `read_link`, resolve relative targets against the link entry's parent,
find and canonicalize the target's deepest existing ancestor, and classify an
unresolvable or outside ancestor as `Invalid`, never `ContainedMissing`. The
named negative test must exercise that branch through `--allow-planned`,
`projection-check-all`, and the strict route.

This is the same ratified finding, not scope expansion. No new full review is
authorized. After the correction, the original round-14 reviewer may perform
one targeted proof limited to this dangling-symlink branch. A CLEAR result at
the exact green head activates Gate 1 approval and authorizes merge of the
plan-only MR !1847 on the governor-reviewer's behalf. A further material
failure returns once with concrete evidence; advisory nuance does not block
delivery.
