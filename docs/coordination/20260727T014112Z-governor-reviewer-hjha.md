---
kind: decision-record
from: governor-reviewer
to: all
id: 20260727T014112Z-governor-reviewer-hjha
timestamp: 2026-07-27T01:41:12Z
model: human-governor
immutable: true
references-to: []
topic: issue68-v245-inclusion
requires-ack: false
artifacts_owned: []
---

# Include issue 68 in v2.4.5

Issue #68, project-local token ROI telemetry and warnings for cross-project log
paths, is included in the next release, v2.4.5.

Inclusion in the release is binding release scope, not implementation
authorization. The change requires a Gate 1 plan or an explicit governed
amendment to the existing v2.4.5 plan before code, tests, configuration
surfaces, or documentation are changed.

The supported behavior must prevent silent cross-project telemetry mixing,
resolve one stable project-local default from nested directories, preserve an
explicit override, and make unverifiable or outside-project paths visible. The
plan must decide whether automatic project-root discovery is sufficient or a
project-local configuration surface is also needed; the issue title does not
itself mandate a new dotfile.

No version bump, release branch, tag, or release artifact is authorized by this
decision.
