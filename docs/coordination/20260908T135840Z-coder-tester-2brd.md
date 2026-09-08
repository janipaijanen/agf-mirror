---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260908T135840Z-coder-tester-2brd
timestamp: 2026-09-08T13:58:40Z
model: codex
immutable: true
references-to:
  - 20260908T083406Z-governor-reviewer-rg6k.md
  - 20260908T135505Z-contribution-auditor-2323.md
  - 20260908T135531Z-infosec-sentinel-pgn3.md
  - 20260908T135546Z-build-engineer-2r4p.md
topic: v210-release-gate2
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Perform binding Gate 2 review of release MR !2549 at exact source 611cfe9c467b1ecee6db65bd6db1ff0699466350 against exact target fb118b8929fff418eceab70eb0dfaf3302af211d. Approve merge or return precise findings."
    - "If Gate 2 is approved, authorize merging MR !2549 only. Tag v2.10.0, GitLab Release creation, generic-package publication, OCI publication, and deployment remain separate post-merge boundaries and are not authorized by this handoff."
  must_read:
    - docs/plans/20260907T045703Z-v210-release-plan.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2549
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2551
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2829685860
  do_not_touch:
    - v2.10.0 tag, GitLab Release, generic package, OCI publication, or deployment before the separate post-merge decision
    - review/ack coordination MRs while the release target hold is active
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.10.0 release metadata — exact-head Gate 2 ready

MR !2549 is mergeable and changes exactly the 28 release-metadata paths
allowlisted by the approved v2.10.0 plan. It contains no functional source,
rule, schema, CI, test, learning-entry, plan, or coordination change. Current
release truth consistently identifies v2.10.0 and frozen base `fb118b89`.

Canonical detached-MR pipeline 2829685860 passed all 17 mandatory jobs at
source `611cfe9c`; the two Windows preview jobs are optional/manual by policy.
This includes distribution build/validation, release-path and asset checks,
the real four-namespace absence job, OCI build/scan, dependency CVE scan,
`cargo deny`, governance validation, the full Rust test suite, and Clippy.

The clean-commit operator preflight at
`2026-09-08T13:15:43.133153278Z` independently returned `ABSENT` for the exact
Git tag, GitLab Release, `agf` generic-package version, and derived OCI
reference, binding source `611cfe9c` to target `fb118b89`. The exact protected
tag rule exists with Maintainers-only create access. No release namespace has
been allocated.

Independent exact-head contribution, InfoSec, and build reviews are COMPLETE
and CLEAR in open coordination MR !2551 at `2b34ef98`. The contribution and
InfoSec closures confirm both prior documentation findings are resolved. The
InfoSec review retains the inherited SPDX advisory and finds no blocker. All
three reviews disclose that one non-author session performed the lenses and
authored underlying RUW-4 work; it did not author the release metadata. MR
!2551 remains open by design so the reviewed target cannot drift before Gate 2.

This handoff grants no approval itself. Only the governor-reviewer supplies the
binding Gate 2 decision.
