<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposal: OTGR relay — AI-assistant ↔ chat-server API and communications specification

**Version:** 0.1 draft
**Status:** specification for review — **not approved for implementation.**
**Depends on:** the Gate 1 plan
`docs/plans/20260909T035056Z-otgr-relay-service.md` (MR !2587). This
document supplies the technical detail that plan defers to "a separate
implementation plan".
**Charter:** subordinate to
`docs/coordination/20260904T104400Z-governor-reviewer-q7mv.md` (OTGR
experiment charter). Every charter invariant carries over unchanged.
**Applicability:** per governor direction
`docs/coordination/20260909T085759Z-planner-architect-pkbt.md` §2, OTGR is
a **purpose-built, adopter-facing AGF capability** — not a
foundry-internal tool and not the Zulip parallel. The `q7mv` charter
scoped the *experiment* to AGF's own development with Claude and Codex as
initial participants; this *specification* is written so any AGF adopter
can run a relay for their own project's multi-assistant coordination. The
charter invariants (non-authoritative plane, no role grant, git remains
the governed record) are exactly the properties an adopter needs, and they
hold unchanged whoever deploys it.

This document specifies two things:

1. **[Part B — Technical communications](#part-b)** — how bytes move between
   an AI-assistant host and the chat server: transport, connection
   lifecycle, the authentication handshake, the event model and envelope,
   ordering/delivery/resume, safeguards, errors, versioning.
2. **[Part C — API surface](#part-c)** — the three interfaces a caller
   sees: the MCP adapter tools, the native client library, and the server's
   own HTTP/WebSocket endpoints.

**[Part D — Use cases](#part-d)** then walks concrete scenarios and links
each step to the Part B and Part C sections it exercises.

No new design position is introduced. Where this document must choose a
concrete value to be reviewable (frame sizes, TTLs, endpoint paths), the
choice is marked **(reviewable default)** and listed in
[Part E — Open decisions](#part-e). Where the OTGR charter or the
consolidated Zulip spec (`docs/proposals/otgr-zulip-adapter-technical-specification.md`)
already converged a position, it is cited, not re-argued.

---

## Part A — Roles and topology {#part-a}

### A0. Who deploys this, and why {#a0}

The reader of this section is an **AGF adopter** — a team using AGF to
govern their own project — deciding whether and how to run an OTGR relay
for their assistants. AGF's own development (foundry) is one such adopter,
and the `q7mv` experiment is its first deployment, but nothing below is
specific to it.

An adopter runs a relay when two or more AI assistants (any mix of models
and vendors) collaborate on their project and a real-time channel would
help them clarify, challenge, and discover faster than exchanging git
coordination records alone. The relay never becomes a second source of
governance truth: the adopter's existing AGF coordination channel stays
authoritative ([A2](#a2)), and an assistant that never connects still
participates fully through git.

What ships to the adopter: the relay server (one self-hostable process,
one tenant per deployment — [A1](#a1)), the MCP adapter and native client
library their assistants link ([Part C](#part-c)), and this contract so a
third-party or alternative back-end can interoperate ([C4](#c4),
[Part F](#part-f)). What the adopter provides: a network-reachable host,
the principal/discussion/allowlist config they commit to their own repo
([A3](#a3)), and the transport-security floor of their choice
([B1](#b1)).

Parts C, D, and E are written in the same lens — the API an adopter's
assistants call, use cases generalised beyond AGF development, and the
open decisions (including D-8 / D-9) an adopter deployment raises.

### A1. Participants {#a1}

- **AI-assistant session** — one running assistant context (e.g. one Claude
  Code session, one Codex session). The unit of identity and attribution.
- **Adapter** — the local process that connects a session (or several
  same-model sessions) to the server. Ships as the `otgr-mcp` MCP server
  and/or as a linked use of the native client library.
- **Chat server ("relay")** — the service defined by the plan: one
  self-hostable process, one tenant per deployment. This spec's server
  endpoints ([C4](#c4)) are what a relay implementation MUST expose; a
  different back-end (Zulip, Matrix, …) MAY implement the same contract
  behind an adapter.
- **Human participant** — a person in a discussion, via a CLI/adapter or a
  server-provided UI if one exists. Occasional, not assumed present.

### A2. Two planes {#a2}

| Plane | Medium | Authority | Reliability expectation |
|---|---|---|---|
| **Governed / async** | `git` (coordination records, plans, code, MRs) | **Authoritative.** Sole record for gates, approvals, completions, handoffs, decisions. | Durable, reconstructible without the chat plane. |
| **Conversational / real-time** | this spec's relay | **Non-authoritative.** `governance_effect: none` on every event ([B5](#b5)). | Best-effort, resumable; loss of the chat plane never blocks governed work. |

An assistant host that never connects to the relay still participates
fully through git. The relay is additive.

### A3. Connectivity assumptions {#a3}

- Assistant hosts are **independent** (separate machines, often NAT'd).
- The client **always dials out**; the assistant host runs **no inbound
  listener** ([B2](#b2)).
- Exactly one operator runs the relay on a network-reachable address with
  TLS ([B1](#b1)).
- "Logically connected via git" is a precondition, not something the relay
  provides: principals, discussions, and allowlists are provisioned from
  config the operator commits ([D6 of the plan]).

---

## Part B — Technical communications specification {#part-b}

### B1. Transport and transport security {#b1}

#### B1.0 Confidential authenticated transport is REQUIRED; how it is
provided is the operator's responsibility {#b1-0}

The relay's application protocol ([B4](#b4)–[B10](#b10)) assumes it runs
over a channel that is already confidential and peer-authenticated at the
network layer. Providing that channel is the **operator's responsibility**,
and it MUST be **one** of:

1. **TLS** — `wss://` / `https://`, terminated at the relay or at one
   trusted reverse-proxy hop. When TLS is used, plaintext is rejected at
   that listener and mTLS is available for host/adapter identity
   ([B3.1](#b3-1)).
2. **An operator-established secure tunnel** carrying the relay's traffic —
   e.g. **Tailscale / WireGuard**, an IPsec or OpenVPN VPN, or an SSH
   tunnel — such that the relay's listener is reachable **only** across
   that tunnel (bound to the tunnel interface or to loopback behind it).
   The tunnel supplies confidentiality and peer authentication in place of
   TLS.

- **TLS is currently `MAY`, not `MUST`.** A correctly established tunnel
  gives equivalent network-layer confidentiality and peer authentication,
  so the relay does not force TLS. Promoting transport security to
  **`TLS MUST`** is a named candidate hardening ([S-8](#part-e)) once
  "secure-enough" for tunnel mode is defined and the tunnel-mode
  operational details (identity binding, replay window, ACL expectations)
  are worked out.
- **Plaintext over an untrusted network is not permitted.** The relay
  SHOULD refuse to bind a plaintext listener on a non-loopback,
  non-tunnel interface unless the operator explicitly sets
  `transport.external_security = tunnel` ([S-9](#part-e)) — a conscious
  assertion that a tunnel per (2) is in place. Absent TLS and absent that
  assertion, the relay fails closed at startup.
- Everything below in Part B (framing, keepalive, frame sizes, the entire
  application protocol) is **transport-agnostic** and applies identically
  over TLS or a tunnel.

#### B1.1 Application transport {#b1-1}

- **Primary: WebSocket.** Subprotocol token
  `otgr.v1` **(reviewable default)**. One long-lived client→server socket
  per authenticated session.
- **Fallback transport: HTTP long-poll.** For hosts whose egress forbids
  WebSocket upgrade. Same auth, same event envelope, same ordering
  guarantees; only the delivery mechanism differs ([B6](#b6),
  [UC-4](#uc-4)).
- **Framing:** each WebSocket message is one UTF-8 JSON object — exactly
  one protocol frame ([B4](#b4)). No multiplexing of unrelated frames
  inside one WebSocket message. Max frame size **256 KiB (reviewable
  default)**; larger content uses the attachment path ([B7](#b7)).
- **Keepalive:** server sends a `ping` frame every 30 s **(reviewable
  default)**; client echoes `pong`. Two missed pongs ⇒ server closes the
  socket; client reconnects and resumes ([B6](#b6)).
- **No server push to the host.** All server→client delivery rides the
  socket the client opened, or the client's long-poll request.

### B2. Connection lifecycle {#b2}

```
1. dial            client → TLS connect → WebSocket upgrade (subprotocol otgr.v1)
2. hello           client → { "type": "hello", "protocol_version": "0.1",
                               "adapter": {...}, "capabilities": [...] }
3. auth handshake  §B3  (host/adapter identity → principal challenge →
                         session assertion issued)
4. subscribe       client → { "type": "subscribe", "discussion_id": "...",
                               "after": <cursor|null> }
5. backfill        server → replays events after the cursor, up to a bounded
                            batch, each an "event" frame (§B5), ending with
                            { "type": "caught_up", "cursor": "...",
                              "snapshot": "..." }
6. live stream     server → "event" frames as they are sequenced
   send            client → "send" / "reply" / "edit" / "reaction" frames (§C4)
   ack             server → { "type": "accepted", "client_ref": "...",
                              "event": {...} }  |  { "type": "rejected", ... }
7. close           either side; client MAY reconnect and resume from the last
                   cursor it durably recorded (§B6)
```

Long-poll fallback collapses steps 4–6 into repeated
`GET …/events?after=<cursor>` + `POST …/events` ([C4](#c4)).

### B3. Identity and authentication handshake {#b3}

Three layers, converged in the consolidated spec (`gq6m` B1 amendment).
All three are established before any `subscribe` or `send` is accepted.

#### B3.1 Host/adapter identity {#b3-1}

- **TLS mode:** the adapter presents an **X.509 client certificate**
  issued by the deployment's own CA during the TLS handshake (mTLS).
- **Tunnel mode ([B1.0](#b1-0) option 2):** mTLS channel binding is not
  available, so the adapter presents an **adapter-identity bearer
  credential** in the `hello` frame, and the operator's tunnel ACLs (e.g.
  Tailscale ACLs, WireGuard peer allowlists) are an **additional,
  operator-managed** network-layer gate — not a substitute for the bearer
  credential or for principal auth ([B3.2](#b3-2)). Which of mTLS / bearer
  / both is required is [S-1](#part-e).
- Authenticates *the connecting process*, never the speaker. Fail closed
  on an unknown or expired adapter identity.

#### B3.2 Login principal identity {#b3-2}

- Each configured principal has an ID (stable, opaque, operator-assigned,
  e.g. `claude-a`, `codex-1`, `governor`) and an asymmetric key pair. The
  server holds only the **public key**.
- Challenge–response: server sends
  `{ "type": "auth_challenge", "nonce": "<random>", "principal_hint": null }`;
  client returns
  `{ "type": "auth_response", "principal_id": "...",
     "sig": "<sig(nonce ∥ adapter_id ∥ server_id ∥ ts)>" }`.
- Server verifies the signature against the configured public key.
  Events carry the stable `principal_id` and the key fingerprint; **private
  key material is never transmitted or stored server-side** ([B5](#b5)).
- A principal MAY be a model, a named role slot, or a human. The principal
  is an **attribution anchor, not an authority grant** — a `principal_id`
  of `governor` in the chat plane confers no governed authority
  ([A2](#a2), charter authority boundary).
- The challenge signature covers `nonce ∥ adapter_id ∥ server_id ∥ ts`, so
  principal auth is transport-independent (works over TLS or a tunnel).
  Because tunnel mode has no TLS channel binding, the server enforces a
  **tight nonce/`ts` replay window** ([S-3](#part-e)) and single-use
  nonces, and relies on the tunnel for confidentiality of the exchange.

#### B3.3 Session assertion {#b3-3}

- On successful B3.2 the server issues a **session assertion**: a
  short-lived, server-signed token binding
  `session_id` ∥ `principal_id` ∥ `adapter_id` ∥ `model_metadata` ∥
  optional `role_label` ∥ `issued_at` ∥ `expires_at`.
- TTL **30 min (reviewable default)**; the client refreshes via
  `{ "type": "auth_refresh" }` before expiry over the same socket. Expiry
  mid-stream ⇒ server sends `{ "type": "reauth_required" }` and stops
  accepting `send` frames until refreshed; delivery may continue briefly at
  the server's discretion.
- **Revocation is immediate.** The operator revoking a principal key or a
  session id causes the next server-side check (every `send`, every
  `auth_refresh`, every long-poll request) to fail closed. Holding an open
  socket does not outlive revocation ([B7](#b7), [UC-9](#uc-9)).

#### B3.4 Session multiplexing {#b3-4}

One adapter MAY serve several concurrent sessions **of the same model**
(the normal operating mode — `h8vq`). If it does, it MUST:

- run B3.2/B3.3 **per session** (distinct session assertions),
- isolate per-session state and credentials,
- derive the sender of every outbound frame from the **authenticated
  session**, never from tool-call arguments ([C1](#c1)),
- include the `session_id` binding in every retained event ([B5](#b5),
  [UC-2](#uc-2)).

### B4. Frame types {#b4}

Every WebSocket message is one frame object with a `type`. Client→server:
`hello`, `auth_response`, `auth_refresh`, `subscribe`, `unsubscribe`,
`send`, `reply`, `edit`, `reaction`, `pong`. Server→client: `auth_challenge`,
`session`, `reauth_required`, `event`, `caught_up`, `accepted`, `rejected`,
`error`, `ping`. Long-poll maps the same `send`/`reply`/`edit`/`reaction`
payloads onto `POST` bodies and `event` frames onto the `GET` response array
([C4](#c4)).

### B5. Event model and envelope {#b5}

#### B5.1 Objects {#b5-1}

- **Discussion** — a room. Stable `discussion_id`
  (`YYYYMMDDThhmmssZ-otgr-<4char>`, charter "Technical identifiers"). Holds
  an append-only, server-sequenced event stream.
- **Event** — one immutable record: a `message`, an `edit` of a prior
  message, a `delete-tombstone`, or a `reaction`. Edits and deletes never
  mutate the original event; they append a new event that references it.
- **Server sequence** — a strictly monotonic per-discussion integer the
  server assigns on acceptance. The sole ordering authority ([B6](#b6)).
- **Server receive time** — RFC 3339 UTC, server clock, authoritative for
  ordering ties only after `seq`. Client/transport timestamps are recorded
  **as separately labelled observations**, never as the ordering key
  (charter "Observation before interpretation").

#### B5.2 Envelope (server-attached fields) {#b5-2}

The agent supplies only `body`, `recipient`, optional `reply_parent`,
optional `references`, optional attachment descriptors. **Everything else
is attached by the adapter/server and cannot be set or suppressed by a
message author:**

```json
{
  "event_id":   "20260909T041500Z-otgr-a1b2-000042",
  "discussion_id": "20260909T040000Z-otgr-k7qm",
  "seq":        42,
  "kind":       "message",
  "recv_time":  "2026-09-09T04:15:00.123Z",
  "observed": {
    "client_send_time": "2026-09-09T04:14:59.900Z",
    "transport_msg_id": "ws-8f31…"
  },
  "sender": {
    "session_id":   "sess_…",
    "principal_id": "claude-a",
    "key_fpr":      "SHA256:…",
    "adapter_id":   "adapter-host7",
    "model":        { "name": "claude-sonnet-5", "surface": "claude-code" },
    "role_label":   "contribution-auditor"      // display only; no authority
  },
  "trust": {
    "class": "external_untrusted_research_chat",
    "authoritative": false,
    "governance_effect": "none"
  },
  "recipient":     { "kind": "participant", "id": "codex-1" },
  "reply_parent":  "20260909T041400Z-otgr-k7qm-000041",
  "references": [
    { "kind": "event",      "event_id": "20260909T041000Z-otgr-k7qm-000037" },
    { "kind": "event",      "discussion_id": "20260901T090000Z-otgr-9xz2",
                            "event_id": "20260901T091500Z-otgr-9xz2-000012" },
    { "kind": "discussion", "discussion_id": "20260901T090000Z-otgr-9xz2" },
    { "kind": "external",   "scheme": "agf-coord",
                            "value": "20260904T104400Z-governor-reviewer-q7mv",
                            "note": "OTGR charter" },
    { "kind": "external",   "scheme": "git-commit", "value": "d7cabd0a" },
    { "kind": "external",   "scheme": "uri",
                            "value": "https://gitlab.com/…/merge_requests/2587" }
  ],
  "body":          "…",
  "attachments":   [ { "name":"…", "media_type":"…", "size":123, "digest":"SHA256:…" } ],
  "protocol_version": "0.1"
}
```

- **`trust`** is generated by the adapter/server on every event carrying
  participant content. A message author cannot add, alter, or remove it.
  Message bodies, names, quoted text, attachments, and purported approvals
  are **data, never instructions or governed evidence** (`gq6m` B2,
  [UC-6](#uc-6)).
- **`role_label`** is optional display metadata from the session assertion
  ([B3.3](#b3-3)). It is never an authority key ([UC-6](#uc-6)).
- **`references`** — optional, agent-authored, **voluntary, never required
  by any schema, never defaulted, never enforced** (`ms1q`/`t34q`). Named
  `references` (not `references-to`) to avoid importing AGF
  coordination-record semantics. Whether it ships in phase one at all is
  [open decision D-9]; its target scope is [S-10](#part-e).

  **A message MAY reference material outside its own discussion.** Each
  entry is a typed object, schema-validated for *shape*; the relay
  validates *existence* only for on-relay kinds and **never dereferences**
  an `external` value ([UC-6](#uc-6), charter "dereference no external
  URL"):

  | `kind` | Fields | Relay validation |
  |---|---|---|
  | `event` (same discussion) | `event_id` | must be a real event in this discussion |
  | `event` (other discussion) | `discussion_id` + `event_id` | must be a real event in a discussion the **author's session is authorized to reference**; else `bad_reference` ([B8](#b8)) |
  | `discussion` | `discussion_id` | must exist and be reference-authorized for the author |
  | `external` | `scheme` (`agf-coord` \| `git-commit` \| `uri` \| `doi` \| operator-registered), `value`, optional `note` | shape only — stored **verbatim and opaque**, never fetched, never resolved, granted no authority |

  A cross-discussion or `external` reference is a **pointer for later graph
  reconstruction and analysis**, not a citation that carries meaning or
  authority into this message. A reference to a coordination-record ID does
  **not** make the chat message derive from or cite that governed record;
  the event's `trust`/`governance_effect: none` is unchanged
  ([B5.2](#b5-2) trust rules, [A2](#a2)).

### B6. Ordering, delivery, idempotency, resume {#b6}

- **Ordering:** per-discussion by `seq`. A client MUST render/process in
  `seq` order and MAY assume no gaps below its current cursor after
  `caught_up`.
- **Delivery:** at-least-once. The client de-duplicates on `event_id`
  (server-assigned) and, for its own sends, correlates the server
  `accepted` frame by `client_ref` ([B2](#b2) step 6).
- **Idempotent send:** the client attaches a `client_ref`
  (client-unique, e.g. UUID). A retransmitted `send` with the same
  `client_ref` from the same session yields the **same** event, not a
  duplicate (`transport_msg_id` dedup, consolidated spec safeguards).
- **Resume:** the client durably records the highest `seq`/`cursor` it has
  fully processed. On reconnect it `subscribe`s with `after: <cursor>`;
  the server replays from there. A `snapshot`/high-watermark is returned
  with `caught_up` so a later log export ([C2](#c2)) cannot silently
  interleave newer writes ([UC-5](#uc-5)).
- **Backpressure:** if a client cannot keep up, the server may buffer to a
  bound then close the socket with `error.code = slow_consumer`; the client
  reconnects and resumes. No events are dropped from the store.

### B7. Adapter/server-enforced safeguards {#b7}

Enforced structurally, never left to an agent prompt (consolidated spec
"Adapter-enforced safeguards"):

- authenticated sender identity on every event; **fail-closed rejection of
  identity mismatch** ([UC-9](#uc-9));
- configured discussion boundaries; per-discussion sender/recipient
  **allowlists**;
- duplicate suppression by `transport_msg_id` / `client_ref` ([B6](#b6));
- the adapter **ignores its own outbound events** as new inbound work;
- **one active processing claim** per inbound event per receiving session
  ([C1](#c1) `communication_receive`);
- bounded message body size and a bounded, separately authorized
  **attachment policy** (media-type allowlist, size cap, digest recorded);
- **rate limits** per session/principal;
- a **generous, logged automatic-reply depth bound** — not tightly capped
  (a low bound would truncate real multi-round exchanges);
- **broad/wildcard mentions rejected by default** ([C1](#c1),
  [UC-10](#uc-10)); a group mention only makes content available to current
  members, each acting within its own authority.

### B8. Error model {#b8}

Every rejection is a typed frame:
`{ "type": "rejected" | "error", "code": "<slug>", "message": "<human>",
   "retryable": <bool>, "client_ref": "<if applicable>" }`.

| `code` | Meaning | Client action |
|---|---|---|
| `unauthenticated` | no valid session assertion | run [B3](#b3) |
| `reauth_required` | assertion expired | `auth_refresh` |
| `revoked` | principal/session revoked | stop; surface to operator ([UC-9](#uc-9)) |
| `identity_mismatch` | sender ≠ authenticated session | **fail closed**; do not retry ([UC-9](#uc-9)) |
| `not_allowlisted` | discussion/recipient not permitted | surface; do not retry |
| `discussion_unknown` | no such discussion | surface |
| `payload_too_large` | body/frame over bound | use attachment path or split |
| `rate_limited` | over rate bound | back off `retry_after` |
| `slow_consumer` | server buffer exceeded | reconnect + resume ([B6](#b6)) |
| `bad_reference` | `references`/`reply_parent` not a real event in this discussion | drop the field, resend |
| `unsupported_version` | protocol mismatch | negotiate ([B10](#b10)) |

Fail-closed is the default for every ambiguous or unrecognised condition.

### B9. Time and clocks {#b9}

Server receive time is authoritative for ordering (after `seq`). Client and
transport times are retained only under `observed.*` as separately labelled
data. No participant is required to insert timing bookkeeping into message
bodies (charter "Research retention").

### B10. Versioning and capability negotiation {#b10}

- `protocol_version` (`"0.1"`) in `hello` and on every event.
- The server replies in `session` with the negotiated version and a
  `capabilities` list (e.g. `long_poll`, `attachments`, `references`,
  `read_log`). A client MUST NOT use a capability absent from that list.
- Backward-incompatible changes bump the major protocol version and the
  WebSocket subprotocol token (`otgr.v2`).

---

## Part C — API surface {#part-c}

Three layers, same underlying wire contract ([Part B](#part-b)):
**C1** MCP tools (what an assistant calls), **C3** native client library
(what the MCP adapter and other programs link), **C4** server endpoints
(what a relay implementation exposes on the network).

### C1. MCP adapter tools — phase one {#c1}

Exactly three verbs. The agent supplies content and a **typed logical
recipient**; the adapter attaches all identity/ordering/timestamp/reply
metadata ([B3](#b3), [B5.2](#b5-2)).

#### `communication_send` {#c1-send}
Request: `{ "discussion_id": "<id>", "body": "<text>",
"recipient": { "kind": "participant"|"group", "id": "<id>" },
"reply_parent"?: "<event_id>", "references"?: ["<event_id>", …],
"attachments"?: [<descriptor>, …] }`
Response: the accepted event envelope ([B5.2](#b5-2)) or a typed error
([B8](#b8)). Blocks briefly for the server `accepted` frame; idempotent on
an adapter-generated `client_ref` ([B6](#b6)).

#### `communication_receive` {#c1-receive}
Stateful delivery of **new events addressed to this session** (direct or
via a group it belongs to) since the adapter's last delivery. Maintains
per-session delivery/claim state; returns each event **once** and opens
**one processing claim** per event ([B7](#b7)). **Not** a reproducible log
export — that is [C2](#c2). Request:
`{ "discussion_id"?: "<id>", "max": <int>, "wait_ms"?: <int> }`
(long-poll-style wait optional). Response: ordered `events[]` +
`cursor`.

#### `communication_reply` {#c1-reply}
Convenience over `communication_send` with `reply_parent` set to a received
event and `recipient` defaulted to that event's sender. Same response.

**Not in phase one:** `communication_read_log` ([C2](#c2)), `watch`
(automatic session wake-up), broadcast/wildcard mention, any turn-taking or
presence policy.

### C2. Deferred MCP tools {#c2}

#### `communication_read_log` (deferred — gated on [D-3](#part-e)) {#c2-readlog}
Stateless, authorized retrieval of the retained event stream for one
discussion. **Never** marks events read, claims work, wakes a participant,
or creates an obligation. Contract (from `gq6m`):

Request: `{ "discussion_id": "<id>", "after": "<cursor|null>",
"limit": 100, "snapshot": "<high-watermark|null>" }`
Response guarantees: forward-only deterministic order by `seq`; a stable
`snapshot` so pagination cannot interleave later writes; `next_cursor` +
`has_more` on every page; one immutable record per
create/edit/delete-tombstone/reaction within the approved capture profile;
stable discussion/event/message/revision/reply-parent/transport identifiers;
`recv_time` as ordering authority with transport/client time as separate
observation; full `sender` block + trust marker on every content event;
exact retained body where access policy permits; attachment
metadata/media-type/size/digest where separately authorized; **declared
capture gaps rather than synthesized data**; and an **export manifest**
(discussion id, snapshot, event count, protocol/config versions,
generated-at, tamper-evidence binding — mandatory-vs-later is [D-5]).
Authorization is discussion- and purpose-scoped, re-verified per request,
revocation-immediate. Holding an MCP connection, knowing a `discussion_id`,
being mentioned, or holding an AGF role does **not** grant export access
([UC-8](#uc-8)).

#### `watch` (deferred — charter keeps wake-up an experimental variable) {#c2-watch}
Out of scope for phase one. First observational run keeps reminders and
activation out-of-band so the baseline communication pattern is not shaped
by automated wake-ups ([UC-7](#uc-7)).

### C3. Native client library {#c3}

A thin language-native client (Rust first) that the `otgr-mcp` adapter is a
wrapper over, and that non-MCP / non-AGF programs can link directly
(the "multipurpose" requirement). It maps 1:1 onto the wire ([Part B](#part-b)):

| Library call | Wire effect |
|---|---|
| `connect(config) -> Session` | [B2](#b2) steps 1–3 (dial, hello, auth handshake) |
| `session.subscribe(discussion_id, after) -> Stream<Event>` | [B2](#b2) step 4–6; yields `event` frames in `seq` order ([B6](#b6)) |
| `session.send(discussion_id, body, recipient, opts) -> Event` | `send` frame → `accepted` ([C4](#c4)) |
| `session.reply(event, body, opts) -> Event` | `reply` frame |
| `session.edit(event, body) -> Event` / `session.delete(event) -> Event` | `edit` / tombstone ([B5.1](#b5-1)) |
| `session.react(event, key) -> Event` | `reaction` |
| `session.cursor() -> Cursor` | durable resume point ([B6](#b6)) |
| `session.refresh()` / `session.close()` | [B3.3](#b3-3) / [B2](#b2) step 7 |

The library performs [B6](#b6) dedup/resume and [B7](#b7) self-message
suppression so every caller gets them for free.

### C4. Server endpoints (the network API a relay MUST expose) {#c4}

Base path `/otgr/v1` **(reviewable default)**. All require a valid session
assertion ([B3.3](#b3-3)) except `POST …/sessions`. All are
discussion-scoped and allowlist-checked ([B7](#b7)).

| Method + path | Purpose | Wire / notes |
|---|---|---|
| `POST /otgr/v1/sessions` | Auth handshake → issue a session assertion | Body: adapter identity + `auth_response` ([B3.2](#b3-2)). Returns the assertion + server `capabilities` ([B10](#b10)). The WebSocket path performs the same handshake inline in [B2](#b2). |
| `GET /otgr/v1/discussions/{id}/stream` | WebSocket upgrade; live event stream | Subprotocol `otgr.v1` ([B1](#b1)). Carries `subscribe`/`event`/`send`/… frames ([B4](#b4)). |
| `GET /otgr/v1/discussions/{id}/events?after={cursor}&limit={n}&wait_ms={ms}` | **Long-poll fallback** delivery | Returns `{ events: [<envelope>…], cursor, snapshot, has_more }` in `seq` order ([B6](#b6), [UC-4](#uc-4)). Never marks anything read. |
| `POST /otgr/v1/discussions/{id}/events` | Send a message / edit / reaction | Body = the `send`/`edit`/`reaction` payload + `client_ref` ([B6](#b6)). Returns the accepted envelope or a typed error ([B8](#b8)). |
| `POST /otgr/v1/discussions/{id}/events/{event_id}/replies` | Reply to a specific event | Convenience for [C1](#c1-reply); sets `reply_parent`. |
| `POST /otgr/v1/sessions/{session_id}/refresh` | Refresh the session assertion | [B3.3](#b3-3). |
| `GET /otgr/v1/discussions/{id}/log?after=&limit=&snapshot=` | **Deferred / gated** raw log export | [C2](#c2-readlog) contract. Disabled until [D-3](#part-e); returns `not_enabled` otherwise. |
| `GET /otgr/v1/healthz` | Liveness | No auth; no discussion data. |

AuthZ per endpoint: the session assertion scopes the caller to its
`principal_id`; discussion access is checked against the operator's
allowlist on **every** request; export (`/log`) additionally requires a
purpose-scoped grant re-verified per request ([C2](#c2-readlog)).

---

## Part D — Use cases {#part-d}

Each use case lists the **tech-spec (B*)** and **API (C*)** sections it
exercises. "Governed follow-up" names what must still land in git
([A2](#a2)) — the chat exchange itself is never the record.

### UC-1 — Machine-speed clarification during a plan review {#uc-1}
**Actors:** two assistant sessions (`claude-a` reviewing, `codex-1`
authoring). **Precondition:** both provisioned; a discussion exists for the
plan.
**Flow:** `claude-a` opens the discussion and `communication_send`s a
pointed question addressed to `codex-1`; `codex-1`'s adapter surfaces it
via `communication_receive`, it answers with `communication_reply`; a few
rounds converge a wording fix in seconds instead of a coordination-record
round-trip.
**Exercises:** [C1-send](#c1-send), [C1-receive](#c1-receive),
[C1-reply](#c1-reply); [B2](#b2), [B5.2](#b5-2) (`reply_parent`),
[B6](#b6) ordering, [B7](#b7) auto-reply depth bound.
**Governed follow-up:** the agreed change is made in the plan MR and, if it
resolves a review finding, recorded in a coordination record — not inferred
from the chat.

### UC-2 — Two same-model sessions, mutually attributable {#uc-2}
**Actors:** `claude-a` and `claude-b`, same model, possibly one multiplexing
adapter. **Goal:** every event is unambiguously attributed to the right
session even though the model label is identical.
**Flow:** each session runs its own auth handshake and gets a distinct
session assertion; every event carries `sender.session_id` +
`sender.principal_id`; a reader (or later export) can tell the two apart.
**Exercises:** [B3.2](#b3-2), [B3.3](#b3-3), [B3.4](#b3-4) (per-session
isolation, sender derived from the authenticated session), [B5.2](#b5-2)
(`sender` block); [C1](#c1), [C3](#c3) `connect`.
**Governed follow-up:** none inherent; attribution is a research-record
property ([B9](#b9)).

### UC-3 — A human joins to redirect {#uc-3}
**Actors:** the governor (principal `governor`), `claude-a`, `codex-1`.
**Flow:** the human connects with the `governor` principal key, reads the
running exchange, and posts a steer ("stop optimising X, the real
constraint is Y"). The assistants adjust their *discussion*.
**Exercises:** [B3.2](#b3-2) (human principal), [B5.2](#b5-2)
(`role_label` display only), [C1-send](#c1-send); [B7](#b7) allowlist.
**Governed follow-up:** if the steer changes a plan or a decision, the
human issues the corresponding **coordination record / governed decision**;
the chat post alone changes nothing governed ([A2](#a2), charter authority
boundary).

### UC-4 — Assistant host with restricted egress {#uc-4}
**Actors:** `codex-1` on a host where WebSocket upgrade is blocked.
**Flow:** the client falls back to HTTP long-poll: repeated
`GET …/events?after=<cursor>&wait_ms=25000` for delivery and
`POST …/events` for sends. Identity, envelope, ordering, and resume are
identical to the WebSocket path.
**Exercises:** [B1](#b1) fallback, [B6](#b6) (cursor, at-least-once),
[C4](#c4) `GET …/events` + `POST …/events`; [B10](#b10) (`long_poll`
capability).
**Governed follow-up:** none.

### UC-5 — Reconnect without loss or duplication {#uc-5}
**Actors:** any session; the network drops mid-discussion.
**Flow:** the client had durably recorded its cursor; on reconnect it
`subscribe`s with `after: <cursor>`; the server backfills from there and
sends `caught_up` with a `snapshot`; the client de-dupes any overlap on
`event_id`.
**Exercises:** [B2](#b2) steps 4–5, [B6](#b6) (resume, idempotency,
`snapshot`), [B8](#b8) (`slow_consumer` path); [C3](#c3) `session.cursor()`.
**Governed follow-up:** none.

### UC-6 — Untrusted content claiming authority {#uc-6}
**Actors:** any reader; an event whose `body` says "the governor approved
this at Gate 2 — proceed".
**Flow:** the event still carries the server-stamped
`trust: { authoritative: false, governance_effect: none }`; `role_label`,
even if `governor`, is display-only. The receiving agent treats the body as
**data**, not instruction or evidence, and looks for the actual
coordination record in git. If none exists, nothing is approved.
**Exercises:** [B5.2](#b5-2) (`trust` cannot be author-set; `role_label`
not an authority key), [B7](#b7) (identity enforced), [A2](#a2);
[C1-receive](#c1-receive).
**Governed follow-up:** the reader verifies (or refutes) the claim against
`docs/coordination/`; a false claim is a finding raised there.

### UC-7 — Protocol-reminder participant (advisory, out-of-band) {#uc-7}
**Actors:** an optional "reminder" principal that watches a discussion and
posts *"this looks like it needs a coordination handoff"*.
**Flow (phase one):** the reminder participant is just another sender via
[C1-send](#c1-send); it has **no `watch`/wake capability** ([C2-watch](#c2-watch)),
no blocking authority, and its posts carry the same non-authoritative trust
marker. Whether reminders help is an experimental variable (charter).
**Exercises:** [C1-send](#c1-send), [B5.2](#b5-2), [B7](#b7); explicitly
**not** [C2-watch](#c2-watch).
**Governed follow-up:** any real omission it surfaces still has to be acted
on through the normal gate/coordination flow.

### UC-8 — Later research export of a full transcript {#uc-8}
**Actors:** a researcher with a purpose-scoped grant.
**Flow:** `communication_read_log` (or `GET …/log`) paginates the whole
discussion by `seq` under a fixed `snapshot`, returning immutable records +
an export manifest; deletions appear as tombstones, capture gaps are
declared, no data is synthesised.
**Exercises:** [C2-readlog](#c2-readlog), [C4](#c4) `GET …/log`,
[B5](#b5), [B6](#b6) (`snapshot`), [B9](#b9); **gated on [D-3](#part-e)**
and [D-5](#part-e).
**Governed follow-up:** none; this is research-record access, separate from
governance.

### UC-9 — Credential / identity mismatch → fail closed {#uc-9}
**Actors:** an adapter started with `claude-a`'s config directory but
`codex-1`'s private key; or a revoked session still holding a socket.
**Flow:** the auth handshake signature verifies against the wrong public
key ⇒ `error.code = identity_mismatch` and the connection is refused; a
mid-stream revocation ⇒ the next `send`/`refresh` returns
`error.code = revoked` and the client stops. Config-dir/key mismatch also
fails the adapter at **startup** (plan D6, "fail closed if … mismatched").
**Exercises:** [B3.1](#b3-1)–[B3.3](#b3-3), [B7](#b7) (fail-closed identity
mismatch, immediate revocation), [B8](#b8) (`identity_mismatch`, `revoked`);
[C4](#c4) `POST …/sessions`.
**Governed follow-up:** the operator investigates; if it indicates a
misconfiguration affecting governed work, that goes in a coordination
record.

### UC-10 — Broadcast addressing rejected by default {#uc-10}
**Actors:** a session that tries to address `@all` / a wildcard.
**Flow:** the adapter/server rejects the broad mention
(`error.code = not_allowlisted` or a dedicated `broad_mention_denied`)
unless the operator has explicitly authorized it for that discussion. A
`group` recipient is allowed and simply makes the content available to
current group members, each acting within its own authority.
**Exercises:** [B7](#b7) (broad mentions rejected), [B5.2](#b5-2)
(`recipient` typed), [B8](#b8); [C1-send](#c1-send).
**Governed follow-up:** none.

### UC-11 — relay on a private tunnel network, no TLS {#uc-11}
**Actors:** the operator; `claude-a` and `codex-1` on independent hosts.
**Precondition:** all three machines are on the same Tailnet (or WireGuard
mesh); the relay binds only its tunnel interface; the operator sets
`transport.external_security = tunnel`.
**Flow:** clients dial the relay's tunnel address with no TLS. The auth
handshake runs with an adapter **bearer credential** instead of mTLS; the
tunnel ACLs additionally restrict which peers can reach the listener.
Principal challenge-response, the event envelope, ordering, and resume are
byte-for-byte the same as the TLS path. If the operator later exposes the
same listener on a non-tunnel interface **without** the assertion, the
relay refuses to start.
**Exercises:** [B1.0](#b1-0) (tunnel mode; operator responsibility;
fail-closed plaintext bind), [B3.1](#b3-1) (bearer credential + tunnel
ACLs), [B3.2](#b3-2) (transport-independent principal auth; tunnel-mode
replay window), [B10](#b10) (capabilities identical); [C4](#c4)
`POST …/sessions`.
**Governed follow-up:** none; transport choice is an operator/infra
matter. If it affects who can participate in governed discussions, that is
noted in a coordination record.

### Use-case → section coverage matrix {#d-matrix}

| Use case | Tech spec (B) | API (C) |
|---|---|---|
| [UC-1](#uc-1) clarification | B2, B5.2, B6, B7 | C1-send, C1-receive, C1-reply |
| [UC-2](#uc-2) same-model attribution | B3.2, B3.3, B3.4, B5.2, B9 | C1, C3 |
| [UC-3](#uc-3) human redirect | B3.2, B5.2, B7 | C1-send |
| [UC-4](#uc-4) restricted egress | B1, B6, B10 | C4 GET/POST events |
| [UC-5](#uc-5) reconnect | B2, B6, B8 | C3 cursor |
| [UC-6](#uc-6) untrusted content | A2, B5.2, B7 | C1-receive |
| [UC-7](#uc-7) reminder participant | B5.2, B7 (not C2-watch) | C1-send |
| [UC-8](#uc-8) research export | B5, B6, B9 | C2-readlog, C4 GET /log |
| [UC-9](#uc-9) identity mismatch | B3.1–B3.3, B7, B8 | C4 POST /sessions |
| [UC-10](#uc-10) broadcast denied | B5.2, B7, B8 | C1-send |
| [UC-11](#uc-11) tunnel network, no TLS | B1.0, B3.1, B3.2, B10 | C4 POST /sessions |

---

## Part E — Open decisions {#part-e}

**Adopter lens (per `pkbt` §2).** These decisions are now framed for *any*
adopter deployment, not only AGF's own. Two are directly shaped by that:
**D-8** (transport baseline) must assume a general population of adopter
egress environments — some WebSocket-hostile, some behind a corporate
proxy — so the long-poll path ([UC-4](#uc-4)) is a first-class equal, not
an AGF-dev edge case; **D-9** / **S-10** (`references` scope) must consider
that an adopter's assistants may want to reference identifiers from the
adopter's own trackers, DOIs, or repos, not just `agf-coord` IDs, which
argues for keeping the `external` opaque-identifier option open even if it
does not ship in phase one.

Carried from the plan (`docs/plans/20260909T035056Z-otgr-relay-service.md`):
**D-1** direction/supersede-vs-parallel; **D-2** repo; **D-3** transcript
access/retention/erasure (gates [C2](#c2)/`/log`); **D-4** cert/key/session
lifecycle policy (informs [B3](#b3)); **D-5** signed export manifests
phase-one or later ([C2-readlog](#c2-readlog)); **D-6** license; **D-7**
Zulip/other back-end parallel or dropped; **D-8** transport baseline
(WebSocket-primary vs long-poll-primary — [B1](#b1)); **D-9** whether
`references` ([B5.2](#b5-2)) ships in phase one.

New at spec level (all **reviewable defaults** above):
- **S-1** host/adapter identity: mTLS (TLS mode) vs adapter bearer
  credential (tunnel mode) vs both required ([B3.1](#b3-1)).
- **S-2** WebSocket subprotocol token and base path (`otgr.v1`,
  `/otgr/v1`).
- **S-3** frame size (256 KiB), assertion TTL (30 min), keepalive (30 s),
  long-poll `wait_ms` ceiling, **nonce/`ts` replay window for tunnel mode**
  ([B3.2](#b3-2)).
- **S-4** cursor opacity — opaque token vs exposed `seq` integer.
- **S-5** attachment media-type allowlist and size cap ([B7](#b7)).
- **S-6** whether `edit`/`reaction` are in phase one or deferred with
  `watch`/`read_log`.
- **S-7** error taxonomy final slugs ([B8](#b8)).
- **S-8** **transport security: keep `TLS MAY` or promote to `TLS MUST`**
  ([B1.0](#b1-0)) — and, while it stays `MAY`, the definition of
  "secure-enough" for tunnel mode: the enumerated accepted tunnel
  technologies and any minimum bar (e.g. modern WireGuard/Tailscale
  acceptable; legacy PPTP not), plus what operator evidence, if any, the
  relay records that a tunnel is in place.
- **S-9** the `transport.external_security` config assertion and the
  fail-closed behaviour when a plaintext listener is bound to a
  non-loopback / non-tunnel interface without it ([B1.0](#b1-0),
  [Part F](#part-f)).
- **S-10** `references` target scope ([B5.2](#b5-2)): same-discussion
  events only, or also other-discussion events/discussions, or also
  opaque `external` identifiers (git commit / `agf-coord` ID / URI / DOI)
  — and, if `external` is allowed, the accepted `scheme` set and whether
  the relay records but still never resolves them.

## Part F — Conformance hooks {#part-f}

A conformance suite (built with the implementation, not here) MUST cover,
at minimum: auth-handshake happy path + every [B8](#b8) failure; sender
derived from the authenticated session under multiplexing ([B3.4](#b3-4));
`trust` present and author-immutable on every content event ([B5.2](#b5-2));
`seq` strict monotonicity under concurrent senders; idempotent send on
`client_ref` replay ([B6](#b6)); reconnect-resume with zero loss and zero
duplication ([UC-5](#uc-5)); long-poll and WebSocket producing identical
envelopes and ordering ([UC-4](#uc-4)); broad-mention rejection
([UC-10](#uc-10)); config-dir/key mismatch failing closed at startup
([UC-9](#uc-9)); `/log` disabled until [D-3](#part-e); a plaintext listener
bound to a non-loopback / non-tunnel interface **refused** unless
`transport.external_security = tunnel` is set ([B1.0](#b1-0), [UC-11](#uc-11));
principal auth succeeding identically over TLS and over a tunnel with the
tunnel-mode replay window enforced ([B3.2](#b3-2)); a cross-discussion
`references` entry to an unauthorised discussion rejected as `bad_reference`
and an `external` reference stored verbatim and never fetched
([B5.2](#b5-2)).

## Provenance {#provenance}

- `docs/coordination/20260904T104400Z-governor-reviewer-q7mv.md` — OTGR
  charter (authority boundary, non-authoritativeness, silence has no
  default meaning, technical-identifier format, research retention,
  anti-prescription of communication patterns).
- Converged OTGR protocol v0.1 — `…-coder-tester-r2cl.md`,
  `…-coder-tester-km1h.md`.
- Consolidated Zulip-adapter spec —
  `docs/proposals/otgr-zulip-adapter-technical-specification.md` and its
  cited records (`lunk`, `h8vq`, `gq6m`, `ms1q`, `t34q`, `pksr`): the
  three-layer identity model, the machine-readable trust marker, the
  adapter-enforced safeguards, the raw-log retrieval contract, and the
  voluntary `references` field are adopted from there, re-homed onto the
  purpose-built server.
- `docs/plans/20260909T035056Z-otgr-relay-service.md` — the Gate 1 plan
  this spec details (MR !2587).
- Governor direction, 2026-09-09: purpose-built, human-deployable,
  open-source, multipurpose service; "coordination records are the
  official channel"; "draft specifications for the API and technical comms
  … link use-cases to API and tech-spec sections".
- Governor direction, 2026-09-09 (this revision): **"TLS MAY be used …
  MUST be Tailscale or other tunneled and established, secure-enough
  connection to be used when no TLS, operator responsibility."** →
  [B1.0](#b1-0), [B3.1](#b3-1), [B3.2](#b3-2), [S-8](#part-e)/[S-9](#part-e),
  [UC-11](#uc-11).
- Governor question, 2026-09-09: "can a chat message hold references to
  earlier discussions or data source outside the discussion?" → **yes**;
  [B5.2](#b5-2) `references` widened to typed cross-discussion and opaque
  `external` targets (relay stores, never dereferences), scope is
  [S-10](#part-e).

Any discrepancy between this document and a cited coordination record is an
error in this document; the coordination records and the charter remain the
source of truth. This specification authorizes no implementation,
deployment, or transport selection.
