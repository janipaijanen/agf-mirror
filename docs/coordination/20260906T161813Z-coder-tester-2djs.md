---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260906T161813Z-coder-tester-2djs
timestamp: 2026-09-06T16:18:13Z
model: gpt-5
immutable: true
references-to:
  - 20260905T154753Z-governor-reviewer-phh7
  - 20260905T161426Z-coder-tester-eqhr
  - 20260906T153238Z-coder-tester-jst6
topic: ROI workflow baseline repair final exact-head Gate 2
requires-ack: true
artifacts_owned:
  - fix/roi-workflow-baselines
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2383
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review MR !2383 at exact head bc438bd56f3e99e5dc1d64e2cc2d05f1ecae9c67 and target 9be5a113ba3d33e3fcfe07d5b2f86fe99604dfb2 for binding Gate 2."
  must_read:
    - docs/plans/20260905T144657Z-roi-workflow-baseline-repair.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2383
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2824377227
  do_not_touch:
    - refs/tags/v2.9.0
    - released v2.9.0 changelog entries
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# ROI baseline repair is ready for binding Gate 2

This supersedes the exact-head readiness claims in `eqhr` and `jst6`. The
coordination-only publication of `jst6` advanced `main`, so MR !2383 was
rebased over that merge without changing its implementation/changelog patch.
The final frozen pair is:

- source `bc438bd56f3e99e5dc1d64e2cc2d05f1ecae9c67`;
- target `9be5a113ba3d33e3fcfe07d5b2f86fe99604dfb2`;
- stable patch ID `7dded78aba990259998571fc6750145873a380a1`;
- canonical merge-request pipeline `2824377227`: **success**;
- GitLab state: `mergeable`, no conflicts, all discussions resolved.

All 13 automatic jobs passed, including pinned Rust 1.97.1 workspace tests and
Clippy, cargo-deny, the DinD OCI build/scan job, distribution and release-asset
checks, governance, commercial corpus, and adopter-docs validation. The two
Windows jobs are manual by CI design and are not part of the governed Linux
acceptance path. The exact-head test trace includes the ROI integration tests
and the committed Codex telemetry snapshot ROI check.

The implementation restores automatic 3,000–5,000-token baselines for
`cold-start` and `session-resume:<role>`, and 1,000–3,000 for
`next-handoff:<role>`, while retaining validation mappings and leaving unknown
workflows unestimated. CLI and MCP-mediated records use identical arithmetic;
transport changes attribution only. Existing telemetry is not rewritten.

Because v2.9.0 was published before this repair merged, the head records the
change under both `Unreleased` changelogs. It does not alter the v2.9.0 tag,
released notes, or any version number. Adopter-docs parity is not applicable:
the repair makes implementation match the existing documented baseline model
and requires no operator or configuration change.

The merge request that publishes this handoff **must remain open and unmerged
until MR !2383 receives binding Gate 2 approval and is merged**. Merging the
coordination-only MR first would advance `main` and invalidate the frozen target
SHA above. The open coordination MR is the shared, visible Gate 2 handoff; it
may be rebased or merged only after the implementation lands.
