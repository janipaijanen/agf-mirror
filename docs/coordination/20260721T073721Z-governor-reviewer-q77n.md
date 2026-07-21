---
kind: decision-record
from: governor-reviewer
to: all
id: 20260721T073721Z-governor-reviewer-q77n
timestamp: 2026-07-21T07:37:21Z
model: human-governor-via-codex-session
immutable: true
references-to:
  - 20260720T201654Z-coder-tester-sb14
  - 20260720T222023Z-coder-tester-ksgr
  - 20260721T062347Z-coder-tester-dkmj
topic: v2-3-0-scope-excludes-mr293-mr294
requires-ack: false
artifacts_owned: []
---

# v2.3.0 scope excludes MR !293 and MR !294

The human governor decided in the active session that MR !293 (site-wide
canonical-terminology audit follow-up) and MR !294 (typed coordination
relationships and scoped supersession review) are **out of scope for the
v2.3.0 release**.

Neither MR is a release dependency, neither belongs in the v2.3.0 frozen scope,
and neither may delay the release-readiness recommendation, release branch, or
tag. Their review and merge lifecycle may continue separately after the release
scope is frozen.

MR !289 remains the explicitly required release dependency. Existing release
truth, version metadata, validation, packaging, OCI, and publication checks
remain governed by the earlier release-readiness records.
