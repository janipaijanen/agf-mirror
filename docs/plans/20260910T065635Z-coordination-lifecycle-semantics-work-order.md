---
id: 20260910T065635Z-coordination-lifecycle-semantics-work-order
status: pending-gate1
topic: coordination-lifecycle-semantics-work-order-v211-row5
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This work order commits schema and `agov` changes to the coordination
      channel: a session-identity field (#121), a self-announcing
      supersession field pair (#122), an urgency/priority field and a
      challenge-response obligation type (!2027), and a queue-state change
      to `agov handoff next` for CP-A. InfoSec must confirm at this Gate 1
      that: none of the new fields is a channel for credentials or
      cross-record leakage; a session-identity field cannot be spoofed to
      redirect another session's obligations; the urgency field is not
      author-gameable (this field is defined here and consumed by CP-B-2's
      release-lane hold, so a wrong value there suppresses or force-publishes
      real evidence); and the CP-A ack-satisfaction path cannot let an
      unauthorized role clear a `requires-ack` obligation. Each
      implementation slice returns for its own exact-head InfoSec review;
      this row covers the design decision, which is not deferrable.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Coordination-protocol semantics change touching the audit trail.
      Contribution-auditor must confirm the four members are sequenced and
      sliced without demoting any of them, that #122's field pair does not
      predate the `qbka` Slice A provenance freeze it depends on, that the
      CP-A ack-satisfaction path does not weaken the "only an authorized
      role issues a binding decision" guarantee, and that the
      challenge-response obligation type does not create a way to stall a
      valid finding indefinitely. Each implementation slice returns for its
      own exact-head audit.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Coordination lifecycle semantics — Gate 1 work order (v2.11 IN row 5)

## Provenance and authority

- **v2.11 scope plan** `docs/plans/20260909T021841Z-v211-release-scope.md`
  row 5 (IN by governor scope decision `er78`; `sh3m` approved the expanded
  scope): "planner must return **one bounded Gate 1 work order** and may
  split implementation slices without demoting any member; each
  implementation slice retains Gate 2."
- **Handoff `20260909T040330Z-capability-developer-5jde`** to
  planner-architect: challenge the Propeller-derived coordination-protocol
  assessment and its evidence strength; deduplicate and sequence the
  proposed adjustments against #121/#122/#123/!2027; return a bounded
  Gate 1 work-order proposal **or** a material challenge-response.
- **`jprj`** (`20260909T013812Z-capability-developer-jprj`) — the
  capability-developer analysis this work order responds to.
- **`ds4k` Finding 2** already routes #121/#122/#123/!2027 to their own
  Gate 1 as the coordination-protocol-semantics workstream.

This is a Gate 1 **work order**: it fixes membership, sequence, and slice
boundaries. It authorizes no schema, `agov`, or rule change by itself —
each implementation slice below returns for its own Gate 1-level review
detail is not re-litigated, its own exact-head advisory reviews, and its
own binding Gate 2.

## Members (bounded set — none demoted)

| Member | Gap | Source evidence |
|---|---|---|
| **#121** | Coordination addressing has no session-level identity; concurrent same-role sessions cannot be reliably addressed | 2026-09-05 multi-session (`esr1`/`2s3a` filed by a Codex planner while a Claude planner was active; `7sjq`/`8k1s` disambiguation done by hand) |
| **#122** | Supersession is convention-only; a superseded record does not announce itself | 2026-09-05 (`th6m` still reads as a live operator instruction after `mhq1` withheld it); out-of-band MR-comment instructions compound it |
| **!2027** | No protocol notion of which pending obligation ranks highest, and no required challenge-response path for a disputed finding | `7dba`/`hg52` review disputes resolved by ad-hoc record exchange |
| **CP-A** | A substantive answer (`review`/`completion`/`note` that `references-to` a `requires-ack` handoff) does not clear it in `agov handoff next` | This session + historical (`n7p1` re-reviewed MR !2038 after `eb87`); `rkbk`/`mhq3` are ack-only MRs filed solely to clear the queue |

**Not in this work order:** CP-C (`agov handoff next` reads only the base
branch) folds into row 6 / #123's tooling slice, per `jprj` §C and the
scope plan row 6. CP-B (publication efficiency) is row 7, with its own
work order `20260909T133651Z-cpb-coordination-publication-efficiency`.

## `5jde` challenge-response — independent assessment of evidence strength

Required by the handoff. My independent read, not a restatement of `jprj`:

1. **#121 / #122 / !2027 are well-evidenced.** The 2026-09-05 records cited
   are multi-session, reproduced, and have concrete costs (a near-miss
   runner change on `th6m`; hand-built disambiguation exchanges). No
   narrowing warranted.
2. **CP-A is real but the weaker case, and partly self-inflicted.** The
   "duplicate work" cost (a second session redoes a still-`ready` item) is
   structural and corroborated (`n7p1`/`eb87`). The "ack-only overhead"
   cost is partly a choice — a session can decline to file an ack-only MR
   and just leave the queue item, accepting the stale display. So CP-A's
   **first** slice should be the minimal, display-only change
   (`answered — ack pending` distinct from `ready`); **auto-satisfying**
   the ack from an authorized-role `review`/`completion` is a change to
   ack *semantics* — closer to decision-authority mechanics — and should
   be a separate, later decision, not bundled.
3. **No member is rejected or deferred out of v2.11.** All four proceed;
   the sequencing below is the only constraint added.
4. **Dedup with adjacent work:**
   - The **urgency/priority field** (#! 2027 / S3) and CP-B-2's
     "urgent-publication bypass / non-urgent hold" want the *same* field.
     This work order **owns** the field's definition; CP-B-2 **consumes**
     it. Neither defines its own.
   - **#122's field pair** must not predate `qbka` Slice A — see sequence.
   - **CP-C** stays in #123 (row 6); this work order does not touch
     `agov handoff next`'s branch coverage.

## Ordering constraint

```
qbka Slice A (provenance-bound `superseded` schema + validator)
    └── S2 (#122 self-announcing supersession field pair)   [HARD predecessor]

S1 (#121 session identity)   — independent; may start immediately
S3 (!2027 priority/urgency + challenge-response)  — independent; field
     shape must be published before CP-B-2 consumes it
S4 (CP-A ack display)   — independent; `agov`-only for the first slice
```

Only **S2 → qbka Slice A** is a hard predecessor. S1, S3, S4 may proceed in
any order or in parallel, each on its own branch, each its own Gate 2.

## Implementation slices (each its own Gate 1 detail + exact-head reviews + Gate 2)

### S1 — #121 session-level addressing
- **Change:** add an **optional** session-identity field to
  `coordination-message.schema.json` (record level) and to the `handoff`
  block (e.g. `for_session` / `handoff.next_session`), carrying an opaque
  session token already available to each assistant. Absent = current
  role-only behaviour (backward compatible).
- **`agov`:** `agov handoff next <role>` and `agov coord visibility` gain a
  `--session <token>` filter; without it, behaviour is unchanged. With it,
  role-addressed items not scoped to another session still show; items
  scoped to a *different* session show as `addressed elsewhere`.
- **Not a gate.** `agov handoff next` is advisory queue display, not an
  MR-blocking check. No `gate-self-test-rule` trigger unless the slice adds
  a blocking validator (it does not).
- **Rule projection:** `always/coordination-message-rule.md` and the
  foundry `AGENTS.md` "Coordination channel" section reference role
  addressing; the six-surface manifest is owed at this slice's Gate 1.

### S2 — #122 self-announcing supersession  *(predecessor: qbka Slice A)*
- **Change:** after `qbka` Slice A freezes the `superseded` provenance
  shape, add a coordination-record field pair (`supersedes` /
  `superseded_by`) whose vocabulary and provenance fields **match** the
  frozen `qbka` shape (no second, divergent shape).
- **`agov`:** `agov coord list --since` and `agov coord pending` mark a
  record whose `superseded_by` target is present on the same branch or
  `main` as `superseded — see <id>`, and drop it from the live-obligation
  set. A record with no successor on the reader's ref stays live.
- **Out-of-band-instruction note (from #122 evidence):** this slice does
  **not** solve binding instructions arriving as MR comments outside the
  channel. That is called out as a known residual for a separate
  discovery-side item (row 6 / #123), not silently absorbed here.
- **Rule projection:** `workflow/coordination-publication-rule.md`
  ("Lifecycle and supersession semantics") is the normative owner; this
  slice adds the mechanical half. Six-surface manifest owed at Gate 1.

### S3 — !2027 priority + bounded material challenge-response
- **Change:** add an **optional** `urgency` (or `priority`) enum field to
  the coordination schema — **the single definition** consumed by CP-B-2 —
  and a `challenge-response` obligation type (or a `kind: review` subtype)
  that a disputed finding can raise, with a bounded response window and a
  fail-state (unanswered by the window → escalates to governor, never
  silently drops the finding).
- **`agov`:** `agov coord pending` / `agov handoff next` order by urgency
  where set; `agov coord` surfaces an open challenge-response distinctly.
- **gate-self-test:** if the challenge-response window is enforced by a
  blocking check (e.g. CI refusing a Gate 2 while an unanswered
  challenge-response is open), that check is a gate and ships with a
  negative fixture proving it fires. If it is advisory display only, no
  trigger.
- **Rule projection:** likely a new short rule owner
  (`workflow/coordination-challenge-response-rule.md`, `stage: planned` at
  this slice's Gate 1) plus `systematic-code-review-rule.md`'s finding
  format. Six-surface manifest owed.

### S4 — CP-A: a substantive answer surfaces distinctly from `ready`
- **Change (first slice, `agov`-only):** `agov handoff next <role>` shows a
  `requires-ack` handoff that has a later record `references-to`-ing it
  **from an authorized role** as `answered — ack pending`, visually
  distinct from `ready`. No schema change; no automatic ack.
- **Deferred (separate later decision, not this work order):**
  auto-satisfying the ack from that record. That changes ack semantics and
  needs its own evidence + Gate 1.
- **gate-self-test:** `agov handoff next` is not a gate. The negative check
  for this slice is a positive/negative pair proving a genuinely
  unanswered handoff still shows `ready` and only a real authorized-role
  reference flips it to `answered — ack pending`.

## Affected files (work-order level — indicative; each slice's own Gate 1 is authoritative)

- `agf/schemas/coordination-message.schema.json` — S1, S2, S3
- `tooling/rust/` `agov coord` / `agov handoff` command crates — S1–S4
- `tooling/rust/` tests — per slice (positive + negative fixtures)
- `agf/rules/workflow/coordination-publication-rule.md` — S2 (owner, pointer)
- `agf/rules/always/coordination-message-rule.md` — S1 (pointer)
- new `agf/rules/workflow/coordination-challenge-response-rule.md` — S3 (planned)
- `docs/reviews/rule-projection/<slice-id>.yaml` — one per slice
- `AGENTS.md` / `agf/templates/AGENTS.md` / `.claude/CLAUDE.md` — projection pointers per slice

## Decomposition (each its own gate)

| Slice | Deliverable | Gate path |
|---|---|---|
| S1 | #121 session identity: schema field + `agov` filter + fixtures | own Gate 1 → exact-head reviews → Gate 2 |
| S2 | #122 supersession field pair (matches `qbka` shape) + `agov` live-obligation filtering + fixtures | **after `qbka` Slice A** → own Gate 1 → Gate 2 |
| S3 | !2027 urgency field (single owner) + challenge-response obligation + fixtures | own Gate 1 → Gate 2; gate-self-test if any blocking check |
| S4 | CP-A `answered — ack pending` display + positive/negative fixtures | own Gate 1 → Gate 2 |

## Risks and controls

- **Two divergent `superseded` shapes.** Control: S2 is hard-blocked on
  `qbka` Slice A and must reuse its vocabulary and provenance fields
  verbatim; contribution-auditor confirms at S2 Gate 1.
- **Urgency field defined twice** (here and in CP-B-2). Control: this work
  order owns it; CP-B-2's plan is amended to consume, not define. Flagged
  to the CP-B-2 author.
- **Challenge-response used to stall a valid finding.** Control: bounded
  window + mandatory governor escalation on expiry; never a silent drop.
- **Session-identity field spoofed to steal/redirect obligations.**
  Control: field is advisory routing only; it never grants authority, and
  InfoSec reviews the spoofing surface at S1 Gate 1.
- **Scope creep into an `agov handoff next` redesign.** Control: S4 first
  slice is display-only; auto-ack is explicitly deferred; CP-C stays in
  #123.

## Rollback

Each slice is optional-field-additive on the schema and additive on
`agov` surfaces; reverting a slice's MR restores prior behaviour with no
migration (absent fields = current semantics). No slice rewrites existing
coordination records.

## Test and acceptance strategy

Per slice: schema round-trip tests for new optional fields; `agov` command
tests with and without the new field/flag proving backward-compatible
default behaviour; a positive + negative fixture pair for every
behaviour-changing surface (and a firing negative test for any blocking
check S3 introduces). No slice merges without its exact-head advisory
reviews and binding Gate 2.

## Rule-projection screen (work-order level)

This work order itself changes no normative text — it sequences slices.
Each slice that introduces or changes MUST/blocking language in an
adopter-facing page or runbook carries its own
`docs/reviews/rule-projection/<slice-id>.yaml` six-surface manifest, built
and validated at that slice's Gate 1 (S2 and S3 are the likely triggers;
S1 and S4 are `agov`-surface and loader-pointer changes). `not applicable`
for this parent plan, with that reason.

## Decision requested at Gate 1

Approve (a) the four-member bounded set with **no demotion**, (b) the
single hard predecessor (`S2 → qbka Slice A`) and otherwise-parallel
slices, (c) the slice boundaries S1–S4 and that each retains its own
Gate 1 detail and binding Gate 2, (d) the `5jde` challenge-response
conclusion (CP-A ships display-only first; auto-ack deferred; nothing
rejected), and (e) that the urgency field is defined here once and
consumed by CP-B-2. This work order grants no schema/`agov`/rule change
authority on its own.
