---
kind: handoff
from: contribution-auditor
to: coder-tester
id: 20260803T174851Z-contribution-auditor-ejs6
timestamp: 2026-08-03T17:48:51Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T173905Z-governor-reviewer-p1mm
  - 20260803T163251Z-contribution-auditor-kbdk
topic: v250-rc1-release-execution-authorized
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Execute the approved release-only change set in docs/plans/v250-rc1-release-publication.md on branch release-2.5.0-rc.1 from current origin/main.
    - Publish the release MR and exact-head acceptance evidence for independent Gate 2 review; do not create the tag or release before that MR is governor-approved and merged.
  must_read:
    - docs/plans/v250-rc1-release-publication.md
    - docs/coordination/20260803T173905Z-governor-reviewer-p1mm.md
    - docs/coordination/20260803T163251Z-contribution-auditor-kbdk.md
    - .claude/rules/workflow/release-via-merge-request-rule.md
    - agf/dist/release-manifest.yaml
  do_not_touch:
    - MR !1061 or docs/swatch-post250-palette
    - any product behavior, dependency, CI behavior, schema semantic, or v2.6.0/v2.7.0 scope outside the approved release-only plan
    - any v2.5.0-rc.1 tag, GitLab Release, generic package, or OCI tag before the release MR passes exact-head Gate 2 and merges
    - .claude/settings.local.json
---

# Approved v2.5.0-rc.1 release-only execution may begin

The approved plan is now on `main` via MR !1051. The binding Gate 1 decision
is `20260803T173905Z-governor-reviewer-p1mm`; the plan frontmatter is
`status: approved`; exact approval head `45c26bcc26f0ebf10bdc3e13ff344457bb1baefe`
passed pipeline `2728016150`; merge commit is
`be857a342315c3e4d544c1aab7cd44518c055832`.

Independent post-merge verification confirms:

- `origin/main` is `be857a342315c3e4d544c1aab7cd44518c055832`;
- remote branch `plan/v250-rc1-release-publication` still exists at the exact
  approval head because the merge endpoint used
  `should_remove_source_branch: false`;
- `agov plan validate docs/plans/v250-rc1-release-publication.md` passes on
  current `main`;
- issues #33/#67/#78 remain open in milestone v2.5.0 and #79/#80 remain
  closed, so no issue state changed silently;
- remote branch `release-2.5.0-rc.1` does not exist;
- Git tag and GitLab Release `v2.5.0-rc.1` do not exist;
- generic package `agf/v2.5.0-rc.1` does not exist; and
- OCI repository `agov` contains neither `2.5.0-rc.1` nor
  `v2.5.0-rc.1`.

The three Gate 1 choices are settled, not open implementation questions:

1. Option A includes all three added surfaces;
2. add the short issue-#101 entry to `agf/CHANGELOG.md`; and
3. keep #33/#67 open and unmilestone them, close #78 with the complete
   four-finding disposition note.

The older “decision requested” and “pending confirmation” prose retained in
the plan records the proposal history; the later explicit
“Governor decision (confirmed at Gate 1)” paragraphs, approved frontmatter,
and decision record are controlling. Do not reopen those decisions during
release implementation.

Create `release-2.5.0-rc.1` from the then-current `origin/main` and follow the
plan's exact release-only change set and validation matrix. Publish a release
MR targeting `main`, retain its source branch, and request independent exact-
head Gate 2 review. Stop before merge/tag/publication if any planned check
fails or a new shipping surface appears.
