---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260902T150855Z-coder-tester-t2nj
timestamp: 2026-09-02T15:08:55Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260902T090720Z-coder-tester-skh5.md
topic: ephemeral-deliberation-installation-guide-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the checksum-bound installation and configuration guide for the proposed three-VM ephemeral AI deliberation service.
    - Challenge the stated intention and whether the design can improve speed or quality without becoming a hidden coordination queue, authority surface, reviewer-independence hazard, or availability dependency.
    - Verify the Ubuntu, Tailscale grants and enrollment, digest-pinned Ergo/Podman/Quadlet, loopback bootstrap, SASL-only, TLS fingerprint pinning, WeeChat/tmux, no-history, outage, upgrade, rollback, and removal instructions for correctness and operational safety.
    - Return concrete findings with severity, proposed corrections, and a recommendation on whether a bounded deployment requires a proportionate Gate 1 plan before any service is created.
    - Do not deploy the service or infer implementation authority from this review request.
  must_read:
    - AGENTS.md
    - 20260902T090720Z-coder-tester-skh5.md
    - agf/rules/always/tool-authority-boundary-rule.md
    - agf/rules/workflow/coordination-publication-rule.md
    - https://gitlab.com/-/project/83321848/uploads/9c822d5c2ca83fbc57c741e6db72b085/agf-ai-deliberation-chat-guide.md
  do_not_touch:
    - Tailscale tailnet policy or enrolled nodes
    - any chat-server or client VM
    - .gitlab-ci.yml
    - agf/rules/
    - release branches, tags, or published artifacts
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review the three-VM ephemeral deliberation installation guide

The governor asked Codex to deliver the installation and configuration guide
to Claude for review, including the intended role of the service.

## Intended role

The proposed service is a disposable, non-authoritative drafting room for
Claude and ChatGPT/Codex running on separate client VMs, with Ergo on a third
server and Tailscale as the private transport. Its intended benefit is faster
provisional critique and convergence before a normal coordination file is
written.

It must not become an unofficial coordination channel:

- chat cannot create claims, obligations, evidence, approvals, scope changes,
  merge or release authority;
- every relied-upon outcome must still be faithfully published and validated
  in `docs/coordination/` before downstream action;
- chat unavailability must never block or delay the repository queue;
- material dissent, participation, and reviewer-independence implications must
  survive in the durable record;
- no secret, credential, personal data, proprietary third-party content, or
  unpublished vulnerability detail may enter chat.

The guide makes this fail-soft operationally by disabling server history and
client logging, using one SASL identity per assistant VM, pinning the server
certificate, restricting TCP 6697 through tagged Tailscale grants, and keeping
the file-based path as the unconditional fallback. It also states that IRC
cannot wake an AI model; assistants still inspect it only while active or in
an existing poll cycle.

## Review packet

- Uploaded guide:
  <https://gitlab.com/-/project/83321848/uploads/9c822d5c2ca83fbc57c741e6db72b085/agf-ai-deliberation-chat-guide.md>
- Filename: `agf-ai-deliberation-chat-guide.md`
- SHA-256:
  `e93b8fc862dfd8e2c6609e2ba7be16797fc1aa7132e33a4d46860b5c8ce60981`
- Source date: 2026-09-02
- Scope: operator guide only; no service has been deployed and no governed
  product documentation has been changed.

Please treat the attachment as a review draft. A corrected guide or deployment
must follow the disposition and any required normal gates rather than acting
from this handoff alone.
