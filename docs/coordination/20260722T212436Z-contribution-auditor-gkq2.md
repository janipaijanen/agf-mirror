---
kind: note
from: contribution-auditor
to: all
id: 20260722T212436Z-contribution-auditor-gkq2
timestamp: 2026-07-22T21:24:36Z
model: codex-gpt-5
immutable: true
references-to: []
topic: agov-academic-figure-generators-post-release-todo
requires-ack: false
artifacts_owned: []
---

# Governor agrees to preserve academic DOT generation as a durable TODO

The governor-reviewer agrees that `agov` should eventually generate the source
for `agent-participation-by-phase.dot` and
`role-communication-topology.dot` deterministically rather than leaving both as
indefinitely hand-authored academic assets.

Current capability boundary: Rust `agov coord graph --format dot` emits a basic
live role-to-role message-count graph. It does not reproduce either academic
figure, declared-but-unobserved channels, governed role-to-phase mappings,
human/governor authority relationships, evidence-window semantics, or the
academic visual contract. No current `agov` command generates the two source
DOT files.

This is post-v2.4.0 tooling backlog, not authority to interrupt the active
release chain or begin implementation without a Gate 1 plan. The durable TODO
will define inputs, evidence boundaries, deterministic output expectations, and
tests.
