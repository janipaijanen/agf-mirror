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
CP-B-2 has since landed a **different**, simpler field, already
**integrated and validated on `main`** (merge commit `68003b56`) —
correction per Codex's `tkkp` review: this is not yet "shipped" in the
sense of a released artifact, only merged to the development branch:
`urgency` (`normal`/`urgent` only, no authority object — see
`agf/schemas/coordination-message.schema.json` lines 19-26). The parent
work order is explicit: "S3 **consumes** \[CP-B-2's `urgency`\] for
priority-ordering and does **not** define \[a competing field\]." `!2027`
remains open until this plan is approved and its replacement preserves
!2027's residual scope, per Codex's `tkkp` review — it is not closed
merely because this plan exists.

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

**Revised per Codex's `tkkp` review**, which found two real defects in
the original draft: (a) this section's prose said `references-to` names
"the challenge's `challenge_id`-bearing record" while the JSON
description said it names an arbitrary `challenge_id` string —
`references-to` elsewhere in this schema always names a record's own
`id`/filename, never an opaque secondary identifier, so the original
draft was internally inconsistent; (b) "answered" from "any role" lets
an unrelated third party silently suppress someone else's valid
challenge merely by posting a matching field. Both are fixed below.

Reuses `kind: review` (already one of the nine message kinds — no new
kind added, consistent with CP-B-2's own precedent of shipping new
behavior via optional fields rather than a tenth kind) plus one new
optional object field:

```json
"challenge": {
  "type": "object",
  "additionalProperties": false,
  "required": ["challenge_id"],
  "properties": {
    "challenge_id": {
      "type": "string",
      "minLength": 1,
      "description": "Minted once, only by the raising record. Every other record participating in this exchange (answer or escalation) carries the SAME challenge_id here AND names the raising record's own id/filename in its top-level references-to -- two independent links, both required, so a record cannot claim to respond to a challenge it does not actually reference."
    },
    "window_hours": {
      "type": "integer",
      "minimum": 1,
      "description": "Required only on the raising record; the clock anchor is the raising record's own timestamp field (deadline = raising_record.timestamp + window_hours). A window_hours value on any other record in the exchange is ignored for deadline computation -- informational only -- so conflicting windows across responses cannot move the deadline."
    },
    "resolution": {
      "type": "string",
      "enum": ["open", "answered", "escalated"],
      "description": "Absent or 'open' on the raising record. Never set on any other record type; see the per-role restrictions below for which record author may set 'answered' or 'escalated'."
    }
  }
}
```

- **Raising a challenge**: a `kind: review` record with a `challenge`
  object (`resolution` absent, `challenge_id` freshly minted,
  `window_hours` present) disputes a specific finding, naming the
  disputed record in its own `references-to`. This record's own
  `timestamp` is the deadline anchor for the whole exchange.

- **Identity and linking (fixes the `references-to` inconsistency)**:
  every subsequent record in the exchange (answer or escalation) MUST
  carry the identical `challenge.challenge_id` string AND name the
  *raising record's own id* in its `references-to` — not the
  `challenge_id` value itself, which is never a valid `references-to`
  target. A record satisfying only one of the two links is not a valid
  response and `agov` treats it as an ordinary unrelated record, not as
  evidence closing the challenge. Two different raising records must
  never mint the same `challenge_id`; `agov coord validate` rejects a
  new raising record whose `challenge_id` collides with any existing
  record's `challenge_id` on the target branch.

- **Who may answer (fixes the silent-suppression gap)**: `resolution:
  answered` may be set only by a record authored by one of the two
  direct parties to the dispute — **the raising record's own author**
  (withdrawing their own challenge) or **the disputed record's own
  author** (defending the original finding with a substantive reply).
  `agov` determines "author" from each record's own `from` field. A
  record from any third role — including another reviewer, another
  build-engineer session, or an unrelated coder-tester — is valid
  coordination content in its own right, but does **not** set
  `resolution: answered` and does not close the challenge; `agov` keeps
  displaying the challenge as open even if such a record exists,
  precisely so an unrelated party cannot suppress someone else's
  dispute by merely posting a matching field. This is the direct fix
  for `tkkp`'s S3-1 finding.

- **Timing edge cases (fixes the missing clock/conflict semantics)**:
  a candidate answering/escalating record with a `timestamp` earlier
  than the raising record's own `timestamp` is invalid and ignored (no
  backdated closure). Among valid candidates, the first one
  chronologically after the raising record's timestamp that
  legitimately sets `answered` or `escalated` closes the challenge;
  once closed, later records referencing the same `challenge_id` are
  informational only and never reopen or overwrite the closed state. A
  record whose `references-to` names the raising record's id but whose
  own `challenge.challenge_id` does not match is rejected as
  mismatched, not treated as a response.

- **Escalation on expiry**: if the deadline (raising record's
  `timestamp` + its `window_hours`) passes with no valid `answered`
  successor found on the reader's ref, `agov coord pending` displays
  the challenge as `challenge — window expired, escalate to governor` —
  a display state, **never** a silent drop and **never** an automatic
  resolution in either direction. Only a `governor-reviewer` record may
  set `resolution: escalated` (schema-enforced: `escalated` requires
  `from: governor-reviewer`, mirroring CP-B-2's
  `governor-reviewer`/`urgent`/`decision-record` binding for
  `publication_hold` end records). Routing a matter to the governor is
  not itself a binding judgment on the dispute's substance — it is
  visibility, which any assistant can already do via an ordinary
  handoff; restricting the schema field to `governor-reviewer` instead
  reflects that *closing* the challenge (removing it from the open-
  obligation display) is the governed action, not the act of raising
  attention to it.

- **Authority boundary, explicit**: `resolution: answered` is
  restricted to the two direct parties (not "any role," corrected
  above); `resolution: escalated` is schema-restricted to
  `governor-reviewer`. Both match this project's standing rule that
  only an authorized role's coordination message supplies binding
  outcomes — a third party's opinion is evidence for either direct
  party to use in their own answer, not itself a closing record.

### 3. `agov` surfaces

- `agov coord pending`: sorts by `urgency` (as above); additionally
  surfaces any record carrying an unresolved `challenge` (`resolution`
  absent/`open`, or window-expired) in a distinct labeled section, not
  merged into or counted within the ordinary pending list — a challenge
  row never duplicates as an ordinary pending row even though the
  raising record is itself a `kind: review` message. A raising record
  defaults `requires-ack` to `false` (it is advisory, not a fresh
  obligation on top of the dispute itself) unless its author explicitly
  sets `requires-ack: true` for a different, unrelated reason. Multiple
  simultaneous open challenges are tracked independently, keyed by each
  one's own `challenge_id`; there is no limit on how many may be open
  at once.
- `agov handoff next <role>`: **corrected per `tkkp`'s S3-5 finding** —
  this is not "unaffected." `handoff next` **selects** one specific
  next obligation today (role/session-scoped, existing
  `(timestamp, file)` ordering per candidate); inserting `urgency` as a
  new primary sort key ahead of that existing ordering means the
  *selected* obligation itself can change, not merely its display
  position. This plan intentionally makes that behavioral change: an
  urgent-but-later-filed obligation is now selected ahead of an
  older-but-normal one for the same role/session. Existing
  role/session/supersession filtering and the existing tie-break within
  each urgency group are frozen exactly as today — `urgency` is
  inserted as the new outermost sort key, nothing else in the selection
  pipeline changes. This plan does **not** assume CP-B-2's
  `publication_hold` fail-closed override behavior automatically
  extends to `coord pending`/`handoff next` — if a future slice wants
  a hold's global override to also affect urgency ordering, that is a
  separate, explicit design decision, not inherited silently here.
- No new subcommand. No new required flag — both changes are
  read-side against already-optional fields; `handoff next`'s output
  selection changes as described above, `coord pending`'s does not
  (it already lists every pending item, only the order changes).

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

**Corrected per `tkkp`'s S3-4 finding**: the manifest itself must exist
at Gate 1 (with `stage: planned` for the one genuinely net-new owner),
not be entirely deferred to Gate 2 — only the *owner file's own
existence* may be planned-and-absent at this stage; the other five
projection rows need real, named paths now, not reference pointers.
The six-surface manifest, at
`docs/reviews/rule-projection/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.yaml`:

| Surface | Disposition |
|---|---|
| Normative owner | `planned`: `agf/rules/workflow/coordination-challenge-response-rule.md` (does not exist yet; created at implementation time, states challenge-response is advisory-only, the bounded-window/escalation-not-drop guarantee, the governor-only `escalated` restriction, and the answered-by-direct-parties-only restriction) |
| Adopter-facing explanation | `updated`: `agf/docs/site/docs/concepts/coordination-protocol.md` gains a "Challenge-response" subsection (same page CP-B-2 added its own "Publication-hold declarations" subsection to) |
| Runbook/situation map | `updated`: `agf/docs/site/docs/operate/poll-and-coordinate.md` gains a concise step: how to raise, answer, or recognize an escalated challenge during a poll cycle |
| AI landing/loaders | `updated`: `AGENTS.md`, `agf/templates/AGENTS.md`, and `.claude/CLAUDE.md`'s "Coordination channel" section gain one pointer sentence next to the existing nine-message-kinds table — not a restatement of the full contract |
| Validation/drift evidence | `implemented`: `agov coord validate` enforces the schema-level restrictions (challenge_id uniqueness, `escalated` requires `governor-reviewer`); `not mechanically decidable`: whether a given "answered" record is actually a direct party (schema can check `from` against the raising/disputed record's own `from`, which is mechanical) — this one is actually `implemented` too, since both authors' identities are already recorded fields `agov` can compare directly, not a human judgment call |
| Distribution | `updated`: confirm the new rule ships in the same `agf/rules/` tree distributed via the release archive as every other workflow rule — no separate packaging step needed |

Real `infosec-sentinel` and `contribution-auditor` consultations
(not the self-assessment recorded in this plan's own front matter) are
still required before Gate 2, per `tkkp`'s point that self-assessment
is disclosed honestly here but does not itself satisfy the
`advisory_consultation` requirement's intent for a schema-changing,
new-normative-rule change.

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
- **Added per `tkkp`'s S3-1/S3-2 findings**:
  - Negative: a third-party (neither raising nor disputed record's
    author) record claiming `resolution: answered` does not close the
    challenge; `agov coord pending` still displays it as open.
  - Negative: a record naming the raising record's id in `references-to`
    but with a mismatched `challenge_id` is rejected as not a valid
    response.
  - Negative: two different raising records that mint the same
    `challenge_id` — the second is rejected by `agov coord validate`.
  - Negative: an answering record with a `timestamp` earlier than the
    raising record's own `timestamp` is ignored as backdated.
  - Positive: a race between two valid closing candidates — the
    chronologically first one wins; the later one is informational only
    and does not reopen or override.
  - Positive: `handoff next`'s selected obligation changes when an
    urgent item exists for the same role/session, versus today's
    baseline where only the oldest normal item would be selected.

## Migration / impact

Per `always/downstream-impact-rule.md`: **ADDITIVE**. One new optional
schema field (`challenge`); no existing field removed or its meaning
changed; `urgency`-based ordering is a display change with no schema
impact (the field already exists). No migration required — absent
fields mean current behavior exactly.

## Rollback

**Expanded per `tkkp`'s S3-3 finding**, which correctly noted the
original one-line rollback statement didn't distinguish pre-use from
post-use rollback, and that a strict old schema (one that rejects
unknown properties) would fail to parse an already-immutable record
that carries `challenge` if the schema were later reverted.

- **Pre-use rollback** (before any real record has used `challenge` —
  e.g. reverting this MR shortly after merge, before adoption): a plain
  code/schema revert with no migration, identical to reverting any
  other purely additive change.
- **Post-use rollback** (after one or more immutable coordination
  records already carry `challenge`): the schema field itself is
  **never removed** once used — per this project's own standing rule
  ("Never remove fields from immutable records to downgrade," already
  established by CP-B-2's plan and unchanged here). Instead, rollback
  is *behavioral*: `agov coord pending`/`handoff next` stop reading or
  displaying the `challenge` object (a code-level, not schema-level,
  disable), so the feature is inert going forward while every existing
  record — including ones with `challenge` — remains schema-valid and
  readable by any reader that already accepted the field. This is the
  same forward-disablement shape CP-B-2 itself uses (an `enabled: bool`
  evaluator gate, per that plan). Sort order also reverts to today's
  behavior at the same point rollback takes effect.
- **Compatible-reader rollout**: exactly as CP-B-2's own plan already
  documents for its fields — an older, strict reader that rejects
  unknown schema properties is not a supported reader once a channel
  uses `challenge`; this is a known, accepted, documented boundary, not
  a new one this plan introduces.
