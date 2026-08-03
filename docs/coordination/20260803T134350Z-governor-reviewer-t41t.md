---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260803T134350Z-governor-reviewer-t41t
timestamp: 2026-08-03T13:43:50Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T133541Z-governor-reviewer-1gbs
  - 20260729T092303Z-governor-reviewer-bn8g
  - 20260731T065654Z-governor-reviewer-88mk
topic: v250-rc1-release-publication-plan
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft the Gate 1 release-publication plan for v2.5.0-rc.1.
    - Reconcile the complete current-main delta with the approved v2.5 batching boundary.
    - Request independent contribution-auditor review before Gate 1 approval.
  must_read:
    - docs/plans/v250-statement-of-work.md
    - docs/plans/post-v248-release-batching-amendment.md
    - docs/plans/v250-rc1-contract-core.md
    - docs/plans/v2411-release-publication.md
    - .claude/rules/workflow/release-via-merge-request-rule.md
    - CHANGELOG.md
    - agf/CHANGELOG.md
  do_not_touch:
    - .claude/settings.local.json
---

# Draft the v2.5.0-rc.1 release-publication plan now

The human governor directed the team to continue with work related to
v2.5.0 so the release can be completed and published soon. Issues #79 and
#80 are merged, their issue records are closed, and MR !1025's additional
learning-schema implementation is also now on `main`. The next authorized
step is planning the required RC publication—not creating a release branch,
version bump, tag, or Release yet.

Draft a fresh Gate 1 plan, preferably
`docs/plans/v250-rc1-release-publication.md`, from refreshed `main` after MR
!1010's approved-metadata synchronization is merged. Keep the critical path
short, but do not weaken the release gates.

## Required scope reconciliation

The release branch must start from current `main`. The delta since v2.4.11
contains more than the original #79/#80 contract core, including:

- learning-record `status` / `supersedes` and validator enforcement;
- CI cache-policy and `agov ci cache-topology-check` changes;
- governance-layout corrections and the #79/#80 product changes;
- documentation, sources/radar, learning, and coordination additions.

The approved batching amendment described v2.5.0 narrowly as authority and
acknowledgement contract core. The plan must inventory the complete
adopter-facing and tooling delta, identify which items are release content
versus repository-only evidence, and present an explicit governor choice if
shipping current `main` requires amending that boundary. Do not hide the
extra additive surfaces under generic "release metadata" language, and do
not propose rewriting history or silently releasing from an older commit.

## Required release design

- Target the governed RC version `2.5.0-rc.1` as the exact-artifact
  acceptance checkpoint selected by the approved SoW.
- Define the release-only branch/MR changes: version surfaces, dated
  changelog sections, and release metadata only.
- Reproduce the full release-truth/version audit used by recent releases.
- Require clean tar and zip validation plus upgrade/overlay tests from the
  SoW's v2.4.5 and v2.4.6 baselines; also address the direct prior release
  v2.4.11 so the actual supported transition is not assumed.
- Verify archive layout for `MANIFEST.example.yaml` and absence of a shipped
  colliding adopter `MANIFEST.yaml`.
- Verify CLI/MCP parity and packaged self-reporting for the new `coord
  pending` and learning supersession surfaces.
- Define tag-pipeline, provenance, checksums, GitLab Release, generic
  package, OCI, anonymous-download, and post-publication checks from the
  current supported release process.
- State the exact rc.2 trigger: any functional, schema, documentation,
  migration, test, or artifact-layout correction after rc.1. If none occurs,
  final v2.5.0 must use the same functional candidate under its own later
  release-publication plan.
- Keep release/tag/deployment execution out of scope until this plan receives
  Gate 1 and the later release MR receives exact-head Gate 2.

Submit the plan with honest advisory statuses and request a bounded,
independent contribution-auditor review. The plan should make the remaining
human decisions explicit enough that approval can happen in one pass rather
than through avoidable ambiguity.
