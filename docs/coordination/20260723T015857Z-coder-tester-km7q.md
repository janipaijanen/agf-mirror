---
artifacts_owned: []
from: coder-tester
id: 20260723T015857Z-coder-tester-km7q
immutable: true
kind: completion
model: claude-sonnet-5
references-to:
- 20260723T015528Z-coder-tester-mbhr.md
- 20260723T012716Z-governor-reviewer-re1q.md
requires-ack: false
timestamp: 2026-07-23T01:58:57Z
to: governor-reviewer
topic: v240-release-merged-and-tagged
---

# v2.4.0 merged and tagged

Human governor approved Gate 2 handoff `mbhr` directly ("approved.
proceed.") and instructed merge. Recorded here since no separate written
decision-record preceded the merge action — this entry is that record.

## Sequence

1. MR !446 (`release/v2.4.0`, SHA `9220423`) merged into `main` via
   `glab mr merge 446`. Pipeline `2698823782` (the exact SHA reviewed at
   Gate 2) had already succeeded before this approval.
2. Merge commit: `0d3e18ea` (`Merge branch 'release/v2.4.0' into 'main'`).
   Verified `agf/VERSION` reads `2.4.0` on `origin/main` post-merge.
3. Tagged the merge commit: `v2.4.0` at `0d3e18e`, annotated, pushed
   immediately per the release-via-merge-request workflow (no additional
   confirmation required for this specific step once merge lands).
4. Tag pipeline `2698842544` (publish jobs: OCI build/push, GitLab
   Release, archives, provenance) is running; polling in background. Will
   file a separate completion record enumerating the full release-wide
   acceptance criteria (tag object, peeled commit, every pipeline/job,
   Release contents, checksums, provenance, OCI digest, runtime checks,
   Pages) once that pipeline resolves, per
   `docs/plans/v2.4.0-release.md`'s completion requirement.

## Not yet verified

Per `mbhr`'s own scope note, this record does not claim OCI runtime smoke
tests, downloaded-artifact checksum/provenance re-verification, packaging
path-hygiene inspection, or host release preflight have run yet — those
depend on the tag pipeline currently in flight.
