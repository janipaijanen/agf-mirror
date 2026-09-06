<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Release scope inventory — the release that lands after v2.9.0

<!--
Reference material, not a plan. Lives under docs/reference/ (not docs/plans/)
because it is an advisory contribution-auditor inventory, not a schema-valid
Gate 1 plan. id: post-v290-release-scope-inventory | status: draft |
authored_by: contribution-auditor | authored_at: 2026-09-06T12:52:34Z |
last_recut: 2026-09-06T18:10Z against the v2.9.0 tag | gate: none.
-->

**Status: DRAFT / advisory.** Authored by contribution-auditor at the
operator's request. **Not a Gate 1 plan; grants no approval.** Every
disposition is a candidate. A planner-architect converts it into the Gate 1
release plan (handoff `mjqa` / MR !2442). Re-cut 2026-09-06T18:10Z against
the now-existing `v2.9.0` tag.

## Caveats

- **Scope boundary is fixed:** `v2.9.0` = tag `ee27978b`, 2026-09-06
  13:13:29Z. The next release's content is `v2.9.0..main` plus whatever
  merges before its own cut.
- **The delta is currently tiny** (see Part A). It grows as the queued
  rule / schema / Propeller / polling-model work merges.
- **Version is downstream and currently PATCH-shaped.** `v2.9.0..main`
  today is one fix (ROI baseline repair) → `2.9.1`. It becomes MINOR
  (`2.10.0`) only when a queued item that adds a rule, schema, MANIFEST
  role, or CLI surface actually merges. The earlier "MINOR → 2.10.0" claim
  was premature — it was measuring `v2.8.0..main`, which is v2.9.0's own
  content.
- **The post-v2.9.0 window adds items.** Propeller's first friction-mining
  pass and capability-developer's one-shot review will surface candidates.
  Part D items are parked for that window and must not be pulled forward.
- **Operating model:** governor decision `mskd` (2026-09-06, merged
  `34b9c650`) adopts the layered autonomous poll/action model for all AGF
  development AIs and directs planner-architect to revise MR !2405 as its
  canonical normative owner with the six-row projection matrix. That !2405
  revision is now a named release-relevant workstream.

---

## Part A — `v2.9.0..main` (auto-included; needs changelog authoring)

The non-coordination delta since the `v2.9.0` tag is **one change**:

| Change | Merges | Files | Nature | Recorded |
|---|---|---|---|---|
| **ROI workflow baseline repair** (MR !2383) | `be62df46` `fix/roi-workflow-baselines`; Gate 2 chain `9be5a113`/`558d6b0d` (`gmm7`/`j53q`) | `tooling/rust/crates/agov-core/src/roi.rs` (+156), `CHANGELOG.md` (+7), `agf/CHANGELOG.md` (+11) | `fix` — restores automatic token-saving estimates for `cold-start` / `session-resume` / `next-handoff`; parity preserved | `## [Unreleased]` in both changelogs |

Plus one coordination-only merge with a release-planning consequence:

| Merge | Effect |
|---|---|
| `34b9c650` `coordination/layered-poll-guidance` (`mskd`) | Not release content itself. Establishes the operating objective and a **planning obligation on MR !2405** (planner-architect revises it as the polling model's normative owner). Any rule/loader/AGENTS.md diff from that work is future release content once `!2405` clears Gate 1. |

**Everything else previously listed here shipped IN v2.9.0**, not after it:
the `gate-registry.schema.json` addition, `capability-developer` activation,
`improvement-propeller` Gate 1, `agf-development-starter`, native-musl plan,
`ci-mandatory-needs-implication` fix, and the `gate-self-test-rule.md`
broadening (commit `7ca76d71`, merged 2026-08-30).

### Resolved: the `gate-self-test-rule.md` flag was a false alarm

Verified this session. `7ca76d71` shipped in v2.9.0 under Gate-1-approved
plan `gate-change-discipline` (decision `20260830T085834Z-governor-reviewer-dq5a`).
The gate mechanic it carried (`.gitlab/scripts/changed-files.sh`) shipped
with a proper negative test: `tests/changed-files-failclosed.sh`, 17
assertions, CI job `governance:changed-files:selftest`, demonstrating both
new-code-fails-on-bad-input and old-code-passed-on-same-input. The rule-prose
broadening itself introduces no mechanism, so needs no self-test. Not a
release concern.

**Action for the Gate 1 plan:** author the `v2.9.0..<next tag>` changelog
section from Part A; it currently contains one `Fixed` entry.

---

## Part B — Open MRs, dispositioned with cause and priority

Priority uses the `mskd` Layer 2 order (screened-security → in-role
obligation → advance-ready → next-release readiness → quality unit →
approved features). "Ready" = required pipeline green, findings resolved,
exact head confirmed.

### B1 — Coordination-only: drain before the cut, not content (~20)

!2352, !2353, !2361, !2363, !2366, !2370, !2376, !2377, !2378, !2382,
!2399, !2400, !2403, !2407, !2409, !2410, !2411, !2417, !2437, !2442,
!2443, !2445

**Cause:** each publishes an audit-trail record or routes a review; no
release diff. **Priority within B1:** !2400/!2409 (canonical run-list) and
!2399 merge **first** — the run-list is branch-local (`bridge: missing`),
and several later dispositions cite it. Then the rest drain in any order.

### B2 — Substantive, near-ready content (priority order)

| Pri | MR | Readiness | Cause / dependency |
|---|---|---|---|
| 1 | !2405 universal polling plan | **not ready** — `p6j5` CHANGES REQUESTED (C1 simplicity-rule collision) | `mskd` names it the polling-model normative owner; planner-architect MUST revise (not replace). Blocks the polling-model rule/loader diffs. |
| 2 | !2362 build: route CI jobs to dedicated runner | **not ready** — gate-blocked per !2366 | needs the accompanying gate; pairs with the merged v290 atomic routing |
| 3 | !2392 build: remove DinD from mandatory Linux packaging | near-ready | unblocks native-musl packaging; **blocked-by** #109 runner-migration direction |
| 4 | !2434 learning: release-metadata preflight | at Gate 2 (`rgkp`) | audited clean this session |
| 5 | !2368 / !2394 / !2412 learnings | near-ready | all audited clean; !2394 feeds #121-#123 |
| 6 | !2390 / !2408 / !2306 docs sources+radar | near-ready | audited clean; research-maintenance fast-path style; !2410 wants Batch 34 release-first |
| 7 | !2375 docs: OTGR Zulip-adapter spec v0.1 | proposal | routed to governor (!2376); not a commitment |
| — | ~~!2383 ROI baseline repair~~ | **MERGED** `be62df46` | now Part A |

### B3 — Gate-1-pending plans: not content unless approved + implemented + merged

!1817, !2027, !2038, !2275, !2416 (draft), !918, !923, !745, !654, !658,
!998, !459, !1451, !1617, !1659

**Cause:** plan-only; each needs Gate 1 → a separate implementation cycle
before any diff reaches `main`. Sub-notes:
- **!2416 Propeller** — Gate 1 *effective* at `4a267765` (`ksbs` D3). Needs
  mechanical closure (pipeline, un-draft, publish binding Gate 1 handoff —
  `6pss` item 1). Implementation (#126/#127) **blocked-by** the
  capability-developer `capability-review` pass (`skr6` / MR !2443).
- **!2038** stale-worktree rule — `eb87` CHANGES REQUESTED (2 blocking). Not
  ready.
- **!1817, !2027** — see Part D.
- !654, !658, !745, !998, !459, !1451, !1617, !1659 — July/Aug plans;
  **verify not superseded** before carry-forward (own pass needed).

### B4 — Stale / superseded: close, do not include

| MR | Confidence | Cause |
|---|---|---|
| !1492 plan: publish v2.6.0 release | **close** (high) | project is at v2.9.0; the v2.6.0 line is historical |
| !1657 plan: v2.6.1 release-only correction | **close** (high) | same |
| !1444 mark diligence plan ABANDONED | **close/merge the note** (high) | housekeeping; abandonment already effective |
| !1723, !1750 commercial-licensing | reconcile | CL-4 shipped separately — see digest `52nt` / MR !2437 |
| (11 July MRs — verify-each pass done 2026-09-06T18:5x, see sub-table below) | mixed | 7 close-recommend, 3 need-decision, 1 close-or-refile |
| !1446, !1691, !1718, !1722 | historical | v2.5.0-era external due-diligence artifacts; not next-release content |
| !915 | close | frozen-release two-AI evaluation, tied to an abandoned line |

#### B4 verify-each pass — 11 stale July learning/rule MRs (2026-09-06)

Method: `glab api .../changes` for each MR's changed paths + grep of current
`agf/learnings/foundry.yaml` (62 records), `downstream-impact-rule.md`,
`coordination-publication-rule.md`, and `AGENTS.md` for the theme. Not
implemented — dispositions are recommendations for the MR owner.

| MR | Changed | Finding | Recommend |
|---|---|---|---|
| !341 roadmap/tracker coexistence | new Pages page + position/faq edits | target page absent on main; title says "per Gate 2 BLOCK" (already bounced once); relates to issue #118 | **need-decision** — revive under #118 or close with cause |
| !349 coordination-before-bridge-notes | learning → **bootstrap** `agents-govern/learnings/` | bridge-note discipline is live (`coordination-publication-rule`, visibility rule); record targets the non-canonical dir | **close or re-file** against `agf/learnings/foundry.yaml` if the record is still wanted |
| !407 CI-cost-aware push cadence | `agf/learnings/foundry.yaml` | no matching theme on main; adjacent to issue #106 + `ci-trigger-scoping-rule` | **need-decision** — fold into #106 or land as a learning; not auto-close |
| !430 tooling doc-impact assessment | `downstream-impact-rule.md` + new plan | rule goal met — `downstream-impact-rule.md` already covers `tooling/` on main; plan file stale | **close**; salvage learning portion only if novel |
| !461 assistive-tech boundary (DRAFT) | `agf/learnings/foundry.yaml` | draft, 6+ wk stale, no theme on main | **close** as abandoned draft unless author revives |
| !465 autonomous-polling cross-branch gap | `agf/learnings/foundry.yaml` | cross-branch visibility scanning is now core to the poll contract (AGENTS.md, `mskd`) | **close**; salvage learning if novel |
| !739 commit-message CI-skip-substring FP | `coordination-publication-rule.md` + learning | rule heavily addresses `[ci skip]`/`[skip ci]` (incl. forbidden-in-coordination-commits); verify the exact false-positive nuance is caught | **close** (likely) after nuance check |
| !776 release agent verify canonical identity | `agf/learnings/foundry.yaml` | no matching theme; `external-anonymization-rule` adjacent but different | **need-decision** — land as a learning or close with cause |
| !829 atomic MR ≠ pipeline dedup | `agf/learnings/foundry.yaml` | derived operational guidance IS on main (AGENTS.md first-push paragraph); record itself not in foundry.yaml | **close** — guidance already shipped |
| !850 release-transition duplicate matrix | `agf/learnings/foundry.yaml` | no matching theme; release-transition dup handling had heavy v2.7/v2.8 work | **close** (likely) after a spot check vs the release rules |
| !932 coordination-title-per-commit mistake | `agf/learnings/foundry.yaml` | `coordination-publication-rule` addresses `coordination:` title semantics extensively | **close**; salvage if novel |

Summary: **close-recommend** !430, !461, !465, !739, !829, !850, !932 (7);
**need-decision** !341, !407, !776 (3); **close-or-refile** !349 (1).

### B5 — Release chain

!2435 (Release v2.9.0) — merged predecessor, not next-release content.

---

## Part C — Issues selected for the next release, with cause and priority

~50 of 61 carry `release::unmilestoned-backlog` (already triaged as not
release-bound). Selecting **in**:

| Pri | # | Cause |
|---|---|---|
| 1 | **#120** pre-tag namespace-absence obligation | `bug`; release-integrity gap reinforced twice (v2.7.0-rc.1 burn; `ev5m` learning). Cheap, self-contained. Should land before the *following* release runs the same workflow. |
| 2 | **#124–#127** improvement-propeller | Gate 1 approved. Sequence: planner-architect revises !2405 (polling-model owner, per `mskd`) → capability-developer `capability-review` pass → Propeller role/rule implementation (#126/#127). Lands in the next release only if that chain completes in-window. |
| 3 | **#116** token ROI CLI vs agov-mcp | enabler (!2383 baseline repair) **now merged** (Part A); the measurement is a small follow-on. |
| 4 | **#67** v2.4.4 commissioning doc defects | `bug,documentation`; **verify still reproducible** — close if fixed, else cheap doc fix. |
| 5 | **#111 / #113** upgrade deletes adopter governance / ignore rules exclude `agf/` | `learning`; real provenance risk. Candidate — needs a plan for the detecting gate. |
| 6 | **#101 / #102 / #106** CI cache / Elixir exclusion / cancelled-pipeline | build-engineer lane; batch only if build-engineer is active in-window. |
| — | **#119** static context cost (~91k tokens/session) | high impact but **needs its own Gate 1**; include only if a scoped slice is planned in-window. |

Everything else (radar #59–#62/#83/#98/#99/#104; compliance/CRA #40–#49;
#17–#19/#31–#39; #75/#77/#82/#91/#94/#96/#103/#105/#107–#109/#117/#118) —
**deferred.** Cause: not release-bound, each needs its own Gate 1, or parked.

---

## Part D — Do NOT pull forward (with cause)

| Item | Cause |
|---|---|
| **#121 / #122 / #123** coordination-protocol redesign | Run-list item 2; prior-art study (Radia claim-by-pattern, Batch 34 radar) is required *before* its Gate 1. Pre-planning locks a design the study should inform. |
| **#128** cadence register | `33hj`: parked for the post-v2.9.0 window as a Propeller lane-2 target; pre-authoring locks the design. |
| **!2027** coordination priority field | Partial answer to #121-#123; defer with the redesign, don't ship a stopgap it must unwind. |
| **!1817** unassigned-work-discovery-rule | Overlaps the approved Propeller remit; needs a governor scope-reconciliation call. |

---

## Dependency map (blocks → blocked)

```
v2.9.0 tag (done) ──► scope boundary fixed; Part A = v2.9.0..main

!2400/!2409 run-list merge ──► unblocks citing many B1 dispositions

mskd (merged) ──► planner-architect revises !2405 (resolve p6j5 C1)
  !2405 Gate 1 clear ──► polling-model rule + loader/AGENTS.md diffs
                          (six-row projection matrix required)
  !2405 canonical rule ──► must merge before/atomically with Propeller
                            impl (ksbs D4)

!2416 pipeline @4a267765 + un-draft + Gate 1 handoff ──► Propeller Gate 1 closed
capability-developer assignment (skr6 → governor) ──► capability-review pass
  capability-review + governor disposition ──► Propeller impl (#126/#127)

#109 runner-migration direction ──► !2392 (remove DinD) ──► native-musl packaging
!2366 accompanying gate ──► !2362 (route CI jobs to dedicated runner)
```

---

## Next steps

1. **Done:** v2.9.0 tagged; Part A re-cut against it (one `Fixed` entry).
2. planner-architect converts this into the Gate 1 release plan (`mjqa`),
   with build-engineer + capability-developer consultations; the polling
   model piece is the `mskd`-directed !2405 revision.
3. governor sets the version number from the final Part A content (PATCH
   until a MINOR-class item merges).
4. contribution-auditor: run the dedicated B4 verify-each pass on the 11
   stale July learning/rule MRs; maintain the Part B disposition column as
   MRs merge/close.
