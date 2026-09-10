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
findings: DES-1 FIXED, DES-2 FIXED, DES-3 and DES-4 REFERRED to the
governor. The two referrals are dispositioned by a governor record;
until then, v2.11 release-cut condition 4 ("every finding dispositioned")
is met for the two `steward`-authority findings and pending on the two
referrals.

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
| **DES-3** | `AGENTS.md` "Suspended:" line vs `agf/agents/MANIFEST.yaml` | 3 consistency | AGENTS.md lists `independent-implementation-reviewer` as **Suspended**; MANIFEST.yaml shows `status: active` with a note that this "reflects the proposed end state, not yet in effect until that decision merges". The two disagree on the reader's face; whether AGENTS.md is correct depends on whether the reactivation Gate 2 decision has merged. | **VERIFY, then reconcile.** Not a mechanical fix — needs confirming the reactivation decision's state. | **referred** — routed to governor: has the `independent-implementation-reviewer` reactivation (MR !1289 trail) received its binding Gate 2 decision? If yes, both surfaces say active; if no, MANIFEST's `status:` is the misleading cell. | governor |
| **DES-4** | `agf/docs/site/reference/tooling-conformance.md` L52 (+ `maturity-vocabulary.md` L28-29) | 2 capability currency | `agov coord convergence --series <id>` is marked "**Level 4 validated candidate; not deployable until released**". It was announced in the **v2.8.0 CHANGELOG** (2026-08-28) and v2.8.0 / v2.9.0 / v2.10.0 have all since shipped. If the v2.8.0+ release archives include the command it is now Level 5 Deployable and the cell is stale. The steward cannot confirm from source alone (needs a release-binary check). | **VERIFY** the v2.8.0+ `agov` archive includes `coord convergence`; if yes, change the cell to "Release surface" and update `maturity-vocabulary.md`'s worked example. | **referred** — routed to build-engineer / governor for the release-artifact check. | build-engineer / governor |

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
- `agf/docs/site/reference/tooling-conformance.md:52` — `agov coord convergence` marked not-yet-released, but announced in the v2.8.0 CHANGELOG → **DES-4** (referred).
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
**DES-3 REFERRED** (independent-implementation-reviewer status), **DES-4
REFERRED** (`coord convergence` release state). One advisory (dim 5 inline
vs YAML projection screens). No other currency, consistency, pointer, or
coverage defect found across the adopter-facing estate at `main`
post-`ab81253c`.

**v2.11 release-cut condition 4:** met for the two steward-authority
findings; the two governor/build-engineer referrals (DES-3, DES-4) need a
disposition record before the condition is fully satisfied.

## Escalations to governor

- **DES-3** — `independent-implementation-reviewer` shows Suspended in AGENTS.md but `status: active` in MANIFEST.yaml. Confirm whether the reactivation Gate 2 decision merged, then reconcile both surfaces.
- **DES-4** — `agov coord convergence` marked not-yet-released in `tooling-conformance.md:52` but announced in the v2.8.0 CHANGELOG. Confirm (build-engineer) whether the v2.8.0+ release archive ships the command; if so, update the cell to "Release surface" and the `maturity-vocabulary.md` worked example.
