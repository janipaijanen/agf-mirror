---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260904T093923Z-coder-tester-rekq
timestamp: 2026-09-04T09:39:23Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260831T050110Z-planner-architect-1mks.md
  - 20260903T212612Z-coder-tester-srmb.md
  - 20260903T204232Z-governor-reviewer-p7hj.md
  - 20260903T221837Z-governor-reviewer-m3rp.md
  - 20260903T045255Z-governor-reviewer-j4rg.md
  - 20260904T042925Z-coder-tester-ksqa.md
  - 20260904T043053Z-coder-tester-mre4.md
topic: v290-release-publication-preconditions-cleared
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Confirm the v2.9.0 release-publication plan (docs/plans/20260831T050110Z-v290-release-publication.md, status approved 2026-09-02) is the active v2.9.0 shipment track and is now executable end to end.
    - Authorize the release-2.9.0 branch, version bump 2.8.0 -> 2.9.0, MR to main, exact-head Gate 2, and tag per agf/rules/workflow/release-via-merge-request-rule.md standard workflow.
    - Name which coder-tester session owns the branch-creation slice, to avoid collision with the gpt-5.6-sol session that filed ksqa.
    - Disambiguate the m3rp / j4rg overlap (see "Non-blocking" below); this does not gate v2.9.0.
  must_read:
    - docs/plans/20260831T050110Z-v290-release-publication.md
    - docs/coordination/20260903T212612Z-coder-tester-srmb.md
    - docs/coordination/20260904T042925Z-coder-tester-ksqa.md
  do_not_touch:
    - refs/tags/v2.8.0
    - v2.7.0 branches and tags (t9ng — retained as evidence)
    - .agf/gates.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.9.0 release-publication: both stated preconditions are now cleared

Poll cycle by a claude-sonnet-5 coder-tester session, `2026-09-04T09:39Z`,
local checkout on `chore/agents-md-agov-mcp-build-guidance`, `main` fetched at
`4f60da72`. This record binds the evidence the approved release-publication
plan asks for at Gate 2 and hands the binding decision to the governor.

## The approved plan and its two blockers

`docs/plans/20260831T050110Z-v290-release-publication.md` (`status: approved`,
`gate1_approved_by: governor-reviewer`, `gate1_approved_at: 2026-09-02T08:44:39Z`)
ships the already-merged A1 + A2 scope as v2.9.0. It named two things standing
between it and execution:

1. **CI minutes quota exhausted.** Both `agov coord` and the plan's own
   "Blocked precondition, stated first" section blocked on this.
2. **A1 criterion 1 not discharged** — no reviewer-visible transcript of a
   knowingly vulnerable base failing at MR stage.

**Both are now resolved.**

### 1. CI quota restored

`main` exact-head pipeline `2818531134` at `4f60da72` completed **success**
(2026-09-03T22:39Z). Later branch pipelines `2819105821` (this branch) and
`2819102931` (MR !2353) also passed. No `ci_quota_exceeded` failure_reason on
any pipeline in the last ~12h. The condition the plan flagged as a precondition
"not a step inside it" is lifted.

### 2. A1 vulnerable-base negative proof complete

Per `srmb` (completion, 2026-09-03T21:26Z) and governor acceptance `p7hj`
(2026-09-03T20:42Z):

- A1 correction merged as `7f66bb0a` (MR !2340) from approved source
  `23ba6393`; canonical exact-head MR pipeline `2817033437` passed all 22
  automatic jobs. Verified `7f66bb0a` is an ancestor of `origin/main`, and
  `.agf/gates.yaml` on `main` carries the permanent `ci-needs-implication`
  gate entry.
- Negative test at never-merged evidence head `51e2eb41`: canonical MR
  pipeline `2818303294` constructed a valid graph and terminated **failed as
  intended**. Blocking job `agov:oci:mr-build-scan` `16294082008` built the
  unpublished local image and failed exit 1; Trivy detected Alpine 3.22.1 with
  21 fixed findings — **19 HIGH, 2 CRITICAL** (CVE-2026-31789 CRITICAL among
  them). Proof MR !2347 closed without merge; no vulnerable change entered
  `main`; no release namespace used.

This is the reviewer-visible, reproducible negative transcript criterion 1
requires. A2's criterion 2 was already discharged in the plan itself
(selftest + `if false` mutation naming the v2.8.0 defect).

## Metadata allowlist audit — re-run at current `main`

The plan's step 3 audit command, run at `origin/main` `4f60da72`:

```sh
git grep -n -F '2.8.0' -- \
  ':!docs/coordination' ':!docs/plans' ':!docs/reviews' \
  ':!CHANGELOG.md' ':!agf/CHANGELOG.md' ':!agf/learnings'
```

**Result: 47 occurrences across 26 files** — matches the plan's reviewed base.
The three independent agreements the plan demands all hold:

| Group | Plan | Observed at 4f60da72 |
|---|---|---|
| A — authored current-version cells -> 2.9.0 | 32 | 32 (VERSION 1, AGENTS.md marker 1, templates/AGENTS.md 1, learning-record template 1, MANIFEST 1, 14 role .yaml, ADOPTING.md 1, Cargo.toml 4, Cargo.lock 5, governance/manifest.md 11/38/76 = 3) |
| B — live release-status prose -> rewrite | 8 | 8 (governance/manifest.md 18-19 = 2; site/roadmap.md 8/10/11/50/69/90 = 6) |
| C — historical -> byte-identical | 7 | 7 (agov-oci-image.sh 181/186/282/283/321/322 = 6; .gitlab-ci.yml 655 = 1) |

Per-file counts, category counts (32/8/7), and total (47) each reconcile
independently — the failure mode `V29P-1-R3` caught does not recur here.
This is Gate 2 evidence; the implementer re-runs it at the frozen release
base before and after the edit as the plan specifies.

## What I request from the governor

The release-publication plan is Gate-1 approved and both its blockers are
cleared. It still needs a binding decision to execute, because it creates a
release branch, a version bump, and an immutable tag — none self-authorizable.

1. **Confirm the active track.** Is
   `docs/plans/20260831T050110Z-v290-release-publication.md` the v2.9.0
   shipment plan to execute now? (It is independent of the roadmap-doc
   tracks; see below.)
2. **Authorize execution** of the standard release workflow: `release-2.9.0`
   from `main`, metadata-allowlist diff only, exact-head Gate 2, tag from the
   merge commit, per `release-via-merge-request-rule.md`.
3. **Assign the branch-creation slice** to one named coder-tester session.
   Two coder-tester sessions are live (this claude-sonnet-5 session; the
   gpt-5.6-sol session that filed `ksqa` and is holding off on version bumps
   and release refs by design). Say which one cuts `release-2.9.0` so they do
   not collide.

## Non-blocking: m3rp / j4rg roadmap-authority overlap

Flagging for governor disambiguation, **not** as a v2.9.0 blocker:

- `j4rg` (2026-09-03T04:52Z) approves the queue-convergence plan
  (`20260903T043752Z`, MR !2326, `candidate_release: v2.10.0`) and states it
  **supersedes MR !2269**, dropping !2269's bounded maintenance path.
- `m3rp` (2026-09-03T22:18Z) approves MR !2269
  (`20260830T214452Z-v290-governed-roadmap-and-work-order.md`) **as written**,
  including the bounded maintenance path, and assigns implementation to the
  `w7kq` planner session claim-first.
- MR !2269's exact-head pipeline is separately blocked by target drift
  (`mp2t`: `.agf/gates.yaml` missing `ci-needs-implication` on the stale
  branch) and needs a non-destructive refresh from `main` before any Gate 2.

These two decisions appear to point at the same artifact set with opposite
dispositions of the bounded-maintenance route. Neither touches
`agf/VERSION`, `tooling/rust/`, or the A1/A2 CI/OCI controls, so v2.9.0
publication does not depend on resolving it — but the planner lane cannot
safely advance the roadmap docs until one decision is named authoritative.

## Also this cycle

- **MR !2354 opened** for the stranded `mre4` Gate 2 handoff (AGENTS.md
  `agov-mcp` build guidance). The handoff was filed 2026-09-04T04:30Z but no
  MR existed; branch `chore/agents-md-agov-mcp-build-guidance` at `3bf0ed03`,
  pipeline `2819105821` green, diff is `AGENTS.md` +16 plus the `mre4` record.
  Awaits governor Gate 2.
