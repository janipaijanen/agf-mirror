---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260723T012716Z-governor-reviewer-re1q
timestamp: 2026-07-23T01:27:16Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T003326Z-governor-reviewer-erta.md
  - 20260723T012435Z-coder-tester-nnh3.md
topic: v240-release-checklist-start
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Claim the v2.4.0 release checklist and verify main merge pipeline 2698804817 for frozen commit d416309 before the release-branch product push.
    - Create release/v2.4.0 from exact frozen main d416309, update only approved release-truth surfaces, and publish one exact-SHA Gate 2 handoff with the complete release-wide acceptance evidence.
  must_read:
    - docs/plans/v2.4.0-release.md
    - docs/coordination/20260723T012435Z-coder-tester-nnh3.md
    - docs/coordination/20260723T003326Z-governor-reviewer-erta.md
  do_not_touch:
    - .gitlab-ci.yml
    - agf/rules/
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Release handoff: freeze main and prepare v2.4.0

Workstream D Class 5 merged as `d4163098696758e14e53587793ba83e19329bca4`.
Classes 1–5 and the four umbrella workstreams are complete. The approved
release plan now requires the dedicated release branch and truth-surface
checklist.

Treat `d416309` as the frozen product base, conditional on its main merge
pipeline `2698804817` succeeding. Inventory and local preparation may proceed
while it runs, but do not publish the release-branch product commit if that
pipeline fails or main advances unexpectedly; report the changed base instead.

On `release/v2.4.0`, change release truth only: versions, lockfiles where
required by the version change, both changelogs, release metadata, generated
release inventories, documentation release information, and the release
coordination trail. Do not pull issue #51, #53, the post-release Class 4
mobile/screen-reader work, taxonomy, operating-efficiency, or other deferred
product changes into this branch.

Use a skipped seed and MR boundary before the single acceptance push. Run the
release-wide criteria in the approved plan, including version/truth agreement,
strict docs and validators, packaging/path hygiene, provenance and privacy
negative checks, supported runtime checks, and host release preflight. The
Gate 2 handoff must name the exact SHA and every pipeline/job. Do not create a
tag, GitLab Release, upload, or public artifact before a separate exact-SHA
governor Gate 2 decision; publication verification follows the approved merge
and tag sequence.
