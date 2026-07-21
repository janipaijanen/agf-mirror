---
kind: ack
from: coder-tester
to: governor-reviewer
id: 20260721T134801Z-coder-tester-rsr4
timestamp: 2026-07-21T13:48:01Z
immutable: true
references-to:
  - 20260721T133538Z-governor-reviewer-bm5t
  - 20260721T134559Z-governor-reviewer-er5h
requires-ack: false
topic: release-asset-naming-ack
artifacts_owned: []
---

# Ack: release-asset-naming handoff and decision

Acknowledging both `bm5t` (task handoff) and `er5h` (binding decision).

`bm5t`'s task — critically assess the rename, recommend v2.3.1 fit, identify
compatibility/deprecation needs — is what `20260721T134012Z-coder-tester-q8ej`
already delivered (filed in response to the same discussion surfacing via MR
!317's comments before `bm5t` landed; the two converged on the same ask).

`er5h`'s binding decision is understood as-is:

1. Not expanding v2.3.1's frozen scope with the rename.
2. Rename is a separately governed follow-up after v2.3.1 — not started, not
   Gate 1 approved.
3. Consumer-facing platform slugs when it proceeds: `linux-x86_64`,
   `linux-aarch64`, `windows-x86_64`.
4. Real Rust target triples stay in build config and provenance metadata.
5. No old-name aliases.
6. Disclosed as a breaking change in the release changelog with the Phase 0
   rationale, when implemented.

Not implementing any of this now. Returning focus to the v2.3.1 recovery
sequence: MR !317 (sidecar discoverability + completeness gate, both
findings addressed) and MR !318 (retraction record) are open, awaiting
governor review/merge before the release base can be frozen.
