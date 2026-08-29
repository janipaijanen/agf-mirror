---
kind: decision-record
from: governor-reviewer
to: all
id: 20260829T170649Z-governor-reviewer-h3nb
timestamp: 2026-08-29T17:06:49Z
model: human-directed-claude-opus-5
immutable: true
references-to:
  - 20260828T224057Z-coder-tester-58kk.md
  - 20260829T165742Z-planner-architect-kpsm.md
  - 20260829T163248Z-planner-architect-j566.md
topic: v290-gate1-approved-and-gate2-decisions
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.9.0 Gate 1 approved; four Gate 2 decisions

This record transcribes the governor-reviewer's in-session decision, "all
requested gates approved. proceed."

## 1. Gate 1 — v2.9.0 release-integrity plan: APPROVED

`docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md`
at exact MR !2188 head `05c9d07981d88627d0b9a46148d1d7358b0c4ffb`
receives binding Gate 1 approval.

**Approved scope is A1 and A2 only**, with A3 as their shared
negative-test proof obligation. The terminal-disposition table in that
plan is approved as written: no item outside A may enter v2.9.0, and each
deferred item's disposition (later shipment, non-blocking closeout,
exclusion, named deferred risk) is the governing one.

Conditions carried from the plan into implementation authority:

1. The A1 path anchor must cover every input the plan enumerates. An
   incomplete anchor is the defect A1 exists to fix.
2. Reuse of the existing `rust:tooling:preview` archive is binding. No
   second Rust package build may exist in the MR path.
3. The runner-time budget is binding: investigate before widening if the
   scan alone exceeds two minutes on a representative MR. Measured
   incremental time is Gate 2 evidence, not a post-hoc observation.
4. The MR build-and-scan path must never push to, overwrite, or otherwise
   touch a release namespace.
5. Both negative tests must preserve reviewer-visible transcripts while
   the canonical exact-head acceptance pipeline stays green. "Tested
   locally" does not satisfy this.

**If A1 or A2 grows during implementation, stop and return to Gate 1.**
Do not absorb the growth. That is the specific failure this plan's
revision was made to prevent.

`agov plan validate` returns OK (blocking severity) at the approved head.
The plan's own `status`, `gate1_approved_by`, and `gate1_approved_at`
fields are updated in the same review cycle as this approval, per
`AGENTS.md`'s instruction that deferring that update is what caused the
same correction to recur three times in one session on 2026-07-21.

## 2. Independent review `58kk` — accepted

Coder-tester's BLOCK recommendation was correct and is accepted in full.
All four blocking findings are resolved in the approved head; the
resolutions are recorded in `kpsm`. Finding CT-V290-4 is noted as having
demonstrated itself: the review sat unpublished on an unmerged branch for
roughly eighteen hours before reaching `main` at `28861a11`.

This is the fifth consecutive independent review in the v2.7.0–v2.9.0
cycle to find real defects in a self-authored release artifact. The
practice continues.

## 3. Gate 2 decisions

| MR | Decision | Result |
|---|---|---|
| !2191 — `j566` live `/context` evidence | APPROVED | merged `473af667c587` |
| !2190 — Batch 31 sources/radar classification | APPROVED | merged `7798b9f3221c` |
| !2192 — `kpsm` ack and `2dga` scoping handoff | APPROVED, merge on green | pipeline was still running at decision time |
| !2188 — v2.9.0 plan | APPROVED at `05c9d079` | merge after the metadata update in §1 |

!2192's approval is conditional only on its exact-head pipeline passing.
A failed pipeline returns it to the governor rather than being merged
under this record.

## 4. v2.8.0 closure is unblocked

`j566` is now on `main`. It supplies the fresh-Claude live-surface result
`ge7q` and `7m4m` were waiting on: a Claude Code session's `/memory`
showing `.claude/CLAUDE.md` with root `AGENTS.md` nested beneath it and
labelled `@-imported`, on loader blobs byte-identical to `v2.8.0`.

`coder-tester` is cleared to publish the v2.8.0 completion record, stating
the accepted OCI-namespace evidence gap as `gs18` requires. No v2.8.0
identity may be mutated or republished.

## 5. Unaffected

No change to any published v2.8.0 identity. No change to the immutable
`v2.7.0-rc.1` tag. `v2.7.0` remains retired unused; `reconstruct/v2.7` and
`release-v2.7.0-rc.2` remain retained, unmerged and untagged. The three
scoping questions in `2dga` remain recommendation-only and outside v2.9.0
scope; nothing in this record authorizes implementation of any of them.
