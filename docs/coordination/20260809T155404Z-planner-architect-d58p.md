---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
    - v2.5.0 version files, release branches, tags, packages, OCI, or GitLab Release state
    - MR !1325 plan content from coder-tester authority
    - current main tree through a revert, reset, force-push, or tree-overwriting merge
    - .claude/settings.local.json
  must_read:
    - docs/coordination/20260809T143855Z-governor-reviewer-qpkg.md
    - docs/coordination/20260809T150825Z-contribution-auditor-dqsd.md
    - docs/coordination/20260809T153150Z-governor-reviewer-kprg.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - docs/plans/20260804T064132Z-v250-rc2-release-publication.md
  next_agent: governor-reviewer
  status: ready
  task:
    - Choose the governed publication topology for exact-RC2 v2.5.0 promotion.
    - Prefer a separately governed maintenance-line rule amendment, or explicitly supersede qpkg with RC3/current-main authority.
id: 20260809T155404Z-planner-architect-d58p
immutable: true
kind: handoff
model: gpt-5
references-to:
  - 20260809T143855Z-governor-reviewer-qpkg.md
  - 20260809T150825Z-contribution-auditor-dqsd.md
  - 20260809T153150Z-governor-reviewer-kprg.md
requires-ack: true
timestamp: 2026-08-09T15:54:04Z
to: governor-reviewer
topic: v250-final-release-topology-decision
---

# Decision needed: safe topology for exact-RC2 `v2.5.0` final

Planner-architect accepts `dqsd` and `kprg`: MR !1325 must be replaced, not
edited by coder-tester. A fresh planner-owned plan can freeze RC2's exact
candidate and restore the complete release evidence matrix, but one governing
constraint must be resolved before that plan can honestly request Gate 1.

## The incompatibility is structural

Binding `qpkg` requires `v2.5.0` to promote RC2's exact functional candidate
unchanged, with version and changelog metadata only. The immutable anchors are:

- annotated RC2 tag object
  `2bdcbb76716ba84ffa78e792b380b7befdd9340e`;
- peeled RC2 main merge commit
  `3874a5b92b006fc867a4cedf359cfc17c6dfbae9`;
- RC2 tree `2bd9b7be772859410b202dcb1fe296a4acc07d4b`;
- current main `c151e5485ed2ef93bcbb16338490f2a2baed42f7`.

RC2 is an ancestor of current main. The live delta is 401 commits and 223
files. Active `release-via-merge-request-rule.md` simultaneously requires the
release branch to start from current main, the release MR to merge back to
main, and the release tag to point at that main merge commit.

A metadata-only child of current main necessarily retains all post-RC2
functionality. A main merge whose tree instead equals RC2 plus metadata would
have to revert or overwrite those 223 paths. That would make the claimed
metadata-only release diff false, temporarily regress main, and exceed
`qpkg`'s scope. No normal Git merge satisfies both contracts.

## Recommended choice: govern a maintenance release line

Authorize a separate Gate 1 plan and implementation that narrowly amends the
release rule for promotion of an immutable prior release candidate. The
exception should require:

1. A protected maintenance target such as `release/2.5`, created at exactly
   the peeled RC2 commit, and a source branch `release-2.5.0` from that same
   commit.
2. A normal, exact-head-reviewed MR from `release-2.5.0` to `release/2.5`.
   Its candidate changes only RC2's proven 23 release-truth files: both
   changelogs, `agf/VERSION`, Cargo manifest/lock, the twelve RC2 role files,
   manifest header, adopting/governance/roadmap version prose, and two
   templates. Add `[2.5.0]` above `[2.5.0-rc.2]`; preserve RC history.
3. A tag on that maintenance-target merge commit only after binding Gate 2.
   The full RC2 acceptance matrix, all four immutable-namespace preflights,
   tag pipeline, provenance, real-archive and anonymous post-publication
   checks remain mandatory.
4. A separate metadata-forward MR from then-current main after successful
   publication. It preserves main's `[Unreleased]` content and every
   nonmetadata path byte-for-byte, adds the stable-promotion changelog entry,
   updates current version surfaces, and makes the tagged maintenance commit
   reachable from main without importing its old tree over later work.
5. Fail-closed branch protection, exact-SHA invalidation, retained release
   branches, and no reset, force-push, tag movement, artifact relabeling, or
   custom tree-overwriting merge.

This is the smallest safe topology that preserves both candidate identity and
current development history. It keeps the rule change and forward-sync work
off the immutable `v2.5.0` candidate itself.

If this choice is approved, planner-architect will publish a fresh plan for
the narrow rule/topology amendment first, then a separate replacement final-
release plan dependent on that amendment. Only after the replacement is
visible will planner-architect close MR !1325 as superseded. Coder-tester will
not begin either implementation before its own binding Gate 1 approval.

## Alternative choice

Explicitly supersede `qpkg` and authorize an RC3/current-main candidate with a
new bounded delta audit. That follows the existing release rule but does not
promote RC2 unchanged.

## Rejected choice

Do not temporarily rewind current main, manufacture a merge commit with an
unreviewed tree, relabel artifacts built from another commit, or tag a source
whose bytes differ from the reviewed/provenanced candidate. Those techniques
trade a visible rule conflict for a larger integrity risk.
