---
id: 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-13T10:02:24Z
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

**Gate 1: APPROVED** — `20260913T100224Z-governor-reviewer-g2gd`
("approved. proceed as proposed. gate approved too."), which also
resolves the urgency-ordering design question below as unauthenticated
by design. This is S3's own required detail plan under the
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
  plan does not carry forward. **Resolved at Gate 1**
  (`20260913T100224Z-governor-reviewer-g2gd`): unauthenticated,
  advisory-only ordering is sufficient — both this plan's own
  recommendation and Codex's independent `tkkp` review reached the same
  conclusion, and the governor confirmed it. A real contribution-auditor
  pass before Gate 2 may still surface a concrete counter-scenario, but
  the design question itself is closed, not open.

### 2. Challenge-response contract (new optional schema fields)

**Revised twice.** First per Codex's `tkkp` review (references-to
identity conflation; unrestricted "any role" answering). Then per
Codex's follow-up `mqjr` recheck, which correctly found the first
revision still incomplete: (a) checking only the `from` role string
cannot distinguish two different sessions playing the same role (two
`coder-tester` sessions compare equal); (b) letting the disputed
record's own author unilaterally set `answered` and have that
**remove** the obligation from view conflates "a reply was posted"
with "the dispute is actually resolved" — the challenger may still
disagree; (c) a response with a **future-declared** `timestamp` was
incorrectly treated as valid the moment it appeared, closing a
challenge before its own declared time; (d) missing/unavailable
raising or disputed records, equal-timestamp ties, and a maximum
window bound were unspecified; (e) the `resolution` field's own schema
description was self-contradictory ("never set on any other record
type" directly next to record types that do set it). All fixed below.

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
      "description": "Minted once, only by the raising record. Every other record participating in this exchange (a reply or an escalation) carries the SAME challenge_id here AND names the raising record's own id/filename in its top-level references-to -- two independent links, both required, so a record cannot claim to respond to a challenge it does not actually reference."
    },
    "disputed_record_id": {
      "type": "string",
      "minLength": 1,
      "description": "Required only on the raising record. Names exactly one record -- the specific finding being disputed -- and MUST also appear in the raising record's own top-level references-to. A raising record's references-to may separately cite other contextual records (prior reviews, related discussion); this field is what disambiguates which one is actually disputed, since references-to alone is an unrestricted array with no ordering or cardinality contract."
    },
    "window_hours": {
      "type": "integer",
      "minimum": 1,
      "maximum": 720,
      "description": "Required only on the raising record; capped at 720 (30 days) -- no unbounded window. The clock anchor is the raising record's own timestamp field (deadline = raising_record.timestamp + window_hours). A window_hours value on any other record in the exchange is ignored for deadline computation -- informational only -- so conflicting windows across responses cannot move the deadline."
    },
    "resolution": {
      "type": "string",
      "enum": ["open", "replied", "escalated"],
      "description": "Meaningful on every record in the exchange, not just the raising one (corrected: the prior wording that this is 'never set on any other record type' directly contradicted the reply/escalation records that must set it). 'open' (or absent) marks the raising record. 'replied' marks a direct party's substantive reply -- see 'Reply is evidence, not automatic closure' below: it does NOT by itself remove the challenge from display. 'escalated' marks the one record that actually closes the exchange, restricted to governor-reviewer."
    }
  }
}
```

- **Raising a challenge**: a `kind: review` record with a `challenge`
  object (`resolution` absent, `challenge_id` freshly minted,
  `window_hours` present, `disputed_record_id` naming exactly the one
  specific finding under dispute) disputes that finding. Per finding 2
  above, `disputed_record_id` — not `references-to` generally — is the
  single source of truth for which record is disputed; the raising
  record's `references-to` MUST include that same id (so the existing
  "obligation resolved by a later reference" tooling still sees the
  link) but MAY also cite other contextual records without ambiguity,
  since only the `disputed_record_id`-named one counts as "the"
  dispute. This record's own `timestamp` is the deadline anchor and its
  own top-level `session` field (the existing session-identity field
  from Row 5 S1) is the bound identity of the raising party.

- **Identity and linking**: every subsequent record in the exchange
  (a reply or an escalation) MUST carry the identical
  `challenge.challenge_id` string AND name the raising record's own id
  in its `references-to` — not the `challenge_id` value itself, which
  is never a valid `references-to` target. A record satisfying only
  one of the two links is not a valid response and `agov` treats it as
  an ordinary unrelated record. Two different raising records must
  never mint the same `challenge_id`; `agov coord validate` rejects a
  colliding one.

- **Session-bound parties (fixes `mqjr`'s role-vs-session finding)**:
  "direct party" is determined by matching a candidate reply's
  top-level `session` field against the raising record's `session`
  (the challenger replying/withdrawing) or the `session` of the one
  record named by the raising record's own `disputed_record_id` (the
  original author defending) — **not** by `from` role alone, and not
  by any other record `references-to` might also cite. Two sessions
  sharing a role (e.g. two `coder-tester` sessions)
  no longer compare equal. This still relies on a self-declared
  `session` token, exactly as S1 already does — it is not
  cryptographic authentication, and this plan makes no stronger claim
  than S1's own. When a candidate reply's `session` is absent (an
  older-format record, or a session that never adopted S1's field), it
  cannot be bound as a direct party and is treated as third-party
  content per the rule below — never as an assumed match.

- **Reply is evidence, not automatic closure (fixes `mqjr`'s
  conflation finding)**: keeping this bounded and simple, per
  `simplicity-rule.md`, rather than inventing a multi-state dispute
  workflow the parent work order never asked for — there are exactly
  two non-open `resolution` values, and only one of them closes
  anything. A direct party's `resolution: replied` record does **not**
  remove the challenge from `agov coord pending`'s display: the
  challenge's displayed state changes from `open` to `replied —
  decision pending` and **stays visible** in that state indefinitely.
  A reply is evidence for whoever eventually looks at the dispute, not
  a self-executing resolution. The **only** record that removes a
  challenge from the needs-attention display is a governor-reviewer
  record setting `resolution: escalated` — whether that happens
  because the window expired unanswered, or because the governor
  reviewed a reply and decided the matter is settled, is a distinction
  for the governor's own record to state in prose; the schema does not
  need a separate value for each trigger. A record from any third
  role/session — another reviewer, a different build-engineer session,
  an unrelated coder-tester session — is valid coordination content in
  its own right but is not a direct party under the session-binding
  rule above and never sets `replied`.

- **Timing edge cases (fixes `mqjr`'s forward-dated-response and
  missing-record findings)**:
  - A candidate record's own `timestamp` must satisfy
    `raising_record.timestamp <= candidate.timestamp <= now` at the
    moment `agov` evaluates it, where `now` is the reader's current
    observation time, not merely "after the raising timestamp" as the
    prior revision incorrectly stated. A **future-declared**
    `timestamp` (`candidate.timestamp > now`) is invalid **until that
    time actually arrives** — `agov` re-evaluates this on every
    invocation, so a record dated tomorrow cannot close today's
    challenge; a **backdated** one (`candidate.timestamp <
    raising_record.timestamp`) is permanently invalid.
  - **Equal timestamps, scoped to same-class candidates only**:
    **corrected per the independent semantic review's finding 3**,
    which found the prior wording ambiguous about whether a tie-break
    could apply *across* a reply and an escalation (it must not — only
    `escalated` ever closes anything, regardless of any reply's
    timing, per the simplified two-value model above; a reply can
    never "win" against an escalation or vice versa, because a reply
    never closes at all). The lexicographic tie-break applies **only**
    when ordering multiple candidate records of the **same**
    `resolution` value for **display purposes** (e.g. multiple
    `replied` records from the same or different direct parties) — it
    is never used to decide precedence between `replied` and
    `escalated`. Precedence is unconditional: any valid `escalated`
    record closes the challenge outright, at any time, whether or not
    a `replied` record exists or when it was posted; a `replied` record
    never reopens or reduces an already-`escalated` challenge.
  - **Missing/unavailable records**: if the raising record's own
    `disputed_record_id`, or a candidate reply's referenced raising
    record, cannot be read (deleted ref, unreadable object, fetch
    failure), or `disputed_record_id` does not resolve to exactly one
    record, or is absent, duplicated, or ambiguous against
    `references-to`'s other entries, `agov` reports it as an
    `unavailable`/`needs-review` problem and treats the challenge as
    unresolved — never as silently absent, silently closed, or
    resolved by picking a convenient candidate. This mirrors
    `coordination-publication-rule.md`'s existing "a failed read is
    unavailable evidence, not a successful empty result" discipline,
    extended per the independent semantic review's finding 2 to source
    ambiguity, not only source absence.
  - A record whose `references-to` names the raising record's id but
    whose own `challenge.challenge_id` does not match is rejected as
    mismatched, not treated as a response.
  - **Deadline arithmetic, equality, and future-dated raising records
    (fixes the independent semantic review's finding 4)**: deadline
    computation (`raising_record.timestamp + window_hours`) uses
    checked/saturating arithmetic — a value that would overflow the
    supported timestamp range is treated as an invalid raising record
    (`needs-review`), never as silently wrapping to an incorrect,
    earlier-than-intended deadline. At exactly the deadline instant
    (`now == deadline`), the challenge is still `open`, not yet
    expired — expiry requires `now > deadline`, strictly after. A
    **raising record itself** dated in the future relative to the
    reader's observation time (`raising_record.timestamp > now`) is
    not yet effective: `agov` does not display it as an open challenge
    until its own declared timestamp arrives, exactly mirroring the
    treatment already given to future-dated replies above — this
    closes the gap the independent review found, where a far-future
    raising timestamp could otherwise appear to grant far longer than
    its declared 30-day-max window. A `replied` record arriving after
    the deadline has already passed still displays as `replied —
    decision pending` (not rejected) — the window's expiry governs
    *display escalation eligibility*, not whether a late reply is
    valid evidence; the operator's own decision whether to still weigh
    a late reply is out of `agov`'s scope.

- **Escalation on expiry**: if the deadline (raising record's
  `timestamp` + its `window_hours`) passes with no valid `replied`
  record from a direct party found on the reader's ref, `agov coord
  pending` displays the challenge as `challenge — window expired,
  escalate to governor` — a display state, **never** a silent drop.
  Whether or not a reply already exists, only a `governor-reviewer`
  record may set `resolution: escalated` (schema-enforced: `escalated`
  requires `from: governor-reviewer`, mirroring CP-B-2's
  `governor-reviewer`/`urgent`/`decision-record` binding for
  `publication_hold` end records) — this is the single closing action
  regardless of whether it follows a reply or an unanswered expiry, per
  the simplification above.

- **Authority boundary, explicit**: `resolution: replied` is
  restricted to the two session-bound direct parties (not "any role");
  `resolution: escalated` is schema-restricted to `governor-reviewer`
  and is the only value that closes a challenge. Both match this
  project's standing rule that only an authorized role's coordination
  message supplies binding outcomes — a third party's opinion is
  evidence either direct party may use in their own reply, not itself
  a closing record.

### 3. `agov` surfaces

- `agov coord pending`: sorts by `urgency` (as above); additionally
  surfaces any record carrying an unresolved `challenge` (`resolution`
  absent/`open`, or window-expired) in a distinct labeled section. A
  raising record defaults `requires-ack` to `false` (it is advisory,
  not a fresh obligation on top of the dispute itself) unless its
  author explicitly sets `requires-ack: true` for a different,
  unrelated reason. **Corrected per the independent semantic review's
  finding 1**: these two tracked properties — "carries an open
  challenge" and "has its own ordinary requires-ack obligation" — are
  **independent**, not mutually exclusive display modes. The original
  wording ("not merged into or counted within the ordinary pending
  list") incorrectly suppressed a raising record's separate,
  explicitly-set `requires-ack: true` obligation merely because it also
  had an open challenge — a real record satisfying both properties must
  appear in *both* the ordinary pending count (if its own ack-tracker
  eligibility, per `handoff_tracker.rs`'s existing
  `find_pending_all_kinds`/`is_directly_referenced_later` logic,
  remains unsatisfied) *and* the challenge section (for its dispute
  state). A single visual row MAY display both states together; the
  challenge itself is never counted as a second, separate ordinary
  obligation — only the raising record's own independent `requires-ack`
  status is. Multiple simultaneous open challenges are tracked
  independently, keyed by each one's own `challenge_id`; there is no
  limit on how many may be open at once.
- **Three distinct existing paths, characterized separately** — per
  Codex's own source-grounded inventory of `main` at `5917d1ab`
  (`jh8g`), corrected from this plan's earlier, less precise
  description:
  - `agov handoff next <role>` (single-role path,
    `tooling/rust/crates/agov/src/main.rs` ~line 5916): selects
    `max_by_key((timestamp string, filename))` among unresolved
    handoffs already filtered to `to == role`. Session addressing
    (`--session`) is applied as a post-selection "addressed elsewhere"
    warning, not an eligibility filter. **Corrected per `tkkp`'s S3-5
    finding**: inserting `urgency` as a new, outermost primary sort key
    ahead of this existing `(timestamp, file)` key means the *selected*
    obligation can change (an urgent-but-later item now selected ahead
    of an older-but-normal one) — this plan intentionally makes that
    change. Everything else in this path (role filtering, the
    session-addressed-elsewhere warning, the existing tie-break within
    each urgency group) is frozen exactly as today.
  - `agov coord pending` (~line 5985): preserves the existing ascending
    `(timestamp string, filename)` order over its full unresolved set
    (requires-ack records excluding handoff/ack kinds, per the
    existing tracker's cutoff and direct-later-reference resolution).
    `urgency` is inserted the same way, as a new outermost sort key;
    this path already lists every item rather than selecting one, so
    only the *order*, not the *contents*, changes.
  - **Branch-inclusive `handoff next --include-branch-local`**
    (`handoff_branch.rs`, a genuinely separate code path from the
    single-role path above): collects *all* unresolved per-source-role
    candidates across every local/remote-tracking ref, deduplicates by
    `(file, blob)` into a `BTreeMap`, and prints them in map order —
    it does not select one "best" candidate the way the single-role
    path does. `urgency` ordering applies to this path's own printed
    order the same way it does to `coord pending`'s (a full listing,
    not a single selection); it does not change the dedup or
    cross-branch collection logic.
  - This plan does **not** assume CP-B-2's `publication_hold`
    fail-closed override behavior automatically extends to any of the
    three paths above — if a future slice wants a hold's global
    override to also affect urgency ordering, that is a separate,
    explicit design decision, not inherited silently here.
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
| Normative owner | `planned`: `agf/rules/workflow/coordination-challenge-response-rule.md` (does not exist yet; created at implementation time, states challenge-response is advisory-only, the bounded-window guarantee, the governor-only `escalated` restriction as the sole closing action, and the session-bound-direct-parties-only restriction for `replied`) |
| Adopter-facing explanation | `updated`: `agf/docs/site/docs/concepts/coordination-protocol.md` gains a "Challenge-response" subsection (same page CP-B-2 added its own "Publication-hold declarations" subsection to) |
| Runbook/situation map | `updated`: `agf/docs/site/docs/operate/poll-and-coordinate.md` gains a concise step: how to raise, reply to, or recognize an escalated challenge during a poll cycle |
| AI landing/loaders | `updated`: `AGENTS.md`, `agf/templates/AGENTS.md`, and `.claude/CLAUDE.md`'s "Coordination channel" section gain one pointer sentence next to the existing nine-message-kinds table — not a restatement of the full contract |
| Validation/drift evidence | `implemented`: `agov coord validate` enforces the schema-level restrictions (challenge_id uniqueness, `escalated` requires `governor-reviewer`, `window_hours` max 720); **not** claimed fully mechanical for direct-party matching — `mqjr`'s finding stands: `session`-field comparison narrows but does not eliminate false matches (a self-declared token, not cryptographic identity), so this row is `implemented` for the schema-level checks and `not mechanically decidable` for whether a `session` match is the *actual* same real-world actor |
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
  `window_hours` with no `escalated` record from a governor displays as
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
    author) record claiming `resolution: replied` does not close the
    challenge; `agov coord pending` still displays it as open.
  - Negative: a record naming the raising record's id in `references-to`
    but with a mismatched `challenge_id` is rejected as not a valid
    response.
  - Negative: two different raising records that mint the same
    `challenge_id` — the second is rejected by `agov coord validate`.
  - Negative: a reply record with a `timestamp` earlier than the
    raising record's own `timestamp` is ignored as backdated.
- **Added per `mqjr`'s recheck findings**:
  - Negative: two different sessions sharing the same `from` role each
    reply — neither is treated as a direct party unless its `session`
    actually matches the raising or disputed record's own `session`.
  - Positive/negative: a direct party's `replied` record does NOT
    remove the challenge from `agov coord pending`'s output; it only
    changes the displayed sub-state to `replied — decision pending`.
  - Negative: a reply record with a `timestamp` in the future relative
    to the reader's observation time is invalid at read time; re-run
    the same read after that time passes and confirm it becomes valid
    (proving `agov` re-evaluates rather than caching a one-time
    judgment).
  - Negative: the raising or disputed record is deleted/unreadable —
    `agov` reports an `unavailable` problem and treats the challenge as
    needs-review, never as silently closed or silently absent.
  - Positive: two valid candidate records with identical timestamps —
    the lexicographically earlier filename/id wins the tie,
    deterministically.
  - Negative: a `window_hours` value above 720 is rejected by schema
    validation.
  - Positive: disabling challenge display (forward-disablement,
    post-use rollback) still surfaces any previously-open,
    not-yet-escalated challenge in a distinct `disabled — needs review`
    section, rather than making it disappear entirely.
  - Positive: `handoff next`'s selected obligation changes when an
    urgent item exists for the same role/session. **Corrected per
    `mqjr`'s S3-5 finding**: the previous revision asserted "today's
    baseline selects the oldest normal item" as if already established
    fact — it is not yet code-grounded. This test must first capture
    today's actual current selection behavior with a characterization
    test (per `workflow/feature-regression-guard-rule.md`'s pattern for
    modifying an untested-in-this-respect feature), *then* add urgency
    ordering on top and confirm only the urgency-vs-no-urgency delta
    changed, with every other existing tie-break, role/session, and
    supersession-filtering behavior verified unchanged against that
    same characterization baseline.

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
  established by CP-B-2's plan and unchanged here). Rollback is
  *behavioral*: `agov coord pending`/`handoff next` stop evaluating new
  `challenge` semantics (a code-level, not schema-level, disable), so
  the feature is inert going forward while every existing record —
  including ones with `challenge` — remains schema-valid and readable.
  **Corrected per `mqjr`'s S3-3 finding**: "stop displaying the
  `challenge` object" is not itself safe, because a raising record
  defaults `requires-ack: false` and is excluded from the ordinary
  pending list specifically so it doesn't double-count — if disablement
  also removed it from the *challenge* section with nothing replacing
  that visibility, a genuinely still-open, not-yet-escalated challenge
  would disappear entirely, violating the parent work order's
  no-silent-drop guarantee. The disabled state therefore has its own
  conservative fallback: any record carrying an open (non-`escalated`)
  `challenge` object is still surfaced by `agov coord pending`, in a
  distinct `disabled — needs review` section, for as long as disablement
  is active — visibility is preserved even though the ordering/reply
  logic is inert. Sort order (the `urgency` half of this plan) also
  reverts to today's behavior at the same point rollback takes effect;
  this fallback concerns only the `challenge` half. This mirrors the
  same forward-disablement shape CP-B-2 itself uses (an `enabled: bool`
  evaluator gate, per that plan), extended with the conservative
  visibility fallback CP-B-2 didn't need for its own, non-obligation
  fields.
- **Compatible-reader rollout**: exactly as CP-B-2's own plan already
  documents for its fields — an older, strict reader that rejects
  unknown schema properties is not a supported reader once a channel
  uses `challenge`; this is a known, accepted, documented boundary, not
  a new one this plan introduces.
