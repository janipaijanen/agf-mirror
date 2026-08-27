---
artifacts_owned: []
from: governor-reviewer
id: 20260803T173905Z-governor-reviewer-p1mm
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260803T163251Z-contribution-auditor-kbdk.md
requires-ack: false
timestamp: 2026-08-03T17:39:05Z
to: all
topic: v250-rc1-release-publication-gate1
---

# Gate 1 approved: v2.5.0-rc.1 release publication

Claude-scribed governor-reviewer decision for MR !1051
(`docs/plans/v250-rc1-release-publication.md`), transcribing the human
governor's explicit chat approval of contribution-auditor's round-5 CLEAR
recommendation (`20260803T163251Z-contribution-auditor-kbdk`, exact head
`b677f9a68fd0213a535796acf79f758b39004f63`).

## Binding decisions

1. **Scope-reconciliation Option A**, for all three already-merged,
   already-Gate-2-approved surfaces: fold learning-record
   `status`/`supersedes`, the `governance-file-layout-rule.md`
   correction, and issue #101's `agov ci cache-topology-check` command
   into v2.5.0-rc.1 alongside #79/#80.
2. **Issue #101 changelog treatment**: add a short adopter-facing entry to
   `agf/CHANGELOG.md`, rather than accepting unexplained root-only
   coverage for a command shipped in the packaged tooling binary.
3. **v2.5.0 milestone/issue-tracker disposition**: keep #33 and #67 open
   and remove them from the `v2.5.0` milestone (leave unmilestoned); close
   #78 with the plan's full four-finding disposition note (finding 1
   retracted, findings 2-3 resolved by #79/#80, finding 4 routed to open
   `v2.6.0` issue #76).

## What this does not authorize

Gate 1 approval authorizes only this plan's own sequenced release-only
steps (version bump, changelog dating, one immutable tag, publication
verification) once merged to `main`. It does not itself cut the
`release-2.5.0-rc.1` branch, bump any version, create any tag, or change
any issue/milestone state — those happen only after this approved plan is
on `main`, per the plan's own "Gate 1 metadata sequencing" section.

Next: update MR !1051's own frontmatter (`status: approved`,
`gate1_approved_by`, `gate1_approved_at`) on the same branch, confirm a
green exact-head pipeline, and merge it to `main`.
