---
id: 20260913T165127Z-row5-s4-cp-a-ack-pending-display
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-13T16:53:02Z
topic: row5-s4-cp-a-ack-pending-display
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Advisory display change only, computed from already-published,
      already-validated coordination records. No new field, no new
      write path, no authority granted or checked. Nothing to leak and
      no privilege the display could incorrectly grant, since ack
      satisfaction itself is explicitly not touched (see Out of Scope).
  - role: contribution-auditor
    status: waived
    reason: >-
      Bounded, additive, single-file-family display change matching the
      Row 5 work order's own approved S4 scope; a real pass is welcome
      before Gate 2 but is not required to unblock this narrow slice.
affected_paths:
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/
  - docs/reference/session-resume.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: Row 5 S4 — CP-A `answered — ack pending` display

**Gate 1: APPROVED** — blanket pre-approval
`20260913T165302Z-governor-reviewer-8h7s` ("Jani: 'Pre-approved, implement
right away.'"), in response to the four-item batch survey under `jktm`.

Implements S4 of the approved Gate 1 work order
`docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md`,
first slice only (display-only; auto-ack is explicitly out of scope and
deferred to a separate, later decision per the work order).

## Problem

`agov handoff next <role>` shows every pending `requires-ack: true`
handoff as `[ready]`, identically, whether or not anyone has already
substantively responded to it. A responder who posts a `review` or
`note` that `references-to` the handoff gets no visible credit for that
in the queue display — the item still reads exactly like an untouched
handoff, so a second session (or the same session on a later cycle) may
re-review or re-answer it. `agov handoff next` does not currently reveal
whether the answer already exists.

This is distinct from full resolution: `handoff_tracker::is_resolved`
already fully clears a handoff (removes it from the pending set
entirely) when an `ack`, `completion`, or `decision-record` exists
anywhere in its transitive descendant chain. Those three kinds already
carry authority to close the loop. S4 does not touch that logic. It adds
a *third* visible state, strictly between "untouched" and "resolved":
a handoff that is still pending (no ack/completion/decision-record
anywhere in its chain) but that has at least one later `review` or
`note` directly referencing it.

## Design

### Design correction (made before implementation, during this same Gate 1
cycle): reuse existing `reply_evidence`, do not duplicate it

The original draft above proposed a new `handoff_tracker::has_substantive_
answer` function. Reading `agov/src/main.rs` before writing code found
this already substantially exists: `reply_evidence(handoff, messages)`
(main.rs) already computes exactly this signal -- a later `note`/`review`
message, authored by the handoff's own declared `handoff.next_agent` **or**
`governor-reviewer`, timestamped after the handoff, whose `references-to`
(via `frontmatter_refs`) contains the handoff's file -- and
`render_reply_evidence` already prints it as a separate line under the
pending entry: `reply present — acknowledgement unconfirmed;
reply_evidence: [...]`.

This is a better-specified "authorized role" definition than the original
draft's "no allow-list, any schema-valid `from`" -- it is exactly
`next_agent` or `governor-reviewer`, which matches the work order's
"from an authorized role" language far more precisely than "any role"
would. S4 therefore does **not** add a new detection function. It:

1. Reuses `reply_evidence` unchanged as the source of truth for "has a
   substantive answer."
2. Changes `pending_handoff_label` (and its two callers) to fold that
   signal into the bracket state itself, instead of leaving it as a
   separate advisory line the reader could miss.
3. Keeps `render_reply_evidence`'s separate line as-is (it still adds
   value: the reply's `id`/`from`, which the bracket alone does not
   carry) -- this slice is additive to the existing signal, not a
   replacement for it.

Matching is by the handoff's `file`, exactly as `reply_evidence` already
does (`frontmatter_refs(&m.raw).contains(&handoff.file)`) -- no new
matching convention introduced.

### Design: display

`agov/src/main.rs`'s `pending_handoff_label` gains an extra parameter (the
full `messages: &[HandoffMessage]` slice, already in scope at both call
sites) and a third bracket state, computed via
`!reply_evidence(message, messages).is_empty()`:

- No substantive answer (`reply_evidence` empty): unchanged, `[ready]` or
  `[blocked]` (the message's own declared `handoff.status`, exactly as
  today).
- Substantive answer found **and** the message's own declared status is
  `ready` (the common case -- the handoff itself was never marked
  `blocked`): `[answered — ack pending]`.
- Substantive answer found **and** the message's own declared status is
  `blocked`: `[blocked, answered — ack pending]` -- both facts are true
  and neither is dropped; a blocked handoff that has nonetheless received
  a substantive answer is not the same situation as either state alone.

No schema change. No new coordination-message field. The `handoff.status`
enum (`ready`/`blocked`) is unchanged; this is purely a derived display
value computed at read time from existing `references-to` data, never
written back anywhere.

### Non-goals (explicitly out of scope, per the work order)

- Auto-satisfying the ack from the substantive-answer record. That
  changes ack *semantics* (who may clear a `requires-ack` obligation) and
  needs its own evidence and Gate 1, per the work order's own deferral.
- Any change to `is_resolved`, `find_pending`, or which handoffs appear
  in the pending set at all. This slice only changes how an
  already-pending handoff is *labeled*.
- `agov coord pending` (the `find_pending_all_kinds` surface for non-
  handoff/ack `requires-ack` records) -- out of the work order's named
  S4 scope, which is `agov handoff next` only.
- CP-C (`agov handoff next` reading only the base branch) -- stays in
  #123 / row 6, per the work order.

## Test strategy

**Design correction (implementation-time):** `reply_evidence`'s own
role/reference/kind rules are already exhaustively covered by the
existing unit test
`reply_evidence_is_direct_later_recipient_or_governor_only` in
`tooling/rust/crates/agov/src/main.rs`'s `#[cfg(test)] mod tests`. New
tests therefore live in that same module (unit tests, not a new
integration-test file under `tests/` -- `pending_handoff_label` and
`HandoffMessage` are private to `main.rs` and not exposed for external
integration tests) and cover only the new label text derived from that
already-tested signal:

1. `pending_handoff_label_shows_ready_with_no_reply` -- **negative**
   baseline: a handoff with no reply at all still shows `[ready]`.
2. `pending_handoff_label_shows_answered_ack_pending_with_qualifying_reply`
   -- **positive**: a qualifying `review` from `next_agent` referencing
   the handoff -> `[answered — ack pending]`.
3. `pending_handoff_label_non_referencing_reply_stays_ready` --
   **negative**: a `review` from `next_agent` that does *not* reference
   the handoff -> still `[ready]`. Proves the check is reference-bound.
4. `pending_handoff_label_blocked_plus_answered_shows_both` -- a handoff
   with `handoff.status: blocked` plus a qualifying reply ->
   `[blocked, answered — ack pending]`.

The governor-reviewer-as-authorized-role case, the unauthorized-role
negative, and the already-resolving-kinds regression are all covered by
the pre-existing `reply_evidence_is_direct_later_recipient_or_governor_only`
test, which this slice reuses rather than duplicates -- re-run as part of
the full suite below to confirm no regression.

Full workspace test suite, clippy, corpus regression (no plan file
touched by this slice, so no corpus change expected) before Gate 2.

Per `gate-self-test-rule.md`: `agov handoff next` remains advisory queue
display, not an MR-blocking check (explicitly noted in the work order),
so this is not gate mechanics and the rule's negative-test requirement
does not apply as a *gate*. The positive/negative test pair above is
still included because it is good practice and because the work order's
own S4 entry explicitly asks for it ("The negative check for this slice
is a positive/negative pair proving a genuinely unanswered handoff still
shows `ready`...").

## Rollback

Purely additive display logic with no schema or write-path change.
Reverting the branch restores today's undifferentiated `[ready]` label
with zero migration.

## Risks

- **False "answered" on an unrelated reference.** Mitigated by requiring
  a direct reference to the handoff's own id, not a topic/keyword match;
  test 3 above proves a non-referencing later message does not trigger
  the new label.
- **Scope creep into auto-ack.** Explicitly a non-goal above; this slice
  ships no code path that clears `requires-ack` automatically.
