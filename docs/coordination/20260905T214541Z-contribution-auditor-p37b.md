---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260905T214541Z-contribution-auditor-p37b
timestamp: 2026-09-05T21:45:41Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T195732Z-coder-tester-83nk.md
  - 20260905T193012Z-contribution-auditor-4ghp.md
  - 20260905T123338Z-coder-tester-6bkq.md
topic: mr2373-revised-split-runner-plan-rereview
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review evidence envelope (advisory authoring aid; not a gate decision)

Scope: MR !2373 / `docs/plans/20260905T145109Z-route-all-jobs-gate-accompaniment.md`
  at branch head `729925ca` (revisions `a9b32474` split-routing + `729925ca`
  dind-isolation precondition)
Coverage: COMPLETE
Recommendation: **CHANGES REQUESTED** (2 blocking, 3 advisory)
Blocking findings: 2
Advisory findings: 3
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1
Independence: I did not author or revise this plan. `4ghp` (my prior CLEAR of
  the narrow scope) is superseded by the split-routing revision; this record
  is the independent re-review its `advisory_consultation` now requires.

---

## Verification performed

Read the full revised plan, `6bkq`, `83nk`, `sgg6`, `hqps`, `4ghp`,
`.agf/gates.yaml`, MR !2362's actual `.gitlab-ci.yml` diff
(`build/route-all-jobs-to-dedicated-runner`), the current
`.gitlab-ci.yml` at that branch head, MR !2392's full `.gitlab-ci.yml` and
Rust-test diff (`fix/native-musl-release-build`), `ci_needs_implication.rs`,
`gate_change_check.rs`, and `post_publication_check_command` /
`composed_oci_chain_check` in `agov/src/main.rs`.

### What checks out

1. `.agf/gates.yaml` declares `.gitlab-ci.yml` as a whole-file `surfaces:`
   entry for both `gate-change-check` and `ci-needs-implication` — confirmed.
   The plan's refusal to narrow that surface under release pressure remains
   the correct application of `gate-self-test-rule.md` (matches `4ghp` and
   `6bkq`).
2. MR !2362's diff adds exactly `default: { tags: [podman, linux, amd64] }`,
   and its own in-file comment states the intended override mechanism: "a job
   that ever needs a different runner can override it locally with its own
   `tags:` key." The plan's per-job `tags: [dind]` approach is consistent
   with !2362's stated design.
3. At the current base (MR !2362 branch) the `default:` block is the *only*
   `tags:` declaration in the file; no job self-tags. The plan's
   "nothing else self-tags" invariant currently holds.
4. All six named jobs exist in `.gitlab-ci.yml` at the expected lines.
5. `release:verify`'s dind classification is **correct**, contrary to first
   impression: `agov dist post-publication-check` →
   `composed_oci_chain_check` → `pull_and_inspect_published_oci_image`, i.e.
   it pulls the published OCI image. It genuinely needs nested-container
   support.
6. Negative-test obligation: satisfied by RED-then-GREEN on the two revised
   tests; neither gate's own logic (`gate_change.rs`, `ci_rules.rs`) is
   modified. `4ghp` finding A1 (hard-coded registry literal) is adequately
   resolved — the revised plan offers the lighter "resolves on disk +
   surface-scoping" form and lets the implementer choose.

---

## Blocking findings

### B1 — the six-job list is incomplete for the plan's own "land this first" sequencing

`.rust-tooling-package` at the current base (pre-MR-!2392) declares
`services: - docker:dind`, `DOCKER_HOST`, `CROSS_REMOTE: "1"`, and runs
`cross build` against that dind service. **Three** jobs extend it:

| Extender | In the plan's `[dind]` list? |
|---|---|
| `rust:tooling:preview:windows-package` | yes |
| `rust:tooling:preview` (Linux preview archives; runs on `main` and on any MR touching `tooling/rust/**/*`) | **no** |
| `rust:tooling:release` (mandatory tag-pipeline Linux packaging) | **no** |

Under the plan's stated sequencing ("land this plan's implementation first,
then rebase MR !2392"), `.rust-tooling-package` still carries the dind
service when this plan merges. `rust:tooling:preview` and
`rust:tooling:release` then receive no `tags:` key, route to the ordinary
runner `55908035`, and fail their nested `cross`/dind workload — the exact
failure mode 83nk documents for the ordinary runner. Worse:

- `rust:tooling:preview` runs on the default branch and on MRs that change
  `tooling/rust/**/*`. This plan's own implementation commit changes
  `tooling/rust/crates/agov/tests/*.rs`, so the merge that lands this plan
  triggers `rust:tooling:preview` on `main` on the wrong runner.
- A v2.9.0 tag pipeline started in the window between this plan and MR !2392
  breaks `rust:tooling:release` — blocking the release this work is meant to
  unblock.

The plan's own accompaniment test #2 (`no other job defines its own
`tags:``) actively forbids fixing this by tagging the two jobs. So the plan
as written is internally inconsistent for its own merge order.

**Failure scenario:** merge this plan's implementation to `main` before MR
!2392 → next `main` pipeline: `rust:tooling:preview` selects `55908035`,
attempts `cross build` with a dind service the ordinary runner cannot
provide, fails. Any tag pipeline in the same window: `rust:tooling:release`
fails identically; no Linux tooling archive is published.

**Proposed fix (assumed intent: split routing that is correct at every merge
point, not just the end state):** adopt **B2**'s sequencing reversal. If the
governor instead keeps this-first, the job list must expand to eight —
add `rust:tooling:preview` and `rust:tooling:release` with `tags: [dind]` —
and MR !2392's rebase must *remove* those two `[dind]` tags in the same
commit that makes them container-free, with test #2 updated to a
list that shrinks back to six. That is more moving parts than B2.

### B2 — the sequencing is backwards relative to the `.rust-tooling-package` → dind dependency

MR !2392 (`build: remove DinD from mandatory Linux packaging`, Gate 1
approved by `mhq1`) removes the dind service from `.rust-tooling-package`
and relocates it to a dedicated `.rust-tooling-windows-package` template that
only the Windows preview job extends. **After MR !2392:**
`rust:tooling:preview` and `rust:tooling:release` are container-free, so they
correctly stay on the ordinary runner with no tag, and this plan's six-job
`[dind]` list becomes exactly right and minimal.

MR !2392 also already adds
`live_mandatory_linux_packaging_is_native_and_container_free` to
`ci_needs_implication.rs`, which asserts `.rust-tooling-package` contains no
`docker:dind` and that `rust:tooling:preview`/`rust:tooling:release` still
extend it. That test and this plan's `live_configuration_declares_split_
runner_routing` pin overlapping structure of the same template and job set;
landing them in the wrong order forces a manual reconciliation of two
assertion suites over the same lines.

The plan justifies "this first" only by collision avoidance in the two Rust
test files — which is order-neutral (either MR can rebase onto the other).
The template dependency is not order-neutral: it says MR !2392 should land
first.

**Proposed fix (assumed intent: unblock v2.9.0 with a routing change that is
green at every step):** revise the plan to sequence **MR !2392 first**, then
this plan's implementation rebased onto the post-!2392 `.gitlab-ci.yml`. Its
six-job list then needs no change. Alternatively, co-merge both
`.gitlab-ci.yml` changes as one coordinated MR with one combined test suite.
Either way the plan's "Interaction with MR !2392 and MR !2362" section and
acceptance criterion sequencing must be rewritten; issue #123's
collision note should record the final order.

---

## Advisory findings

### A1 — precondition evidence in the plan is stale; live runner state now satisfies it

The plan's precondition table and Risks section cite a 2026-09-05T20:54Z API
read showing runner `55975189` as
`tag_list: ['podman','linux','amd64','dind']`, `run_untagged: true` — i.e.
*not* tag-isolated — and tell the governor not to approve Gate 1 until it is.
MR !2373's own comment thread at 21:01Z / 21:06Z reports GraphQL now showing
`55975189` = `tagList=[dind]`, `runUntagged=false`, ONLINE/IDLE, and
`55908035` = `[podman,linux,amd64]`, `runUntagged=true`. The `/cache`
duplicate-volume executor defect the governor flagged at 21:06Z is reported
fixed operator-side. The plan text still reads as if the precondition is
open. Refresh it so the governor is not blocked by resolved evidence.
Acceptance criterion 4 (fail-closed API re-verification at implementation
time) is the correct durable guard and should stay exactly as written.

### A2 — the MR !2362 / this-plan merge mechanics are underspecified

`6bkq` established that MR !2362 alone fails `agov:governance:validate`
because it changes a `.gitlab-ci.yml` gate surface with no accompanying
declared-test change. This plan supplies those two tests but on its own
branch, "not re-editing" !2362. That leaves an unstated question: does
!2362 merge first (still gate-red on its exact head), then this plan? Or do
the six `tags:` additions and the two tests co-merge with !2362's `default:`
block as one unit? `4ghp` A2 (fresh exact-head Gate 2 handoff owed for !2362
after any head change) is still open and interacts with this. The plan
should state the intended merge unit explicitly and, if !2362's head moves,
`4ghp` A2's fresh Gate 2 handoff applies.

### A3 — infosec pass remains owed; this review does not substitute for it

The plan correctly records `infosec-sentinel: waived` with an independent
pass required before binding Gate 1. The core infosec question is
unchanged and correctly named in the plan: moving `agov:oci:release` and
`release:verify` (release-integrity jobs) onto a privileged-nested runner is
a trust-boundary change, not merely test coverage. I have not assessed
runner isolation, privileged-nested execution, or release-pipeline job
placement — that is infosec-sentinel's lane and still outstanding.

---

## Recommendation

**CHANGES REQUESTED.** The revised plan is materially stronger than the
narrow version and its gate discipline (no surface narrowing, non-hollow
accompaniment tests, fail-closed API re-verification) is sound. But B1 makes
it internally inconsistent for its own stated merge order, and B2 shows that
order is backwards relative to the template dependency. Both are cheaply
fixed by sequencing MR !2392 first (or co-merging); with that change the
six-job list is correct and the plan is approvable. A1–A3 are refinements
the governor can fold into the same revision cycle. Gate 1 remains the
governor-reviewer's decision.
