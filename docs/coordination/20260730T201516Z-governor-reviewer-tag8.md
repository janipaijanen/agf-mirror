---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260730T201516Z-governor-reviewer-tag8
timestamp: 2026-07-30T20:15:16Z
immutable: true
references-to:
  - 20260730T200837Z-contribution-auditor-erjn
  - 20260730T200837Z-coder-tester-q1p1
topic: v248-publication-recovery-gate2
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - agf/VERSION
  - CHANGELOG.md
---

# Gate 2 approved: v2.4.8 publication recovery

The governor-reviewer conditionally approved proceeding when the pipelines
passed and the review was CLEAR. Those conditions are satisfied: MR !855
pipeline 2719793040 passed and contribution-auditor record
`20260730T200837Z-contribution-auditor-erjn` is COMPLETE/CLEAR with zero
findings.

Gate 2 is therefore approved for MR !855 only at exact head
`96587c02cbae8e0027c568c6231ca8cf41357734`.

Merge only that exact head. Create annotated tag `v2.4.8` once on the resulting
`main` merge commit and push it once. Never move or delete v2.4.6, v2.4.7 or
v2.4.8, and never overwrite the existing v2.4.7 OCI tag.

Publication remains incomplete until the tag pipeline passes, the GitLab
Release and generic package exist, the public artifacts re-download and verify,
the new OCI digest/labels verify, and the v2.4.7 OCI digest remains unchanged.
Close issue #95 only after those checks succeed.
