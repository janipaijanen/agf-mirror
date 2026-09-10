---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260910T153713Z-planner-architect-es3m
timestamp: 2026-09-10T15:37:13Z
model: claude-sonnet-5
immutable: true
topic: qc1-ledger-gate2-approved-bridged
references-to:
  - 20260910T133145Z-governor-reviewer-q862.md
  - 20260910T143137Z-contribution-auditor-a585.md
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Bridge for an in-session governor decision (2026-09-10, "Gate approved for
QC-1 queue-hygiene (cond. 3)"). Per the bridge-record pattern (q862; the
AGENTS.md addition in MR !2666), this is provisional until the governor
references it; the merge is the irreversible step and waits for a green
exact-head pipeline. Governor: please ack or correct.
-->

# Governor: QC-1 disposition ledger — Gate 2 approved

## Decision (governor, 2026-09-10, in-session)

> "Gate approved for QC-1 queue-hygiene (cond. 3)."

Applied to the **QC-1 disposition ledger** — decomposition item QC-1 of
the queue-closeout work order `20260909T195116Z`, on branch
`plan/qc1-remaining-mr-dispositioning` (MR !2657).

## Exact head

The branch was **rebuilt** to a single `portfolio:` commit
(`bcfce5c8...`) touching only
`docs/portfolio/post-v210-mr-disposition.md` — a mixed coordination +
portfolio range on the earlier head was tripping `agov coord
publication-check`. The QC-1 review correspondence (`g4kq`, `eqdk`,
`sjnk`, `q281`, `pnmm`, `1ke5`, `tdpj`) is on the separate coordination-
only MR !2674. **No ledger content changed in the rebuild** — the rows,
the tally (`ship-in` 5 · `merge-now` 20 · `close-superseded` 5 ·
`park-radar` 20 · `awaiting-governor` 9), and the governor's !973/!965
`park-radar` disposition are exactly as reviewed at head `6a5ab9e9`.

## What this authorises

Merging the QC-1 ledger to `main`. It grants **no** forge closure: each
QC-4 execution batch returns for its own governor decision record. It
satisfies v2.11 release-cut **condition 3** at the "every eligible open MR
carries a recorded disposition" level (`k8bh` / `q862`).

## Merge gate

Merge only on a green `agov:governance:validate` at exact head
`bcfce5c8...`. Auto-merge is armed on MR !2657.
