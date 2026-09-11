---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260910T160124Z-planner-architect-mb8k
timestamp: 2026-09-10T16:01:24Z
model: claude-sonnet-5
immutable: true
topic: v211-coordination-lead-handoff-claude-to-codex
references-to:
  - 20260910T133145Z-governor-reviewer-q862.md
  - 20260910T153904Z-planner-architect-pg53.md
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Governor direction 2026-09-10: the Claude planner-architect session is token-constrained; Codex takes over coordination of the v2.11 / RUW release drive. Acknowledge and resume."
    - "Pick up the open planner queue and the in-flight MRs below; merge order and blockers are stated per item."
    - "Governor decisions still owed are listed under 'Owed by governor' - route/remind, do not self-authorize."
  must_read:
    - docs/plans/20260909T021841Z-v211-release-scope.md
    - docs/plans/20260909T203636Z-v211-release-publication.md
    - docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md
    - docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md
    - docs/coordination/20260910T153904Z-planner-architect-pg53.md
    - docs/coordination/20260910T144148Z-documentation-estate-steward-pjsp.md
  do_not_touch:
    - binding governor decisions; Gate 1 approvals; Gate 2 merges of anything not already governor-approved
    - the 6sj1 semantic-data exclusion (MR !2460)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11 coordination lead — handoff Claude → Codex

Governor direction 2026-09-10: Claude planner-architect is near its token
limit; Codex takes over the v2.11 / RUW release-drive coordination.

## v2.11 release-cut precondition status (from the release-publication plan + amendments)

| Precondition | State |
|---|---|
| Cond. 3 — open-MR queue disposition hygiene (`k8bh`) | **MET** — QC-1 ledger merged (`4cc016ad`); all 59 eligible open MRs carry a §3b disposition |
| Cond. 4 — documentation-estate assessment (`hsqs`/`q862`) | **MET** — first estate sweep complete (`pjsp`); DES-1/2/3 fixed, DES-4 routed to build-engineer (`sj42`) |
| CP-B (row 7) | ❌ CP-B-1 impl MR !2636 not merged (needs copied-record clarification folded in); CP-B-2 not started |
| Coordination lifecycle semantics (row 5) | work order merged; slice details: **S1 + S4 drafted (MR !2676)**, S2 (#122, blocks on qbka Slice A) + S3 (!2027, blocks on CP-B-2) NOT drafted |
| Coordination discovery/collision (row 6) | work order merged; **S1 drafted (MR !2675)**; S2 (CP-C) + S3 (#123) NOT drafted |
| `qbka` Slice A / B (row 4) | ❌ NOT started — the spine (row-5 S2 depends on it). Needs a player (see below) |
| Propeller Stage 4 genuine wake (#131) | ⏳ calendar-gated ≈ 2026-09-14; then a separate Level-5 record |
| Pre-cut governor dispositions | ❌ 2 owed (see below) |
| Runner / DinD capacity | ❌ degraded; Codex CI performance trial in flight (`g3j5` / MR !2673 / `codex/ci-performance-trial`) |

Governor 2026-09-10: **"Keep all in v2.11"** — no row moves to v2.12.

## In-flight MRs authored by the Claude session (take these over)

**Gate-1 plan MRs awaiting governor Gate 1:**
- `!2675` — Row 6 S1 (channel path contract; handoff `pps7`). Independent.
- `!2676` — Row 5 S1 (#121) + S4 (CP-A) slice details (handoff `jhmp`). Both independent. **Note: the schema already has an optional `session` field ("audit only"); S1 promotes it, does not add a new one.**
- `!2666` — AGENTS.md gate-discipline addition (bridge-record pattern; own Gate 1 plan `20260910T133436Z`).

**Coordination / docs MRs awaiting governor merge:**
- `!2674` — QC-1 review trail (7 records split off the ledger MR; coordination-only, publication-check passes).
- `!2667` — documentation-estate first sweep COMPLETE (DES-1..4). Carries the DES-2 fix (AGENTS.md role table) + DES-3 fix (AGENTS.md was stale; role reactivated by `3eqb`).
- `!2664` — R-3: estate-assessment precondition folded into the release-publication plan.
- `!2660` — batched planner acks + QC-4 batch stage (`gj1e`) + OTGR routing reconcile (`s42g`) + ap4d supersession + `pg53` routing note.
- `!2592` — OTGR spec adopter-facing reframe (per `pkbt` §2); Part D use-case generalisation still owed; reviewers per `epmt`.
- `!2593` / `!2596` — OTGR routing records (merge to publish history; superseded for claims by `s42g`).

**Older Claude-authored MRs, now QC-1-dispositioned (execute per QC-1 rows at QC-4, not before):** !2368/!2394/!2455 (learnings, merge-now), !2409/!2437/!2452 (coordination, merge-now), !2375 (close-superseded), !2390/!2408 (park-radar sources), !2460 (**awaiting-governor, 6sj1 carve-out**), !2587 (ship-in OTGR), !2651 (docs-estate learning + steward directive; carries `jh3g` — merge to put `jh3g` on main).

## Planner queue (draft next, as Codex/planner)

1. **Row 6 S2** (CP-C: `agov handoff next --include-branch-local` opt-in scan) — independent.
2. **Row 6 S3** (#123: optional `affected_paths` on `gate1-plan.schema.json` + advisory `agov plan collision-check`) — independent.
3. **Row 5 S2** (#122 supersession field pair) — draftable now; **implementation blocks on `qbka` Slice A** landing the `superseded` provenance shape.
4. **Row 5 S3** (!2027 priority-ordering over CP-B-2's `urgency` + challenge-response obligation) — blocks on CP-B-2. **`urgency` is owned by CP-B-2**, S3 consumes (`p2rk`).
5. **CP-B-1 impl (MR !2636)** — fold in `20260909T200622Z-cpb1-copied-record-contract-clarification` (byte-identity detector + 4 gate-self-tests), re-run exact-head reviews.
6. **CP-B-2** — coder-tester deliverable per the CP-B work order: `agov` publication-hold status surface (D-2a) + `urgency` enum on the coordination schema (D-3a) + CP-B negative tests.

## Owed by governor (route/remind; do not self-authorize)

- **`qbka` Slice A player** (`pg53` #1) — Claude proposed reassigning to Codex planner + Codex impl (`pe1k` inactive). Needs governor confirm.
- Gate 1 on `!2675`, `!2676`, `!2666`.
- Gate 2 merges of the coordination/docs MRs above.
- **2 pre-cut dispositions** (`pg53` #3): `release-preflight-learning-gate2` (confirm `rg6k` sequence / MR !2434), `capability-developer-startup-sync` (confirm MR !2421 complete).
- **DES-3/DES-4** follow-ups: DES-3 fixed in `!2667`; DES-4 build-engineer check (`sj42`) then the steward applies the `tooling-conformance.md:52` cell fix.
- **!973 / !965** — governor chose `park-radar` (`tdpj`); **MR !2670 must merge before either source MR closes** (Codex owns MR !2670; reframe from "reconstruct proposal" to "park-radar retention carrier").
- **R-5** — capability-developer contract review of `documentation-estate-steward.yaml` (`et3p`).

## Bridge records this session needing governor on-channel confirmation (POLL-G1 pattern)

Ratified by `q862` (on `main`): `pkbt`, `771h`, `k8bh`, `jh3g`, `g1g5`,
`3ts1`, `2skr`, `hsqs`, `tsbh`. Later bridges still requesting ack:
`es3m` (QC-1 Gate 2), `sj42`, `pjsp`, `pps7`, `jhmp`, `mb8k` (this record).

## Acknowledge

Codex: ack this handoff, resume as the v2.11 coordination lead. The
Claude planner-architect session is standing down.
