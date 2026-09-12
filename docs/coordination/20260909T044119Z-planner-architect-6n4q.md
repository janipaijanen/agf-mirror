---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260909T044119Z-planner-architect-6n4q
timestamp: 2026-09-09T04:41:19Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T035322Z-planner-architect-6mpr.md
  - 20260904T104400Z-governor-reviewer-q7mv.md
topic: otgr-relay-api-comms-spec-companion-to-6mpr
requires-ack: false
artifacts_owned:
  - docs/proposals/otgr-relay-api-and-comms-spec.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# OTGR relay API + technical comms spec — companion to the Gate 1 plan

Per the governor request "draft specifications for the API and technical
comms from AI assistant to chat server, describe use cases and link
use-cases to api and tech specs sections" (2026-09-09), authored as
planner-architect.

`docs/proposals/otgr-relay-api-and-comms-spec.md` — **MR !2592**. It is the
technical detail the Gate 1 plan `20260909T035056Z-otgr-relay-service.md`
(MR !2587, handoff `6mpr`) explicitly defers.

## Contents

- **Part A** roles/topology — git = authoritative async plane; relay =
  optional non-authoritative real-time plane; client dials out, no inbound
  listener on assistant hosts (matches the topology you described:
  assistants/roles on independent hosts, logically connected via git,
  additionally connectable to a chat server).
- **Part B** technical communications — TLS transport (WebSocket primary +
  HTTP long-poll fallback), connection lifecycle, three-layer auth
  handshake, event model + server-attached envelope (author-immutable
  trust marker), ordering/at-least-once/idempotency/resume, safeguards,
  typed errors, capability negotiation.
- **Part C** API surface — MCP tools (`send`/`receive`/`reply`; `read_log`
  and `watch` deferred), native client library, server HTTP/WS endpoints a
  relay MUST expose.
- **Part D** 10 use cases (UC-1..UC-10), each linked to the exact B* and
  C* sections it exercises, plus a coverage matrix.
- **Part E** open decisions — plan D-1..D-9 plus spec-level S-1..S-7.
- **Part F** conformance hooks.

## Boundaries held

- Specification for review only. Authorizes no implementation, deployment,
  or transport selection — that stays gated on the three charter
  precondition reviews and the implementation Gate 1.
- Charter `q7mv` invariants unchanged: `docs/coordination/` is the sole
  governed record; every relay event is server-stamped
  `governance_effect: none`; `role_label` is display-only, never authority.
- Adopts the identity model, trust marker, adapter-enforced safeguards,
  raw-log retrieval contract, and voluntary `references` field from the
  consolidated Zulip spec (`lunk`/`h8vq`/`gq6m`/`ms1q`/`t34q`), re-homed
  onto the purpose-built server.

## Owed next

Review of both MR !2587 (plan) and MR !2592 (spec) together; a governor
call on plan D-1/D-7 (purpose-built direction; Zulip parallel or dropped)
and D-8/D-9. Non-author contribution + build-engineer review of the spec if
you want it before the direction call.
