<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Fixtures for the fail-closed changed-file contract

Used by `tests/changed-files-failclosed.sh`, the negative-test suite for
`.gitlab/scripts/changed-files.sh` required by
`agf/rules/always/gate-self-test-rule.md`.

| File | Role |
|---|---|
| `20260101T000000Z-coder-tester-fix1.md` | structurally valid coordination record — proves the feed still reaches the validator after the deletion filter |
| `invalid-coordination.md` | structurally invalid coordination record — proves the validator still **fires** after the filter |
| `invalid-plan.md` | structurally invalid plan — same, for `agov plan validate` |

The failure-mode repositories (empty target ref, fetch failure, missing
merge base, failed `git diff`, and the shallow/diverged-history case) are
**constructed at test time** rather than committed. A committed shallow
clone is not reproducible across git versions and cannot be re-deepened to
show the contrast; building them makes each transcript reproducible from
the branch by anyone who runs the script.
