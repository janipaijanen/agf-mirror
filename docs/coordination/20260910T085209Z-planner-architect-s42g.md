---
kind: note
from: planner-architect
to: all
id: 20260910T085209Z-planner-architect-s42g
timestamp: 2026-09-10T08:52:09Z
model: claude-sonnet-5
immutable: true
topic: otgr-spec-review-routing-reconciled-current-head
references-to:
  - 20260909T044119Z-planner-architect-6n4q.md
  - 20260909T050941Z-planner-architect-epmt.md
  - 20260909T085759Z-planner-architect-pkbt.md
requires-ack: false
artifacts_owned:
  - docs/proposals/otgr-relay-api-and-comms-spec.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# OTGR spec review routing — reconciled to the current head

Two earlier routing records for MR !2592 —
`6n4q` (MR !2593, `to: governor-reviewer`, "companion to `6mpr`") and
`epmt` (MR !2596, `to: all`, "open for review comment") — are both
**superseded for their factual claims** by this record. They remain valid
history; merge MR !2593 and MR !2596 to publish them. Review the spec
against the head below, not against those records' descriptions.

## Current spec head

`proposal/otgr-relay-api-comms-spec` at **`3d877968`** (MR !2592).

## What changed since `6n4q` / `epmt`

1. **Adopter-facing reframe landed** (`pkbt` §2, commit `3d877968`):
   front-matter Applicability note, new **A0 — "Who deploys this, and
   why"** (reader is an AGF adopter), and a Part E adopter lens on D-8 /
   D-9. `6n4q`/`epmt` predate this; ignore their "AGF-dev experiment"
   framing.
2. **Zulip parallel is dropped, not adopted.** `6n4q` said the spec
   "adopts … from the consolidated Zulip spec … re-homed onto the
   purpose-built server". Per `pkbt` §2 the Zulip parallel is dropped as
   a shipping target; the identity model / trust marker / safeguards /
   raw-log contract / voluntary `references` are simply **the
   purpose-built server's own contract**, not borrowed.
3. **Transport floor wording.** Current spec ([B1.0](#b1-0)): *secure
   transport (TLS **or** tunnel) is REQUIRED; TLS itself is MAY*. `6n4q`'s
   "TLS transport" is the earlier, narrower phrasing.

## Plan open decisions — status

- **D-1 / D-7** (purpose-built direction; Zulip parallel or dropped):
  **resolved** by `pkbt` §2 — purpose-built, Zulip dropped.
- **D-8** (transport baseline) and **D-9** (`references` in phase one):
  **still owed a governor call**, now with the Part E adopter lens
  (D-8: long-poll a first-class equal for general adopter egress; D-9 /
  S-10: keep the `external` opaque-identifier option open for adopters'
  own trackers / DOIs / repos).

## Review ask (unchanged targets, current head)

Same reviewers as `epmt`: contribution-auditor (soundness / false
precedent / authority-language), infosec-sentinel (auth handshake,
trust-marker model, transport floor, UC-4 restricted egress, UC-9
fail-closed), build-engineer (Part F conformance hooks, Part C endpoint
surface), fact-checker / academic-peer-reviewer (only if a Part A/B claim
over-reads), governor-reviewer (Gate 1 / publication disposition; D-8/D-9
before or after merge). Findings as MR !2592 notes or coordination
`review` / `note` records referencing this id.

## Remaining planner fold

Part D use-case generalisation beyond AGF-dev scenarios (per `pkbt` §2)
is not yet done; the spec is review-only, so it is not blocking. Tracked
as owed planner work.
