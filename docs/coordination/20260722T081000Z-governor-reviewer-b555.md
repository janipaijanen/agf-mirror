---
kind: decision-record
from: governor-reviewer
to: all
id: 20260722T081000Z-governor-reviewer-b555
timestamp: 2026-07-22T08:10:00Z
model: human-governor-via-codex-session
immutable: true
references-to:
- 20260722T004752Z-contribution-auditor-kaka.md
- 20260722T010251Z-coder-tester-7jab.md
- 20260722T013507Z-coder-tester-m15r.md
topic: v2-4-0-workstream-b-gate2-approved-with-limitations
requires-ack: false
artifacts_owned:
- docs/plans/v2.4.0-workstream-b-provenance-contract.md
---

# Gate 2 approved for MR !351 with current limitations

The human governor-reviewer approves Gate 2 for MR !351, Workstream B of the
v2.4.0 unified release-provenance contract, at branch head `af4c284` and
implementation/pipeline SHA `07cfe5f`.

The exact-code-head pipeline `2695551208` passed, including the non-skipping
OCI evidence job `15466002526`. The governor accepts the current limitations
documented in the MR description:

- `release:verify` has not yet executed on a v2.4.0 tag because no such tag
  exists;
- a live HTTPS cross-host redirect fixture is absent, while the per-hop
  validation decision logic is unit-tested and the real same-host path passes;
- non-native Linux aarch64 and Windows packaged binaries are not executed by
  the Linux x86_64 runner, so their SKIP results identify an evidence gap rather
  than proving self-report agreement.

These limitations are accepted for the current Gate 2 merge and do not block
MR !351. They must remain described as limitations and must not be converted
into claims of evidence that was not gathered. Any stronger non-native
evidence strategy remains follow-up design/infrastructure work under normal
gate discipline.

The governor authorizes merging MR !351 to `origin/main`. This approval does
not authorize a version bump, tag, or release publication.
