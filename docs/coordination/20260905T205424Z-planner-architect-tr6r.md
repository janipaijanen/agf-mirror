---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260905T205424Z-planner-architect-tr6r
timestamp: 2026-09-05T20:54:24Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T201822Z-planner-architect-rnhh.md
  - 20260905T195732Z-coder-tester-83nk.md
  - 20260905T193012Z-contribution-auditor-4ghp.md
  - 20260905T202547Z-planner-architect-mmm3.md
topic: v290-release-chain-status-and-mr2373-rereview-gate1-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Assign an independent re-reviewer for the revised MR !2373 plan (a9b32474) -- not planner-architect (authored it), and ideally not the Codex build-engineer session that authored 83nk. Then decide Gate 1.
    - Confirm dedicated runner 55975189 configuration is stable (tag list, privileged-nested support) before approving, per the plan's runner-volatility risk.
  must_read:
    - docs/plans/20260905T145109Z-route-all-jobs-gate-accompaniment.md
    - docs/coordination/20260905T193012Z-contribution-auditor-4ghp.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.9.0 release chain: one blocker, everything else is coherently queued behind it

Run-list item 1 (`mmm3`). Read-only diagnosis this cycle; nothing here is a
decision or an implementation.

## The chain and where each link is stuck

| # | Link | State | Next move | Owner |
|---|---|---|---|---|
| 1 | **MR !2373** — revised split ordinary/`dind` routing plan (`a9b32474`) | plan-only pipeline **green**; `status: pending-gate1` | **independent re-review, then Gate 1** | reviewer (assign) → governor |
| 2 | Implement MR !2373's plan | not started | add `tags: [dind]` to 6 jobs + the 2 accompaniment tests, RED-then-GREEN, on MR !2362's branch | coder-tester / build-engineer (Codex) |
| 3 | **MR !2362** — base `default: tags:` | head `2e954ebd`, untouched since 12:22Z, pipeline failed (Failure A) | absorbs link 2; then a **fresh exact-head Gate 2 handoff** (owed per `4ghp` A2) | Codex → governor |
| 4 | **MR !2392** — native-musl packaging | pipeline **failed** — `agov:oci:mr-build-scan` (`script_failure`, nested Trivy `/proc` denied) and `rust:tooling:oci-pull-test` (`runner_system_failure`, rootless Podman socket) | **rebase onto `main` after link 3 lands**; those two jobs then run on the `dind` runner | Codex → governor |
| 5 | **MR !2383** — ROI implementation | pipeline **failed** — `rust:tooling:preview` (`script_failure`, DinD `mount: permission denied`) | rebase after link 4; re-run without DinD | Codex → governor |
| 6 | v2.9.0 construction | blocked | re-freeze `main`, re-run version inventory, rebuild release-metadata branch | Codex → governor |

## Every pipeline red in the chain is an expected blocker, not a new regression

- MR !2392's two failures are the exact jobs the split-routing plan (link 1)
  exists to move onto the `dind` runner. Native-musl packaging itself passes.
- MR !2383's failure is the same DinD `mount: permission denied` from `6ng1`,
  which link 4 removes from the mandatory path.
- MR !2362's failure is the Failure A gate-accompaniment finding, which link 2
  satisfies.

None needs a retry-for-green. Each clears when its upstream link lands.

## What is blocking, precisely

The entire chain is gated on **link 1**: the revised MR !2373 plan needs an
independent re-review and a Gate 1 decision. `4ghp` CLEARed only the prior
narrow scope (two additive tests, "no further `.gitlab-ci.yml` edit"); the
revision adds six `tags:` additions and moves release-pipeline jobs
(`agov:oci:release`, `release:verify`) to a different runner, so that CLEAR
is superseded (recorded in the plan's `advisory_consultation`).

### For the re-reviewer

- Does the split-routing design faithfully meet `83nk`'s requirements (the
  six jobs, ordinary-jobs-stay-put, tagged-selection-plus-nested-success
  acceptance)?
- Do `4ghp` findings A1 (registry-literal brittleness — the plan now offers
  the lighter form as an option) and A2 (fresh MR !2362 Gate 2 handoff owed)
  still hold, and are they adequately dispositioned?
- Is moving `agov:oci:release` / `release:verify` to runner `55975189`
  acceptable for release integrity? This is the point the plan flags for
  infosec.
- Is the deferred tag-pipeline verification (acceptance criterion 5)
  acceptable, or must it be proven before Gate 1?

## What I can and cannot do here

I cannot re-review my own plan revision or approve Gate 1. I have made the
downstream state explicit so the one decision that unblocks everything is as
low-friction as possible. Once link 1 clears, links 2-6 are Codex's
implementation and your Gate 2 decisions in sequence.
