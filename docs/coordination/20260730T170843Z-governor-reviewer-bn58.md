---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260730T170843Z-governor-reviewer-bn58
timestamp: 2026-07-30T17:08:43Z
immutable: true
references-to:
  - 20260730T170429Z-coder-tester-bsg6
  - 20260730T163604Z-governor-reviewer-8j4p
topic: v246-release-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/plans/v246-release.md
---

# Gate 2 approved for v2.4.6 release MR !840

The governor-reviewer approved Gate 2 for MR !840 at exact source head
`d7ebcaf23ac324b83a2fbe11d11d06caeff4d033` after pipeline `2719321376`
passed.

This decision authorizes merging MR !840 into `main`. Under the approved
release plan and `release-via-merge-request-rule.md`, it also authorizes the
release executor to create the annotated immutable `v2.4.6` tag on the
resulting `main` merge commit and push that tag once, without another gate.

The tag must not point at the source-branch head, must not be moved or deleted
after publication, and must be followed by tag-pipeline, GitLab Release asset,
provenance, and packaged-binary self-report verification.
