<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Documentation estate — first estate-wide sweep (2026-09-10)

**Role:** documentation-estate-steward (activated 2026-09-10, MR !2661).
**Trigger:** R-4 of the role-instantiation plan
`docs/plans/20260910T115843Z-documentation-estate-steward-role.md`; the
`release-content-freeze` cadence class for v2.11 — this sweep completing
with every finding dispositioned is v2.11 release-cut **condition 4**
(parent scope plan amendment R-2, MR !2662).
**Status:** **File-coverage reconciliation complete (EC-7,
2026-09-19); binding "condition 4 met" disposition still pending a
governor decision.** All 48 current `agf/docs/site/**/*.md` files are
now individually accounted for across EC-1 through EC-6 plus this
reconciliation — see EC-7 below for the method and residual. This
does *not* mean every file received uniform six-dimension depth (EC-6
itself distinguishes broad-scan-only coverage from targeted reads on
higher-drift-risk files); it means the coverage-accounting gap that
kept this report's own completion criterion unmet is closed. Originally
corrected 2026-09-11 (EC-1, `20260911T171634Z-contribution-auditor-s214`)
after the scope table's Pages-site row wrongly said "19 Markdown files";
`git ls-tree -r --name-only e2b786c4 agf/docs/site` returned **47** at
that time (now 48; one file added since). The four *findings that first
sweep did identify* (DES-1 through DES-4) were **FIXED** immediately and
those fixes stood on their own throughout. Several adopter-facing surfaces
the role's read jurisdiction covers were
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
| Governance docs | `agf/docs/governance/manifest.md`, `agf/docs/governance/governance.md` — **checked 2026-09-12 (EC-2), defect found and fixed → DES-6** (rewrite implemented on this MR per `3rha`; see EC-3 below). `agf/docs/governance/disclaimer.md`, `retrospective.md`, `anonymization-notice.md`, `learning-system.md` — **checked 2026-09-12 (EC-2):** scanned for version-currency and stale-role/path claims; none found (no version-number, layout-choice, or role-roster claims in the checked slice). |
| Other agf/docs | `agf/docs/agent-communication-0.26.0.md` — **checked 2026-09-12 (EC-2):** `agf/docs/adopt/compliance-profile.md`, `agf/docs/contracts/binding-contract.md`, `agf/docs/contracts/contract-stability.md`, `agf/docs/contracts/tooling-conformance.md`, `agf/ADOPTING.md` — scanned for version-currency and maturity-claim defects; none found (all speak generically of "AGF v2" rather than a specific numbered version, so they cannot go stale the way a pinned-version claim would; existing "planned/deferred" qualifiers on binding-validation content were confirmed still accurate against current Rust `agov`). `agf/docs/contracts/integration-contract.md` — **checked 2026-09-12 (EC-2): a currency defect was found → DES-7 (fixed EC-4, 2026-09-17).** `agf/docs/adopt/decommission.md` — **checked 2026-09-12 (EC-2), defect found and fixed → DES-5.** |
| Roadmaps | `docs/roadmap.md` + `agf/docs/site/roadmap.md` |
| Version anchors | `agf/VERSION` (2.10.0), `CHANGELOG.md`, `agf/CHANGELOG.md`, `agf/templates/learning-record.yaml` framework_version cell, `agf/learnings/foundry.yaml`'s own `framework_version` cell — **checked 2026-09-12 (EC-2): `agf/learnings/foundry.yaml:38` reads `framework_version: "2.10.0"`, matching `agf/VERSION`. No defect.** |
| Rules estate-wide consistency (read-only duty, not content ownership) | `agf/rules/**` CUSTOMIZE/normative-claim surfaces — **checked 2026-09-12 (EC-2):** scanned every rule body for the same failure classes DES-5/DES-6 surfaced (stale dual-layout language, hardcoded model names, pinned version numbers, role names outside the current roster, dead `agf/` path references). No new defect found; one apparent path miss (`agf/rules/workflow/unassigned-work-discovery-rule.md`, cited in `documentation-rule-projection-rule.md:118`) is that rule's own worked example of a deliberately hypothetical `stage: planned` net-new rule, not a live broken reference. |
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
| **DES-7** | `agf/docs/contracts/integration-contract.md` L147, L315, L385 | 1 currency (internal contradiction) + 4 pointer integrity | Discovered via `q2pt`'s DES-5 review, not an original estate-sweep pass. The contract disagrees with itself on `framework.yaml`'s status: L147's command table calls it "optional... passes when no `framework.yaml` exists" (neutral); L315 says "Once `framework.yaml` ships ([P1])..." (implies not yet shipped); L385's capability table says "[P1] ... **Validator shipped** as `agov validate framework-config`" (implies already shipped). Separately, the "Path-based revalidation triggers" table (L156-163) lists the trigger path as repo-root `framework.yaml`, but the Rust implementation (`tooling/rust/crates/agov/src/main.rs` `ValidateCommand::FrameworkConfig`) actually checks `agf/framework.yaml`. | Route to this contract's own owner to reconcile L315 vs. L385's shipped-status disagreement and correct the trigger-table path to `agf/framework.yaml`. Out of DES-5's mechanical-correction scope — not attempted here per the reviewing findings' own guidance not to rewrite a normative contract beyond that scope. | **FIXED (EC-4, 2026-09-17).** Corrected the trigger-table paths: all three (`agents/**`, `agents/MANIFEST.yaml`, `learnings/**`, `framework.yaml`) were missing the `agf/` prefix — a uniform staleness across the whole table, not only the `framework.yaml` row DES-7 originally named, matching `governance-file-layout-rule.md`'s ownership table and the Rust implementation's actual `agf/framework.yaml` check. Reconciled the L315/L385 disagreement by distinguishing two separate claims that had been conflated: the validator *command* (`agov validate framework-config`) is shipped and passes vacuously when no `agf/framework.yaml` exists; the framework.yaml *capability* (an adopter actually authoring one, and the compliance report comparing against it) remains planned [P1]. Both sides of the contradiction now say the same thing. No policy or maturity-level decision made — this was a wording reconciliation between two paragraphs that were already individually correct about different things, plus a path-prefix correction of the same class as DES-1/DES-5. Within the steward's mechanical-correction mandate; did not require governor disposition the way DES-6 did. | planner-architect |
| **DES-6** | `agf/docs/governance/governance.md` (whole file) | 1 currency + 3 consistency | Footer (L122) reads "Framework 1.1.50 \| Layout B" against a project currently on AGF v2.10.0. Body describes a superseded Phase 1 arrangement: "Claude plays both Planner-Architect and Coder-Tester" (L22, since split — Codex plays coder-tester per the durable role assignment), model name `claude-sonnet-4-6` (L16-17, stale), a coordination-message schema keyed on `seq` (L93, superseded by the current readable-ID scheme), an Artifact Ownership table that does not list `build-engineer`, `documentation-estate-steward`, `capability-developer`, or several other roles `agf/agents/MANIFEST.yaml` and `AGENTS.md` now carry, and a "GOVERNANCE.md" ownership row (L80) pointing at `agf/GOVERNANCE.md`, a path that does not exist — the only `GOVERNANCE.md` in the tree is `agents-govern/GOVERNANCE.md` (bootstrap reference, not this project's own charter) and there is no project-root `GOVERNANCE.md` per `governance-file-layout-rule.md`'s ownership table. | Escalated to governor for disposition (was this file still the live charter needing a rewrite, or superseded and archival). | **FIXED** (2026-09-12, this branch; correction round 2026-09-17), per governor decision `20260912T094217Z-governor-reviewer-3rha` ("rewrite ... to describe the current governed role, ownership, coordination and gate model"). Rewrote the file to summarize and point at live sources (`AGENTS.md`, `agf/agents/MANIFEST.yaml`, `agf/rules/**`) instead of re-encoding a copy of role/schema/gate detail that can drift independently — the same failure mode DES-6 itself demonstrates. Corrected the dead `agf/GOVERNANCE.md` cross-reference and the version/model/schema staleness. The first pass also duplicated `governance-file-layout-rule.md`'s ownership mini-table and silently dropped its `agf/agents/*.yaml` row's "pending #79" qualifier and footnote (contribution-audit finding DES6-R2) — corrected 2026-09-17 by removing the duplicated mini-table entirely and pointing only to the canonical rule, so this file cannot re-drift from it a second time. No role activation, gate power, schema, or policy change made, per the governor decision's explicit bound. | planner-architect |
| **DES-9** | `agf/docs/site/evaluate.md` reviewer evidence index | 4 pointer integrity | Named `agf/schemas/learning-record.schema.json`, which does not exist; the shipped learning-record schema is `agf/schemas/learning.schema.json`. | Correct the literal schema path. | **FIXED** in EC-6. | contribution-auditor acting on `kak5` handoff |
| **DES-10** | `agf/docs/site/content-plan.md` | 1 currency + 4 pointer integrity | The excluded planning-control file still presented its original pre-release v2.0 status taxonomy as current and pointed to removed `docs/paper/agents-govern-framework.md` as the canonical glossary. It is not the current site inventory or release authority. | Mark the file explicitly historical, point current inventory/version readers to MkDocs navigation and generated release information, and remove the dead glossary-owner claim without rewriting the preserved original plan table. | **FIXED** in EC-6. | contribution-auditor acting on `kak5` handoff |
| **DES-11** | `agf/docs/site/reference/figures.md` introduction | 3 consistency + 4 pointer/source integrity | Claimed all figures are generated from DOT, but the shipped `agf-nutshell-flyer.png` is generated from an HTML template and Python build script under its own source directory. | State the DOT/SVG rule accurately and name the flyer exception and its source. | **FIXED** in EC-6. | contribution-auditor acting on `kak5` handoff |
| **DES-12** | `agf/docs/site/reference/environment-variables.md` | 2 capability currency + 3 consistency | Called itself the canonical `AGOV_*`/`AGF_*` reference but omitted operator-facing `AGOV_MCP_AGOV_BIN` and `AGOV_RELEASE_TOKEN`, both consumed by shipped Rust binaries and documented piecemeal elsewhere. The blanket scope also accidentally implied compile/test/job-local variables belonged in an operator reference. | Define the page's operator-runtime boundary and add both shipped operator variables, including secret-handling and fail-closed behavior for the token. | **FIXED** in EC-6. | contribution-auditor acting on `kak5` handoff |

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
`agf/docs/contracts/tooling-conformance.md`, `agf/ADOPTING.md`,
`agf/learnings/foundry.yaml`'s `framework_version` cell, and the four
remaining `agf/docs/governance/**` files (`disclaimer.md`,
`retrospective.md`, `anonymization-notice.md`, `learning-system.md`) —
three defects found: **DES-5** (`decommission.md`'s stale dual-layout
framing) was fixed, then corrected again after `q2pt`'s DES-5 review
found two residual defects (see the DES-5 row above); **DES-7**
(`integration-contract.md`'s internal contradiction over
`framework.yaml`'s shipped status, discovered via `q2pt`'s DES-5 review)
was found and remains open, not yet routed to an owner/decision (see the
DES-7 row above); and **DES-6** (`agf/docs/governance/governance.md`'s
whole-document staleness) was found, escalated to the governor, and then
resolved per decision `3rha` authorizing a pointer-first rewrite — done
on this MR (see the DES-6 row above; a residual DES6-R2 ownership-table
defect from the first rewrite pass was corrected 2026-09-17, see EC-3
below). Also closed the `agf/rules/**` cross-estate-consistency
read-through (no new defect). The remaining checked files had no
currency defect found (all use "AGF v2" generically rather than a pinned
version, so a version bump cannot make them stale the way `DES-1`/`DES-4`
were).

**Still UNEXAMINED after EC-2:** only the ~28 unnamed
`agf/docs/site/**` files beyond those the dimension-2/4 passes named.
This report still does not meet the v2.11 condition 4 completion bar —
EC-2 narrows the gap to that single remaining surface, it does not
close it.

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

**DES-6** (added EC-2, 2026-09-12) — **NEW, requires governor
disposition:** `agf/docs/governance/governance.md` is stale across
several axes at once (framework version, model name, coordination
schema, role roster, a broken `agf/GOVERNANCE.md` cross-reference to a
path that doesn't exist). The steward's estate-sweep mandate is
mechanical currency/consistency correction; deciding whether this file
is still this project's live governance charter (needing a full
rewrite to match `AGENTS.md`/`agf/agents/MANIFEST.yaml`) or has been
superseded and should be marked historical is a role-authority
question outside that mandate. Requesting a governor decision on
disposition before any content rewrite is attempted.

**DES-7** (added while responding to `q2pt`'s DES-5 review,
2026-09-12) — **OPEN, not yet routed.**
`agf/docs/contracts/integration-contract.md` disagrees with itself on
`framework.yaml`'s shipped status (L315 says "once it ships", L385
says "Validator shipped") and its trigger-path table doesn't match the
Rust implementation's actual `agf/framework.yaml` check. Out of
DES-5/DES-6's mechanical-correction scope; needs routing to that
contract's own owner. See Findings table.

## EC-3 continuation (2026-09-17)

**DES-6 — RESOLVED.** Governor decision
`20260912T094217Z-governor-reviewer-3rha` (2026-09-12, republished to
`main` 2026-09-17 after it was found orphaned on a pre-history-rewrite
branch — see MR !2900) settled the disposition question this escalation
raised: rewrite `agf/docs/governance/governance.md` as a pointer-first
description of the current governed role/ownership/coordination/gate
model, sourced from `agf/agents/MANIFEST.yaml` and current rules/
decisions, rather than archiving it. That rewrite is implemented on this
MR (!2717), alongside the `agf/rules/**` cross-estate-consistency
read-through flagged as a still-UNEXAMINED gap above. A residual defect
in the first rewrite pass — a duplicated `governance-file-layout-rule.md`
ownership mini-table that silently dropped the rule's `agf/agents/*.yaml`
"pending #79" qualifier and footnote (contribution-audit finding
DES6-R2) — was corrected the same day by removing the duplicated table
and pointing only to the canonical rule. The "adds a new open item"
characterization in the "Still UNEXAMINED after EC-2" paragraph above is
superseded by this resolution; it is left unedited per this report's
append-only convention rather than rewritten in place.

**Inventory correction.** The EC-1/EC-2 "47 Markdown files" figure for
`agf/docs/site/**` is stale against the branch's current merge target:
`git ls-tree -r --name-only agf/docs/site` on `main` now returns **48**,
the added file being `agf/docs/site/reference/artifact-registry.md`
(2026-09-12). That page has not been examined by any pass in this
report — it is folded into the existing "~28 unnamed... UNEXAMINED"
count above, which is accordingly **~29** as of this correction, not
~28. This MR remains an explicitly partial estate continuation; it does
not claim the added page was inspected, and v2.11 condition 4 is still
not met by this report.

## EC-4 continuation (2026-09-17)

**DES-7 — RESOLVED.** Per this document's "Findings" table above:
`agf/docs/contracts/integration-contract.md`'s L315/L385
shipped-vs-planned disagreement over `framework.yaml` is reconciled by
distinguishing the shipped, vacuously-passing validator *command* from
the still-planned [P1] `framework.yaml` precedence/binding-config
*capability* — both statements were individually accurate about
different things, and now say so explicitly instead of appearing to
contradict each other. The "Path-based revalidation triggers" table's
three path-glob rows (`agents/**`, `agents/MANIFEST.yaml`,
`learnings/**`, `framework.yaml`) were all missing the `agf/` prefix —
the same uniform staleness class as DES-1/DES-5, not limited to the
single `framework.yaml` row DES-7 originally flagged — corrected to
match `governance-file-layout-rule.md`'s ownership table and the Rust
implementation's actual `agf/framework.yaml`/`agf/agents/`/
`agf/learnings/` checks. This was mechanical currency/consistency
correction within the steward's existing mandate; unlike DES-6, no
governor disposition was required first, since no policy or
maturity-level question was actually in dispute — only imprecise
wording that read as contradictory.

**Estate coverage status, unchanged by this correction.** DES-7's
resolution does not touch the "Still UNEXAMINED after EC-2" gap (the
~29 unnamed `agf/docs/site/**` files) — that remains open, and v2.11
release-cut condition 4 is still not met by this report as a whole.
All findings raised to date (DES-1 through DES-7) are now dispositioned
(FIXED or RESOLVED); the coverage gap, not an undispositioned finding,
is the remaining obstacle to condition 4.

## EC-5 continuation (2026-09-17)

**DES-8 — FIXED.** Targeted sweep of previously-UNEXAMINED
`agf/docs/site/**` files for the DES-2/DES-3 failure class (loader/
role-roster staleness). Found: `agf/docs/reference/quick-reference.md`
(the canonical role catalog the estate's own `roles-and-gates.md` and
`applicability-catalog.md` cite) listed only 14 roles and was missing
`documentation-estate-steward` and `improvement-propeller` — both
have shipped `agf/agents/*.yaml` definitions (16 total, confirmed via
`ls agf/agents/*.yaml`). This propagated: `roles-and-gates.md` repeated
the stale "fourteen" count twice; `applicability-catalog.md`'s
classification banner used stale totals (51 rules/15 agents/66 total,
57 remaining) against the actual current counts (52 rules/16 agents/68
total). Fixed all three files: added both missing roles to
`quick-reference.md` in their correct Core/Extended sections per their
`tier`/`phase` fields; corrected "fourteen" to "sixteen" (twice) in
`roles-and-gates.md`; corrected the banner and "Tracked completion"
totals in `applicability-catalog.md` (9 classified stays accurate —
verified against the actual classified-list row count — only the
denominator and unclassified-role enumeration were stale).

**Estate coverage status.** This is a real, bounded advance on the
"Still UNEXAMINED after EC-2" gap, not a claim of completion: a
targeted sweep of a subset of the ~29 unnamed files, not the full
remaining set. The exact number of files still fully unexamined after
this pass has not been recounted. v2.11 release-cut condition 4 is
still not met by this report as a whole.

## EC-6 continuation (2026-09-17)

Accepted the `20260917T202401Z-planner-architect-kak5` continuation handoff and
recomputed its literal remaining inventory as **37 files** (the handoff and
draft continuation record said 36). Ran the documented broad scan across all
37 for dead rule/plan references, version strings, role-count claims, stale
model names, removed dual-layout framing, and documented `agov` commands. All
documented command families in this set resolved against the current Rust CLI.
Historical version citations and tutorial-only sample paths were retained.

Targeted reads then covered the dynamic surfaces most likely to drift:
evaluation/source indexes, the content-control artifact, tooling/validator and
environment-variable references, coordination and polling pages, install and
commissioning guidance, role/product positioning, and the figure inventory.
Four defects were found and corrected as DES-9 through DES-12 above. The
remaining static explanatory, licensing, philosophical, disclaimer, palette,
and acknowledgement pages passed the same pointer/currency scan with no new
defect found. The commercial-corpus page's 142 included / 50 excluded counts
were independently recounted from `LICENSES/commercial-corpus-manifest-v1.yaml`.

This closes the 37-file inventory named by the continuation handoff. It does
not by itself declare v2.11 condition 4 met: the final estate-coverage
reconciliation and binding completion disposition remain with the governor as
the handoff requires. DES-9 through DES-12 are dispositioned as mechanical
currency/consistency corrections; no policy, role authority, schema, gate, or
release decision changed.

## EC-7 — final estate-coverage reconciliation (2026-09-19)

This is the reconciliation EC-6 said remained: a direct diff between the
current live Pages-site tree and every file individually named as checked
across EC-1 through EC-6, to state explicitly whether the file-coverage
gap that has kept this report's `Status:` at `INCOMPLETE` since EC-1 is
actually closed.

**Method:**

1. Enumerated the current tree: `git ls-tree -r --name-only origin/main
   agf/docs/site | grep '\.md$'` → **48 files** (one more than the 47
   counted at EC-1; +1 file added to the Pages site since).
2. Built the "checked" set from three sources: (a) the exact 37-file
   remaining-inventory list in
   `docs/reviews/documentation-estate/20260917T202324Z-sweep-continuation-plan.md`,
   which EC-6 states it covered in full; (b) every `agf/docs/site/*.md`
   path referenced by full path anywhere in this report's EC-1 through
   EC-6 text (11 files: `content-plan.md`, `docs/adopt/
   tooling-and-tokens.md`, `evaluate.md`, `index.md`,
   `reference/artifact-registry.md`, `reference/environment-
   variables.md`, `reference/figures.md`, `reference/maturity-
   vocabulary.md`, `reference/release-artifacts.md`,
   `reference/tooling-conformance.md`, `roadmap.md`); (c) three files
   fixed in this session but outside the 37-file continuation batch —
   `docs/concepts/roles-and-gates.md`, `docs/adopt/mcp-server.md`
   (EC-5-adjacent, MR !2939), `docs/adopt/applicability-catalog.md`
   (EC-5) — plus `docs/index.md`, which the continuation plan itself
   states was "already examined tonight, clean" outside its own
   37-file remaining list. Deduplicated: **48 files.**
3. Diffed (2) against (1) in both directions.

**Result:** zero residual either direction. Every file in the current
48-file Pages-site tree is named as checked by at least one of EC-1
through EC-6; no checked file has been removed or renamed out from
under the record.

**What this does and does not establish.** It closes the specific,
concrete gap EC-1 first identified (an inaccurate file count with most
files never individually named) and that every later EC section said
remained open. It does **not** independently re-verify that each of the
48 files' six-dimension review was uniformly thorough — EC-6 itself
already discloses that its 37-file pass split into a full broad-scan
(all 37) plus targeted deep reads on a higher-drift-risk subset, with
the remainder receiving "the same pointer/currency scan" rather than a
individually-narrated dimension-by-dimension pass. That depth
distinction is accurately represented in this report already and this
reconciliation does not paper over it.

**Recommendation to the governor:** the file-coverage precondition for
v2.11 release-cut condition 4 (`docs/plans/20260909T021841Z-v211-release-scope.md`'s
"every surface... inspected... or explicitly excluded with a reason")
is now satisfiable — every surface has a named check. Whether that
satisfies condition 4 as written, or whether the governor wants a
deeper re-pass on any specific file before declaring the sweep complete,
is the binding decision this report has deferred to the governor since
EC-1. This reconciliation supplies the missing evidence for that
decision; it does not make the decision.
