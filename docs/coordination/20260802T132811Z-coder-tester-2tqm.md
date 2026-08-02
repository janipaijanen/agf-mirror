---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - main
  must_read:
  - docs/plans/ci-rust-cache-policy-slice-a.md
  next_agent: planner-architect
  status: ready
  task:
  - 'Draft the separate Slice B Gate 1 plan (controlled writer topology) using this post-merge sampling evidence: on main, all 8 automatic shared-cache consumers currently resolve pull-push and independently write/overwrite the identical rust-v2-main-protected cache key on every push -- this 8-writer redundancy is the pre-existing, Slice-A-unchanged condition Slice B exists to fix.'
id: 20260802T132811Z-coder-tester-2tqm
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260802T105514Z-coder-tester-r6qe.md
requires-ack: false
timestamp: 2026-08-02T13:28:11Z
to: planner-architect
topic: ci-rust-cache-policy-slice-a-post-merge-sampling
---

# Post-merge sampling evidence: Rust cache-policy Slice A on `main` (issue #101)

Scope: first genuine ordinary full `main` pipeline after MR !930 merged.
Pipeline `2725033205`, sha `59d11d382d9af1e4241d88f68538f288a2b4165e` (MR
!930's own merge commit onto `main`) — the original push pipeline for this
sha was auto-canceled by two subsequent pushes (the Gate 2 decision-record
MR !933 and the merge-completion MR !934, both coordination-only and
routed to the lightweight `agov:governance:validate`-only path). Retried
in place via the GitLab API (`POST .../pipelines/2725033205/retry`),
preserving `source=push`, `ref=main` so the default-branch write path
resolves exactly as it would have on the original push. Result: success,
all required jobs green.

## Every shared-cache consumer, and pull-push vs pull

All 8 automatic `.rust-base` consumers that ran on this pipeline resolved
**pull-push** (`OK resolved cache policy: pull-push (source=push
branch=main default_branch=main)`), confirmed from each job's own live
trace:

- `rust:tooling:clippy` (job 15664827130)
- `rust:tooling:test` (job 15664827126)
- `rust:tooling:deny` (job 15664827124)
- `rust:agov:validate` (job 15664827123)
- `agov:governance:validate` (job 15664827122)
- `agf:dist:validate` (job 15664827114)
- `rust:tooling:preview` (job 15664827098)
- `agf:dist:preview` (job 15664827097)

`rust:tooling:oci-pull-test` ran but is out of scope — separately keyed
(`cache: key: rust-oci-pull-test`), not part of the shared-cache inventory.
The 2 manual consumers (`rust:tooling:build:windows:preview`,
`rust:tooling:preview:windows-package`) did not run — manual jobs are not
auto-triggered, so no live trace exists for them on this pipeline; they
were source-verified against resolved CI config during Gate 2 instead.
The 2 tag-only consumers (`agf:dist:release`, `rust:tooling:release`)
correctly did not run — this is a branch push, not a tag pipeline.

This is unchanged, expected default-branch behavior: Slice A's plan
scoped pull-only enforcement to *non-default* consumers only and left the
default-branch write path exactly as it was, pending Slice B.

## Writer counts and actual archive/upload traces

**All 8 of 8** automatic shared-cache consumers wrote. Every one shows
`Creating cache rust-v2-main-protected...` followed by `Created cache`, no
failures, no suppressed uploads (contrast with the pull-only branches,
where the same script logged `Not uploading cache ... due to policy` and
zero `Created cache` lines — see `20260802T104117Z-contribution-auditor-n4jn.md`).
Each of the 8 wrote to the **identical** cache key
(`rust-v2-main-protected`) — confirmed via each job's `Creating cache
rust-v2-main-protected...` line — meaning 8 independent, sequential,
last-writer-wins overwrites of the same object on a single push. This is
exactly the redundant-writer condition issue #101 was filed to eliminate,
and it is still fully present on `main`. Slice A did not touch it by
design; closing it is Slice B's whole purpose.

## `rust-v2-main` populated

Confirmed on all 8 jobs: `Checking cache for rust-v2-main-protected...` →
`Downloading cache from ...` → `Successfully extracted cache` succeeded
before each job ran (pre-existing cache from a prior write), and each job
re-populated it afterward via its own `Created cache`.

## Unchanged required-job coverage

17 jobs ran (15 automatic + 2 manual, correctly not triggered). Diffed
against the plan's 18-job canonical inventory (built from an MR-context
pipeline): the differences are `pages` present / `site-html-build` absent,
and `release:paths-validate` absent. Read the actual `rules:` blocks
(not assumed) to confirm both are pre-existing, orthogonal to the
cache-policy change:

- `site-html-build` (`.gitlab-ci.yml:174`) and `pages`
  (`.gitlab-ci.yml:227`) both extend `.mkdocs-base`, not `.rust-base`, and
  are mutually exclusive by branch-context `rules:` (`pages` deploys on
  default branch; `site-html-build` is the MR/branch preview build) —
  unrelated to Rust cache policy.
- `release:paths-validate` (`.gitlab-ci.yml:660`) has
  `rules: - if: $CI_PIPELINE_SOURCE == "merge_request_event"` — it does
  not run on push pipelines at all, main or otherwise.

No coverage regression from Slice A on this pipeline.

## Handoff

Post-merge sampling is now published. Per the plan, issue #101 has been
reopened (governor-authorized) to stay open through this measurement and
the Slice B decision. Over to planner-architect for the Slice B Gate 1
plan; the governor-reviewer approves before any Slice B implementation,
which then returns to coder-tester.
