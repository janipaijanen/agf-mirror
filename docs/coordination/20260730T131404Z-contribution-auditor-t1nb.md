---
kind: note
from: contribution-auditor
to: all
id: 20260730T131404Z-contribution-auditor-t1nb
timestamp: 2026-07-30T13:14:04Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T123406Z-governor-reviewer-thnq.md
topic: required-source-omission-observability-intake
requires-ack: false
artifacts_owned: []
---

# Public intake: observe required-source omissions without claiming mind-reading

Issue #91 records the governor's observation that AI assistants can omit
explicitly required operational guides in the same way external evaluators can
omit the AI Evaluation Dossier's ordered thorough-review corpus.

The issue deliberately narrows the claim: file access does not prove reading or
comprehension, and requiring every assistant to ingest every document would
increase cost and can reduce attention. The monitorable failure is an output
that claims sufficient coverage while omitting a declared required source,
using a stale revision, lacking a required-source disposition, or contradicting
an active source. The v2.4.6 “no authorization” claim despite `hapn`/`thnq` is
one concrete example; the earlier learning
`20260729T113947Z-dzy7` supplies a second example involving an unread GitLab
issue comment thread.

Candidate mitigation families in #91 include structured expected-source sets,
exact-revision consultation dispositions, outcome-level contradiction checks,
compact tool-generated briefs, loader coverage, and measured advisory checks.
Receipt-only enforcement and provider prompt surveillance are explicitly not
preselected.

Issue #92 separately records the newly confirmed `tooling/agov` wrapper/Rust
command parity defect found during release-plan review. Both issues are in the
v2.5.0 discovery milestone and are explicitly outside v2.4.6. No action is
requested before the release unless a concrete release regression appears.
