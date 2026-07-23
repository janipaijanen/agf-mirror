---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read: []
  next_agent: planner-architect
  status: ready
  task:
  - Install agov and agov-mcp v2.3.1 from the same published GitLab release archive and report the installed binary SHA-256 values.
id: 20260722T002509Z-coder-tester-4mkp
immutable: true
kind: handoff
model: codex-gpt-5.6-sol
references-to: []
requires-ack: true
timestamp: 2026-07-22T00:25:09Z
to: planner-architect
topic: agov-mcp-install-version-sha-parity
---

# Message

The governor requested that Codex and Claude use the same `agov-mcp` version,
installed from the same source, with binary SHA-256 parity verified.

Codex installed both executables from the published GitLab Release v2.3.1
Linux x86_64 musl archive:

- Release archive: `agov-2.3.1-x86_64-unknown-linux-musl.tar.gz`
- Release URL: `https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.3.1`
- Published and locally verified archive SHA-256:
  `4ae5a8c3070065252d8117a38e688ba5f1deb0fa699f5fdadd5c916c2259713b`
- Installed `agov` SHA-256:
  `196157bcc59fbe03a57b74b7bb2b5aa366ff98e39e2a7915b960fbc329ec24d4`
- Installed `agov-mcp` SHA-256:
  `f948e8879f2bbbe72721ec2c06754a7fbc82232ac10fd78900502e46f996324f`
- Runtime checks: `agov 2.3.1` at source commit
  `1f7359896e81edcc7f092f55d156c6be9b2eecdc`; MCP initialize reports
  `agov-mcp` version `2.3.1`.

Claude/planner-architect: install the same two binaries from that exact release
archive, independently verify the archive checksum against the published
`.sha256` asset, then reply with a validated ACK or completion record containing
the installed paths, version output, and SHA-256 values for both binaries. If
either binary hash differs, stop using the installation and report the mismatch
instead of treating version-string equality as sufficient.
