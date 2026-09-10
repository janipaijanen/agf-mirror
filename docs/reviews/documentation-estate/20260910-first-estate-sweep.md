<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Documentation estate — first estate-wide sweep (2026-09-10)

**Role:** documentation-estate-steward (activated 2026-09-10, MR !2661).
**Trigger:** R-4 of the role-instantiation plan
`docs/plans/20260910T115843Z-documentation-estate-steward-role.md`; the
`release-content-freeze` cadence class for v2.11 — this sweep completing
with every finding dispositioned is v2.11 release-cut **condition 4**
(parent scope plan amendment R-2, MR !2662).
**Status:** IN PROGRESS — dimensions 1, 3, 4 first pass done; 2, 5, 6
pending. This file accumulates findings across poll cycles; it is not
"complete" until every dimension is swept and every finding dispositioned.

## Estate scope (the touchpoint manifest — dimension 6, provisional)

Adopter-facing surfaces inspected or queued for this sweep:

| Group | Surfaces |
|---|---|
| Root | `README.md`, `AGENTS.md` |
| Templates | `agf/templates/*` (10 files incl. `AGENTS.md`, `CLAUDE.md`, `gate1-plan.md`, `learning-record.yaml`, coord-* templates, `README.md`) |
| Pages site | `agf/docs/site/**/*.md` (19 md files) + `agf/docs/site/roadmap.md` |
| Governance docs | `agf/docs/governance/manifest.md` |
| Other agf/docs | `agf/docs/agent-communication-0.26.0.md` |
| Roadmaps | `docs/roadmap.md` + `agf/docs/site/roadmap.md` |
| Version anchors | `agf/VERSION` (2.10.0), `CHANGELOG.md`, `agf/CHANGELOG.md`, `agf/templates/learning-record.yaml` framework_version cell |
| Projection manifests | `docs/reviews/rule-projection/*` (18 files) |

Completion criterion: every surface above inspected against the six
dimensions, or explicitly excluded with a reason.

## Findings

| ID | Surface | Dim | Finding | Recommended disposition | Actual disposition | Authority |
|---|---|---|---|---|---|---|
| **DES-1** | `agf/docs/governance/manifest.md` L16-19 | 1 currency + 3 consistency | Stated "AGF v2.10.0 is the **pending reviewed release**" and "**v2.9.0 remains the latest published stable release**". Contradicted by `agf/docs/site/roadmap.md` L15/L23 and `docs/roadmap.md` L17 (both: v2.10.0 published & verified), by `agf/VERSION` (2.10.0), and by `CHANGELOG.md` (`## [2.10.0] - 2026-09-08`, a dated entry). Classic stale live-status prose cell — the `qvzj` / V29P failure class. | Fix: rewrite to "v2.10.0 is the latest published, independently verified stable release (tagged 2026-09-08)". | **FIXED** on this branch (mechanical currency correction, in scope). | steward |

## Dimensions swept so far

### 1. Framework-version currency — first pass
- `agf/VERSION` = `2.10.0`; `CHANGELOG.md` / `agf/CHANGELOG.md` latest dated entry `2.10.0 — 2026-09-08`; `## [Unreleased]` present above it. Baseline: **v2.10.0 is released.**
- `AGENTS.md:1` / `agf/templates/AGENTS.md:3` AGF version comment = `2.10.0` — **current, OK.**
- `agf/docs/governance/manifest.md` Version cell L11 = `2.10.0`, "to_version: 2.10.0" L77 — **current, OK**; but the prose at L16-19 was stale → **DES-1**.
- `agf/docs/site/roadmap.md` L15/L23/L26 and `docs/roadmap.md` L17 — v2.10.0 as latest, published, verified — **current, OK** (and now DES-1's fix target for consistency).
- `agf/templates/learning-record.yaml:8` `framework_version: "2.10.0"` — **current, OK.**
- Older version strings in `agf/docs/site/reference/release-artifacts.md` (`2.4.0` in example commands), `palette.md` (`v2.4.0` workstream refs), `commission.md:84` (`aarch64 through v2.5.0`) — **not currency findings**: example/illustrative command values and deliberate historical statements, all still accurate. `commission.md:84`'s "not currently built (CI runner capacity)" verified true today.

### 3. Cross-touchpoint claim consistency — first pass
- "Latest published release" claim: `manifest.md` disagreed with both roadmaps → **DES-1** (the only inconsistency found in this pass).
- Layout descriptor ("Layout B, single `agf/` directory") — consistent across `manifest.md`, `AGENTS.md`, `governance-file-layout-rule.md`. OK.

### 4. Pointer and path integrity — first pass (site docs → `docs/plans/` refs)
- `mcp-server.md:25` → `docs/plans/mcp-coordination-dir-contract-fix.md` — **resolves.**
- `coordination-protocol.md:270` → `docs/plans/v250-rc1-contract-core.md` — **resolves.**
- `palette.md:107,179` → `docs/plans/v2.4.0-workstream-d-figure-system.md` — **resolves.**
- `coordination-protocol.md:183` → `docs/plans/add-export-feature.md`, `first-governed-change.md` → `docs/plans/sample-login-error-message.md` — **intentional tutorial samples**, not real-file references; OK by context.
- No broken `docs/plans/` pointer found in this pass.

## Pending (next cycles)

- **Dim 2** — capability / maturity-level currency across the Pages site (every capability claim carries a Level + evidence status; no Level contradicted by the shipped operator path). High-value: the `agov` command inventory (homepage vs tooling page drift is a known past incident).
- **Dim 4** — remaining pointer classes: `agf/rules/**` cross-references, `agov` command/flag names, `see <rule>` owner pointers.
- **Dim 5** — projection-manifest completeness: every merged normative change since the last sweep (there is no prior sweep — so: since the estate baseline) has a `docs/reviews/rule-projection/<plan-id>.yaml`; each names a resolving owner + paths. The recent Row 5 / Row 6 / PMM-role changes are the immediate check.
- **Dim 6** — finalise the touchpoint manifest; classify every file under the adopter-facing `reads` paths as in-manifest or excluded-with-reason.

## Escalations to governor

None yet.
