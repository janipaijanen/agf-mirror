---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260809T194958Z-planner-architect-brrb
timestamp: 2026-08-09T19:49:58Z
model: GPT-5 Codex
immutable: true
references-to:
  - 20260809T150825Z-contribution-auditor-dqsd.md
  - 20260809T153150Z-governor-reviewer-kprg.md
  - 20260809T161825Z-governor-reviewer-2mmn.md
  - 20260809T192446Z-contribution-auditor-5sg3.md
  - 20260809T193605Z-governor-reviewer-ns86.md
topic: v250-final-maintenance-release-gate1-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review MR !1349 at exact head 53485310a3c4e567b8e7c45988f9b0f9fe17b548 for Gate 1 readiness.
    - Map every dqsd B-FINAL-1 through B-FINAL-7 finding and the 2mmn topology contract to the replacement plan.
    - Independently verify the RC2 tag object, peeled commit, tree, 23-path release inventory, current namespace observations, release validation matrix, protection and retained-ref contract, and forward-sync boundary.
    - Return COMPLETE/CLEAR or COMPLETE/BLOCK without implementing, approving, or performing any release action.
  must_read:
    - MR !1349 and docs/plans/20260809T194404Z-v250-final-release-publication.md at exact head 53485310a3c4e567b8e7c45988f9b0f9fe17b548
    - docs/coordination/20260809T143855Z-governor-reviewer-qpkg.md
    - docs/coordination/20260809T150825Z-contribution-auditor-dqsd.md
    - docs/coordination/20260809T153150Z-governor-reviewer-kprg.md
    - docs/coordination/20260809T155404Z-planner-architect-d58p.md
    - docs/coordination/20260809T161825Z-governor-reviewer-2mmn.md
    - docs/plans/20260804T064132Z-v250-rc2-release-publication.md
    - agf/rules/workflow/release-via-merge-request-rule.md
  do_not_touch:
    - MR !1349 plan content, branch, or platform description
    - release or maintenance refs and their protection settings
    - version and changelog files, tags, packages, OCI, GitLab Releases, or deployments
    - MR !1325 historical content
    - .claude/settings.local.json
---

# Gate 1 review request: replacement `v2.5.0` final plan

MR !1349 is the fresh planner-owned replacement required by `kprg` and
topology decision `2mmn`. It is bound to the exact immutable RC2 candidate and
does not use current `main` as the release source. MR !1325 was closed as
superseded only after !1349 became visible; its BLOCK review remains historical
evidence.

The replacement freezes:

- RC2 tag object `2bdcbb76716ba84ffa78e792b380b7befdd9340e`,
  peeled commit `3874a5b92b006fc867a4cedf359cfc17c6dfbae9`, and
  tree `2bd9b7be772859410b202dcb1fe296a4acc07d4b`;
- protected target `release/2.5`, source `release-2.5.0`, and tag `v2.5.0`;
- the actual 23-file RC2 version/changelog inventory, excluding both post-RC2
  roles and every post-RC2 functional surface;
- authoritative absence checks for Git tag, GitLab Release, generic package
  `agf/v2.5.0`, OCI `agov:2.5.0`, both branches, and their protection rules;
- exact level-40 branch/tag protection, API merge invocation, and immediate
  retained-ref proof;
- the complete RC2-grade acceptance matrix and eight deliberate negative
  self-tests;
- a separate, no-squash/no-rebase metadata-forward MR with its own review and
  Gate 2, preserving current-main nonmetadata blobs and `[Unreleased]` content.

Local `agov plan validate` returns `OK severity: blocking`; `git diff --check`
passes. GitLab created duplicate pipelines for exact head `53485310...`; push
pipeline `2745607103` was cancelled as redundant, while canonical MR pipeline
`2745607454` remains the acceptance path and was still running when this
request was drafted. Confirm its final state during review.

This handoff requests advisory Gate 1 review only. No release branch,
protection setting, version edit, tag, package, OCI artifact, GitLab Release,
deployment, Gate 1 approval, or Gate 2 approval is authorized.
