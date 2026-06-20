---
kind: note
from: planner-architect
to: governor-reviewer
id: 01KVK0R5S4C65RST2ZX5CF6G7J
timestamp: 2026-06-20T17:20:10Z
immutable: true
references-to:
  - 01KVJZBMN8FRGPZZYPAQDAW3MF-planner-architect.md
  - 01KVK0BC3EM069SBJRTBHZVBZ5-coder-tester.md
topic: rust-agov-port-saved-token-evidence-amendment
requires-ack: false
model: codex-gpt-5
artifacts_owned:
  - docs/plans/rust-agov-port.md
  - MR_DESCRIPTION.md
---

# Rust agov saved-token evidence amendment

Updated `docs/plans/rust-agov-port.md` so Phase 1 includes local saved-token
evidence logging for Rust validation commands.

The amendment requires successful Phase 1 validation commands to append
Elixir-compatible token ROI JSONL records unless disabled. The default path is
repository-root `tmp/agov-token-roi.jsonl`; `AGOV_TOKEN_ROI_LOG` overrides it;
`AGOV_TOKEN_ROI=0`, `false`, or `off` disables it.

The log remains local-only and must not store prompts, command output bodies,
model responses, secrets, billing data, or remote telemetry. Existing Elixir
`mix agov.token.roi report` remains the consumer; a Rust-native token ROI
report command is not authorized in Phase 1.

`MR_DESCRIPTION.md` was updated to surface the new Gate 1 authorization.
