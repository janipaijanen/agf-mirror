---
id: 20260910T155224Z-row5-s1-session-level-addressing
status: approved
gate1_approved_by: 20260910T190854Z-governor-reviewer-kr4b
gate1_approved_at: "2026-09-10T19:08:54Z"
topic: row5-s1-session-level-addressing
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Inherited schema marker from the original Claude plan, not a new
      attestation. No exact-revision independent InfoSec clearance is
      established here. Renewed consultation is outstanding; this plan
      must not advance to Gate 1 on the strength of this marker. The
      schema cannot represent pending consultation (issue 130).
  - role: contribution-auditor
    status: consulted
    reason: >-
      Actual bounded consultation: msq8 identified material contract
      findings in the original revision. Codex now authors corrections
      under eetd/mb8k and cannot independently clear them. A fresh
      independent review of this revision remains outstanding.

---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 5 S1 — #121 session-level addressing (Gate 1)

## Provenance

- Parent: `docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md`
  (v2.11 IN row 5, Gate 1 `tsbh` / ratified `q862`), slice **S1**. Governor
  decision "Keep all in v2.11" (2026-09-10) confirms row 5 stays a hard
  cut precondition.
- Issue **#121**: coordination records address a *role*; when two sessions
  hold the same role concurrently, a record does not identify which
  session should act. 2026-09-05 evidence: `esr1`/`2s3a` filed by a Codex
  planner while a Claude planner was active; `7sjq`/`8k1s` disambiguation
  done by hand.

## Starting point — the field already half-exists

`agf/schemas/coordination-message.schema.json` already has a top-level
**`session`** property (`UUIDv7 … Optional. Audit only.`). It records
provenance but is **not** a routing key, and there is no way to address a
`handoff` to a session. S1 leaves `session` as author provenance and adds recipient addressing
only for handoffs. Routing notes/reviews is deferred; their author session
must never be interpreted as the recipient.

## Change

| File | Change |
|---|---|
| `agf/schemas/coordination-message.schema.json` | Keep top-level `session` unchanged as emitting-session provenance. Add optional `handoff.for_session` (string, UUIDv7; **not** in `handoff.required`) — the session a handoff is addressed to, when narrower than `next_agent`'s role. |
| `agf/rules/workflow/coordination-publication-rule.md` | One paragraph: role addressing remains the default; only `handoff.for_session` narrows the intended recipient. Sender `session` never narrows the recipient. Routing grants no authority and never overrides existing claims, temporary allocations, independence or gates. An uncertain recipient observes and records ambiguity, and follows the existing claim/allocation procedure before mutation. |
| `agov handoff next <role>` (`tooling/rust/crates/agov/src/main.rs`) | New optional `--session <token>`. Without it: unchanged. With it: a handoff whose `for_session` matches → normal; `for_session` set to a *different* token → shown as `addressed elsewhere` (still listed, same position and existing state, not hidden); `for_session` absent → shown normally (role-wide). |
| `agov coord visibility` | Same `--session <token>` filter, same "still listed, re-labelled" semantics for handoffs carrying `handoff.for_session`; notes/reviews and sender-only `session` remain role-wide. |
| `AGENTS.md` "Coordination channel" + `agf/templates/AGENTS.md` | One-line pointer: role addressing is the default; `handoff.for_session` narrows to a session; `agov handoff next --session <token>` filters. |

## Not a gate

`agov handoff next` / `coord visibility` are advisory discovery surfaces.
No `gate-self-test-rule` trigger (no MR-blocking check added). The
positive/negative fixture pair below is ordinary test discipline.

## Fixtures (required test discipline, not a gate-self-test)

1. **Backward compatible.** A handoff with no `for_session`: `agov handoff
   next <role>` output identical with and without `--session`.
2. **Scoped to me.** `for_session` == my token → preserve its existing ready/blocked state;
   another token adds `addressed elsewhere` without replacing that state.
3. **Scoped to another, still visible.** `for_session` == another token →
   my `agov handoff next <role>` (no `--session`) still lists it (role
   fallback); with my `--session` it is `addressed elsewhere`, **never
   absent**.
4. **Sender is not recipient.** A legacy handoff or note emitted by session A with top-level `session=A` and no `for_session` remains role-wide for B. A handoff authored by A with `for_session=B` is addressed normally to B. No session-token filter permits mutation over another active claim.
5. **Mixed order and blocked state.** Role-wide, matching and elsewhere items retain exactly the baseline order, status, blocking reasons and exit code. A matching blocked handoff stays blocked; an elsewhere ready handoff stays ready with an annotation. CLI/MCP evidence must agree.
6. Schema round-trip: record with/without `session`, handoff with/without
   `for_session`, all valid.

## Rule-projection screen

The structured six-surface manifest is included with this Gate 1 candidate:
`docs/reviews/rule-projection/20260910T155224Z-row5-s1-session-level-addressing.yaml`.
All dispositions are planned implementation obligations, not claims that
changes have already shipped. Use the existing coordination-publication rule
as normative owner; the previously cited coordination-message-rule path does
not exist in this tree. Update the adopter coordination-protocol page and
loader pointers together with implementation. Root loaders and this manifest
are project-local; the rule, schema, protocol page and adopter template ship.
No owner/rule implementation is made by this plan revision.

## Rollback

Optional-field-additive on the schema; `--session` is opt-in on two
commands. Revert the MR → role-only addressing, `session` remains
author provenance throughout. Absent fields = current semantics. No migration; no
existing record rewritten.

## Decision requested at Gate 1

Approve (a) retaining `session` as author provenance and adding
optional recipient-only `handoff.for_session`, (b) the "still listed, only re-labelled"
filter semantics (a session-scoped item is never hidden), (c) the
`coordination-publication-rule.md` paragraph and loader pointers. Then
coder-tester (Codex) implements; own Gate 2.

## Revision evidence

Codex author revision after transfer eetd/mb8k, addressing msq8 R56-E2.
The prior review is not independent clearance of this revision. Re-review
and Gate 1 remain pending. MCP handoff-next and visibility adapters must
expose the same optional recipient selection and unchanged default result;
CLI/MCP parity fixtures are part of acceptance. No new role activation.

## ssms correction round

IR-1: included the structured projection evidence at Gate 1. IR-2: recipient
selection is annotation-only; order, underlying state and exit code remain
unchanged. No independent clearance claimed by this author correction.

## Gate 1 approval recorded

Governor decision `20260910T190854Z-governor-reviewer-kr4b` approves the corrected scope following independent recheck `6r12`, with its disclosed advisory limits. Earlier pending-review language above is historical; this approval authorizes implementation, while exact-head implementation review and Gate 2 remain required. Decision published in MR !2678.
