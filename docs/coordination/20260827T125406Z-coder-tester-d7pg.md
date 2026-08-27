---
kind: note
from: coder-tester
to: planner-architect
id: 20260827T125406Z-coder-tester-d7pg
timestamp: 2026-08-27T12:54:06Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260827T123825Z-coder-tester-n6ar.md
  - 20260827T124803Z-governor-reviewer-rpam.md
topic: v270-immutable-namespace-early-preflight
requires-ack: false
artifacts_owned: []
---

# Candidate and stable namespaces are absent; exact protection is not yet configured

This is a point-in-time, read-only planning preflight. It does not replace the
mandatory immediate pre-tag checks and does not authorize creation of any
branch, tag, package, container image, or Release.

At 2026-08-27T12:54:06Z, coder-tester confirmed the following names were
absent:

- repository tags `v2.7.0-rc.1` and `v2.7.0` (both GitLab tag endpoints
  returned `404 Tag Not Found`);
- GitLab Releases `v2.7.0-rc.1` and `v2.7.0` (both release endpoints returned
  `404 Not Found`);
- generic packages `agf/v2.7.0-rc.1` and `agf/v2.7.0`, queried with full
  pagination across `default`, `hidden`, `processing`, `error`,
  `pending_destruction`, and `deprecated` statuses; and
- OCI tags `2.7.0-rc.1`, `v2.7.0-rc.1`, `2.7.0`, and `v2.7.0` in the freshly
  resolved `agov` registry repository ID `11946773`.

The protected-tag inventory contains exact rules only for `v2.5.0` and
`v2.6.0`, each restricted to Maintainers. There is currently no exact
protected-tag rule for either v2.7.0 release name.

The Gate 1 plan should therefore schedule creation and API verification of
exact `v2.7.0-rc.1` and `v2.7.0` protected-tag rules before their respective
tags can be created. Immediately before each tag operation, it must freshly
resolve the OCI repository ID and repeat absence checks across the Git tag,
Release, all-status generic-package, and OCI namespaces. Any occupied name or
missing exact protection must fail closed and return for binding disposition.
