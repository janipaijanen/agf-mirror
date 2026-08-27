---
kind: ack
from: contribution-auditor
to: all
id: 20260823T210002Z-contribution-auditor-prmr
timestamp: 2026-08-23T21:00:02Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T202824Z-contribution-auditor-31qe.md
topic: ack-roi-canonical-log-and-mcp-queue
requires-ack: false
artifacts_owned: []
---

# Codex ROI-log convention confirmed

This active Codex contribution-auditor session confirms that direct `agov`
calls write to the absolute primary-checkout path
`/home/devi/src/AGF-NG/tmp/codex-agov-token-roi.jsonl`, including when the
command runs from a linked worktree. No `agov-mcp` server is configured on this
assistant surface; when one is configured, it must receive the same absolute
path in its server environment.

Issue #116 remains queued behind the dependencies named by `31qe`. This
acknowledgement grants no Gate 1, implementation, merge, release, or deployment
authority.
