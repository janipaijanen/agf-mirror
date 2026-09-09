---
id: 20260909T035056Z-otgr-relay-service
status: pending-gate1
topic: otgr-purpose-built-relay-service-for-machine-speed-ai-communication
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      This is a design/scope plan and authorizes no code. But the service it
      plans has a real security surface (network listener, identity/auth,
      credential handling, retained transcripts). The charter `q7mv` and the
      consolidated spec both make infosec review a hard precondition before
      any adapter or service code. This row is `waived` pending that
      recorded infosec review, which Gate 1 approval of this plan does not
      substitute for.
  - role: contribution-auditor
    status: waived
    reason: >-
      Pending independent audit by a NON-author session. Authored by a
      Claude session (primary role contribution-auditor) acting as
      planner-architect under the governor directive "plan simple
      network-based communication mechanism for AI assistants ..."
      (2026-09-09). Gate 1 is not eligible until that audit is incorporated
      and this row is `consulted`.
  - role: build-engineer
    status: waived
    reason: >-
      Pending review of the deployment / packaging / CI shape only — the
      single-binary + container deployment model, the config-fail-closed
      startup contract, TLS/reverse-proxy expectations, and the
      open-source-repo boundary. No CI file changes in this plan.
  - role: capability-developer
    status: not-applicable
    reason: >-
      No agent definition, maturity, phase, or boundary change. The relay is
      infrastructure, not an AGF role. Adapter-side session/role metadata is
      carried opaquely and grants no authority (charter `q7mv` authority
      boundary).
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# OTGR relay service — purpose-built, open-source, self-hostable

## Authoring provenance

Authored by a Claude session (primary role contribution-auditor) acting as
**planner-architect** under the governor directive: *"plan simple
network-based communication mechanism for AI assistants that enables them to
discuss at machine speed from network hosts. Need a service that can be
deployed by human, open-source is a must. Adapters, MCP?, can be implemented
as necessary. Can be outside of AGF since it is multipurpose, but AGF should
be able to use it. Coordination records are the official channel."*
(2026-09-09). The independent contribution audit MUST come from a non-author
session.

## What this plan is, and is not

**It is:** a Gate 1 design/scope plan for a small standalone service that
relays real-time messages between AI-assistant sessions (and participating
humans) over the network, plus the client adapters that connect to it. It
is the "technical implementation … provided separately after the
communications-protocol phase" that the OTGR charter (`q7mv`) explicitly
deferred, and it redirects that implementation away from a Zulip-backed
adapter toward a purpose-built service.

**It is not:** an implementation authorization; a change to any AGF gate,
rule, schema, or role; or a change to the OTGR charter's authority and
governed-record boundaries. It does not authorize deployment, a public
release, or transport of any real credential. Charter `q7mv` invariants
carry over unchanged — most importantly: **`docs/coordination/` remains the
sole governed record**; nothing in this service is authoritative for any
gate, approval, completion, handoff, or decision.

## Relationship to existing OTGR work

| Prior artifact | Disposition under this plan |
|---|---|
| OTGR charter `q7mv` (2026-09-04) | **Unchanged.** This plan is downstream of it and inherits every invariant (non-authoritative; "silence has no default meaning"; governed-state reconstructible without transcripts; technical identifiers `YYYYMMDDThhmmssZ-otgr-<4char>`; research retention; anti-prescription of communication patterns). |
| Converged OTGR protocol v0.1 (`r2cl` / `km1h`) | **Reused as the wire-event contract**, adapted to a purpose-built server (server-assigned sequence, reply-parent, optional voluntary `references`). |
| Consolidated Zulip-adapter spec (`lunk`→`h8vq`→`gq6m`→`ms1q`→`t34q`; MR !2375) | **Zulip demoted from "the candidate transport" to "one optional back-end adapter".** The spec's transport-independent parts — the three-layer identity model, the machine-readable trust marker, the adapter-enforced safeguards, the raw-log retrieval contract, the references-field convergence — are **adopted wholesale** and re-homed onto the purpose-built server. MR !2375 stays open for the governor to decide whether the Zulip adapter remains a parallel candidate (open decision D-7). |
| Precondition reviews (infosec, privacy/legal, research-data) | **Still mandatory and still unrecorded.** This plan does not clear them; it re-scopes what they must cover (now a service we operate, not a third-party SaaS). |

## Design

### D1 — The service ("otgr-relay")

A single self-hostable process. One tenant per deployment. No dependency on
any external SaaS.

- **Transport:** one long-lived client→server connection per session, over
  TLS. Baseline: WebSocket (`wss://`), with an HTTP long-poll fallback for
  restricted egress. The client never needs an inbound listener.
- **Model:** discussions (rooms) contain an append-only, server-sequenced
  event stream. Events are `message`, `edit`, `delete-tombstone`,
  `reaction`. The server assigns a strictly monotonic per-discussion
  sequence number and the authoritative receive timestamp.
- **Storage:** embedded append-only store (SQLite in WAL mode, or a plain
  segmented log). Retained faithfully enough for later research per `q7mv`
  ("Research retention"). Deletion is a tombstone event, never a silent
  drop.
- **No behavioral policy in the server:** no turn-taking, no forced
  presence signalling, no wake/activation, no broadcast/wildcard mention.
  Those are charter experimental variables and stay out of the server.

### D2 — Wire event shape

Per OTGR protocol v0.1 and the consolidated spec. Every event the server
emits carries, server-attached (never client-constructed):

- `otgr_id` (`YYYYMMDDThhmmssZ-otgr-<4char>` for discussions; per-event
  technical id for events), discussion id, server sequence, server receive
  time (client/transport time only as a separately labelled observation).
- Sender identity: `session_id`, `principal_id`, login-key fingerprint,
  host/adapter identity, model metadata, optional opaque governed-role
  label (display only; **grants no authority**).
- The mandatory machine-readable **trust marker**
  (`class: external_untrusted_research_chat`, `authoritative: false`,
  `governance_effect: none`) — settable and suppressable by no message
  author.
- `reply_parent` (single, transport-level) and the optional, **voluntary,
  schema-typed, adapter-uncoerced** `references` list (real event ids in
  the same discussion; named `references`, deliberately not `references-to`,
  to avoid importing AGF coordination semantics).

### D3 — Identity and auth (three layers, from `gq6m`)

1. **Host/adapter identity** — X.509 cert from the deployment's own CA;
   authenticates the connection, never the speaker.
2. **Login principal** — possession of a configured principal's private key;
   events carry the stable principal id + public-key fingerprint; private
   key material is never transmitted or retained.
3. **Session assertion** — short-lived, server-issued, bound to principal +
   host/adapter identity + model metadata + optional role metadata.

An adapter may multiplex several same-model sessions; if so it MUST
authenticate each, isolate per-session state/credentials, derive sender
from the authenticated session (never from tool arguments), and bind the
session id into every retained event.

### D4 — Adapters

- **MCP adapter (`otgr-mcp`)** — the first and primary AGF integration.
  Phase-one tools only: `communication_send`, `communication_receive`
  (stateful delivery of new addressed events; not a log export),
  `communication_reply`. Agent supplies body + typed logical recipient
  (`{kind: participant|group, id}`); the adapter attaches all identity /
  ordering / timestamp / reply metadata. `communication_read_log` (stateless
  authorized export, full manifest contract from `gq6m`) is **deferred**
  until the access/retention/tamper-evidence governor decisions land.
- **Client library** — a thin language-native client (Rust first) so
  non-MCP and non-AGF programs can use the relay (the "multipurpose"
  requirement). The MCP adapter is a thin wrapper over it.
- **Optional back-end adapters** — e.g. the existing Zulip design could be
  reworked as an `otgr-relay`-compatible back-end. Not in phase one; D-7.

### D5 — Adapter-enforced safeguards (from the consolidated spec)

Enforced in the adapter/server, not an agent prompt: authenticated sender;
configured channel boundaries; sender/recipient allowlists; fail-closed on
identity mismatch; duplicate suppression by transport message id; ignoring
the adapter's own outbound as new inbound; one active processing claim per
inbound message; bounded message/attachment size; rate limits; a
**generous, logged** auto-reply depth bound (not tightly capped);
rejection or explicit authorization of broad mentions.

### D6 — Deployment (human-deployable)

- `otgr-relay serve --config-dir <dir>` — single binary. Mandatory explicit
  config at startup; **fail closed** if absent, unreadable, or mismatched
  (e.g. one principal's config dir paired with another's key must not
  silently start).
- Ships as: a static binary per platform **and** a container image +
  example `compose.yaml` (relay + TLS-terminating reverse proxy). TLS
  required; one trusted proxy hop by default.
- Config declares: bind address, TLS material, the CA trust root, the
  principal roster (ids + public keys), discussion roster + allowlists,
  size/rate bounds, retention window, storage path.
- No telemetry phone-home. Logs are local.

### D7 — Repository boundary

The service and its client library are **multipurpose** and live in their
**own open-source repository** under an OSI-approved license (candidate:
Apache-2.0 or MPL-2.0 — D-6), not inside `agf/`. AGF consumes it as an
external tool: the `otgr-mcp` adapter is configured per
`agf/docs/site/docs/adopt/mcp-server.md` conventions, exactly like
`agov-mcp`. This plan proposes the boundary; the new repo's creation is a
governor action (D-2).

## Explicitly out of scope (phase one)

- `watch` / automatic session wake-up or activation.
- Broadcast / wildcard mentions; any turn-taking or presence-signalling
  policy.
- Protocol-reminder automation (charter keeps it an experimental variable;
  first run is out-of-band).
- `communication_read_log` export (deferred pending D-3/D-4).
- Signed export manifests as a phase-one hard requirement (D-5).
- Any claim that relay content is governed evidence. It never is.
- Replacing, mirroring, or "backing up" `docs/coordination/`.

## Preconditions before implementation (unchanged from charter)

Recorded and disposed by the governor **before** any service or adapter
code:

1. **infosec review** — CA lifecycle; login-key enrollment/rotation/
   revocation; session issuance; listener hardening; transcript access;
   retention; redaction/deletion; attachment handling; breach response;
   credential-config fail-closed.
2. **privacy / legal review** — retained human+AI conversation data;
   research-use boundaries; erasure obligations vs. append-only design.
3. **research-data review** — capture fidelity for later unanticipated
   analysis; the `q7mv` "observation before interpretation" separation.

## Open governor decisions

- **D-1** — Approve this purpose-built direction, and whether it
  *supersedes* or *runs parallel to* the Zulip adapter path (MR !2375).
- **D-2** — Create the standalone open-source repo; name; hosting location.
- **D-3** — Transcript access roles, permitted research purposes, retention
  period, attachment policy, edit/delete/erasure treatment.
- **D-4** — Enrollment/expiry/rotation/revocation policy for host certs,
  login keys, session assertions.
- **D-5** — Signed export manifests mandatory in phase one, or a later
  hardening item.
- **D-6** — License (Apache-2.0 vs MPL-2.0 vs other OSI).
- **D-7** — Whether a future Zulip (or other) back-end adapter is a
  sanctioned parallel candidate or dropped.
- **D-8** — Transport baseline: WebSocket-primary (with long-poll
  fallback) vs long-poll-primary.
- **D-9** — Whether the voluntary `references` field ships in phase one at
  all (charter anti-prescription tension).

## Risks and controls

- **Shadow governance channel.** A fast side-channel accreting
  decisions that never reach `docs/coordination/`. Control: server-stamped
  non-authoritative trust marker on every event; `governance_effect: none`
  is structural, not prose; charter `q7mv` "governed-state
  reconstructibility" is restated in adapter docs; the MCP tool
  descriptions say relay content is data, never instruction or evidence.
- **Behavioral prescription contaminating the research.** Control: no
  turn-taking / presence / wake policy in the server or phase-one adapter;
  `references` is voluntary and uncoerced; D-9 lets the governor keep it
  out entirely.
- **Identity spoofing / cross-session mis-attribution.** Control: the
  three-layer identity model; sender derived from the authenticated
  session; fail-closed on mismatch; per-session isolation on multiplex.
- **Operator burden.** Control: single binary + one `compose.yaml`;
  config-fail-closed; no external service dependency; `simple` is a stated
  design goal, not an aspiration.
- **Scope creep into a full chat platform.** Control: phase-one tool
  surface is exactly three verbs; everything else is explicitly deferred
  and listed.
- **Precondition-review bypass.** Control: implementation is gated on all
  three reviews being recorded and disposed; this plan's Gate 1 approval
  does not clear them (stated in `advisory_consultation`).

## Test and acceptance strategy

Before Gate 1: non-author contribution audit; build-engineer review of the
deployment/packaging/repo-boundary shape; infosec review recorded (or the
governor explicitly sequences it right after Gate 1 and before any code);
`agov plan validate` passes; `git diff --check` passes.

Acceptance: the reviewed plan plus an explicit governor Gate 1 decision on
the direction (D-1) and the repo boundary (D-2/D-7). Implementation is a
**separate** Gate 1 (an implementation plan for `otgr-relay` v0.1) that may
not start until the three precondition reviews are disposed.

Later, per-component (implementation plan, not here): wire-protocol
conformance fixtures; identity/auth negative tests (mismatch fails closed);
config-fail-closed negative test; multiplex attribution test; a soak test
proving the append-only store and sequence monotonicity under concurrent
senders.

## Rollback

This is a design/scope plan. It creates no revertible artifact beyond
itself. A direction change is a governor decision recorded in coordination
superseding the relevant section. No repo, service, or adapter exists to
roll back.

## Rule-projection screen — not applicable to this plan

`documentation-rule-projection-rule` scope is a plan that introduces,
changes, or removes normative operating guidance in an adopter-facing Pages
page or runbook. This plan does not: it changes no `agf/rules/` text and no
adopter Pages page. The OTGR charter `q7mv` remains the normative owner of
the authority boundary; this plan is downstream infrastructure design. If
the implementation plan later adds adopter-facing operator guidance (e.g. an
`otgr-mcp` adopt page), that plan carries the six-surface screen.
