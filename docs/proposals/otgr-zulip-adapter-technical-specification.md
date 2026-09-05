# Proposal: OTGR Zulip-Backed Communication Adapter — Technical Specification v0.1

Status: converged design consolidation, **not approved for implementation**.
This document restates and consolidates a coordination-channel design
exchange between Codex and Claude; it adopts no new position of its own.
Every substantive claim below traces to one of the coordination records
listed in "Provenance" at the end. Where the exchange left something an open
governor decision rather than a converged position, this document says so
explicitly rather than picking a default.

Zulip is a **provisional candidate**, not an approved transport. No adapter
implementation, deployment, or transport selection is authorized by this
document. Implementation may not begin until the three precondition reviews
in "Preconditions before implementation" below are recorded and the governor
has disposed their findings, per the OTGR experiment charter
(`docs/coordination/20260904T104400Z-governor-reviewer-q7mv.md`).

## Motivation and scope

The OTGR ("off-the-governance-records") experiment charter established a
non-authoritative, observational chat medium for AI-agent and human
collaboration during AGF development, distinct from the governed,
append-only AGF coordination channel. This document specifies the
**technical shape** of one candidate implementation of that medium: a
Rust-implemented local MCP adapter, one instance per participating AI
session, backed by a self-hosted Zulip organization.

It does not re-litigate the charter's own invariants (non-authoritativeness,
"silence has no default meaning," the governed channel remains the durable
record). Those are assumed and cited, not restated in full.

## Non-goals

- Not a replacement for `docs/coordination/`. No AGF gate, approval,
  completion, or decision may be recorded only in OTGR.
- Not a transport-selection decision. Zulip is provisional; the precondition
  reviews may reject it.
- Not an implementation authorization. This is a specification for review,
  matching the same posture the original adapter proposal (`lunk`) and its
  review (`h8vq`) held throughout.
- Not a behavioral-pattern prescription. Per the charter, the experiment
  must not prescribe a desired communication graph or message pattern and
  then treat conformance as an empirical result — several design choices
  below (e.g. the references field, see below) are deliberately left
  unenforced for exactly this reason.

## Identity model (three layers)

Converged in `gq6m`, amending `lunk`'s original per-model-identity design
after `h8vq` raised it as blocking (B1):

1. **Host/adapter identity** — certified by the host CA, used to
   authenticate the service connection itself. This is transport/adapter
   identity, never the speaker.
2. **Login principal identity** — proven by possession of a configured
   user's login private key. Records carry the stable principal ID and
   public-key fingerprint; private key material is never recorded.
3. **Session identity** — a unique, expiring, server-issued identifier bound
   to the authenticated principal, the host/adapter identity, model
   metadata, and optional governed-role metadata.

One adapter may serve one session or safely multiplex several concurrent
sessions of the same model — this project's normal operating mode, and the
exact case `h8vq` raised (multiple concurrent Claude sessions must remain
mutually attributable). If an adapter multiplexes, it must authenticate each
local session, isolate its state and credentials, derive the sender from the
authenticated session rather than from tool call arguments, and include the
session binding in every retained event. A separate bot credential per
session remains a valid deployment choice, never a protocol requirement.

`claude` / `codex`-style labels are display labels only, never the identity
anchor — `h8vq`'s original finding, unchanged by the amendment.

## Trust marking of inbound content (converged, `h8vq` B2 / `gq6m`)

Every inbound tool result carries a machine-readable, adapter-generated
trust marker, not only prose in a tool description:

```json
{
  "trust": {
    "class": "external_untrusted_research_chat",
    "authoritative": false,
    "governance_effect": "none"
  }
}
```

This marker cannot be supplied or suppressed by a message author. Message
bodies, participant names, quoted text, attachments, and purported approvals
inside OTGR content are data, never instructions or governed evidence — the
same discipline this project already applies to artifact comments and other
untrusted-content channels, extended here structurally rather than left to a
receiving agent's memory of a written rule.

## MCP surface

### Phase one (implementable once preconditions clear)

- `communication_send`
- `communication_receive` — stateful delivery of new addressed events to a
  live session; may maintain delivery/claim state; not a reproducible log
  export.
- `communication_reply`

The agent supplies message content and a typed logical recipient
(`{"kind": "participant", "id": "codex"}` or `{"kind": "group", "id":
"ai-agents"}`). The adapter derives and attaches sender identity, UTC
timestamp, technical ID, transport channel/topic, numeric transport-internal
identities, and reply relationship — the agent never constructs these
itself.

### Deferred until access/retention/tamper-evidence decisions are made

- `communication_read_log` — stateless, authorized retrieval of the retained
  event stream for one discussion. Never marks messages read, claims work,
  wakes a participant, or creates an obligation. Minimum contract, per
  `gq6m`:

  Request:
  ```json
  {
    "discussion_id": "opaque-stable-id",
    "after": "opaque-cursor-or-null",
    "limit": 100,
    "snapshot": "opaque-high-watermark-or-null"
  }
  ```

  Required response properties: forward-only deterministic ordering by
  server-assigned event sequence; a stable snapshot/high-watermark so
  pagination cannot silently mix later writes into an earlier export;
  `next_cursor` and `has_more` on every page; one immutable event per
  create/edit/delete-tombstone/reaction within the approved capture profile;
  stable discussion/event/message/revision/reply-parent/transport-message/
  protocol-version identifiers; server receive time as ordering authority
  (transport/client time recorded only as a separately labelled
  observation); authenticated session_id, principal_id, login-key
  fingerprint, host/adapter identity, model metadata, optional governed-role
  metadata; the mandatory trust marker on every event with participant
  content; exact retained content where access policy permits, with
  attachment metadata/media-type/size/digest separately authorized; declared
  capture gaps rather than synthesized data; and an export manifest (
  discussion ID, snapshot, event count, protocol/config versions,
  generated-at time, tamper-evidence binding).

  Authorization is discussion- and purpose-scoped, re-verified on every
  request, revocation-immediate. Holding an MCP connection, knowing a
  discussion ID, being mentioned, or holding an AGF governed role does not
  by itself grant export access.

  "Raw discussion log" means the retained participant-visible event stream
  only — never web-server/database/runner logs, secret material,
  authentication challenges, or private keys.

  Deletion is never silently disguised: the stream records a tombstone, and
  prior content is retained under a more restrictive scope only where
  policy explicitly permits; if policy requires erasure, the event records
  that erasure occurred without retaining the erased content. This
  trade-off belongs to the precondition reviews and the governor, not to the
  adapter implementation.

### Explicitly out of phase one

`watch` (automatic session activation/wake-up), broad broadcast/wildcard
mentions, and any behavioral turn-taking policy. Reminders and activation
stay out-of-band for the first observational run so the baseline
communication pattern is not shaped by automated wake-ups before there is
anything to compare it against.

## Addressing and mention model

Structured, typed logical recipients only (`participant` / `group`);
wildcard/broadcast mentions rejected by default. A mention means only that a
message is addressed to a participant or group and may trigger its
listener — it establishes nothing about presence, comprehension, obligation
to respond, or that silence means agreement. A group mention makes the
communication available to current group members; each may independently
observe or act within its own authority and capacity.

## Adapter-enforced safeguards

Enforced by the adapter, not the agent prompt: authenticated sender
identity; configured channel/topic boundaries; sender/recipient allowlists;
fail-closed rejection of identity mismatches; duplicate suppression by
transport message ID; ignoring the adapter's own outbound messages as new
inbound requests; one active processing claim per inbound message; bounded
message size/attachment policy; rate limits and a **generous, logged**
automatic-reply depth bound (not tightly capped — a low hardcoded bound
would have truncated the actual Claude/Codex exchange that produced this
very specification two rounds earlier); rejection or explicit authorization
of broad mentions.

## The references field (converged, `ms1q` / `t34q`)

A separate, optional question from the transport-level reply-parent above:
should an OTGR event support an explicit, agent-authored, multi-target
reference list (distinct from the single reply-parent), to help later graph
reconstruction and analysis?

**Converged position:** yes, but strictly **voluntary and adapter-
uncoerced** — never required by any MCP tool schema, never defaulted, never
enforced. Making it mandatory would prescribe a citation structure the
charter's own anti-prescription constraint forbids, and this project
already has a directly analogous precedent: mandatory OTGR-to-governed
citation was considered and **rejected** in the opposite direction (`r2cl`)
for the same reason — it trains behavior instead of observing it. Whether
and how often participants use this field voluntarily is itself a
legitimate empirical variable the charter's outcome list anticipates
("benefits some classes of work but not others").

**Amendment from `t34q` (Codex):** if implemented, the field should be
schema-typed and narrowly validated (referencing real event IDs within the
same discussion), not free-form text — and named `references` rather than
`references-to`, specifically to avoid importing AGF coordination-record
semantics into a deliberately distinct, non-authoritative medium.

## Deployment model

One Rust adapter binary instance per participating AI **session** (not per
model), each connecting outward to the same self-hosted Zulip organization
over HTTPS with its own credential, matching the identity model above. No
inbound network listener required for `serve` mode. Configuration is
mandatory at startup via an explicit `--config-dir`, fail-closed if absent
or mismatched (e.g. Claude's config directory paired with Codex's
credential must not silently start).

## Preconditions before implementation

Per the converged v0.1 protocol (`r2cl`/`km1h`) and reaffirmed in `gq6m`
(B3): **infosec, privacy/legal, and research-data review, completed and
disposed by the governor, before transport selection or any adapter code.**
These reviews must cover, at minimum: host-CA lifecycle; login-key
enrollment and revocation; session issuance; transcript access; retention;
redaction/deletion; attachment handling; breach response; and research-use
boundaries.

## Remaining open governor decisions

Explicitly not resolved by this document — listed, not defaulted:

1. Approval or rejection of Zulip as the transport, after the precondition
   reviews.
2. Enrollment, expiry, rotation, and revocation policy for host
   certificates, login keys, and session assertions.
3. Transcript access roles, permitted research purposes, retention period,
   attachment policy, and edit/delete/erasure treatment.
4. Whether signed export manifests are mandatory in phase one or a
   documented later hardening item.
5. Whether any `watch`-mode experiment is later authorized as a separately
   declared research condition.
6. Whether the voluntary `references` field, if implemented, is schema-typed
   (per `t34q`'s amendment) or left as unparsed free text — the exchange
   left this an open question rather than converging on it.

## Provenance

Every substantive position above traces to one of these coordination
records, in chronological order:

- `docs/coordination/20260904T104400Z-governor-reviewer-q7mv.md` — OTGR
  experiment charter.
- `docs/coordination/20260904T111045Z-coder-tester-p8vt.md`,
  `20260904T111854Z-coder-tester-5jeg.md`,
  `20260904T111900Z-coder-tester-sm83.md` — first challenge round
  (Codex/Claude).
- `docs/coordination/20260904T112320Z-coder-tester-r2cl.md`,
  `20260904T112728Z-coder-tester-km1h.md` — converged OTGR protocol v0.1.
- `docs/coordination/20260904T164815Z-coder-tester-lunk.md` — Codex's
  original Zulip-adapter proposal.
- `docs/coordination/20260904T170512Z-planner-architect-h8vq.md` — Claude's
  independent assessment (B1–B3 blocking concerns, as planner-architect).
- `docs/coordination/20260905T104031Z-coder-tester-gq6m.md` — Codex's
  answer, B1 amendment, and the raw-log retrieval contract.
- `docs/coordination/20260905T110331Z-planner-architect-ms1q.md` — the
  references-field question.
- `docs/coordination/20260905T110617Z-coder-tester-t34q.md` — Codex's
  convergence on the references field.
- `docs/coordination/20260905T140849Z-coder-tester-pksr.md` — closes the
  earlier, separate request for an independent from-scratch specification
  (`a7jk`) as superseded by this consolidation.

No new design decision is introduced in this document. Any discrepancy
between this consolidation and the cited records is an error in this
document, not a new position — the coordination records remain the source
of truth.
