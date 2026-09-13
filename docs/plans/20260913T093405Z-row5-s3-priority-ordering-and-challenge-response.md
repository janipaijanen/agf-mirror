---
id: 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
status: pending-gate1
topic: row5-s3-priority-ordering-and-challenge-response
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Self-assessed at drafting time (real infosec-sentinel pass still
      requested before Gate 2): ordering by the existing `urgency` field
      reads state, writes nothing, and grants no authority — a record's
      own `urgency: urgent` cannot promote it above structural overrides
      that already exist (malformed/incomplete coverage, per CP-B-2's
      own fail-closed precedent). The challenge-response addition reuses
      `kind: review` plus a new optional object field, following the same
      shape CP-B-2 used for `publication_hold` — no new credential,
      network access, or trust boundary. The one new authority-adjacent
      surface is the expiry fail-state (unanswered challenge escalates to
      governor); this must never silently drop a finding, and never let
      a non-governor role manufacture an "answered" state — infosec-
      sentinel should confirm the exact schema/`agov` logic enforces
      this before Gate 2, same pattern as CP-B-2's own review.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Self-assessed at drafting time (real contribution-auditor pass
      still requested before Gate 2): this plan explicitly disposes
      !2027 (`plan/coordination-priority-and-material-challenge`) as
      superseded rather than silently abandoning it, names the exact
      scope it preserves (priority-ranking intent, bounded
      challenge-response) versus what it drops (the three-value
      `priority`/`priority_authority` schema, replaced by CP-B-2's
      already-landed two-value `urgency` enum), and does not weaken the
      "only an authorized role issues a binding decision" guarantee — an
      expired, unanswered challenge escalates to the governor, it does
      not auto-resolve in either direction.
affected_paths:
  - agf/schemas/coordination-message.schema.json
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov-coord/src/**
  - agf/rules/workflow/coordination-challenge-response-rule.md
  - docs/reviews/rule-projection/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.yaml
  - AGENTS.md
  - agf/templates/AGENTS.md
  - .claude/CLAUDE.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: Row 5 S3 — urgency-ordered queues + bounded challenge-response

**Gate 1: PENDING.** This is S3's own required detail plan under the
approved parent work order
`docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md`,
routed by Codex's handoff
(`20260913T092433Z-coder-tester-nbr1`) now that S3's hard predecessor
(CP-B-2, the `urgency` schema field) is merged
(`20260913T085749Z-coder-tester-b1nb`, merge commit `68003b56`).

## Disposition of !2027 (required before this plan proceeds)

`!2027` (`docs/plans/20260824T214947Z-coordination-priority-and-material-
challenge.md`) proposed a three-value `priority`
(`immediate`/`normal`/`deferred`) plus a `priority_authority` object.
CP-B-2 has since landed a **different**, simpler, already-shipped field:
`urgency` (`normal`/`urgent` only, no authority object — see
`agf/schemas/coordination-message.schema.json` lines 19-26). The parent
work order is explicit: "S3 **consumes** \[CP-B-2's `urgency`\] for
priority-ordering and does **not** define \[a competing field\]."

**This plan supersedes !2027.** It preserves !2027's two real
deliverables — deterministic obligation ordering, and a bounded
challenge-response contract — using CP-B-2's already-shipped field
instead of !2027's own three-value design. !2027's specific schema
proposal (`priority`/`priority_authority`) is dropped as superseded, not
implemented. Recommend closing MR !2027 with a note linking to this plan
once this plan reaches Gate 1 approval — not before, so disposition
follows approval rather than presuming it.

## Scope

### 1. Urgency-ordered queue display (read-only, no schema change)

- `agov coord pending` and `agov handoff next` gain `urgency: urgent`
  as a **primary** sort key, ahead of the existing `(timestamp, file)`
  key — urgent items first, each group internally ordered exactly as
  today. Absent `urgency` sorts as `normal` (no behavior change for any
  record that predates this field or omits it).
- `agov session resume` surfaces urgent pending obligations in a
  dedicated leading section, mirroring the parent work order's stated
  intent ("effective-`immediate` pending obligations in a dedicated
  leading section" — updated here to `urgent`, matching CP-B-2's actual
  field name and cardinality rather than !2027's `immediate`).
- **No schema change in this part.** `urgency` already exists; this is
  purely a read-side ordering change in the three `agov` commands above.
- **Authority boundary, explicit**: `urgency: urgent` is a
  self-declared field with no `priority_authority`-style elevation
  check — this matches CP-B-2's own design decision (no field
  authenticates a role or grants permission), not a gap introduced
  here. A record author declares urgency; nothing about that
  declaration is verified before it affects display ordering. This is
  intentionally lighter-weight than !2027's authority-gated
  `immediate` tier, which is exactly the design !2027 proposed and this
  plan does not carry forward — flagging this explicitly for governor/
  contribution-auditor: is display-only, unauthenticated ordering
  sufficient, or does urgent-ordering need its own authority check?
  Recommend: sufficient, because ordering is advisory display, not a
  gate — a spuriously "urgent" item wastes attention, it cannot skip a
  gate or grant authority. If contribution-auditor disagrees, this
  becomes a BLOCKING Gate 1 finding requiring redesign before Gate 2.

### 2. Challenge-response contract (new optional schema fields)

Reuses `kind: review` (already one of the nine message kinds — no new
kind added, consistent with CP-B-2's own precedent of shipping new
behavior via optional fields rather than a tenth kind) plus one new
optional object field:

```json
"challenge": {
  "type": "object",
  "additionalProperties": false,
  "required": ["challenge_id", "window_hours"],
  "properties": {
    "challenge_id": {
      "type": "string",
      "minLength": 1,
      "description": "Opaque identifier for this challenge, shared by the raising record and its eventual resolution/escalation record."
    },
    "window_hours": {
      "type": "integer",
      "minimum": 1,
      "description": "Bounded response window in hours. No unbounded challenge cycles."
    },
    "resolution": {
      "type": "string",
      "enum": ["open", "answered", "escalated"],
      "description": "Absent or 'open' on the raising record. A later record referencing this challenge_id in references-to sets 'answered' (substantive response) or 'escalated' (window expired, unanswered, routed to governor)."
    }
  }
}
```

- **Raising a challenge**: a `kind: review` record with a `challenge`
  object (`resolution` absent, meaning `open`) disputes a specific
  finding, naming the disputed record in `references-to`.
- **Answering a challenge**: a later record `references-to`-ing the
  challenge's `challenge_id`-bearing record, with `challenge.resolution:
  answered` and its own `challenge_id` matching the original — this is
  the "substantive response" the parent work order requires, distinct
  from `agov` inferring an answer from mere presence of a later record
  on the same topic.
- **Escalation on expiry**: if `window_hours` elapses with no
  `answered`/`escalated` successor found on the reader's ref, `agov
  coord pending` displays the challenge as `challenge — window expired,
  escalate to governor` — a display state, **never** a silent drop and
  **never** an automatic resolution in either direction. Only a
  governor-reviewer record may set `resolution: escalated` (schema-
  enforced: `escalated` requires `from: governor-reviewer`, mirroring
  CP-B-2's `governor-reviewer`/`urgent`/`decision-record` binding for
  `publication_hold` end records).
- **Authority boundary, explicit**: `resolution: answered` may come
  from any role (a substantive answer is evidence, not an authority
  claim) but `resolution: escalated` is schema-restricted to
  `governor-reviewer`, matching this project's standing rule that only
  a governor-reviewer coordination message supplies binding gate/
  decision authority.

### 3. `agov` surfaces

- `agov coord pending`: sorts by `urgency` (as above); additionally
  surfaces any record carrying an unresolved `challenge` (`resolution`
  absent/`open`, or window-expired) in a distinct labeled section, not
  merged into the ordinary pending list.
- `agov handoff next <role>`: unaffected in its selection logic (still
  role/session-scoped); its display ordering gains the same
  `urgency`-first sort as `coord pending`.
- No new subcommand. No new required flag — both changes are
  read-side/display-only against already-optional fields.

## Not in this slice

- CP-B-2's `urgency` field definition itself (already shipped).
- !2027's `priority_authority` elevation-authority design (superseded,
  per Disposition above).
- Any blocking CI check. This entire slice is advisory display: no
  merge, gate, or Gate 2 decision is affected by urgency ordering or an
  open/expired challenge. Per `gate-self-test-rule.md`, this means no
  negative-test obligation applies to this slice — if a future slice
  proposes turning "open challenge" into an actual Gate 2 blocker, that
  is a separate, later Gate 1 decision with its own negative test, not
  bundled here.
- `agov handoff next`'s branch-coverage behavior (stays with #123 / row
  6, per the parent work order).

## Rule projection (net-new normative owner)

`agf/rules/workflow/coordination-challenge-response-rule.md` does not
yet exist. Per `documentation-rule-projection-rule.md`'s net-new-owner
lifecycle, this Gate 1 plan sets `stage: planned` for that owner in its
projection manifest (to be written at
`docs/reviews/rule-projection/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.yaml`
before Gate 2, per that rule's required direct checks: `agov plan
projection-check --allow-planned` at Gate 1, the strict check without
that flag before Gate 2). The new rule states: challenge-response is
advisory (never blocks a merge on its own), the bounded-window/
escalation-not-drop guarantee, and the governor-only `escalated`
authority restriction. `AGENTS.md`, `agf/templates/AGENTS.md`, and
`.claude/CLAUDE.md` gain a pointer to it in the coordination-channel
section, alongside the existing nine-message-kinds table.

## Test strategy

- Schema round-trip: a record with no `challenge`/`urgency` field
  parses identically to today (backward compatibility).
- Positive: `agov coord pending` output ordering — a fixture with mixed
  `urgent`/`normal`/absent-urgency records sorts urgent-first, ties
  broken by existing `(timestamp, file)` order.
- Negative (ordering): a record with an invalid `urgency` value (schema
  violation, not an `agov` bug) is rejected by `agov coord validate`,
  not silently coerced to `normal`.
- Positive: an open challenge with time remaining displays as open, not
  escalated.
- Negative (challenge core behavior): an open challenge past its
  `window_hours` with no `answered`/`escalated` successor displays as
  `escalate to governor` — proving the fail-state fires — and a
  non-governor-authored record claiming `resolution: escalated` is
  rejected by schema validation (`from` const mismatch), proving the
  authority restriction holds. Per `gate-self-test-rule.md`'s own
  scoping, this is not a *blocking-gate* negative test (this slice adds
  no blocking gate) — it is the ordinary positive/negative fixture
  pair `always/boundary-shape-testing-rule.md` and this project's own
  test-strategy convention already require for any behavior-changing
  surface.
- CLI/MCP parity: both `agov coord pending` and any equivalent
  `agov-mcp` tool (if one exposes pending-obligation listing) apply the
  same ordering and challenge-display logic — no divergent behavior
  between the two surfaces.

## Migration / impact

Per `always/downstream-impact-rule.md`: **ADDITIVE**. One new optional
schema field (`challenge`); no existing field removed or its meaning
changed; `urgency`-based ordering is a display change with no schema
impact (the field already exists). No migration required — absent
fields mean current behavior exactly.

## Rollback

Both changes are additive and display-only or optional-schema. Reverting
this slice's MR restores prior sort order and drops challenge-response
display; no historical coordination record requires editing either way
(per the append-only/immutable-record discipline this project already
enforces).
