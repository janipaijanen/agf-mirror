---
kind: decision-record
from: planner-architect
to: coder-tester
id: 20260621T151007Z-planner-architect-n2fq
timestamp: 2026-06-21T15:10:07Z
immutable: true
references-to:
  - 20260621T145800Z-coder-tester-6qbm.md
  - 20260621T145950Z-coder-tester-sqkh.md
topic: docs-remediation-slices-1-2-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: docs remediation Slices 1 and 2

Governor approved both slices. Proceeding to merge.

## Slice 1 — docs/governance-safety-fix (Gate 2: `20260621T145800Z-coder-tester-6qbm`)

**Approved.** Merging `docs/governance-safety-fix` → `main` via !61.

Changes accepted:
- `agf/docs/site/docs/start/first-governed-change.md`: removed three
  premature governance record bullets from setup prompt.
- `agf/docs/site/docs/advanced/contracts-and-validators.md`: CI glob
  changed from `[0-9][0-9][0-9][0-9][0-9]-*.md` to `docs/coordination/*.md`.

## Slice 2 — docs/factual-claims-cleanup (Gate 2: `20260621T145950Z-coder-tester-sqkh`)

**Approved.** Merging `docs/factual-claims-cleanup` → `main` via !62
after !61 lands.

Changes accepted:
- `agf/docs/site/docs/adopt/tooling-and-tokens.md`: corrected Windows
  preview status, removed stale token ROI claim, fixed packaged binary
  availability wording.
- `agf/docs/site/docs/advanced/contracts-and-validators.md`: updated
  validator status rows for agent definitions, learning records, InfoSec,
  and release tags to reflect shipped Mix tasks and Rust commands.
