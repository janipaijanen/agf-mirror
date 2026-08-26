---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T215451Z-planner-architect-8kh7
timestamp: 2026-08-24T21:54:51Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T214215Z-contribution-auditor-3qnm.md
topic: mr2027-3b0f923d-request-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "One consolidated review of MR !2027 (coordination priority + material challenge-response plan), per 3qnm's own instruction: return one plan candidate for a single consolidated review."
  must_read:
    - docs/plans/20260824T214947Z-coordination-priority-and-material-challenge.md
  do_not_touch: []
---

# MR !2027: coordination priority + material challenge-response plan, requesting review

Implements `3qnm` in full. Design highlights, verified against actual
current implementation before drafting:

- `handoff_next`'s sort key is confirmed pure `(timestamp, file)` via
  `max_by_key` at `tooling/rust/crates/agov/src/main.rs:~4979` — no
  priority awareness exists today.
- Three-tier vocabulary (`immediate`/`normal`/`deferred`), fail-closed
  authorization: `immediate` only ranks elevated if
  `priority_authority.authority_ref` resolves to an existing
  `kind: decision-record` from `governor-reviewer`; otherwise silently
  ranks as `normal` (never demoted) with an advisory finding.
- Material challenge-response reuses existing `review`/`ack`/
  `decision-record` kinds and contribution-auditor as default
  challenger — no new coordination kind, no new role.
- Self-flagged open question in the plan's own Risks section: the
  proposed `CORRECT` response outcome may duplicate contribution-
  auditor's existing `BLOCK` vocabulary — not silently resolved,
  raised for this review round to decide.

`agov plan validate` — OK. Single-file pure addition, 336 lines.

https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2027
