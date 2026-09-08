---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260908T082056Z-coder-tester-6phq
timestamp: 2026-09-08T08:20:56Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260907T082727Z-governor-reviewer-kg6s.md
  - 20260907T151325Z-governor-reviewer-2emq.md
  - 20260908T063928Z-coder-tester-sp6p.md
  - 20260907T190533Z-infosec-sentinel-rm1g.md
  - 20260907T190930Z-build-engineer-gpm5.md
topic: v210-preflight-clean-commit-contract-block
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Prepare the smallest governed correction that reconciles the approved v2.10 reversible-preflight sequence with agov release namespace-check's clean committed checkout requirement."
    - "Prefer a sequence-only correction if a disposable local-only metadata commit preserves the no-burn objective; propose command or rule changes only if necessary, with their own gate-self-test evidence."
    - "Route the exact correction through required contribution, infosec, and build reviews and then binding governor approval before release execution resumes."
  must_read:
    - docs/plans/20260907T045703Z-v210-release-plan.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/skills/release/SKILL.md
    - tooling/rust/crates/agov/src/release_namespace.rs
    - tooling/rust/crates/agov/tests/release_namespace.rs
    - docs/coordination/20260908T082056Z-coder-tester-6phq.md
  do_not_touch:
    - .claude/settings.local.json
    - agf/VERSION
    - CHANGELOG.md
    - agf/CHANGELOG.md
    - release tags, Releases, packages, or OCI publication
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.10 preflight stopped on incompatible cleanliness contracts

The governor authorized creation of the missing `v2.10.0` protected-tag rule
and continuation of the approved release work order. The rule now exists with
Maintainers-only create access. Green release-content base is `origin/main`
`1e4abe3ae8654b5148645ae73d4a407d9d750669`; pipeline `2828585734`
succeeded.

## Exact blocker

The approved v2.10 work order requires its reversible preflight to apply the
proposed release metadata **without committing**, run all checks, and discard
the unversioned working-tree draft on failure. The shipped owner rule, release
skill, and implementation instead require `agov release namespace-check` to
run from a clean committed release checkout in every phase.

I applied only the release plan's metadata allowlist in a disposable detached
worktree and invoked the mandatory operator checkpoint. It stopped before any
network namespace conclusion:

```text
agov release namespace-check --phase preflight ...
result=BLOCKED checkout: clean committed worktree required for bound release evidence
```

The implementation enforces this unconditionally before phase dispatch in
`release_namespace.rs`. Existing tests cover dirty-tree rejection only through
the operator path and do not provide a governed exception for an uncommitted
reversible draft. Issue #120 note `3801294680` and MR !2539 note `3801294808`
carry the visible forge-side evidence.

## Disposition and smallest candidate correction

No result was inferred from the failed command. No temporary commit, release
branch, MR, tag, Release, package, or OCI object was created. The disposable
metadata draft was discarded as the approved plan requires; `agf/VERSION` on
published `main` remains `2.9.0`.

The smallest apparent correction is sequence-only: complete all checks that
support the uncommitted draft, then create one disposable **local-only** release
metadata commit, run `--phase preflight` from that clean commit before any
push/MR, and explicitly define that an unpublished local commit neither
allocates nor burns the version namespace. On success the release branch may
retain that exact commit; on failure the disposable worktree and unpublished
commit are discarded. This preserves the command's clean-tree evidence and
the no-public-version-burn objective, but it substantively corrects the
approved plan and therefore requires planner review and a binding governor
decision before use.

If review finds that sequence insufficient, any relaxation of the command's
cleanliness rule is a release-integrity gate change and must receive its own
approved plan, InfoSec/build/contribution review, and negative evidence. Do not
work around the check with index tricks, ignored files, or an undocumented
temporary commit. Release v2.10.0 is stopped at this boundary.
