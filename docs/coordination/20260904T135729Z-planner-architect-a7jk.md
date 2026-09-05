---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260904T135729Z-planner-architect-a7jk
timestamp: 2026-09-04T13:57:29Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260904T104400Z-governor-reviewer-q7mv.md
  - 20260904T112320Z-coder-tester-r2cl.md
  - 20260904T112728Z-coder-tester-km1h.md
topic: otgr-technical-specification-independent-claude-proposal
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Have the active Claude session independently propose an OTGR chat-service technical specification suitable for review by chat-service creators.
    - Start from requirements and trust boundaries; do not select SSH, HTTPS, WebSocket, storage, signatures, hash chains, or another mechanism before comparing alternatives.
    - "Use only the stated available primitives: host CAs, users' login public keys, and a requirement to fetch raw discussions or discussion logs from the chat server."
    - Identify ambiguities that materially change the design, especially what kind of host CA and login key system exists, and return explicit questions or conditional branches rather than inventing answers.
    - Cover protocol operations, identity and authorization, raw-log fidelity and retrieval, ordering and pagination, edits/deletes, retention and access, failure semantics, threat model, observability, versioning, and acceptance tests.
    - Separate consensus requirements, recommended design, alternatives rejected with reasons, open governor decisions, and implementation-deferred matters.
    - Critique the prior Codex answer for premature architecture choice and any other unsound assumptions; do not preserve it merely for consensus.
  must_read:
    - docs/coordination/20260904T104400Z-governor-reviewer-q7mv.md
    - docs/coordination/20260904T112320Z-coder-tester-r2cl.md
    - docs/coordination/20260904T112728Z-coder-tester-km1h.md
  do_not_touch:
    - chat-service implementation or deployment
    - server addresses, credentials, accounts, or secrets
    - AGF role authority, gates, release branches, tags, or published releases
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Request: independent Claude proposal for the OTGR technical specification

The governor rejected Codex's first technical proposal as poor quality and
explicitly directed: “Ask Claude.” This handoff requests that independent
proposal without treating the rejected answer as a baseline to polish.

The design inputs currently stated by the governor are deliberately small:

- hosts have CAs;
- users have login public keys;
- clients need a mechanism to fetch the raw discussion or discussion logs
  from the chat server.

Do not infer that “host CA” means an SSH host CA rather than a TLS/private PKI
CA, or that “login public keys” implies SSH transport. First establish the
trust model and required service behavior. Where an ambiguity changes the
architecture, provide conditional designs or a short governor question.

The output should be concrete enough for chat-service creators to estimate and
challenge, but it remains a proposal: no implementation or transport selection
is authorized by this handoff. Preserve the converged OTGR governance and
research invariants, and make clear which technical controls are required to
satisfy them rather than adding attractive but unmotivated machinery.

This request is analysis-only and must not take over the separately assigned
v2.9.0 release execution slice. If release recovery becomes executable first,
record that priority and return to this proposal afterward.
