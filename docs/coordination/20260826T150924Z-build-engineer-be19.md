---
kind: review
from: build-engineer
to: governor-reviewer
id: 20260826T150924Z-build-engineer-be19
timestamp: 2026-08-26T15:09:24Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T165910Z-governor-reviewer-rjse.md
  - 20260824T205211Z-planner-architect-s74g.md
topic: mr1965-git-depth-build-engineer-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !1965 targeted build-engineer review: CLEAR

**Disclosed self-review notice.** build-engineer is `played_by: "claude
(AI, CI/build and release-build work)"` per `agf/agents/MANIFEST.yaml` —
the same underlying model as planner-architect in this session. This is
self-review by role, not independent verification. Per governor decision
`rjse`'s own framing (a targeted build-engineer assessment, not an
independence-gated consultation like contribution-auditor's), and per
the Human Governor's explicit real-time authorization in this session to
proceed on this basis, this is disclosed and accepted as such — not
presented as cross-role independence.

**Context this review discovered and must flag.** While triaging the
governor's coordination backlog, I found that this exact scope —
`GIT_DEPTH: "0"` on `rust:tooling:test` only — was independently
rediscovered and re-planned from scratch as a separate Gate 1 effort
("CI-HISTORY-1", plan `docs/plans/20260826T044424Z-ci-history-1-job-
local-full-history.md`, MR !2080) over the past several hours, without
either session being aware of the other's work. Both fix the identical
defect (`real_mr1712_generation_acceptance_is_142_included_plus_50_
excluded` failing under default shallow-clone depth) with the identical
one-line change to the identical job. Per the Human Governor's direction
this session: MR !1965's implementation is the one to move forward
(already committed, already governor-approved in `rjse`, only blocked on
this review); CI-HISTORY-1's plan will be marked superseded/redundant in
a separate follow-up record rather than pursued as a duplicate Gate 1
path.

Scope: MR !1965 exact head `b320b406b8b61803f2043aeac3ab68d43610993f`,
the `.gitlab-ci.yml` diff hunk adding `GIT_DEPTH: "0"` to
`rust:tooling:test`'s `variables:` block only.
Coverage: COMPLETE for the three questions `rjse` names (cost, scope
correctness, cheaper-alternative existence).
Recommendation: CLEAR. No blocking findings.
Blocking findings: 0.
Advisory findings: 0.
Authority: build-engineer recommendation only. This record does not
approve Gate 2, merge, release, or deployment.

## 1. Runner/storage/network cost — measured, not estimated

Compared the job's `get_sources` CI section (the actual git fetch/
checkout step) between this exact head's passing pipeline and a recent
default-depth-20 baseline, both `rust:tooling:test`:

- **With `GIT_DEPTH: "0"`** (pipeline `2786195250`, job `16073747621`,
  exact head `b320b406`): `get_sources` section ran 16:34:03.409666Z to
  16:34:08.907715Z — **5.50 seconds**. Total job duration: 150.37s.
- **Baseline, default `GIT_DEPTH: 20`** (pipeline `2791114960`, job
  `16107811743`, an unrelated recent job on this same runner
  environment): `get_sources` section ran 03:58:34.613216Z to
  03:58:36.932295Z — **2.32 seconds**.

**Measured checkout-cost delta: approximately 3.2 seconds**, isolated to
the fetch step specifically (not conflated with cache restore,
compilation, or test-execution time, which are separate CI sections).
This is a small, clearly acceptable cost for one job on one CI pipeline —
no runner-storage or network-bandwidth concern at this scale. No
repository-size-growth projection changes this conclusion materially
within any planning horizon relevant to a CI job's checkout step.

## 2. Single-job scope — verified correct

`glab mr diff 1965` shows exactly one `GIT_DEPTH` occurrence in the
entire MR diff, inside `rust:tooling:test`'s own `variables:` block. No
other job, no shared template (`.rust-base`, `.rust-tooling-package`),
and no project-level CI/CD variable is touched. GitLab CI applies
`GIT_DEPTH` per job when set in a job's own `variables:` block; this
does not silently widen fetch cost to any other job in either pipeline.

## 3. Materially cheaper reliable alternative — none found

Considered the same three alternatives independently analyzed for the
duplicate CI-HISTORY-1 effort (same underlying defect, same constraints
apply identically):

- **Pinned-revision `git fetch --depth 1`** before the test: adds a
  bespoke pre-test step outside Cargo's own harness, requires manual
  sync if the pinned commercial-corpus revision ever changes, and only
  solves the currently-known pinned commit — a future revision bump
  would silently reintroduce the identical failure with no fetch step
  updated to match. Not materially cheaper given the measured ~3.2s
  delta is already small, and this alternative trades that small,
  bounded cost for an ongoing maintenance liability.
- **Self-contained fixture** (bundle historical file content instead of
  resolving from git): would stop exercising the actual git-resolution
  behavior the test exists to catch regressions in — not a reliable
  substitute, regardless of cost.
- **Fixed larger shallow depth** (e.g. 200): not durable — the pinned
  commit is already unreachable at the default depth from current
  `main`, and this repository's commit velocity means any fixed depth
  eventually recedes past the pinned commit again, reintroducing the
  identical failure with no code change to explain why.

`GIT_DEPTH: "0"` (full history for this one job only) remains the
correct choice: durable against repository growth, preserves the test's
real git-resolution purpose, and costs a measured ~3.2 seconds.

## Resolution

`rjse`'s named condition is satisfied. MR !1965 has no remaining blocker
from this review. Next steps (merge, release) remain governor and
Gate 2 decisions, not granted by this record.
