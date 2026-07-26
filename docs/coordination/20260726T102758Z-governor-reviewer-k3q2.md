---
kind: fast-track-waiver
from: governor-reviewer
to: planner-architect
id: 20260726T102758Z-governor-reviewer-k3q2
timestamp: 2026-07-26T10:27:58Z
model: human-governor
immutable: true
references-to:
  - 20260726T102147Z-planner-architect-mhh1
  - 20260726T101903Z-governor-reviewer-hemp
topic: mr574-fast-track-waiver
requires-ack: true
artifacts_owned:
  - AGENTS.md
---

# Fast-Track Waiver — MR !574

Task: document the existing `agov coord list --since` command and configured
`agov-mcp` preference in polling guidance.

Why Gate 1 is waived: bounded operator-documentation clarification of existing
tools and cursor behavior; no schema, authority, security, runtime, data-model,
or adopter-contract change.

Who waived: Jani, human governor-reviewer, by explicitly approving both
dispositions in chat on 2026-07-26.

Risk assessment: low. The Rust CLI exposes `agov coord list --since`; MCP use
is conditional on tool availability with CLI fallback. Exact SHA
`2efeff51894802b50d3f4a893d46f646fbec99ca` passed canonical MR pipeline
`2706454747`.

Required verification at Gate 2: exact diff remains documentation-only,
command syntax is accurate, fallback behavior is explicit, and governance
validation passes.

Revert to full plan if: implementation, authority, mandatory polling scope, or
tool contracts must change.

The missing fast-track commit-message provenance is corrected by this
canonical waiver and the linked review trail. Do not rewrite the green exact
SHA solely to alter its subject; preserving the reviewed SHA is the lower-risk
correction directed by the governor.
