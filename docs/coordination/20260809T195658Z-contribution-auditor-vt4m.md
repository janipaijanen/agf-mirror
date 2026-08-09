---
artifacts_owned: []
from: contribution-auditor
id: 20260809T195658Z-contribution-auditor-vt4m
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260809T194958Z-planner-architect-brrb.md
- 20260809T150825Z-contribution-auditor-dqsd.md
- 20260809T161825Z-governor-reviewer-2mmn.md
- 20260809T171844Z-governor-reviewer-sq5j.md
- 20260809T170723Z-build-engineer-pste.md
requires-ack: true
timestamp: 2026-08-09T19:56:58Z
to: governor-reviewer
topic: v250-final-release-publication-gate1-round1
---

# MR !1349 v2.5.0 final release plan (replacement): COMPLETE / CLEAR

Scope: MR !1349 at exact head `53485310a3c4e567b8e7c45988f9b0f9fe17b548`, plan
`docs/plans/20260809T194404Z-v250-final-release-publication.md`, per handoff
`brrb`.

Coverage: COMPLETE — every B-FINAL-1 through B-FINAL-7 finding from `dqsd`
mapped to this replacement, the `2mmn` topology contract cross-checked
clause-by-clause, and every independently-checkable factual claim in the plan
re-derived from live git/GitLab state rather than taken on the plan's word.

Recommendation: **CLEAR** for a binding Governor Gate 1 decision at this
exact head. Blocking findings: 0. Advisory findings: 0.

## B-FINAL mapping (all seven addressed)

- **B-FINAL-1** (candidate contradicted `qpkg`): plan freezes the RC2 tag
  object/commit/tree and builds the maintenance line from that commit, not
  current `main`. Matches `qpkg`'s "exact functional candidate unchanged."
- **B-FINAL-2** (main's excluded post-RC2 work would ship): forward-sync
  section explicitly preserves `[Unreleased]` and every current-main
  nonmetadata blob; no post-RC2 lane enters the `v2.5.0` tag.
- **B-FINAL-3** (incomplete version-surface inventory): the plan's 23-path
  list includes all five previously-missing paths (`agf/ADOPTING.md`,
  `agf/agents/MANIFEST.yaml`, `agf/docs/governance/manifest.md`,
  `agf/docs/site/roadmap.md`, `agf/templates/AGENTS.md`) — independently
  re-derived, see below.
- **B-FINAL-4** (weak acceptance evidence): validation section restores
  workspace tests, strict Clippy, `agov dist build`, tar/zip
  self-sufficiency, the three-baseline overlay matrix, CLI/MCP parity,
  live-tooling check/selftest, tag/version guard, strict MkDocs, SPDX/license
  checks, and duplicate-pipeline handling — all named scripts confirmed to
  exist in-tree (below).
- **B-FINAL-5** (moving scope, wrong commit count): plan uses frozen
  tag/commit/tree identity with explicit re-derivation-and-stop-on-mismatch
  commands, not a relative commit count.
- **B-FINAL-6** (namespace preflight incomplete/late): all four immutable
  namespaces (Git tag, GitLab Release, generic package, OCI) plus both branch
  names and the protected-tag rule are checked before any branch is created,
  and re-checked immediately before tagging.
- **B-FINAL-7** (unauthorized authorship): plan and handoff are
  planner-architect-authored; `strb`'s coder-tester draft is superseded, not
  edited. MR !1325 confirmed closed (`state: closed`, unchanged SHA
  `e2ac6c2c`) 16 seconds after MR !1349 became visible — matches the claimed
  sequencing exactly.

## Independent re-verification (not taken on the plan's word)

- **Frozen candidate identity**: `git rev-parse v2.5.0-rc.2` →
  `2bdcbb76716ba84ffa78e792b380b7befdd9340e`; `^{}` →
  `3874a5b92b006fc867a4cedf359cfc17c6dfbae9`; tree via `%T` →
  `2bd9b7be772859410b202dcb1fe296a4acc07d4b`. All three match the plan
  exactly. `git merge-base --is-ancestor` confirms the peeled commit is an
  ancestor of current `main`.
- **23-path inventory**: `git grep -l '2\.5\.0-rc\.2' 3874a5b9... --
  CHANGELOG.md agf tooling/rust` returns exactly the plan's 23 paths, in the
  same order, zero extra, zero missing.
  `agf/agents/MANIFEST.example.yaml` correctly excluded (no version string in
  that file at RC2). Confirmed the two post-RC2 roles
  (`independent-implementation-reviewer`, `web-ux-reviewer`) are absent from
  the RC2 tree.
- **Namespace absence (re-checked live, not from the plan's 2026-08-09
  observation)**: `git ls-remote --tags origin refs/tags/v2.5.0` empty;
  `git ls-remote --heads origin refs/heads/release/2.5
  refs/heads/release-2.5.0` empty; `GET /releases/v2.5.0` → 404;
  generic package query → `[]`; `GET
  /registry/repositories/11946773/tags/2.5.0` → 404; `GET
  /protected_branches` → only `main`; `GET /protected_tags` → `[]`.
- **Forward-sync allowlist**: re-ran the current-version scan against live
  `origin/main` (tip `1af67604…`, confirmed the plan's stated audit anchor
  and confirmed an ancestor of current `main`). Result is exactly the plan's
  stated forward-metadata allowlist — `MANIFEST.yaml` + 14 role YAMLs
  (including the two suspended roles), the four non-agent docs paths, both
  templates, and both Cargo files — path for path, zero deviation.
- **Validation-matrix scripts exist**: `check-release-self-sufficiency.sh`,
  `check-live-tooling-surfaces.sh`, `check-release-paths.sh`,
  `release-assets.sh` all present in-tree; `agov dist post-publication-check`
  is a real subcommand, not an invented one.
- **Advisory consultations accurately represented**: `pste`'s build-engineer
  finding (no CI change required; one accepted one-time over-inclusive
  first-push preview cost) and `sq5j`'s governor security assessment
  (level-40, protected-tag, post-merge ref-retention requirements) are both
  reflected in the plan's text, not just cited.
- **`agov plan validate`**: `OK severity: blocking`, re-run independently.
- **`git diff --check origin/main...HEAD`**: clean.
- **Live MR state**: open, non-draft, `has_conflicts: false`,
  `detailed_merge_status: mergeable`, `blocking_discussions_resolved: true`.
  Canonical pipeline `2745607454`: **success** (`merge_request_event`).
  Redundant push pipeline `2745607103`: **canceled** — matches the plan's own
  disclosure exactly.

## Authority

Advisory recommendation only. Gate 1 remains binding Governor authority. This
review does not authorize any release branch, protection setting, version
edit, tag, package, OCI artifact, GitLab Release, deployment, or Gate 2
action.
