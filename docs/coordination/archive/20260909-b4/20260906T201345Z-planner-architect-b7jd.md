---
kind: note
from: planner-architect
to: all
id: 20260906T201345Z-planner-architect-b7jd
timestamp: 2026-09-06T20:13:45Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260903T045255Z-governor-reviewer-j4rg
topic: draft-release-horizon-ruw-through-v215
requires-ack: false
artifacts_owned: []
---

# Advisory release horizon: RUW through v2.15.0

The governor asked that the release sequence developed interactively from the
release under work (RUW) through v2.15.0 be preserved for future planning.
This note records that sequence and routes it to the active and future
planner-architect, governor-reviewer, and other AGF development roles.

## Authority boundary

This is an **advisory roadmap hypothesis**, not a Gate 1 plan, approved release
scope, version decision, milestone mutation, implementation claim, or release
authorization. Every version number and candidate item below remains
provisional. Each release needs its own current inventory, Gate 1 plan,
objective readiness evidence, Gate 2 decision, and explicit release/tag
authorization.

The approved queue-convergence and roadmap-authority plan in
`docs/plans/20260903T043752Z-queue-convergence-and-roadmap-authority.md` names
`docs/roadmap.md` as the eventual single governed internal roadmap owner and
`agf/docs/site/roadmap.md` as its adopter-facing projection. This note is an
input to that owner; it must not become a competing roadmap. Accepted portions
may be transferred to `docs/roadmap.md` only through the governed implementation
of that plan. The public projection follows afterward and must not assert more
than the owner.

The current advisory release-scope inventory is MR !2439,
`docs/reference/post-v290-release-scope-inventory.md`. It remains the near-term
inventory input and does not gain Gate 1 authority from this note.

## Release horizon

### RUW / candidate v2.10.0 — Autonomous development operation

**Theme:** make every AGF development assistant run the layered poll/action
cycle, keep release work moving to the next real gate, and launch the bounded
improvement-propeller/capability-review loop.

**Candidate inputs:** MR !2405 and its implementation, MR !2438, issues
#120 and #124-#127, plus only the additional items selected by the eventual
RUW Gate 1 release plan.

**Dependencies:** !2405's exact-head objective preconditions and implementation
complete before Propeller projections depend on it; capability-developer review
precedes Propeller launch; every included MR still needs exact-head acceptance.

**Flagship demonstration:** a development AI cold-starts, discovers all relevant
coordination and forge work, advances the highest-priority authorized release
item, coalesces timer triggers while busy, and uses idle capacity without
manufacturing scope.

**Excluded by default:** coordination-protocol redesign, broad compliance work,
and unplanned radar experiments.

### Candidate v2.11.0 — Lifecycle Convergence

**Theme:** make work state unambiguous, preserve unique evidence, and retire the
historical queue without losing live commitments.

**Candidate inputs:** issues #75 and #121-#123; reconciliation of MRs !2027,
!1817, and !1617; provenance-bound supersession, session ownership/claiming,
plan/MR collision detection, lifecycle transitions, and deterministic queue
semantics.

**Dependencies:** RUW's poll/action contract must be stable; the active
plan-status/lifecycle handoff and existing queue-convergence plan must be
reconciled into one atomic Gate 1 work order rather than duplicated.

**Flagship demonstration:** every open MR and issue receives an evidence-backed
terminal or active disposition; unpublished coordination is preserved first;
only current implementation vehicles and explicitly retained roadmap items
remain open.

**Preferred acceptance target:** zero stale MRs and zero unexplained issues.
If the governor selects a truly empty issue tracker, future candidates move to
the governed roadmap or radar before closure. Age alone is never closure
evidence, and branch deletion remains a separate authorization.

**Excluded by default:** blanket closure, evidence loss, and treating forge
state changes as release content without a shipped lifecycle mechanism.

### Candidate v2.12.0 — Agent-native Governed Operations

**Theme:** make AGF safely operable end-to-end through `agov` and `agov-mcp`,
not merely inspectable through them.

**Candidate inputs:** issues #32-#35, #37, #49, #111, #113, and #116. Candidate
surfaces include governed coordination/handoff write paths, concurrency and
identity semantics, exact-artifact gate enforcement, MCP packaging/discovery
and commissioning, governed assistant-input provenance, upgrade preservation,
and measured CLI-versus-MCP token ROI.

**Dependencies:** v2.11 lifecycle, supersession, and session-claim primitives;
MR !2438's learning supplies evidence but does not itself fix MCP exposure.

**Flagship demonstration:** a fresh Codex or Claude session installs the
release, discovers `agov-mcp`, resumes, polls, claims, posts, completes, hands
off, validates, and presents exact-head Gate 2 evidence without manually
editing coordination files or using generic shell commands for supported MCP
operations.

**Excluded by default:** unsafe remote mutation, identity claims without
attestation evidence, and token-saving claims without measured baselines.

### Candidate v2.13.0 — Portable Assurance and Buyer Verification

**Theme:** turn repository-native governance evidence into portable,
independently verifiable assurance packages.

**Candidate inputs:** issues #39-#43, #45-#47, and #108, subject to scope
reduction at Gate 1. Candidate surfaces include a declared gate-chain threat
model, explicit human accountability, reviewer-independence evidence,
incident-to-governance evolution, reusable due-diligence bundles, buyer-side
verification, evidence mappings, amendment/sunset state, and a removability
drill.

**Dependencies:** v2.12 supplies trusted execution identity, write paths, and
provenance. Public assurance claims require explicit maturity and evidence
status and must expose gaps rather than infer success.

**Flagship demonstration:** an unfamiliar project is commissioned, exports a
signed provenance-linked assurance bundle, and an independent reviewer verifies
its gates, decisions, revisions, threats, incidents, control evidence, gaps,
and removability without access to the original AI sessions.

**Excluded by default:** automatic certification or legal-compliance claims.
Issue #44's CRA entity/legal-person determination stays outside AI governance;
AGF may record an externally supplied determination and provenance but must not
manufacture it.

### Candidate v2.14.0 — Federated and Sovereign Governance

**Theme:** allow independently governed repositories, organizations, models,
and execution environments to collaborate without surrendering local gate
authority or requiring a central AGF service.

**Candidate inputs:** issues #38, #77, #91, #98, #99, #103, and #104; a future
Gate 1 disposition may also mature OTGR as an optional adapter for importing
raw discussion evidence. Candidate primitives include a federation manifest,
host/project/tenant identity, trusted public keys and CAs, local governor
authority, accepted evidence roots, bounded cross-project handoffs, revocation,
and offline transport.

**Dependencies:** v2.13 portable assurance is the exchange format; v2.12
identity and execution primitives are the operational base. Technical
InfoSec/privacy boundaries require their governed specialist route before
implementation.

**Flagship demonstration:** three independently governed repositories use both
cloud and local-model agents, private and shared runners, signed cross-project
handoffs, offline evidence exchange, and participant revocation while the full
federated assurance graph remains reconstructible from released artifacts.

**Invariant:** federation shares evidence and requests, not authority. Each
receiving project applies its own rules. Git remains sufficient when optional
live transports are unavailable.

**Excluded by default:** central-service dependence, cross-tenant authority
inheritance, and treating a chat/transport server as a gate authority.

### Candidate v2.15.0 — Resource-aware and Adaptive Governance

**Theme:** make governance proportionate to risk, evidence, cost, attention,
and observed failure without weakening hard authority boundaries.

**Candidate inputs:** issues #48, #53, #60, #62, #82, #100, #107, #117, #119,
and #128 after re-observation. Candidate surfaces include governed time/token
budgets, reviewer-attention economics, context compression with evidence
fidelity, material caveat/blind-spot signals, learning-candidate discovery,
explicit state-machine semantics, consolidated cadence, and measured release
feedback latency.

**Dependencies:** v2.14 supplies multi-project observations; v2.13 supplies
portable evidence; v2.12 supplies trustworthy telemetry and operations.
Adaptive behavior remains advisory until a named rule explicitly grants any
mechanism blocking authority.

**Flagship demonstration:** AGF selects a proportionate governed workflow for
several risk classes, stays within declared time/token/attention budgets,
preserves exact decision evidence through compression, escalates material
blind spots, and proves that optimization cannot bypass Gate 1, Gate 2, human
authority, or a security stop-and-plan boundary.

**Excluded by default:** autonomous weakening of gates, hidden model-based risk
scores, unreviewed self-modification, and treating cost reduction as evidence
of governance quality.

## Planning use

At each release boundary, planner-architect should re-observe the repository,
coordination channel, issues, open MRs, released capabilities, and external
evidence before proposing scope. Items may move, split, merge, or disappear.
The value of this horizon is the dependency-shaped product narrative, not a
promise that every listed issue ships under the displayed version.

The intended progression is:

1. operate autonomously within gates;
2. make lifecycle state trustworthy;
3. expose safe agent-native operations;
4. export independently verifiable assurance;
5. federate without exporting authority;
6. optimize resources and learning without weakening governance.

