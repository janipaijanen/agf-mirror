<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Release scope inventory — the release that lands after v2.9.0

<!--
Reference material, not a plan. Lives under docs/reference/ (not docs/plans/)
because it is an advisory contribution-auditor inventory, not a schema-valid
Gate 1 plan. id: post-v290-release-scope-inventory | status: draft |
authored_by: contribution-auditor | authored_at: 2026-09-06T12:52:34Z |
gate: none.
-->


**Status: DRAFT / advisory.** Authored by contribution-auditor at the
operator's request, 2026-09-06, during idle poll capacity. **This is not a
Gate 1 plan and grants no approval.** A planner-architect converts it into
the actual Gate 1 release plan once the post-v2.9.0 window opens — i.e. once
`v2.9.0` is tagged and its scope boundary is fixed. Until then every
disposition below is a candidate, not a decision.

## Accepted caveats

- **Scope cannot be frozen until v2.9.0 is tagged.** The "since" boundary is
  that tag. Everything here is measured from `v2.8.0..main` and will be
  re-cut against the v2.9.0 tag when it exists.
- **The post-v2.9.0 window will add items.** Propeller's first
  friction-mining pass and capability-developer's one-shot review are
  expected to surface new candidates. Several current backlog items are
  explicitly parked for that window and must **not** be pulled forward
  (Part D).
- **Version number is downstream.** 2.9.1 vs 2.10.0 is the governor's / the
  `release` skill's call. Evidence points to **2.10.0** (MINOR): `v2.8.0..main`
  already carries a new schema (`agf/schemas/gate-registry.schema.json`), a
  MANIFEST role addition (capability-developer, improvement-propeller), and a
  gate-mechanic rule change (`gate-self-test-rule.md`) — none PATCH-class.

---

## Part A — Already on `main` since v2.8.0 (auto-included; needs changelog authoring)

215 merges `v2.8.0..main`; ~150 are `coordination:` audit-trail commits with
no release-content diff. Substantive clusters:

| Cluster | Representative merges | Nature |
|---|---|---|
| **CI / release integrity** | `7f66bb0a` ci-mandatory-needs-implication fix; `02be60d5`/`77832439` v2.9.0 vulnerable-proof + autocancel; `8d71eca5` native-musl-release-build plan; `3572854f` v290 atomic runner routing; `5aaacff1`/`04432e95` ROI workflow baselines; pre-tag OCI CVE scan (`agf/CHANGELOG.md` Unreleased) | fix + build; low adopter impact |
| **Governance capability** | `2f78c907` capability-developer activation; `b034dbbb` improvement-propeller Gate 1 (plan approved — *not implemented*); `9ed176de`/`0344fe99` agf-development-starter guideline | MINOR (MANIFEST role additions) |
| **Rules / schema** | `agf/schemas/gate-registry.schema.json` (new, +60); `gate-self-test-rule.md` (+25 — gate-mechanic; confirm it shipped with its own negative test); `simplicity-rule.md` (+4) | MINOR + one gate-mechanic change to confirm |
| **Docs / sources / radar** | OTGR Zulip-adapter spec + charter; queue-convergence & plan-status work; deliberation guide | docs |
| **Learnings** | release-preflight version reservation (`cab3c0d6`), + others | learnings |

**Action for the real plan:** author the `v2.8.0..<v2.9.0 tag>` changelog;
this section then becomes "everything after the v2.9.0 tag."

---

## Part B — Open MRs (77 at authoring time), dispositioned with cause

### B1 — Coordination-only: drain before the cut, not "content" (~20)

!2352, !2353, !2361, !2363, !2366, !2370, !2376, !2377, !2378, !2382,
!2399, !2400, !2403, !2407, !2409, !2410, !2411, !2417, !2437

**Cause:** each publishes an audit-trail record or routes a review; no
release diff. Merge (or close if superseded) so `main` is clean at the
v2.9.0 cut, but they are not content line-items. !2400/!2409 (run-list
consolidation) and !2399 should merge **first** — the canonical run-list is
currently branch-local (`bridge: missing` in `coord visibility`).

### B2 — Substantive, near-ready: include if Gate 2 clears in the window

| MR | Cause to include | Watch-out |
|---|---|---|
| !2383 fix: restore ROI workflow baselines | Gate 1 approved; small fix | — |
| !2392 build: remove DinD from mandatory Linux packaging | native-musl release-build; unblocks packaging | ties to #109 runner migration |
| !2362 build: route every CI job to dedicated runner | pairs with v290 atomic routing | **gate-blocked** per !2366 — needs the accompanying gate |
| !2368 learning: controlled test before fix | audited clean this session | low-risk |
| !2394 learnings: coordination work-order gaps | audited clean; feeds #121-#123 | — |
| !2412 learning: non-reentrant polling | audited clean | — |
| !2390 / !2408 docs: sources Batch 33 / 34 | audited clean; research-maintenance fast-path style | !2410 wants Batch 34 redirected release-first |
| !2306 docs: classify AgentConnect radar | audited clean | — |
| !2375 docs: OTGR Zulip-adapter spec v0.1 | consolidation review routed to governor (!2376) | proposal, not a commitment |
| !2434 learning: release-metadata preflight | audited clean; at Gate 2 now (`rgkp`) | — |

### B3 — Gate-1-pending plans: NOT release content unless approved + implemented + merged in-window

!1817, !2027, !2038, !2275, !2405 (draft), !2416 (draft), !918, !923,
!745, !654, !658, !998, !459, !1451, !1617, !1659

**Cause:** plan-only. Each needs Gate 1 approval, then a separate
implementation cycle, before any diff reaches `main`. Run-list items, not
release line-items. Sub-notes:
- !2416 (Propeller) / !2405 (universal polling) — Gate 1 effective;
  *implementation* is the post-v2.9.0 window (#124-#127). Land in the next
  release only if built in the window.
- !2038 stale-worktree-pruning — Gate 1 review `eb87` = CHANGES REQUESTED
  (2 blocking). Not ready.
- !1817, !2027 — see Part D.
- !654, !658, !745, !998, !459 — July/early-Aug plans; **verify not
  superseded** before carrying forward.

### B4 — Stale / superseded: close, do not include

| MR | Cause |
|---|---|
| !1723, !1750 | commercial-licensing; CL-4 shipped separately, umbrella needs reconciliation — see digest `52nt` / MR !2437 |
| !1492 | plan: publish v2.6.0 release — obsolete (project at v2.9.0 construction) |
| !1657 | plan: v2.6.1 release-only correction — obsolete |
| !1444 | plan: mark diligence plan ABANDONED — merge the abandonment note or close |
| !341, !349, !407, !430, !461, !465, !739, !776, !829, !850, !932 | July learnings/rule proposals, 6-8 weeks stale; **verify each** vs merged work — most likely superseded |
| !1446, !1691, !1718, !1722 | v2.5.0-era external due-diligence artifacts; historical, not next-release content |
| !915 | frozen-release two-AI evaluation proposal; tied to an abandoned line |

### B5 — The release chain itself

!2435 (Release v2.9.0) — the *predecessor*, not next-release content.
!2437 — the stale-plan digest filed 2026-09-06.

---

## Part C — Issues selected for the next release, with cause

61 open at authoring time; ~50 carry `release::unmilestoned-backlog`
(already triaged as *not* release-bound). Selecting **in**, with cause:

| # | Title | Cause to include in the next release |
|---|---|---|
| **#120** | Release workflow has no pre-tag namespace-absence obligation | `bug`; release-integrity gap reinforced twice (v2.7.0-rc.1 burn; `ev5m` preflight learning). Cheap, self-contained; should land before the *following* release relies on the same workflow. |
| **#119** | Static context cost: rule corpus ~91k tokens/session | `governance`; largest adopter-facing cost item. **Candidate but needs its own Gate 1** — include only if a scoped slice is planned in-window. |
| **#116** | Measure AI token ROI: direct CLI vs agov-mcp | ROI baseline work (!2383) is the enabler and is in-flight; the measurement is a small follow-on. |
| **#124–#127** | Improvement-propeller role | Gate 1 approved; implementation *is* the post-v2.9.0 window. Lands in the next release if built there. |
| **#67** | v2.4.4 commissioning: 5 doc defects | `bug,documentation`; **verify still reproducible** on current docs — close if fixed, else cheap doc fix. |
| **#111 / #113** | Upgrade can silently delete adopter governance / ignore rules exclude `agf/` | `learning`; real provenance risk, no release binding yet. Candidate — needs a plan for the detecting gate. |
| **#101 / #102 / #106** | CI cache writers / Elixir exclusion / cancelled-pipeline frequency | build-engineer lane; include a batch only if build-engineer is active in-window. |

Everything else (radar items #59–#62, #83, #98, #99, #104; compliance/CRA
#40–#49; #17–#19, #31–#39; #75, #77, #82, #91, #94, #96, #103, #105,
#107–#109, #117, #118) — **stays deferred.** Cause: not release-bound, each
needs its own Gate 1, or explicitly parked.

---

## Part D — Do NOT pull forward (with cause)

| Item | Cause |
|---|---|
| **#121 / #122 / #123** coordination-protocol redesign | Run-list item 2; prior-art study (Radia claim-by-pattern, Batch 34 radar) is required *before* its Gate 1. Pre-planning locks a design the study should inform. |
| **#128** cadence register | `33hj`: parked for the post-v2.9.0 window as a Propeller lane-2 friction-mining target; pre-authoring locks the design. |
| **!2027** coordination priority field | Partial answer to #121-#123; defer with the redesign rather than ship a stopgap it then has to unwind. |
| **!1817** unassigned-work-discovery-rule | Overlaps the now-approved Propeller remit; governor scope-reconciliation call first. |

---

## Next steps

1. v2.9.0 tags → re-cut Part A against the tag.
2. planner-architect (or a governor-authorized Codex session, with an
   explicit session-scoped coordination claim) converts this into the Gate 1
   release plan for the post-v2.9.0 window.
3. governor decides the version number from the final Part A/B content.
4. contribution-auditor maintains the Part B open-MR disposition column as
   MRs merge/close.
