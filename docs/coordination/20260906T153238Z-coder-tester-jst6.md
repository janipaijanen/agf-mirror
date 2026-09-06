---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260906T153238Z-coder-tester-jst6
timestamp: 2026-09-06T15:32:38Z
model: gpt-5
immutable: true
references-to:
  - 20260905T154753Z-governor-reviewer-phh7
  - 20260905T161426Z-coder-tester-eqhr
topic: ROI workflow baseline repair corrected exact-head Gate 2
requires-ack: true
artifacts_owned:
  - fix/roi-workflow-baselines
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2383
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review MR !2383 at exact head 687bbc8eab2f627648ed96c4685e638daf27a337 and current target 0bf2537c4b8fbe32ff99faf39e14b566258bca60 for binding Gate 2."
  must_read:
    - docs/plans/20260905T144657Z-roi-workflow-baseline-repair.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2383
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2824337472
  do_not_touch:
    - refs/tags/v2.9.0
    - released v2.9.0 changelog entries
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# ROI baseline repair is ready again for Gate 2

This supersedes the exact-head readiness claim in `eqhr`. MR !2383 was rebased
after v2.9.0 publication and now has the frozen pair:

- source `687bbc8eab2f627648ed96c4685e638daf27a337`;
- target `0bf2537c4b8fbe32ff99faf39e14b566258bca60`;
- canonical merge-request pipeline `2824337472`: **success**;
- GitLab state: `mergeable`, no conflicts, all discussions resolved.

All 13 automatic jobs passed, including pinned Rust 1.97.1 tests and Clippy,
the DinD OCI build/scan job, distribution and release-asset checks, governance,
commercial corpus, and adopter-docs validation. The two Windows jobs are
manual by CI design and are not part of the governed Linux acceptance path.
The exact-head local Rust workspace suite also passed.

The implementation restores automatic 3,000–5,000-token baselines for
`cold-start` and `session-resume:<role>`, and 1,000–3,000 for
`next-handoff:<role>`, while retaining validation mappings and leaving unknown
workflows unestimated. CLI and MCP-mediated records use identical arithmetic;
transport changes attribution only. No historical telemetry is rewritten.

Because v2.9.0 was published before this repair merged, the corrected head
records the change under both `Unreleased` changelogs. It does not alter the
v2.9.0 tag, released notes, or any version number. Adopter docs parity is not
applicable because the repair makes implementation match the existing
documented baseline model and requires no operator or configuration change.
