---
kind: decision-record
from: governor-reviewer
to: all
id: 20260803T183732Z-governor-reviewer-rntt
timestamp: 2026-08-03T18:37:32Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T182718Z-contribution-auditor-hkrp
  - 20260803T181735Z-coder-tester-phq3
  - 20260803T173905Z-governor-reviewer-p1mm
topic: v250-rc1-overlay-baseline-amendment
requires-ack: false
artifacts_owned: []
---

# v2.5.0-rc.1 overlay baseline amendment approved

Codex-scribed governor-reviewer decision, transcribing the human governor's
explicit approval in the live session on 2026-08-03 after reviewing
contribution-auditor finding B-RC-BASELINE-GOV-1.

## Binding decision

For the v2.5.0-rc.1 release candidate, replace unpublished baseline
`v2.4.6` with published recovery baseline `v2.4.8` in both occurrences of
the three-baseline framework-archive overlay acceptance matrix:

1. the release-MR, pre-Gate-2 overlay check; and
2. the post-publication check against the real immutable-tag artifacts.

The resulting governed baseline set is `v2.4.5`, `v2.4.8`, and `v2.4.11`.
This is an evidence-baseline amendment only. It is approved because GitLab's
Release and generic-package APIs independently show no published v2.4.6
framework artifact to test, while v2.4.8 is the real published recovery
release and has both surfaces available. No synthetic v2.4.6 archive is to be
fabricated and no named check is silently dropped.

This decision resolves B-RC-BASELINE-GOV-1 for MR !1067 at reviewed head
`49dc769ffc0287fa0b63210ccf6bfd2f2b00b8c4`, subject to the replacement test
already reported in the MR remaining valid at the eventual exact Gate 2 head.

## Boundary

This decision does not approve MR !1067 at Gate 2, merge it, create or push a
tag, or authorize publication ahead of the approved sequence. The two other
round-1 findings — required changelog test-count evidence and the overbroad
SemVer compatibility sentence — must be corrected, the renewed exact head
must receive a green pipeline and independent CLEAR review, and the governor
must then issue the binding exact-head Gate 2 decision.
