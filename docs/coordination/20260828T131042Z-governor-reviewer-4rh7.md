---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260828T131042Z-governor-reviewer-4rh7
timestamp: 2026-08-28T13:10:42Z
model: human-directed-gpt-5-codex
immutable: true
references-to:
  - 20260828T121848Z-coder-tester-q5mm.md
  - 20260828T110944Z-governor-reviewer-prn4.md
topic: v280-published-state-accepted
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding disposition: accept and finish the published v2.8.0 release

This record transcribes the governor-reviewer's in-session instruction,
"wanna publish? publish", given after the exact pre-tag OCI namespace evidence
gap in `q5mm` was disclosed.

## Decision

The existing immutable v2.8.0 publication is accepted. The `q5mm` finding is a
process/evidence nonconformance, not evidence of an artifact-content,
provenance, security, or namespace-collision defect:

- the exact tag pipeline passed every job;
- the actual OCI tag was created by that pipeline and binds to its provenance;
- the published OCI digest and anonymous fresh-download verification agree;
- the tag-pipeline Trivy scan reported zero findings; and
- Git tag, Release, package, protected-tag rule, and published commit identities
  are mutually consistent.

Do not create v2.8.1 solely because of this evidence gap. Never move, delete,
recreate, overwrite, or force any v2.8.0 identity.

## Authorized next actions

Coder-tester shall complete the remaining read-only fresh-clone checks,
including the fresh Claude Code `/context` live-surface check, and publish the
validated release-completion record if they pass. A failure in those checks
still returns to the governor under `prn4`.

Record a corrective learning/control so future release namespace checks derive
the OCI tag from the authoritative release script or provenance contract rather
than assuming the Git-tag spelling. Route any implementation through an
explicit future work order and normal gates; do not modify v2.8.0.

