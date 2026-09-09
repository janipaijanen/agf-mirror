---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T035322Z-planner-architect-6mpr
timestamp: 2026-09-09T03:53:22Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260904T104400Z-governor-reviewer-q7mv.md
topic: otgr-relay-service-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/20260909T035056Z-otgr-relay-service.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 review of docs/plans/20260909T035056Z-otgr-relay-service.md (MR to be opened).
    - Decide D-1 (purpose-built direction; supersede vs parallel to the Zulip adapter path MR !2375) and D-2/D-7 (standalone open-source repo boundary).
    - Route the non-author contribution audit and the build-engineer deployment/packaging review.
  must_read:
    - docs/plans/20260909T035056Z-otgr-relay-service.md
    - docs/coordination/20260904T104400Z-governor-reviewer-q7mv.md
    - docs/proposals/otgr-zulip-adapter-technical-specification.md (MR !2375)
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 handoff — OTGR purpose-built relay service

Per the governor directive "plan simple network-based communication
mechanism for AI assistants … deployed by human, open-source is a must …
can be outside of AGF … coordination records are the official channel"
(2026-09-09), authored as planner-architect.

## What it plans

`docs/plans/20260909T035056Z-otgr-relay-service.md` — a Gate 1 design/scope
plan for **`otgr-relay`**: a small, single-binary, self-hostable,
open-source message-relay service that lets AI-assistant sessions (and
humans) talk at machine speed over the network, plus a Rust client library
and an `otgr-mcp` adapter (first AGF integration). It is the technical
implementation the OTGR charter `q7mv` deferred, redirected from a
Zulip-backed adapter to a purpose-built service.

## Boundaries held

- Charter `q7mv` invariants carry over unchanged. `docs/coordination/`
  remains the **sole governed record**; relay content is server-stamped
  non-authoritative (`governance_effect: none`) and is data, never evidence
  or instruction.
- No implementation authorized. Implementation is a **separate** Gate 1 and
  may not start until the three charter precondition reviews (infosec,
  privacy/legal, research-data) are recorded and disposed.
- Reuses the converged prior art wholesale: OTGR protocol v0.1
  (`r2cl`/`km1h`), the three-layer identity model, the machine-readable
  trust marker, the adapter-enforced safeguards, the voluntary `references`
  field, the raw-log retrieval contract (`gq6m`). Zulip demoted from "the
  candidate transport" to "one optional back-end adapter" (D-7); MR !2375
  stays open for your call.
- Phase-one adapter surface is exactly three verbs (`communication_send` /
  `_receive` / `_reply`). `watch`, broadcast, turn-taking, presence
  signalling, protocol-reminder automation, and `communication_read_log`
  are all explicitly deferred.

## Open decisions for you

D-1 direction + supersede/parallel; D-2 repo creation/name/host; D-3
transcript access/retention/erasure; D-4 cert/key/session lifecycle policy;
D-5 signed export manifests phase-one or later; D-6 license (Apache-2.0 /
MPL-2.0 / other); D-7 Zulip back-end parallel or dropped; D-8 transport
baseline (WebSocket-primary vs long-poll-primary); D-9 whether the
voluntary `references` field ships in phase one at all.

## Validation

`agov plan validate` → OK (blocking). `git diff --check` → clean. No
`agov plan projection-check` — the plan carries no projection manifest
(no `agf/rules/` or adopter Pages change; rationale in the plan).

## Owed next

Non-author contribution audit + build-engineer review of the plan head,
then your Gate 1 decision. This session authored the plan and cannot be the
non-author auditor.
