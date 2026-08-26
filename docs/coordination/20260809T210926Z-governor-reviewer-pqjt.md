---
kind: decision-record
from: governor-reviewer
to: build-engineer
id: 20260809T210926Z-governor-reviewer-pqjt
timestamp: 2026-08-09T21:09:26Z
model: human
immutable: true
references-to:
- 20260809T205555Z-build-engineer-m4wt.md
- 20260809T194404Z-v250-final-release-publication.md
topic: v250-release-option-d-runner-capacity-approved
requires-ack: true
artifacts_owned: []
---

# Option D approved for the v2.5.0 release pipeline

The Governor explicitly approved proceeding with Codex's added fourth option:
use sufficiently sized runner capacity outside the candidate tree so the exact
RC2-derived release tree and artifact contract remain unchanged.

This authorizes build-engineer to provision or attach a sufficiently sized
project runner that accepts untagged protected-ref jobs and, if required to
make runner selection deterministic, temporarily disable GitLab instance/shared
runners for this project while the exact-head MR !1357 pipeline and immutable
`v2.5.0` tag pipeline run. The project runner must be protected, scoped to this
project, and configured with adequate disk for Docker-in-Docker cross-builds.
Restore the prior runner setting and retire the temporary project runner after
the governed release and publication pipeline no longer needs it.

The release candidate remains frozen at MR !1357 head
`85394eaf9746f4397f6d8b5a3804618811238493`. This decision authorizes no change
to that tree, the approved 23-path release boundary, `release/2.5`,
`release-2.5.0`, the required Linux-aarch64 artifact, or the four immutable
release namespaces. It does not waive CI, approve Gate 2, merge MR !1357, or
authorize tag/publication by itself.

Preserve failed jobs `15797773900` and `15797811664` as infrastructure evidence.
After runner provisioning, run one fresh exact-head acceptance pipeline, record
the selected runner and all required job outcomes, and return the unchanged head
for independent contribution-auditor review and a fresh binding Governor Gate 2
decision.

Human authorization in the active session: "Approved to proceed with added
fourth option."
