---
kind: decision-record
from: governor-reviewer
to: all
id: 01KV6KSASMMQ5EH22M5FTSPXEK
timestamp: 2026-06-15T21:42:45Z
immutable: true
references-to:
  - 00245-coder-tester.md
  - 00242-coder-tester.md
  - 00089-coder-tester.md
  - 00088-coder-tester.md
  - 00073-coder-tester.md
  - 00071-coder-tester.md
  - 00079-coder-tester.md
  - 00231-coder-tester.md
  - 00228-coder-tester.md
  - 00075-governor-reviewer.md
  - 00080-governor-reviewer.md
  - 00102-governor-reviewer.md
  - 00103-governor-reviewer.md
  - 00115-governor-reviewer.md
  - 00233-planner-architect.md
  - 00277-governor-reviewer.md
topic: stale-handoff-reconciliation-2026-06-15
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Decision: reconcile stale handoff queue entries

Governor-reviewer reconciles the old pending handoff entries still shown by
`mix agov.session.resume`.

This record does not reopen or change historical work. It records the current
disposition of old sequence-era handoffs whose durable decisions were already
made in later coordination records, but whose queue visibility remained noisy.

## Governor-reviewer queue

- Seq 245 `tier1-provenance-labels-gate2-final`: closed. The provenance-label
  branch was later merged; seq 277 records the relevant Gate 2 approval for the
  schema consolidation on `feat/tier1-provenance-labels`.
- Seq 242 `tier1-provenance-labels-gate2-corrected`: superseded by seq 245 and
  now closed by the later provenance-label disposition.
- Seq 89 `agents-govern-2.0-gate1`: superseded by the consolidated Gate 1
  review bundle at seq 98 and approved by seq 103; later Gate 2 approval and
  merge are recorded at seq 115 and seq 116.
- Seq 88 `dist-foundry-replace-on-upgrade-gate2`: approved by seq 102; no
  further action needed in the current AGF v2 queue.
- Seq 73 `downstream-impact-rule-gate2`: completed by seq 75; branch
  `feat/downstream-impact-rule-v2` was merged.
- Seq 71 `downstream-impact-rule-gate2`: superseded by seq 73 after the seq 72
  blocker review; closed by seq 75.
- Seq 79 `T2-8-small-task-fast-path-gate1`: approved by seq 80; no further
  action needed in the current queue.

## Planner-architect queue

- Seq 228 `agf-v2-inherited-material-verification-policy`: planner response
  filed in seq 233. The tiered verification model is accepted as advisory
  framing already reflected in later provenance work; any additional tooling or
  release-policy change still requires its own Gate 1 plan.
- Seq 231 `shareable-workitems-mechanism`: planner response filed in seq 233.
  The manifest direction is accepted as advisory backlog. No immediate
  implementation is authorized by this reconciliation record; a future rule,
  schema, or tooling slice must come through normal Gate 1.

## Current disposition

All handoffs listed above are considered closed, superseded, or deferred to
future explicitly approved plans. They should no longer be treated as active
queue items.
