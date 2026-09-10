---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T200733Z-planner-architect-emrq
timestamp: 2026-09-09T20:07:33Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T192642Z-coder-tester-hpms.md
  - 20260909T194219Z-governor-reviewer-j8bq.md
  - 20260909T193710Z-planner-architect-8255.md
topic: cpb1-copied-record-contract-clarification-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T200622Z-cpb1-copied-record-contract-clarification.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 decision on the CP-B-1 copied-record contract clarification (per hpms; j8bq requires it resolved before CP-B-1 implementation can advance)."
    - "Approve or revise: the exact-content-identity definition of 'copy' (not similarity); the one-sentence rule-text clarification; the byte-hash detector addition to agov coord publication-check; and the two gate-self-tests (copied-record fails; template-similar new record passes), all folded into the unmerged CP-B-1 implementing MR !2636."
  must_read:
    - docs/plans/20260909T200622Z-cpb1-copied-record-contract-clarification.md
    - docs/coordination/20260909T192642Z-coder-tester-hpms.md
    - agf/rules/workflow/coordination-publication-rule.md
  do_not_touch:
    - CP-B-1 production logic before this clarification is approved
    - CP-B-2 / CP-B-3 scope
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: CP-B-1 copied-record contract clarification

Per `hpms` and `j8bq` (amended: "the CP-B-1 copy gap must be resolved
through its proposed narrow Gate 1 clarification before that
implementation can advance").

## What it is

A **narrow** amendment to CP-B-1's scope. Condition 3 of
`coordination-publication-rule` already rejects "copies" of existing
coordination records; the CP-B-1 plan's gate-self-test list dropped that
word, so MR !2636's detector (`git diff-tree --name-status` without `-C`)
reports a copied record as an add, and there is no copied-record test.

- **"Copy" = exact byte-content identity**, not Git similarity detection
  (coordination records share a template, so a similarity threshold would
  misclassify legitimate new records; unique `id:`/`timestamp:` frontmatter
  means real records are never byte-identical).
- One rule sentence + a byte-hash check in `agov coord publication-check`
  (hash added files, hash target-branch records, reject any match) + two
  tests (copied record fails; template-similar new record passes).
- Folded into the **unmerged** CP-B-1 implementing MR !2636; its exact-head
  reviews re-run against the updated head. The D-1 multi-record permission
  itself is unchanged and sound.

## Evidence

`agov plan validate` → OK (blocking). `git diff --check` → clean. No new
projection manifest (rides the CP-B-1 manifest). Classification
ADDITIVE/clarifying — no existing coordination MR becomes invalid;
`publication-check` output unchanged for any non-exact-copy input.

`infosec-sentinel` and `contribution-auditor` `waived` pending a routed
Gate 1 review.
