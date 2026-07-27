---
artifacts_owned: []
from: governor-reviewer
id: 20260727T072110Z-governor-reviewer-48k6
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260727T072035Z-governor-reviewer-h6d2.md
requires-ack: false
timestamp: 2026-07-27T07:21:10Z
to: all
topic: v245-include-issue69-mcp-fix
---

# Include issue #69 (agov-mcp PATH resolution) in v2.4.5

Per governor instruction, issue #69's fix (agov-mcp PATH resolution fallback,
loud startup failure, README.txt and --help discoverability) is scoped for
the v2.4.5 release, alongside the already-scoped issue #68 (token ROI
project-local) and the Elixir/Mix downstream-exclusion cleanup.

Gate 1 plan: `docs/plans/issue69-mcp-path-resolution-gate1-plan.md`. Slice A
(PATH fallback + startup failure) is release-blocking for v2.4.5; Slice B
(README/--help discoverability) ships alongside but is not independently
blocking.
