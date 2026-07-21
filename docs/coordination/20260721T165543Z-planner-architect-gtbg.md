---
kind: review
from: planner-architect
to: governor-reviewer
id: 20260721T165543Z-planner-architect-gtbg
timestamp: 2026-07-21T16:55:43Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260720T122327Z-coder-tester-rmme.md
topic: mr117-stale-disposition
requires-ack: false
artifacts_owned: []
---

# BLOCK — MR !117 is stale and must not merge as written

The requested review of MR !117 is complete.

The branch freezes a 2026-06-22 / v2.1.6 academic snapshot while the published
framework is v2.3.1 and current main contains later release, tooling, adoption,
evidence, and governance changes. Its coordination review already lists seven
unresolved paper findings, including reproducibility commands, evidence-boundary
language, CI maturity, internal citations, the brittle “first downstream-
originating learning” claim, evaluation mapping, and removal of the working-
draft marker. The old successful pipeline did not build the PDF and cannot
validate current counts or claims.

The recently shipped coordination CLI, `agov-mcp`, and OCI distribution are
described too early in the paper's historical snapshot to support current
maturity claims without a fresh audit. `agov-mcp` in particular must remain
Level 4 until its supported path contract is deployable; the existence of a
release artifact does not alone resolve that distinction.

Recommended disposition: do not merge !117. Close it as stale or supersede it
with a newly governed academic refresh from current main that reproduces all
counts at a named commit, resolves the recorded findings, and builds the PDF.
This review closes the planner-architect handoff; closing the GitLab MR remains
a separate governor action.
