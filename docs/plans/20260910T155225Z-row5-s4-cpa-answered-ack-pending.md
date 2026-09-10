---
id: 20260910T155225Z-row5-s4-cpa-answered-ack-pending
status: pending-gate1
topic: row5-s4-cpa-answered-ack-pending
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This slice is `agov`-display-only: `agov handoff next` shows a
      `requires-ack` handoff that has a later authorized-role record
      `references-to`-ing it as `answered — ack pending`, distinct from
      `ready`. No schema change, no automatic ack, no authority change.
      InfoSec confirms the "authorized role" check for the answering
      record cannot be satisfied by an unauthorized role, and that the
      display state never causes an obligation to disappear.
  - role: contribution-auditor
    status: consulted
    reason: >-
      CP-A is the weakest-evidenced member of row 5 (planner's `5jde`
      challenge-response); this slice is deliberately the minimal
      display-only change with auto-ack **deferred** to its own later
      decision. Contribution-auditor confirms the slice does not
      auto-clear the handoff, does not treat the answering record as a
      binding ack, and that `answered — ack pending` is visually and
      exit-code-distinct from both `ready` and cleared.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 5 S4 — CP-A: a substantive answer surfaces distinctly from `ready` (Gate 1)

## Provenance

- Parent work order `docs/plans/20260910T065635Z-…` slice **S4**; governor
  "Keep all in v2.11" (2026-09-10) keeps row 5 a hard cut precondition.
- `jprj` CP-A: a `handoff` with `requires-ack: true` stays `ready` in
  `agov handoff next <role>` until a `kind: ack` references it. A
  `review`/`completion`/`note` that *substantively answers* it does not
  clear it → duplicate work (a second session redoes it) **or** ack-only
  MRs filed just to clear the queue (`rkbk`, `mhq3`).
- `5jde` challenge-response (in the work order): CP-A is real but partly
  self-inflicted; **ship display-only first, defer auto-ack.**

## Change (display-only, no schema change)

`agov handoff next <role>` (and the same surface in `agov coord pending` /
`agov session resume` where they list ack-pending handoffs) classifies a
`requires-ack: true` handoff into three states instead of two:

| State | Condition |
|---|---|
| `ready` | no later record references this handoff |
| **`answered — ack pending`** (new) | a later record `references-to`s this handoff **and** is `from` a role **authorized to answer it** (the handoff's `next_agent`, or `governor-reviewer`), **and** its kind is `review` / `completion` / `note` (a substantive answer) — but no `kind: ack` yet |
| cleared (not listed) | a `kind: ack` references it (unchanged) |

`answered — ack pending` sorts **below** `ready` and contributes a
distinct (lower) weight to any exit code, so a polling session sees "this
is probably handled, confirm before redoing" rather than "do this now".

**Explicitly deferred to its own later decision:** auto-satisfying the ack
from that record. That changes ack *semantics* (who clears an obligation)
and needs its own evidence + Gate 1.

## Not a gate — but a required fixture pair

`agov handoff next` is advisory. The fixture pair (ordinary discipline,
and the thing `5jde` asks be proven):

1. **Genuinely unanswered stays `ready`.** A `requires-ack` handoff with
   no referencing record → `ready`. A referencing record from an
   **unauthorized** role, or a `kind` that is not review/completion/note
   → still `ready` (does not flip).
2. **Real answer flips it.** A `review` from the handoff's `next_agent`
   that `references-to`s it → `answered — ack pending`, sorted below
   `ready`, distinct in text and exit weight.
3. **Ack clears it.** A subsequent `kind: ack` → not listed (unchanged).

## Rule-projection screen

`docs/reviews/rule-projection/20260910T155225Z-row5-s4-cpa-answered-ack-pending.yaml`
— owner: `always/coordination-message-rule.md` (`existing pointer
sufficient` — it already governs ack discipline; S4 adds a display state,
not a new obligation) **or** `not normative` if the reviewer judges a
display label carries no MUST/SHOULD; adopter explanation / runbook `not
applicable`; AI loaders `not applicable` (no loader text changes — the
behaviour is discoverable in `agov handoff next --help`); validation/drift
`advisory only`; distribution — `agov` ships in the tooling artifact.
(The parent work order flagged S1/S3 as the likely projection triggers,
not S4.)

## Rollback

Pure `agov` display logic. Revert the MR → two-state `ready` / cleared.
No schema, no rule text, no migration.

## Decision requested at Gate 1

Approve (a) the three-state classification with `answered — ack pending`
as a distinct display + sort + exit-weight state, (b) the "authorized
role + substantive kind" condition for the flip, (c) that auto-ack stays
deferred to its own decision, (d) the fixture pair. Then coder-tester
(Codex) implements; own Gate 2.
