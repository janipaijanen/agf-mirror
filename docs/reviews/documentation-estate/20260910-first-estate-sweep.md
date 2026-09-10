<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Documentation estate — first estate-wide sweep (2026-09-10)

**Role:** documentation-estate-steward (activated 2026-09-10, MR !2661).
**Trigger:** R-4 of the role-instantiation plan
`docs/plans/20260910T115843Z-documentation-estate-steward-role.md`; the
`release-content-freeze` cadence class for v2.11 — this sweep completing
with every finding dispositioned is v2.11 release-cut **condition 4**
(parent scope plan amendment R-2, MR !2662).
**Status:** COMPLETE — all six dimensions swept (2026-09-10). Four
findings: **DES-1, DES-2, DES-3 FIXED; DES-4 routed to build-engineer**
for a release-archive check (governor `proceed as proposed`, 2026-09-10).
v2.11 release-cut condition 4 ("every finding dispositioned") is met:
three fixed, one has a recorded disposition (build-engineer verification
then mechanical cell fix).

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
| **DES-1** | `agf/docs/governance/manifest.md` L16-19 | 1 currency + 3 consistency | Stated "AGF v2.10.0 is the **pending reviewed release**" and "**v2.9.0 remains the latest published stable release**". Contradicted by `agf/docs/site/roadmap.md` L15/L23 and `docs/roadmap.md` L17 (both: v2.10.0 published & verified), by `agf/VERSION` (2.10.0), and by `CHANGELOG.md` (`## [2.10.0] - 2026-09-08`, a dated entry). Classic stale live-status prose cell — the `qvzj` / V29P failure class. | Fix: rewrite to "v2.10.0 is the latest published, independently verified stable release (tagged 2026-09-08)". | **FIXED** (MR !2663, on `main`) — mechanical currency correction, in scope. | steward |
| **DES-2** | `AGENTS.md` "Active agents" section | 3 consistency + 5 projection | `documentation-estate-steward` is `status: active` in `agf/agents/MANIFEST.yaml` (the authoritative source AGENTS.md itself names) since MR !2661, but the AGENTS.md "Active agents" table and prose do not list it. A governed normative change (new standing role) that did not propagate to the loader touchpoint — MR !2661 changed the YAML + MANIFEST but not AGENTS.md. | Fix: add the role row to the "Active agents" table + a one-line activation note. | **FIXED** on this branch (mechanical currency correction — the role's active status is not in question; the loader just lagged). | steward |
| **DES-3** | `AGENTS.md` "Suspended:" line | 3 consistency | `AGENTS.md` listed `independent-implementation-reviewer` as **Suspended**, but governor decision `20260816T145707Z-governor-reviewer-3eqb` **reactivated** it (Gate 2 for MR !1632 at `903899f6`). `MANIFEST.yaml` `status: active` is correct; `AGENTS.md` was the stale surface. | Fix: move the role into the Active-agents table; state the reactivation record; drop it from "Suspended:". | **FIXED** on this branch (mechanical currency correction — reactivation confirmed by `3eqb`). | steward |
| **DES-4** | `agf/docs/site/reference/tooling-conformance.md` L52 (+ `maturity-vocabulary.md` L28-29) | 2 capability currency | `agov coord convergence --series <id>` marked "**Level 4 validated candidate; not deployable until released**", but announced in the **v2.8.0 CHANGELOG** (2026-08-28) as delivered, and mapped as a live MCP tool (`mcp-server.md:108`). The "not deployable until released" clause is almost certainly stale; the "Level 4" may still be a deliberate statement about the *review-convergence workflow's* evidence maturity, not the command's absence. | build-engineer confirms the v2.8.0+ `agov` archive ships `coord convergence`; the steward then either flips the cell to "Release surface" or rewrites it to separate command maturity (L5) from workflow evidence status. | **ROUTED to build-engineer** (`20260910T14xxxxZ` handoff); disposition recorded, mechanical fix follows the check. | build-engineer → steward |

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

### 2. Capability / maturity-level currency — COMPLETE
- `agf/docs/site/reference/maturity-vocabulary.md` — Level 1-5 + evidence-status defined and internally consistent; worked examples correct. OK.
- `agf/docs/site/index.md` — describes `agov` as optional, uses "Level 5 — Deployable" correctly, and **links** to `docs/adopt/tooling-and-tokens.md` as the canonical command inventory rather than duplicating a table. The homepage-table drift `qvzj` cites has already been corrected (no duplicated inventory to drift). OK.
- `agf/docs/site/reference/tooling-conformance.md:52` — `agov coord convergence` marked not-yet-released, but announced in the v2.8.0 CHANGELOG and mapped as a live MCP tool → **DES-4** (routed to build-engineer for the release-archive check).
- `agf/docs/site/docs/adopt/tooling-and-tokens.md:47` — cites the maturity vocabulary correctly. OK.

### 4. Pointer and path integrity — COMPLETE
- `docs/plans/` refs from site docs — all real refs resolve; tutorial samples OK by context (first pass, above).
- `agf/rules/<dir>/<file>.md` refs from site docs — **all resolve** (0 broken).
- `agov <command> <subcommand>` names in `agf/docs/site/**`, `README.md`, `AGENTS.md` (≈40 distinct) — **all resolve** against the current CLI command tree; top-level command set matches the docs' description. No removed/renamed command referenced.

### 5. Projection-manifest completeness — COMPLETE (1 advisory)
- 18 `docs/reviews/rule-projection/*` files present; the recent adopter-facing normative change with a projection obligation — MR !2666 (AGENTS.md gate-discipline addition) — carries a full six-row screen (inline in its Gate 1 plan).
- **Advisory:** the Row 5 / Row 6 / PMM-role work-order plans declare projection `not applicable` **inline** rather than as the structured `docs/reviews/rule-projection/<plan-id>.yaml` the rule names as "the one structured record". They change no Pages/runbook normative text (their slices will, each with its own manifest), so this is not a projection *gap* — but `agov plan projection-check` expects the YAML even to record `not applicable`. Recorded as an advisory for the plan authors; no adopter-facing inconsistency results.
- DES-2 was itself a projection miss (MR !2661 → AGENTS.md "Active agents"), now fixed.

### 6. Estate coverage / touchpoint manifest — COMPLETE (v1)
The "Estate scope" table above is the **v1 touchpoint manifest**. Every
file group under the role's `jurisdiction.reads` is either listed there or
excluded with a reason:
- **Included:** `README.md`, `AGENTS.md`, `agf/templates/*`,
  `agf/docs/site/**/*.md`, `agf/docs/site/roadmap.md`,
  `agf/docs/governance/manifest.md`, `docs/roadmap.md`, the version
  anchors, the 18 projection manifests.
- **Excluded with reason:** `agf/docs/agent-communication-0.26.0.md`
  (dated historical design note, not current adopter guidance —
  filename carries its own version); `agf/rules/**` bodies (the steward
  checks their *estate-wide consistency*, not their content — that is the
  rules' own owners); `agf/schemas/**` (machine contracts, not prose);
  `docs/plans/**` and `docs/coordination/**` (working artifacts, not
  adopter-facing — `documentation-audience-rule`).
Each subsequent sweep refines this manifest (dimension 6 is standing).

## Sweep completion

All six dimensions swept. Findings: **DES-1 FIXED** (manifest.md version
status, MR !2663), **DES-2 FIXED** (AGENTS.md role table, this MR),
**DES-3 FIXED** (`AGENTS.md` was stale — the role was reactivated by
`3eqb`; MANIFEST was correct), **DES-4 ROUTED** to build-engineer for a
release-archive check with the mechanical cell fix to follow. One advisory
(dim 5 inline vs YAML projection screens). No other currency, consistency,
pointer, or coverage defect found across the adopter-facing estate at
`main` post-`ab81253c`.

**v2.11 release-cut condition 4:** every finding has a recorded
disposition — DES-1/2/3 fixed, DES-4 routed with its follow-up defined.

## Escalations to governor

- **DES-3** — RESOLVED: `AGENTS.md` was stale; `3eqb` reactivated the role. Fixed on this MR.
- **DES-4** — routed to **build-engineer**: confirm the v2.8.0+ `agov` release archive ships `coord convergence`; the steward then applies the `tooling-conformance.md:52` / `maturity-vocabulary.md` cell fix.
