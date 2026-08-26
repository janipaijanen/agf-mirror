---
artifacts_owned: []
from: governor-reviewer
id: 20260809T161825Z-governor-reviewer-2mmn
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260809T155404Z-planner-architect-d58p.md
- 20260809T143855Z-governor-reviewer-qpkg.md
requires-ack: true
timestamp: 2026-08-09T16:18:25Z
to: planner-architect
topic: v250-final-release-topology-approved
---

# Topology approved: maintenance line (`release/2.5`) for exact-RC2 `v2.5.0`

**Transcription note.** This decision was given directly by the human
governor in a Claude Code session. Claude/coder-tester files this
record verbatim at that instruction.

## Decision

Approves `d58p`'s recommended choice: a separate protected maintenance
target (`release/2.5`, created at exactly the peeled RC2 commit
`3874a5b92b006fc867a4cedf359cfc17c6dfbae9`), with a `release-2.5.0`
source branch from that same commit, changing only RC2's proven
release-truth files. Tag `v2.5.0` lands on that maintenance target's
own Gate 2-approved merge commit — not on current `main`. A separate,
later metadata-forward MR syncs `main` with only the `[2.5.0]`
changelog entry, leaving `main`'s other 401 commits and their own
tracks (independent-implementation-reviewer, Pages/README,
commissioning, CI-latency, v2.7) completely untouched by the release.

The **alternative choice** (superseding `qpkg` to authorize RC3/
current-main) is **not** chosen. `qpkg`'s "RC2's exact functional
candidate unchanged" constraint stands, now with an actual mechanism
that can honor it.

## Authorization

1. Planner-architect drafts and requests Gate 1 review for the narrow
   `release-via-merge-request-rule.md` amendment first (the maintenance-
   line exception `d58p` specified: protected `release/X.Y` targets,
   fail-closed branch protection, exact-SHA invalidation, retained
   release branches, no reset/force-push/tag-movement/tree-overwriting
   merge).
2. Only after that amendment is itself approved, planner-architect
   drafts the replacement `v2.5.0` final release-publication plan
   against the new topology, and closes MR !1325 as superseded — not
   before, and not by editing MR !1325's existing plan content in
   place.
3. Coder-tester does not begin implementing either the rule amendment
   or the release itself before each has its own binding Gate 1
   approval, per `d58p`'s own stated boundary.

## What remains bound

`5m41` remains lifted for `v2.5.0` final only (per `qpkg`), still
binding for `v2.5.1-rc.1`/`v2.5.1-rc.2`. This decision does not touch
that boundary. It also does not authorize any change to `v2.5.0`
version files, release branches, tags, packages, OCI, or GitLab Release
state — per `d58p`'s own `do_not_touch` list, unchanged here.
