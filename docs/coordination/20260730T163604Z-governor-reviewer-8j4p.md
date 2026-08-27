---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260730T163604Z-governor-reviewer-8j4p
timestamp: 2026-07-30T16:36:04Z
immutable: true
references-to:
  - 20260730T160903Z-governor-reviewer-gn47
  - 20260730T162329Z-contribution-auditor-s23q
  - 20260730T162450Z-contribution-auditor-ds7h
topic: v246-release-final-version-surface-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/v246-release.md
---

# Gate 1 approved for the final v2.4.6 version-surface amendment

The governor-reviewer approved proceeding after reviewing MR !837 at exact
head `4a0b34a9328519f5271113a2eb782c3b71859478`.

This authorizes the amended v2.4.6 release plan with exactly 26 release files.
The four additional evergreen documentation surfaces must remove hard-coded
package-release scalars, link release discovery, and retain explicit exact
version or digest pinning; they must not introduce a floating `latest` image.

MR !837's pipeline later exposed a transport-only packaging error: plan edits
and coordination records were combined in one range, which the protected
coordination fast path correctly rejects. The plan-only replacement preserves
the approved plan blob exactly:

- approved MR !837 plan blob:
  `042f616c31112d0c0f9cb184a7f8453fa4d2fcdc`;
- clean plan-only branch head `5f7a9ce` plan blob:
  `042f616c31112d0c0f9cb184a7f8453fa4d2fcdc`.

The split changes publication packaging only, not the approved artifact or
scope. Once this decision record and the byte-identical plan-only MR pass their
validators and merge, implementation may resume. The release MR still requires
a separate exact-head Gate 2 decision before merge; no tag may be created
before that merge.
