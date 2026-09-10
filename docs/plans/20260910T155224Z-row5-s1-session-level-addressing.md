---
id: 20260910T155224Z-row5-s1-session-level-addressing
status: pending-gate1
topic: row5-s1-session-level-addressing
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This slice promotes the coordination schema's existing `session`
      field from "audit only" to an optional routing key and adds
      `handoff.for_session`. InfoSec must confirm at Gate 1 that a
      session token is advisory routing only — it never grants authority,
      never gates a decision, and cannot be spoofed to steal or redirect
      another session's obligation (a wrong `for_session` value at worst
      makes an item show as "addressed elsewhere"; the item is still
      visible without the `--session` filter). No credential surface; the
      token is an opaque UUIDv7 the assistant already writes.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Coordination-protocol addressing change. Contribution-auditor
      confirms the field stays optional (absent = current role-only
      behaviour, fully backward compatible), that `agov handoff next` and
      `agov coord visibility` default behaviour is unchanged without
      `--session`, and that a session-scoped item is never *hidden* — only
      re-labelled — so a cold-starting session cannot miss work by not
      passing a token.
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
`handoff` to a session. S1 makes `session` routable and adds the handoff
side.

## Change

| File | Change |
|---|---|
| `agf/schemas/coordination-message.schema.json` | `session` description updated: "Optional. Audit, **and an advisory routing key** — see `handoff.for_session`." Add optional `handoff.for_session` (string, UUIDv7; **not** in `handoff.required`) — the session a handoff is addressed to, when narrower than `next_agent`'s role. |
| `agf/rules/always/coordination-message-rule.md` | One paragraph: a role-addressed record is answerable by any holder of that role unless `for_session` (handoff) or `session` (as an addressing hint on a `note`/`review` reply) narrows it; a session that cannot confirm an item is "theirs" still may act, but SHOULD note the ambiguity (the current `rkbk`/`mhq3` pattern). Advisory, not blocking. |
| `agov handoff next <role>` (`tooling/rust/crates/agov/src/main.rs`) | New optional `--session <token>`. Without it: unchanged. With it: a handoff whose `for_session` matches → normal; `for_session` set to a *different* token → shown as `addressed elsewhere` (still listed, de-prioritised, not hidden); `for_session` absent → shown normally (role-wide). |
| `agov coord visibility` | Same `--session <token>` filter, same "still listed, re-labelled" semantics for records carrying a `session`/`for_session` hint. |
| `AGENTS.md` "Coordination channel" + `agf/templates/AGENTS.md` | One-line pointer: role addressing is the default; `handoff.for_session` narrows to a session; `agov handoff next --session <token>` filters. |

## Not a gate

`agov handoff next` / `coord visibility` are advisory discovery surfaces.
No `gate-self-test-rule` trigger (no MR-blocking check added). The
positive/negative fixture pair below is ordinary test discipline.

## Fixtures (required test discipline, not a gate-self-test)

1. **Backward compatible.** A handoff with no `for_session`: `agov handoff
   next <role>` output identical with and without `--session`.
2. **Scoped to me.** `for_session` == my token → `ready` for me,
   `addressed elsewhere` for another token.
3. **Scoped to another, still visible.** `for_session` == another token →
   my `agov handoff next <role>` (no `--session`) still lists it (role
   fallback); with my `--session` it is `addressed elsewhere`, **never
   absent**.
4. Schema round-trip: record with/without `session`, handoff with/without
   `for_session`, all valid.

## Rule-projection screen

`docs/reviews/rule-projection/20260910T155224Z-row5-s1-session-level-addressing.yaml`
— owner `always/coordination-message-rule.md` (`updated`); adopter
explanation the coordination-protocol Pages page (`existing pointer
sufficient` — addressing is already described; add the session note if the
reviewer judges it needed); runbook `not applicable`; AI loaders
`AGENTS.md` + `agf/templates/AGENTS.md` (`updated`, one-line pointer),
`.claude/CLAUDE.md` `not applicable`; validation/drift `advisory only`;
distribution — schema + `coordination-message-rule.md` +
`agf/templates/AGENTS.md` ship in the AGF artifact.

## Rollback

Optional-field-additive on the schema; `--session` is opt-in on two
commands. Revert the MR → role-only addressing, `session` back to
audit-only wording. Absent fields = current semantics. No migration; no
existing record rewritten.

## Decision requested at Gate 1

Approve (a) promoting `session` to an advisory routing key and adding
optional `handoff.for_session`, (b) the "still listed, only re-labelled"
filter semantics (a session-scoped item is never hidden), (c) the
`coordination-message-rule.md` paragraph and loader pointers. Then
coder-tester (Codex) implements; own Gate 2.
