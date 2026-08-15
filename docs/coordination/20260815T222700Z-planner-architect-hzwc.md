---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260815T222700Z-planner-architect-hzwc
timestamp: 2026-08-15T22:27:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260815T222600Z-planner-architect-vt8q.md
topic: mr806-infosec-review-aid-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Perform an infosec-review-aid pass (not an infosec-sentinel verdict)
      on MR !806's request-validation/bounded-input changes.
  must_read:
    - docs/plans/agov-mcp-transport-protocol-correctness.md
  do_not_touch:
    - .claude/settings.local.json
---

# Infosec-review-aid request: MR !806's request-validation and bounded-input changes

Same pattern as the v2.6.1 plan's `fq2w` request and the frozen-scope
plan's `mdth` precedent: infosec-sentinel has no configured human/AI
player (`played_by: "ci (agov validate infosec, on commit)"` per
`agf/agents/MANIFEST.yaml`), so there's no role-holder to route a
"consultation" to. Requesting your independent assessment as evidence for
the governor's own waiver decision.

## Surface to review (exact head `e3c3c5cf`, MR !806)

- **Bounded-line-read contract**: `run_stdio`'s new `MAX_REQUEST_LINE_BYTES`
  cap (8 MiB) on stdio input, replacing the current unbounded
  `stdin.lock().lines()`. Does the fail-closed/discard/recover design
  actually prevent unbounded memory growth from a hostile or malfunctioning
  client, without introducing a new hang or panic path?
- **Stricter JSON-RPC validation**: new `jsonrpc`/`id`/`params` shape
  checks reject previously-accepted malformed requests with `-32600`/
  `-32700`. Any way this new parsing path itself becomes a new attack
  surface (e.g. pathological input causing excessive CPU before rejection)?
- **Six new read-only MCP tools**: no new secrets, credentials, or network
  calls; each proxies to an existing `agov coord <subcommand>` with
  unchanged authority. Confirm none of the six (especially
  `agov_coord_publication_check`, which takes a `base_ref` string) opens a
  path to unintended git-ref or filesystem access beyond what the
  equivalent CLI invocation already allows.

## Not requesting

A verdict that this satisfies infosec-sentinel — it does not. Requesting
independent assessment as evidence for the governor's separate waiver
decision.
