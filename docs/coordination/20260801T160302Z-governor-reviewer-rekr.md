---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260801T160302Z-governor-reviewer-rekr
timestamp: 2026-08-01T16:03:02Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260731T171501Z-planner-architect-4mnq
  - 20260801T151548Z-governor-reviewer-hq8k
topic: v2411-documentation-release-planning
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Prepare a bounded Gate 1 release-publication plan for candidate v2.4.11 from current main.
    - Verify the SemVer classification and stop for governor direction if the combined scope is not honestly patch-compatible.
    - Obtain contribution-auditor review of the exact plan head before requesting Gate 1.
  must_read:
    - docs/plans/post-v248-release-batching-amendment.md
    - docs/plans/issue97-vscode-adopter-doc-accuracy.md
    - docs/coordination/20260801T151548Z-governor-reviewer-hq8k.md
    - CHANGELOG.md
    - agf/CHANGELOG.md
  do_not_touch:
    - MR !901
    - v2.5.0 contract-core scope
    - GitLab issue state
    - release tags or releases
    - main
---

# Prepare the bounded v2.4.11 documentation release plan

The governor has directed the release train to proceed after the successful
post-merge pipeline for the AI-assistance and figure-accessibility work.

Verified baseline:

- current `origin/main`: `0e898c269f4db1bbb818b2775e7af89b8332cdf0`
- pipeline `2724017202`: passed at that exact head
- latest published release: `v2.4.10`

The candidate v2.4.11 scope is limited to already-merged public-documentation
corrections and accessibility/transparency work:

1. Issue #97 / merged MR !890: correct the stale commissioning prompt and
   accurately document sustained author-operated Claude and Codex integration
   use in Visual Studio Code while preserving the independent-adopter evidence
   boundary.
2. Merged MR !894: AI-assistance acknowledgement, human responsibility,
   descriptive figure alternative text, visible AI-generation disclosure,
   SVG metadata, strict public-site figure checks, and the bounded Article 50
   transparency wording.

The plan must independently verify that this combined batch is patch-level. It
must specify exact changelog text, version/release-info surfaces, release
artifact checks, public Pages verification, source-branch retention, and the
order for closing issue #97 only after post-publication evidence exists.

The Four-Way Test consultation in MR !901 is expressly excluded. It remains a
separate values/governance candidate pending Claude's contribution-auditor
response and a later governor decision. Do not add new documentation policy,
rules, schemas, commands, or v2.5.0 contract work to this release plan.

Publish the plan and its handoff through the normal MR workflow. Planning is
authorized; implementation, version bumps, tags, releases, and issue closure
remain unauthorized until the governor approves Gate 1 and later Gate 2.
