---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260902T090720Z-coder-tester-skh5
timestamp: 2026-09-02T09:07:20Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260902T063431Z-coder-tester-mjak.md
  - 20260902T064458Z-planner-architect-a7jr.md
topic: ephemeral-ai-deliberation-channel-service-recommendation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Challenge the Ergo IRC recommendation and the authority, availability, identity, security, independence, and publication boundaries below.
    - Recommend whether to run a bounded local experiment and whether that experiment needs a Gate 1 plan before deployment.
    - Do not deploy a service or create product integration from this handoff.
  must_read:
    - AGENTS.md
    - agf/rules/workflow/coordination-publication-rule.md
    - agf/rules/always/tool-authority-boundary-rule.md
  do_not_touch:
    - .gitlab-ci.yml
    - agf/rules/
    - release branches, tags, or published artifacts
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Recommend a disposable Ergo IRC deliberation channel for a bounded experiment

The governor proposes a non-authoritative chat where AI assistants can refine
questions and prepare normal coordination records. The chat may be unavailable
at any time. Coordination files remain the sole project record and every
existing gate remains unchanged.

## Recommendation

Use a small self-hosted **Ergo IRC** instance for the first experiment, not
Discord or Matrix.

Ergo is a modern IRCv3 server with first-class SASL account authentication,
message tags, labelled responses, WebSocket support, and optional chat
history. Its official manual says history can be an in-memory buffer or a
persistent database; persistence is explicitly configurable. That permits the
experiment to disable persistent history and retain, at most, a short volatile
buffer.

Primary sources:

- <https://ergo.chat/specs/>
- <https://github.com/ergochat/ergo/blob/master/docs/MANUAL.md>
- <https://ircv3.net/specs/extensions/chathistory>

Matrix is a poor first fit because its homeserver and federation model is
designed around stored and replicated room events/history. Discord is a cloud
dependency whose bot path requires authenticated Gateway/HTTP integration,
connection state and rate-limit handling. Both may be valuable for a future
human-facing community, but they add capabilities and failure modes that this
disposable internal experiment does not need.

Primary comparison sources:

- <https://spec.matrix.org/latest/server-server-api/>
- <https://docs.discord.com/developers/events/gateway>
- <https://docs.discord.com/developers/topics/rate-limits>

Evidence status: **Theoretical**, informed by current protocol and server
documentation. No AGF assistant-to-assistant chat experiment has yet been run.

## Required boundary

Name the facility an **ephemeral deliberation channel**, not an unofficial
coordination channel. It has no authority and no delivery guarantee.

- Chat may refine a question, challenge an assumption, divide read-only
  investigation, or draft a coordination record.
- Chat may not create a claim, authorize a file change, expand scope, approve
  a gate, assign a binding obligation, merge, release, or supply acceptance
  evidence.
- A participant must publish and validate every relied-upon outcome through
  `docs/coordination/` before anyone acts. Published coordination wins on any
  conflict.
- A material disagreement, uncertainty, participant list and designated
  recorder must survive in the published summary. Consensus laundering is a
  failure.
- A reviewer who materially co-designed the work in chat must disclose that
  participation and cannot be represented as independent for that work.
- No secrets, credentials, personal data, proprietary third-party content, or
  unpublished vulnerability detail may enter chat.
- If the server or a client is unavailable, work falls back immediately to the
  repository queue. No task waits for chat, and reconnecting agents do not
  infer obligations from missed history.

Operationally: listen only on loopback for one-host use; use an authenticated
private network and TLS for cross-host use; require one SASL account per
assistant instance; disable federation and public discovery; use invite-only
topic channels; disable server-side persistent history; and ensure local
client logs are either disabled or explicitly treated as disposable drafts.
Identity should name the assistant instance and currently assumed role rather
than treating a model nickname as authority.

Suggested protocol:

1. Check official claims and coordination before joining.
2. Open or join `#agf-draft-<topic>` and state the intended durable output.
3. Keep discussion bounded by time or message count.
4. Name one recorder before concluding.
5. Publish one faithful coordination summary, including dissent.
6. Obtain normal acknowledgement where required; only then act.
7. Destroy the topic channel without treating its transcript as evidence.

## Experiment and stop conditions

If approved, run three low-risk, read-only deliberations. Measure time to a
visible record, correction rounds, duplicated investigation, omitted dissent,
premature action, and reviewer-independence disclosures. Compare against three
similar file-only coordination exchanges. Stop immediately if any participant
acts from chat before publication, a secret enters chat, a reviewer is falsely
represented as independent, or chat availability becomes a work dependency.

This affects governance workflow, identity and a network service. A deployment
or AGF/tool integration should therefore receive its own proportionate Gate 1
plan and security boundary. This handoff asks for critique only and carries no
deployment authority.
