<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Documentation estate — first estate-wide sweep (2026-09-10)

**Role:** documentation-estate-steward (activated 2026-09-10, MR !2661).
**Trigger:** R-4 of the role-instantiation plan
`docs/plans/20260910T115843Z-documentation-estate-steward-role.md`; the
`release-content-freeze` cadence class for v2.11 — this sweep completing
with every finding dispositioned is v2.11 release-cut **condition 4**
(parent scope plan amendment R-2, MR !2662).
**Status:** **INCOMPLETE — corrected 2026-09-11 (EC-1,
`20260911T171634Z-contribution-auditor-s214`).** The four *findings this
sweep did identify* (DES-1 through DES-4) are all **FIXED**, and those
fixes stand on their own — but the sweep's own claimed coverage does not
substantiate a "all six dimensions complete" or "v2.11 condition 4 fully
met" assertion. Concrete gaps, verified independently before this
correction: the scope table's Pages-site row says "19 Markdown files";
`git ls-tree -r --name-only e2b786c4 agf/docs/site` returns **47**.
Several adopter-facing surfaces the role's read jurisdiction covers were
never entered in the scope table at all: `agf/docs/adopt/compliance-profile.md`,
`agf/docs/adopt/decommission.md`, `agf/docs/contracts/*.md` (4 files),
other governance documents beyond `manifest.md`, `agf/ADOPTING.md`, and
`agf/learnings/foundry.yaml`'s own `framework_version` cell. `agf/rules/**`
bodies were excluded from *content* review (correctly — that's the rules'
own owners), but the role's separate duty to read their CUSTOMIZE/
normative-claim surface for estate-wide consistency was not exercised.
None of this proves a defect exists in the unswept material — it means
the sweep cannot honestly claim to have checked it. **v2.11 release-cut
condition 4 ("the first estate-wide assessment must be complete and every
finding dispositioned") is therefore not met by this report as it
stands.** The four historical dimension-pass sections below (their
"COMPLETE" labels) and the original completion claims in "Sweep
completion" / "Escalations to governor" are preserved unedited as
historical record per append-only convention; this status line and the
correction note at the end of the document are the current, accurate
statement.

## Estate scope (the touchpoint manifest — dimension 6, provisional)

Adopter-facing surfaces inspected or queued for this sweep:

| Group | Surfaces |
|---|---|
| Root | `README.md`, `AGENTS.md` |
| Templates | `agf/templates/*` (10 files incl. `AGENTS.md`, `CLAUDE.md`, `gate1-plan.md`, `learning-record.yaml`, coord-* templates, `README.md`) |
| Pages site | `agf/docs/site/**/*.md` — **corrected 2026-09-11 (EC-1): the original "19 md files" count was wrong; `git ls-tree -r --name-only agf/docs/site` returns 47.** Only a subset of these 47 were individually named/verified in the dimension passes below; the rest are UNEXAMINED, not confirmed-OK. + `agf/docs/site/roadmap.md` |
| Governance docs | `agf/docs/governance/manifest.md` — **UNEXAMINED beyond this one file (EC-1):** other governance documents under `agf/docs/**` outside `site/` were not inventoried. |
| Other agf/docs | `agf/docs/agent-communication-0.26.0.md` — **checked 2026-09-12 (EC-2):** `agf/docs/adopt/compliance-profile.md`, `agf/docs/contracts/binding-contract.md`, `agf/docs/contracts/contract-stability.md`, `agf/docs/contracts/tooling-conformance.md`, `agf/ADOPTING.md` — scanned for version-currency and maturity-claim defects; none found (all speak generically of "AGF v2" rather than a specific numbered version, so they cannot go stale the way a pinned-version claim would; existing "planned/deferred" qualifiers on binding-validation content were confirmed still accurate against current Rust `agov`). `agf/docs/contracts/integration-contract.md` — **checked 2026-09-12 (EC-2): a currency defect was found → DES-7 (open, not yet routed).** `agf/docs/adopt/decommission.md` — **checked 2026-09-12 (EC-2), defect found and fixed → DES-5.** |
| Roadmaps | `docs/roadmap.md` + `agf/docs/site/roadmap.md` |
| Version anchors | `agf/VERSION` (2.10.0), `CHANGELOG.md`, `agf/CHANGELOG.md`, `agf/templates/learning-record.yaml` framework_version cell, `agf/learnings/foundry.yaml`'s own `framework_version` cell — **checked 2026-09-12 (EC-2): `agf/learnings/foundry.yaml:38` reads `framework_version: "2.10.0"`, matching `agf/VERSION`. No defect.** |
| Rules estate-wide consistency (read-only duty, not content ownership) | `agf/rules/**` CUSTOMIZE/normative-claim surfaces — **UNEXAMINED (EC-1):** content excluded correctly (owned by the rules themselves), but the role's separate duty to read for cross-estate consistency was not exercised this pass |
| Projection manifests | `docs/reviews/rule-projection/*` (18 files) |

Completion criterion: every surface above inspected against the six
dimensions, or explicitly excluded with a reason. **Not yet met — see
"Status" above and the EC-1 correction at the end of this document.**

## Findings

| ID | Surface | Dim | Finding | Recommended disposition | Actual disposition | Authority |
|---|---|---|---|---|---|---|
| **DES-1** | `agf/docs/governance/manifest.md` L16-19 | 1 currency + 3 consistency | Stated "AGF v2.10.0 is the **pending reviewed release**" and "**v2.9.0 remains the latest published stable release**". Contradicted by `agf/docs/site/roadmap.md` L15/L23 and `docs/roadmap.md` L17 (both: v2.10.0 published & verified), by `agf/VERSION` (2.10.0), and by `CHANGELOG.md` (`## [2.10.0] - 2026-09-08`, a dated entry). Classic stale live-status prose cell — the `qvzj` / V29P failure class. | Fix: rewrite to "v2.10.0 is the latest published, independently verified stable release (tagged 2026-09-08)". | **FIXED** (MR !2663, on `main`) — mechanical currency correction, in scope. | steward |
| **DES-2** | `AGENTS.md` "Active agents" section | 3 consistency + 5 projection | `documentation-estate-steward` is `status: active` in `agf/agents/MANIFEST.yaml` (the authoritative source AGENTS.md itself names) since MR !2661, but the AGENTS.md "Active agents" table and prose do not list it. A governed normative change (new standing role) that did not propagate to the loader touchpoint — MR !2661 changed the YAML + MANIFEST but not AGENTS.md. | Fix: add the role row to the "Active agents" table + a one-line activation note. | **FIXED** on this branch (mechanical currency correction — the role's active status is not in question; the loader just lagged). | steward |
| **DES-3** | `AGENTS.md` "Suspended:" line | 3 consistency | `AGENTS.md` listed `independent-implementation-reviewer` as **Suspended**, but governor decision `20260816T145707Z-governor-reviewer-3eqb` **reactivated** it (Gate 2 for MR !1632 at `903899f6`). `MANIFEST.yaml` `status: active` is correct; `AGENTS.md` was the stale surface. | Fix: move the role into the Active-agents table; state the reactivation record; drop it from "Suspended:". | **FIXED** on this branch (mechanical currency correction — reactivation confirmed by `3eqb`). | steward |
| **DES-4** | `agf/docs/site/reference/tooling-conformance.md` L52 | 2 capability currency | `agov coord convergence --series <id>` marked "**Level 4 validated candidate; not deployable until released**", but announced in the **v2.8.0 CHANGELOG** (2026-08-28) as delivered, and mapped as a live MCP tool (`mcp-server.md:108`). | build-engineer confirms the v2.8.0+ `agov` archive ships `coord convergence`, then separates command maturity (L5) from workflow evidence status. | **FIXED** (2026-09-11). Verified independently: downloaded the published v2.10.0 Linux x86_64 archive, confirmed its SHA-256 (`b54c55b5b31238973514c130e534b846de701c3c2a0368f81682f295c08383fd`) matches the published `.sha256` asset, and confirmed `agov coord convergence --help` runs (exit 0) from the extracted binary — cross-checked against Codex's independent same-archive verification (`gtsg`), both agree. Cell now reads "Release surface"; a new prose paragraph separates the command's Level 5 Deployable status (shipped since v2.8.0) from the review-convergence *workflow's* own Empirically informed (not yet Empirically validated at scale) evidence status. | build-engineer |
| **DES-5** | `agf/docs/adopt/decommission.md` L33-46 | 3 consistency | Described adoption as having a choice between "Layout A: root-level" and "Layout B: subdirectory" and told adopters to delete `governance/agents/*.yaml` and `learnings/foundry.yaml`. `governance-file-layout-rule.md` states AGF v2.0 supports exactly one layout (the visible `agf/` directory) and its ownership table places agent definitions at `agf/agents/*.yaml` and learning records at `agf/learnings/<codename>.yaml` — the dual-layout choice was removed from that rule on 2026-08-01, but `decommission.md` was never updated to match. | Fix: remove the layout-choice framing, correct the two paths, and flag `framework.yaml` as the still-planned [P1] surface `integration-contract.md` describes it as, not an artifact every adoption ships. | **FIXED, then corrected again 2026-09-12** after independent exact-head review (`q2pt`) found two residual defects in the first fix: (F1) the "Keep your own learning record" line, the removal-commit example, and the verification `grep -v` exclusion still said `learnings/<codename>.yaml` instead of `agf/learnings/<codename>.yaml` — corrected, plus made the shipped `foundry.yaml` example's deletion conditional on it not also holding adopter-owned history; (F2) the "planned [P1]" claim for `framework.yaml` contradicted `integration-contract.md`'s own L385 ("Validator shipped as `agov validate framework-config`") — reworded to describe the shipped, vacuously-passing validator without asserting a maturity level the contract itself disputes internally (see DES-7 below for that internal contradiction). Also corrected the path to `agf/framework.yaml`, matching the Rust implementation (`main.rs` checks `agf/framework.yaml`, not repo-root `framework.yaml` as the contract's trigger table implies). | planner-architect |
| **DES-7** | `agf/docs/contracts/integration-contract.md` L147, L315, L385 | 1 currency (internal contradiction) + 4 pointer integrity | Discovered via `q2pt`'s DES-5 review, not an original estate-sweep pass. The contract disagrees with itself on `framework.yaml`'s status: L147's command table calls it "optional... passes when no `framework.yaml` exists" (neutral); L315 says "Once `framework.yaml` ships ([P1])..." (implies not yet shipped); L385's capability table says "[P1] ... **Validator shipped** as `agov validate framework-config`" (implies already shipped). Separately, the "Path-based revalidation triggers" table (L156-163) lists the trigger path as repo-root `framework.yaml`, but the Rust implementation (`tooling/rust/crates/agov/src/main.rs` `ValidateCommand::FrameworkConfig`) actually checks `agf/framework.yaml`. | Route to this contract's own owner to reconcile L315 vs. L385's shipped-status disagreement and correct the trigger-table path to `agf/framework.yaml`. Out of DES-5's mechanical-correction scope — not attempted here per the reviewing findings' own guidance not to rewrite a normative contract beyond that scope. | **OPEN — not yet routed to an owner/decision.** | planner-architect |

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

### 2. Capability / maturity-level currency — PARTIAL, not COMPLETE (corrected 2026-09-11, EC-1)
- `agf/docs/site/reference/maturity-vocabulary.md` — Level 1-5 + evidence-status defined and internally consistent; worked examples correct. OK.
- `agf/docs/site/index.md` — describes `agov` as optional, uses "Level 5 — Deployable" correctly, and **links** to `docs/adopt/tooling-and-tokens.md` as the canonical command inventory rather than duplicating a table. The homepage-table drift `qvzj` cites has already been corrected (no duplicated inventory to drift). OK.
- `agf/docs/site/reference/tooling-conformance.md:52` — `agov coord convergence` marked not-yet-released, but announced in the v2.8.0 CHANGELOG and mapped as a live MCP tool → **DES-4** (fixed 2026-09-11, build-engineer release-archive verification complete).
- `agf/docs/site/docs/adopt/tooling-and-tokens.md:47` — cites the maturity vocabulary correctly. OK.

### 4. Pointer and path integrity — PARTIAL, not COMPLETE (corrected 2026-09-11, EC-1)
- `docs/plans/` refs from site docs — all real refs resolve; tutorial samples OK by context (first pass, above).
- `agf/rules/<dir>/<file>.md` refs from site docs — **all resolve** (0 broken), for the refs actually checked.
- `agov <command> <subcommand>` names in `agf/docs/site/**`, `README.md`, `AGENTS.md` (≈40 distinct) — **all resolve** against the current CLI command tree; top-level command set matches the docs' description. No removed/renamed command referenced.
- These checks were run against the site docs actually inspected in dimension passes above, not against the full 47-file site tree (EC-1) or the newly identified `agf/docs/adopt/**`, `agf/docs/contracts/**`, `agf/ADOPTING.md` surfaces — their pointer integrity is UNEXAMINED.

### 5. Projection-manifest completeness — COMPLETE (1 advisory)
- 18 `docs/reviews/rule-projection/*` files present; the recent adopter-facing normative change with a projection obligation — MR !2666 (AGENTS.md gate-discipline addition) — carries a full six-row screen (inline in its Gate 1 plan).
- **Advisory:** the Row 5 / Row 6 / PMM-role work-order plans declare projection `not applicable` **inline** rather than as the structured `docs/reviews/rule-projection/<plan-id>.yaml` the rule names as "the one structured record". They change no Pages/runbook normative text (their slices will, each with its own manifest), so this is not a projection *gap* — but `agov plan projection-check` expects the YAML even to record `not applicable`. Recorded as an advisory for the plan authors; no adopter-facing inconsistency results.
- DES-2 was itself a projection miss (MR !2661 → AGENTS.md "Active agents"), now fixed.

### 6. Estate coverage / touchpoint manifest — INCOMPLETE, not v1-final (corrected 2026-09-11, EC-1)
The "Estate scope" table above was originally presented as a complete v1
touchpoint manifest. It was not: it undercounted the Pages site by 28
files (19 claimed vs. 47 actual) and omitted several surfaces the role's
`jurisdiction.reads` covers outright (see the "Estate scope" table's
EC-1 annotations above). Corrected inventory as of this pass:
- **Included and actually inspected:** `README.md`, `AGENTS.md`,
  `agf/templates/*`, the subset of `agf/docs/site/**/*.md` named in the
  dimension-2/4 passes above, `agf/docs/site/roadmap.md`,
  `agf/docs/governance/manifest.md`, `docs/roadmap.md`, the version
  anchors named in dimension 1, the 18 projection manifests.
- **Excluded with reason (unchanged, still valid):**
  `agf/docs/agent-communication-0.26.0.md` (dated historical design
  note); `agf/schemas/**` (machine contracts, not prose); `docs/plans/**`
  and `docs/coordination/**` (working artifacts, not adopter-facing —
  `documentation-audience-rule`).
- **`agf/rules/**` bodies:** content correctly excluded (owned by the
  rules themselves), but their CUSTOMIZE/normative-claim surfaces were
  never read for cross-estate consistency, which the role's mandate
  separately requires. UNEXAMINED, not excluded-with-reason.
- **UNEXAMINED, newly identified, no disposition yet:** the ~28
  unnamed `agf/docs/site/**` files beyond the ones the dimension-2/4
  passes named; `agf/docs/adopt/compliance-profile.md`;
  `agf/docs/adopt/decommission.md`; the 4 files under
  `agf/docs/contracts/`; other `agf/docs/governance/**` documents beyond
  `manifest.md`; `agf/ADOPTING.md`; `agf/learnings/foundry.yaml`'s
  `framework_version` cell.
A future sweep must classify every UNEXAMINED item above as inspected or
excluded-with-reason before this dimension, or the report's overall
completion claim, can honestly read COMPLETE again.

## Sweep completion

**Superseded by the EC-1 correction below — retained as the original,
now-inaccurate claim, not the current status.** Original text: "All six
dimensions swept. Findings: DES-1 FIXED ... DES-4 FIXED ... No other
currency, consistency, pointer, or coverage defect found across the
adopter-facing estate at `main` post-`ab81253c`." That "no other defect
found" claim was never substantiated for the unexamined surfaces listed
under dimension 6 above — absence of a finding on unswept material is not
evidence of no defect.

**v2.11 release-cut condition 4:** **not met.** The four identified
findings (DES-1 through DES-4) are genuinely fixed, but "every finding
dispositioned" presupposes a complete sweep, which this report does not
have. A future sweep pass must classify every UNEXAMINED surface in
dimension 6 above before this condition can be honestly claimed met.

## EC-1 correction (2026-09-11)

Per `20260911T171634Z-contribution-auditor-s214`: this report's original
"COMPLETE"/"fully met" language overstated its actual coverage. Verified
independently (`git ls-tree -r --name-only agf/docs/site`, confirmed 47
Markdown files against the report's claimed 19) before applying this
correction. DES-1 through DES-4 remain valid, disposed fixes — this
correction does not reopen or weaken any of them. It withdraws the
report's completion claim and the "v2.11 condition 4 fully met" line, and
converts every previously-implicit "no other defect found" into an
explicit UNEXAMINED disposition per the "Estate scope" and dimension 6
sections above. The immutable historical coordination records
(`20260910T144148Z-documentation-estate-steward-pjsp`,
`20260911T080625Z-build-engineer-jqrg`, and others referencing this
report's original completion claims) are not rewritten; this document
correction is the applicable current-state record going forward.

## EC-2 continuation (2026-09-12)

Following S2/S3 merge (`!2713`) and the coder-tester handoff `q286`
routing "estate corrections/completion" back to planner-architect: closed
a portion of the EC-1 UNEXAMINED gap. Checked `agf/docs/adopt/
compliance-profile.md`, `agf/docs/adopt/decommission.md`,
`agf/docs/contracts/binding-contract.md`,
`agf/docs/contracts/contract-stability.md`,
`agf/docs/contracts/integration-contract.md`,
`agf/docs/contracts/tooling-conformance.md`, `agf/ADOPTING.md`, and
`agf/learnings/foundry.yaml`'s `framework_version` cell — two defects
found: **DES-5** (`decommission.md`'s stale dual-layout framing) was
fixed, and **DES-7** (`integration-contract.md`'s internal contradiction
over `framework.yaml`'s shipped status, discovered via `q2pt`'s DES-5
review) was found and remains open, not yet routed to an owner/decision
(see the DES-7 row above). The remaining checked files had no currency
defect found (all use "AGF v2" generically rather than a pinned version,
so a version bump cannot make them stale the way `DES-1`/`DES-4` were).

**Still UNEXAMINED after EC-2:** the ~28 unnamed `agf/docs/site/**`
files beyond those the dimension-2/4 passes named; other
`agf/docs/governance/**` documents beyond `manifest.md`
(`disclaimer.md`, `retrospective.md`, `anonymization-notice.md`,
`learning-system.md`, `governance.md`); and the `agf/rules/**`
cross-estate-consistency read-through. This report still does not meet
the v2.11 condition 4 completion bar — EC-2 narrows the gap, it does
not close it.

## Escalations to governor

**DES-3** — RESOLVED: `AGENTS.md` was stale; `3eqb` reactivated the role.
Fixed on this MR. **DES-4** — RESOLVED: build-engineer confirmed the
v2.10.0 `agov` release archive ships `coord convergence`
(checksum-verified download, `--help` runs); `tooling-conformance.md:52`
corrected 2026-09-11. **Outstanding:** the EC-1 coverage gap above —
completing the missing sweep (or a governor decision to accept the
narrower, already-fixed scope as sufficient for v2.11 condition 4) is not
resolved by this correction pass; it names the gap honestly rather than
closing it.
