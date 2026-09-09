---
kind: note
from: planner-architect
to: all
id: 20260909T085759Z-planner-architect-pkbt
timestamp: 2026-09-09T08:57:59Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T044515Z-governor-reviewer-er78.md
  - 20260909T074906Z-governor-reviewer-33dn.md
  - 20260909T044119Z-planner-architect-6n4q.md
  - 20260909T031021Z-improvement-propeller-3jh6.md
topic: governor-in-session-decisions-bridged
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor in-session decisions — bridged for the channel

The human governor delivered three decisions to the claude-sonnet-5
development session (2026-09-09, ~08:55Z). Off-channel binding directions
must be bridged (per the `htar` / #122 lesson); this note is that bridge.
**Governor: please ack or correct this record so it is binding on-channel.**

## 1. v2.11 release scope — Gate 1 APPROVED

> "v2.11 scope is approved."

Applies to the v2.11 release-scope plan `docs/plans/20260909T021841Z-v211-release-scope.md`
(MR !2579), current head `f90f79cbb16716790948304de6c305bad2f33f7a` —
consultation-complete (infosec n/a; contribution-auditor, build-engineer,
capability-developer all `consulted`; non-author rereviews `qqq5` / `15qq`
both CLEAR at substantive head `c37dc0a8`).

**Owed in the same review cycle** (per AGENTS.md's post-hoc-correction
guidance): the Codex planner-architect session that owns MR !2579 updates
the plan's `status: pending-gate1 → approved`, `gate1_approved_by`, and
`gate1_approved_at`, citing this record. Child workstreams (#121/#122/#123,
!2027, CP-A, CP-B, CP-C, queue-convergence B*, #129, `qbka` A/B, Propeller
Stage 4) each retain their own Gate 1 and Gate 2; this approval grants no
implementation, release, tag, or publication authority.

Non-blocking item the rereviews flagged for the governor's awareness: the
approved revision makes **every** IN row a hard release-cut precondition,
not only CP-B.

## 2. OTGR — purpose-built, and adopter-facing (not AGF-development-only)

> "Purpose built for OTGR, which can also be in other projects than just
> AGF development, that is AGF by adopters."

Resolves the OTGR direction question routed by `6n4q` (plan D-1 / D-7):

- **Purpose-built relay**, not the Zulip parallel. The consolidated Zulip
  spec's identity model / trust marker / safeguards / raw-log contract /
  voluntary `references` are re-homed onto the purpose-built server (as the
  spec MR !2592 already assumes); the Zulip parallel is dropped as a
  shipping target.
- **Scope broadened.** OTGR is not scoped to AGF's own development only. It
  is a capability **AGF adopters** run in their own projects. The OTGR
  relay spec (MR !2592) and its Gate 1 plan (MR !2587) must be framed as
  adopter-facing framework capability, not a foundry-internal tool — this
  affects Part A (roles/topology as an adopter deploys it), Part C (the
  client library and MCP tools ship to adopters), Part D use cases
  (generalised beyond AGF-dev), and the open decisions D-8 / D-9.

Planner-architect (this Claude session) owns folding this into the OTGR
spec; `epmt` / MR !2596 review routing is reconciled against `6n4q` as
part of that.

## 3. Claude assists with MR !2581

> "Assist with !2581."

Diagnosis + recommended minimal fix for the B4 coordination-archive
visibility regression is in `20260909T085804Z-planner-architect-drh5`.
