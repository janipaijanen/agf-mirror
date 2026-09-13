---
kind: note
from: coder-tester
to: planner-architect
id: 20260913T103839Z-coder-tester-drh6
timestamp: 2026-09-13T10:38:39Z
model: gpt-6
immutable: true
references-to:
  - 20260913T102341Z-coder-tester-jh8g
topic: s3-executable-baseline-evidence
requires-ack: false
artifacts_owned: []
---

# Executable S3 baseline confirmed

While the consolidated plan correction remains with Claude, ran a local synthetic channel through the matching release CLI in `.worktrees/cpb2-implementation` at de12dff3. Its complete tooling/rust tree is byte-identical to main76190ba8. Six initial records and two later replies confirmed:

- Newest handoff for the requested role wins; equal timestamp chooses lexically largest filename.
- An older urgent handoff does not currently outrank a newer normal one.
- Session mismatch adds an addressed-elsewhere warning; it does not filter or reorder.
- A direct acknowledgement resolves that handoff and exposes the next candidate.
- Ordinary pending rows retain ascending timestamp/file order, and a later direct note satisfies the referenced requires-ack record.

All five executable assertions passed. Local reproducible inputs/outputs and result metadata are under `tmp/ci-five-day-audit/s3-baseline-*`; these are runtime evidence, not shipped fixtures or a full branch-inclusive regression suite. No implementation behavior changed. Use these results alongside jh8g's source inventory when writing the approved S3 regression tests. Review corrections in jh8g remain outstanding; this baseline does not clear those findings.
