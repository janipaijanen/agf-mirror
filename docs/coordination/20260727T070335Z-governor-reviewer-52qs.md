---
artifacts_owned: []
from: governor-reviewer
handoff:
  do_not_touch:
  - agf/VERSION
  - any release branch or tag
  must_read:
  - .gitlab-ci.yml
  - docs/coordination/20260727T051441Z-governor-reviewer-m2rg.md
  next_agent: coder-tester
  status: ready
  task:
  - Create a corrective branch from current main.
  - Fix the duplicate .gitlab-ci.yml `rules:` key under `release:paths-validate`
    (it currently shadows its own intended path-scoping with the block that
    belongs to `release:assets-selftest`, and `release:assets-selftest` has
    lost its rules entirely -- see 20260727T051441Z-governor-reviewer-m2rg for
    the full diagnosis).
  - Test both jobs' triggers independently (e.g. `yaml.safe_load` resolution
    of each job's `rules:`) and exercise the negative release-path gate
    (`tooling/rust/scripts/check-release-paths.sh selftest` and `check agf`).
  - Submit a new Gate 2 MR for the corrective fix. Do not reopen or amend the
    already-merged !680.
id: 20260727T070335Z-governor-reviewer-52qs
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260727T051441Z-governor-reviewer-m2rg.md
requires-ack: true
timestamp: 2026-07-27T07:03:35Z
to: coder-tester
topic: issue67-slice-b-ci-yaml-corrective-fix
---

# Corrective fix required: !680's merged .gitlab-ci.yml still has the duplicate-key defect

The Gate 2 approval that merged !680 (`430091661eaba8437686d07c8510ef524733f3cf`)
did not pick up the blocking review finding filed in
`20260727T051441Z-governor-reviewer-m2rg` before merging. The defect described
there is confirmed still present on current `main`: `release:paths-validate`
resolves to the `rules:` block meant for `release:assets-selftest`, and
`release:assets-selftest` now has no `rules:` at all (runs unconditionally on
every pipeline).

## Responsibilities for this correction (governor-assigned)

- **coder-tester** (you): create a corrective branch from current `main`, fix
  the duplicate `rules:` structure, test both job triggers and the negative
  release-path gate, and submit a new Gate 2 MR. This is the immediate next
  action.
- **governor-reviewer**: will wait for the corrective MR, then review and
  approve its exact tested head. No action on the already-merged !680 itself.
- **planner-architect**: no new plan required -- this correction stays inside
  the already-approved issue #67 scope.
- **infosec-sentinel / contribution-auditor**: no additional review required
  unless the fix expands beyond CI trigger restoration.

After Gate 2 approval, merge the corrective MR and publish a coordination
record that explicitly supersedes the invalidated !680 Gate 2 approval
(`20260727T053557Z-governor-reviewer-2sp1`).
