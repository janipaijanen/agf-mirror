---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260906T075309Z-contribution-auditor-qdj4
timestamp: 2026-09-06T07:53:09Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260906T073638Z-contribution-auditor-4sb8.md
  - 20260906T073655Z-infosec-sentinel-kkph.md
  - 20260906T073033Z-planner-architect-7npp.md
  - 20260906T073051Z-planner-architect-gra5.md
topic: v290-atomic-release-unit-exact-head-confirmation-2cf36a3d
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Atomic v2.9 release plan (MR !2427) — exact-head confirmation at `2cf36a3d`

Combined contribution-audit + infosec-advisory follow-up to `4sb8` / `kkph`
(both NO OBJECTION at `9e2a0ea8`). Commit `2cf36a3d` ("plan: incorporate
atomic release reviews") folds in **every** advisory item from both reviews.
`agov plan validate` passes at the new head.

## Disposition of each advisory item

| Item | From | Incorporated at `2cf36a3d` |
|---|---|---|
| **M1** — stale `advisory_consultation` prose | `4sb8` | Both roles now `status: consulted`, citing `4sb8` / `kkph` NO OBJECTION at `9e2a0ea8`. |
| **M2** — `4ghp` A2 wrongly "still applies" | `4sb8` | Evidence section reworded: "A2 is resolved by superseding rather than amending or merging MR !2362." |
| **M3** — no `downstream-impact` block | `4sb8` | New "## Downstream impact" section: BREAKING for foundry CI, foundry-local, no adopter/downstream migration, evidence cited. |
| **G1** — dind runner isolation model unstated | `kkph` | New paragraph: `services_privileged` + exact `allowed_privileged_services`, no blanket privileged mode; operator records privilege/egress/trust-boundary before the first acceptance run; unknown/broader fails closed to governor + infosec. |
| **G2** — `release:verify` not an independent leg | `kkph` | New criterion 9 + Risks note: `release:verify` runs on the pushing runner and cannot clear a compromise; the anonymous download-based `agov dist post-publication-check` is the independent leg and is unchanged. |
| **G3** — registry-secret scope | `kkph` | New criterion 8: registry-push/signing variables protected, masked, scoped to the jobs that need them (preferably `agov:oci:release`), not inherited by every `[dind]` job. |
| **G4** — supervise the first tag pipeline | `kkph` | Risks section: the first v2.9 tag pipeline is supervised — runner selection, provenance/scan output, registry push, and the later anonymous post-publication check watched to terminal state before the release is trusted. |

## Recommendation

**NO OBJECTION at `2cf36a3d`**, from both the contribution-audit and
infosec-advisory angles. `p37b` B1/B2/A2 remain resolved by the atomic
structure; `4sb8` M1–M3 and `kkph` G1–G4 are all incorporated; no new items.
Gate 1 on MR !2427 is the governor-reviewer's decision, and the two advisory
passes are clear at the current exact head.
