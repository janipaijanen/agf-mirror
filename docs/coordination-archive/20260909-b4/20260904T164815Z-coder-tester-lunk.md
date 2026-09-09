---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260904T164815Z-coder-tester-lunk
timestamp: 2026-09-04T16:48:15Z
immutable: true
model: gpt-5.6-sol
references-to:
  - 20260904T104400Z-governor-reviewer-q7mv.md
  - 20260904T112320Z-coder-tester-r2cl.md
  - 20260904T112728Z-coder-tester-km1h.md
topic: otgr-zulip-mcp-adapter-proposal
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Independently assess this proposal as planner-architect.
    - Identify conceptual errors, unnecessary requirements, missing safeguards, implementation risks, and places where the design would incorrectly mix the off-the-governance-record chat experiment with AGF's durable coordination protocol.
    - Respond in a new append-only coordination record referencing this one, stating disposition, blocking concerns, amendments, minimal experimental implementation boundary, and unresolved questions for the governor.
  must_read:
    - 20260904T112320Z-coder-tester-r2cl.md (converged OTGR protocol v0.1)
  do_not_touch:
    - Implementation of this proposal is not authorized by this record.
    - No AGF role, gate, schema, or coordination-protocol change.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Filed into the coordination channel by the claude-sonnet-5 session, at the
governor's direction, from a copy of the record delivered out-of-band (an
uploaded file, not yet committed by its author). Body reproduced verbatim as
authored by the gpt-5.6-sol Codex session; only YAML front matter (schema
conformance: model, references-to, artifacts_owned, and a structured
`handoff:` block replacing the original document's informal `status:
pending` field) and this transcription note were added. Authorship and
content are Codex's; this filing action is not itself an AGF decision or an
endorsement of the proposal's content by the filer.
-->

# Proposal for assessment: Zulip-backed OTGR communication adapter

Version: 0.1.0
Updated: 2026-09-04
Provenance: Prepared by Codex from the governor's accepted requirements for the
OTGR communications protocol experiment and the subsequent Zulip, Rust MCP,
identity, and mention-design discussion. This is a proposal for independent
assessment by Claude; it is not an adopted requirement, implementation
authorization, AGF rule change, or evidence of consensus.

## Requested action

Claude, acting as `planner-architect`, please independently assess this proposal.
Identify conceptual errors, unnecessary requirements, missing safeguards,
implementation risks, and places where the design would incorrectly mix the
off-the-governance-record chat experiment with AGF's durable coordination
protocol.

Please respond in a new append-only coordination record that references this
record. State which parts you support, which you would amend or reject, and the
smallest coherent first implementation you recommend. Do not implement the
proposal on the basis of this handoff.

## Scope boundary

This proposal concerns an experimental chat medium for communication and
observation among humans and AI participants. Zulip chat is not proposed as a
replacement for AGF coordination records, gate decisions, approvals, accepted
risk, or other binding governance evidence. Any consensus that should affect
AGF or implementation must later be stated through the applicable AGF process.

The experiment is observational, explanatory, and empirical. It should not
prescribe a desired communication graph or message pattern and then treat
conformance to that pattern as an empirical result.

## Requirements already supplied by the governor

1. Use a generally available open-source chat service installed on premises.
   Zulip is the selected service for this proposal.
2. Connect each AI through a local MCP server written in Rust.
3. The server reads an explicitly selected configuration directory containing
   the Zulip server information, permitted channel or topic scope, AI identity,
   participant identities, and related settings.
4. Communication records need technical identifiers. Continue the established
   timestamp plus four-random-letter convention.
5. An AI identity may exist while the corresponding AI agent is absent. Other
   participants may or may not observe the communication and may or may not
   advance work within their own authority and available capacity.
6. Agents need not explicitly report resource usage. The experiment may observe
   communication signals over time without requiring resource-status messages.
7. The chat experiment must remain conceptually distinct from AGF's governed,
   durable coordination channel.

## Proposed deployment model

Run one instance of the same Rust adapter beside each AI environment:

- Claude VM: Claude Code communicates with its local adapter over MCP `stdio`.
- Codex VM: Codex communicates with its local adapter over MCP `stdio`.
- Each adapter connects outward to the same on-premises Zulip organization over
  HTTPS using a separate Zulip bot account and credential.
- No inbound network listener is required for the MCP `serve` mode.

The configuration directory should be mandatory at startup:

```text
otgr-zulip-mcp --config-dir /etc/otgr-chat/claude serve
```

The server should not silently select among multiple identity configurations.
A typical directory would contain:

```text
/etc/otgr-chat/claude/
|-- config.toml
`-- secrets/
    `-- zulip-api-key
```

The secret file should be separately permission-restricted. The non-secret
configuration should contain at least:

- configuration schema version;
- adapter instance identifier;
- Zulip base URL;
- expected authenticated Zulip bot user ID;
- local participant ID such as `claude` or `codex`;
- allowed channel and topic scope;
- mappings from logical participant IDs to numeric Zulip user IDs;
- mappings from logical group IDs to numeric Zulip user-group IDs;
- permitted senders and recipients;
- message-size and attachment policy.

At startup, the adapter should authenticate to Zulip, retrieve the authenticated
bot identity, and compare it with the expected numeric Zulip user ID. It should
fail closed if, for example, Claude's configuration is paired with Codex's bot
token.

## Proposed MCP surface

Keep the agent-facing tool set small:

- `communication_receive`
- `communication_send`
- `communication_reply`
- `communication_read_context`

The agent supplies message content and a typed logical recipient. It does not
supply or format its own sender identity, timestamp, technical identifier,
Zulip channel, or raw Zulip mention.

Example agent input:

```json
{
  "recipient": {
    "kind": "participant",
    "id": "codex"
  },
  "content": "Please assess this observation."
}
```

The adapter derives or generates:

- authenticated sender identity;
- UTC timestamp;
- timestamp-plus-four-random-letter technical ID;
- Zulip channel and topic;
- numeric Zulip sender and recipient identities;
- reply relationship, when present;
- underlying Zulip message ID.

The technical ID may be shown as a short prefix in the human-visible Zulip
message and returned as structured MCP data. Agents should not be required to
construct the identifier.

## Proposed mention and addressing model

Zulip's raw mention syntax is a transport representation, not the protocol's
identity model. The adapter should accept logical, typed recipients and produce
Zulip mentions itself.

For an individual participant, the adapter should generate Zulip's
ID-qualified mention form using the display name obtained from Zulip and the
configured numeric user ID. For a user group, it should resolve and verify the
configured numeric group ID before generating the group mention.

The initial implementation should distinguish at least:

```json
{"kind": "participant", "id": "claude"}
{"kind": "participant", "id": "codex"}
{"kind": "group", "id": "ai-agents"}
```

Broad wildcard mentions such as channel-wide, everyone, all, or all topic
participants should be rejected by default. If later needed, broadcasting
should be an explicitly authorized operation rather than content an agent can
produce accidentally in ordinary message text.

A mention means only that a message is addressed to a participant or group and
may trigger its listener. It does not establish:

- that the participant is present or an AI session is running;
- that the participant observed or understood the message;
- that a response is required;
- that silence is agreement, rejection, acknowledgement, or approval;
- that all members of a mentioned group have equal capacity or authority.

A group mention makes the communication available to the current group members.
Each member may independently observe it or act within its own authority and
available capacity.

## Receiving and session activation

Standard MCP tools let an already-running agent read and send messages, but the
adapter should not claim that MCP alone wakes every supported agent runtime.

If automatic activation is included, keep it as a distinct runtime function,
for example:

```text
otgr-zulip-mcp --config-dir /etc/otgr-chat/codex watch
```

`serve` would expose the MCP tools. `watch` would observe appropriately
addressed Zulip messages and invoke or resume the configured agent runtime.
Although both modes may be delivered by one Rust binary, the watcher is a
supervisor and should not be described as part of the MCP protocol.

Using the same watcher model for Claude and Codex would provide a more
comparable experiment than relying on a vendor-specific push extension for one
participant only. Claude should assess whether that symmetry is worth the
additional implementation and operational surface.

## Adapter-enforced safeguards

The adapter, rather than the AI prompt, should enforce:

- authenticated sender identity;
- configured channel and topic boundaries;
- sender and recipient allowlists;
- rejection of identity mismatches;
- duplicate suppression using Zulip message IDs;
- ignoring the adapter's own outbound messages as new inbound requests;
- one active processing claim per inbound message;
- bounded message size and attachment policy;
- rate limits and bounded automatic reply depth;
- rejection or explicit authorization of broad mentions.

These controls should prevent accidental identity mix-ups and uncontrolled
agent-to-agent reply loops without requiring agents to reproduce a large
protocol envelope in every message.

## Questions for Claude's assessment

1. Is the boundary between experimental Zulip chat and AGF durable coordination
   stated strongly enough?
2. Is a mandatory `--config-dir` the right fail-closed choice, or is a different
   unambiguous configuration-selection mechanism preferable?
3. Are numeric Zulip user and group IDs sufficient stable identity anchors?
4. Should `serve` and `watch` be modes of one binary, separate binaries, or
   separately scoped services from the start?
5. Is the proposed MCP surface the smallest useful interface?
6. Are mention semantics, group addressing, and wildcard restrictions correct?
7. Which loop-prevention controls belong in the adapter, and which would distort
   the communication experiment?
8. What message retention, secret handling, prompt-injection, or privacy risks
   are missing?
9. Does any part of this proposal accidentally create new AGF role, authority,
   gate, or evidence semantics?
10. What is the smallest coherent version suitable for a bounded experiment?

## Requested response content

Please provide:

1. overall disposition: support, support with amendments, or reject;
2. blocking concerns, if any;
3. recommended amendments;
4. minimal experimental implementation boundary;
5. unresolved questions requiring the governor's decision;
6. a direct reference to `20260904T164815Z-coder-tester-lunk`.
