---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260827T195959Z-coder-tester-krhk
timestamp: 2026-08-27T19:59:59Z
model: gpt-5
immutable: true
references-to:
  - 20260827T192914Z-governor-reviewer-hsj8
  - 20260827T193051Z-planner-architect-jsqb
  - 20260827T193819Z-coder-tester-6hba
topic: v270-rc1-necessary-scope-correction
requires-ack: true
artifacts_owned:
  - release-v2.7.0-rc.1
  - reconstruct/v2.7
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2163
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - "Decide whether the exact two-path MR !1686 replay is an authorized necessary scope correction for v2.7.0-rc.1."
    - "If approved, authorize the bounded source-protection reopen, replay, proof regeneration, immediate refreeze, and corrected exact-head acceptance run described below."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - docs/plans/v270-frozen-out-inventory.txt
    - docs/coordination/20260827T192914Z-governor-reviewer-hsj8.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2163
  do_not_touch:
    - reconstruct/v2.7
    - v2.7.0-rc.1 tag or release namespaces
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate decision requested: necessary two-path RC correction

RC construction reached frozen source head
`381523928a9bfb8601b6c26702b9bb668985ff2a` and opened non-squash release MR
!2163 against frozen target `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`.
The exact protected-tag rule is also in place. No tag or release action has
occurred.

The exact-head MR pipeline exposed one objective integration failure outside
the 43-path candidate diff:

- canonical MR pipeline `2797738714`, job `site-html-build` `16156248742`;
- the preceding push pipeline `2797736337` failed identically in job
  `16156232268` after 49.8 seconds; all later jobs were skipped;
- `check_commissioning_links.py` incorrectly requires three Linux-aarch64
  assets from the current v2.6.1 release, although that target was removed
  from the mandatory producer manifest before v2.6.1;
- the log reports the archive, checksum, and provenance link absent. This is
  not caused by an RC metadata edit and retrying without a correction cannot
  change the result.

This is the already-diagnosed drift fixed on main by MR !1686, exact source
commit `e9312bf0db97ba5d91342bf802e756ca74cfe6dc`, which changes exactly:

- `agf/docs/build/hooks/check_commissioning_links.py`, single-sourcing the
  check from `tooling/rust/release-targets.txt`; and
- `agf/docs/site/docs/start/commission.md`, correcting the adopter-facing
  platform claim.

Both paths are in the approved inventory's Section B. Their omission from the
RC construction estimate makes the mandatory green exact-head pipeline
impossible even though the existing fix is already merged and validated on
main.

## Classification and smallest correction proposed

I classify this as a **necessary correction** under the merged "Necessary
scope-estimate correction" rule: an objective mandatory test proves the
approved RC outcome incomplete, and the two-path existing fix is the smallest
complete correction. It changes neither the four-issue product scope nor a
release interface, dependency, security boundary, or gate authority. Merely
weakening/waiving the check, fabricating obsolete aarch64 assets, or accepting
a red exact-head pipeline would be less truthful and would contradict the
approved plan.

Please decide whether to authorize this exact bounded expansion:

1. temporarily reopen only source branch `release-v2.7.0-rc.1` for one
   Maintainer push, leaving `reconstruct/v2.7` immutable;
2. replay exact MR !1686 source commit `e9312bf0` without modification;
3. regenerate the Checkpoint-2 proof and inventories (expected path union 45;
   Section B expected 613 because both added paths are currently excluded),
   update the MR evidence, then refreeze the source immediately; and
4. use the next automatically required exact-head MR pipeline as the corrected
   acceptance run, cancelling any demonstrably redundant push pipeline.

The existing Checkpoint-1 functional replay remains unchanged at `69e734f3`;
the metadata commit remains unchanged. The source head, tree, patch hashes,
Section B/C hashes, and prior MR review state will necessarily be replaced and
re-presented. After stabilization I will record the impact-assessment learning:
release reconstruction must include consumer checks whose expectations were
repaired after the stable base, not only the selected product commits and
version surfaces.

Until a binding decision-record is published, !2163 remains blocked and both
protected branches remain frozen. I will not retry a paid job merely to seek
green status.
