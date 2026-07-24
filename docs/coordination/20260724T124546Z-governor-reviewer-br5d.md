---
kind: decision-record
from: governor-reviewer
to: contribution-auditor
id: 20260724T124546Z-governor-reviewer-br5d
timestamp: 2026-07-24T12:45:46Z
model: human-governor
immutable: true
references-to:
  - 20260724T122510Z-planner-architect-h3hs
  - 20260723T202142Z-planner-architect-dr5h
  - 20260723T202047Z-planner-architect-6get
topic: release-bake-6get-schema-correction-authorized
requires-ack: false
artifacts_owned:
  - docs/coordination/20260723T202047Z-planner-architect-6get.md
---

# Governor authorizes narrow schema correction for `6get`

This record transcribes the governor-reviewer's session instruction, "ok!
proceeds", after disclosure that the v2.4.1 full-channel release bake was
blocked by `6get`'s schema-invalid `handoff.status: in-progress` value.

The governor authorizes the narrow correction of that value to `blocked`, the
value already published in immutable correction note `dr5h`. No other content
in `6get` may change. Git history retains the originally published bytes and
the correction remains explicitly visible in `dr5h`, `h3hs`, and this decision
record.

This is a bounded recovery decision for schema validity, not a general license
to rewrite immutable coordination history. The correction must travel through
its own MR, pass full-channel coordination validation, and merge before the
v2.4.1 release MR proceeds. The release MR itself remains unchanged at its
reviewed exact SHA.
