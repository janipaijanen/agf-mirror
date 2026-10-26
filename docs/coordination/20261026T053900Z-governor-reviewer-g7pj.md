---
kind: decision-record
from: governor-reviewer
to: all
id: 20261026T053900Z-governor-reviewer-g7pj
timestamp: 2026-10-26T05:39:00Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260910T153713Z-planner-architect-es3m
  - 20260918T162104Z-governor-reviewer-reb1
  - 20261021T212208Z-contribution-auditor-p33h
  - 20261021T212216Z-infosec-sentinel-1sjh
  - 20261021T212927Z-governor-reviewer-ntnh
  - 20261022T052553Z-governor-reviewer-qbsb
  - 20261022T201939Z-governor-reviewer-kmhq
  - 20261022T211654Z-governor-reviewer-k6k5
  - 20261022T215322Z-coder-tester-nnmm
  - 20261022T215633Z-coder-tester-hqr3
  - 20261022T215915Z-coder-tester-pde7
  - 20261022T220158Z-planner-architect-jg64
  - 20261023T143019Z-coder-tester-grdo
  - 20261023T150025Z-coder-tester-sdtp
topic: v211-release-disposition-ratification
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3066
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3098
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3100
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3102
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3103
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3104
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3120
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11 release audit ratification and current open-MR dispositions

The human governor delegated full governor-reviewer authority to Codex in the
live session until 2026-10-26 18:00 EET (`2026-10-26T16:00:00Z`) and directed
the release work to proceed. Exercising that bounded authority, this record
closes the durable-authority gaps found by the v2.11 pre-cut audit and assigns
terminal dispositions to every currently open, non-coordination MR that
postdates the existing QC-1 ledger.

## Ratifications

1. `es3m` is ratified. The QC-1 base disposition ledger merged as
   `4cc016ad` is accepted as the governor-recorded disposition decision for
   its enumerated MRs. `reb1` remains the binding decision for the later
   2026-09-18 delta. This ratification changes no row and does not accelerate
   the separately governed post-release close/park/reconstruct execution.
2. MR !3102 exact head `25ab420c22a335fb1a68b44a2bc4e8ff1e6a3a78`
   and its merge commit `7aab12acc97f6ae4dafb5c253d51af798242c66a`
   are accepted. `k6k5` is ratified; exact-head pipeline `2978298338` passed.
3. MR !3120 exact head `6bf5cef6f06d71725c78d299e308572a98868a2d`
   and its merge commit `f58b1d2e82c577e18b4914f05036f84d0c0fdb5a`
   are accepted at Gate 2. Reviews `nnmm`, `hqr3`, and `pde7` and handoff
   `jg64` are accepted; exact-head pipeline `2978548530` passed. This is the
   missing governor-authored ratification requested by `grdo`.

The chronology defect identified by `grdo` is acknowledged. These
ratifications repair the durable audit trail; they do not make a
receiving-assistant bridge valid pre-action authority and do not establish a
precedent for merging before the governor record is visible.

## Current terminal dispositions

| MR | Exact head | Binding disposition and evidence |
|---|---|---|
| !3066 | `f7c1c8ec379bc61fa1b8e1fe4ea42fc4f1731367` | **`awaiting-governor`**. The measurement-first distribution-scan plan is ready for a later Gate 1 choice: approve, defer, or close. It is not v2.11 release scope. |
| !3098 | `6fccd783116bef86a908b4e53eadf9fd32d67b14` | **`awaiting-governor`**. Its three skill-document edits are outside the narrow `docs/sources.md` / `docs/radar.md` research-maintenance exception and need a normal Gate 1 or an explicit deferral. It is not v2.11 release scope. |
| !3100 | `7399490eb3e82f266362504b89ed00c3117eeced` | **`merge-now`**. The DinD-removal plan and its two initial coordination records are approved for Gate 2 after `ntnh`, `p33h`, `1sjh`, and the `kmhq` amendment are visible on `main`. Exact-head pipeline `2978387074` passed. Merge is authorized after this record and those audit records are published. |
| !3103 | `74f536b34a8712e07c3677dc6c25fce05ffc2f97` | **`close-superseded`**. The failed Windows Podman spike remains preserved in the MR, and its conclusion is incorporated by the Windows-retirement plan and implementation now on `main`. |
| !3104 | `107de3cb036c78c279780d31a7bb7329acb89e88` | **`close-superseded`**. The approved Windows-retirement plan is already present on `main` in its fuller implemented form through MR !3105; this older plan branch now conflicts. |

Coordination-only MRs remain covered by the publication carve-out and are
being consolidated onto `main` so the shipped-work audit trail is not
stranded. The still-open `park-radar` rows from the QC-1 base ledger may be
executed after verified v2.11 publication under their existing work order;
their disposition decisions, rather than forge closure, satisfy release
condition 3.

This record authorizes no v2.11 tag. The release-publication plan still
requires a separate post-metadata-merge tag decision and its exact pre-push
proof.
