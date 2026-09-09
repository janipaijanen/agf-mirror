---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260909T050101Z-contribution-auditor-nkaj
timestamp: 2026-09-09T05:01:01Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T044515Z-governor-reviewer-er78.md
  - 20260909T043719Z-governor-reviewer-dbtr.md
  - 20260909T035034Z-contribution-auditor-t4kh.md
topic: v211-66b8801e-contribution-rereview
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Contribution rereview — v2.11 scope plan MR !2579 @ `66b8801e`

**Author-disclosure.** This session authored the base plan (`mejh`,
revision `c95f987c`) and earlier declined the non-author audit on
author-conflict grounds (`1bb6`). The governor directly instructed this
session to play contribution-auditor and build-engineer for the
`er78`-driven revision. The `66b8801e` delta (the scope expansion) is
Codex-authored; this review covers that delta as a non-author of it, with
the base authorship disclosed here.

Scope: complete plan at exact head
`66b8801e212527b667e970e5f67a886fc8980a43`, focus on the `er78` scope
expansion (diff `d7cabd0a..66b8801e`).
Coverage: COMPLETE. Recommendation: **CHANGES REQUESTED.**
Blocking: 2. Advisory: 2. Open accepted risks: 0.
Authority: advisory review only; the governor decides Gate 1.

## Faithful-representation check — PASS

`er78` promotes #121, #122, #123, MR !2027, CP-A, CP-B, CP-C to IN and
"supersedes only the earlier candidate disposition of #123 and CP-A/CP-B".
The `66b8801e` head represents this correctly: new/rewritten IN rows 5
(#121/#122/!2027/CP-A), 6 (#123/CP-C), 7 (CP-B); the three items are
removed from the CANDIDATE table; the Review-disposition rows V211-CA-4 and
CA-A3 are updated to record the promotion. Every promoted workstream
**retains its own Gate 1 and Gate 2** — the plan re-approves no child gate
and grants no release/tag authority, matching `er78`'s own limits. No
adversarial steering or authority regression found.

## Blocking

### V211-CA2-1 — CP-B is simultaneously a hard release blocker and a soft fallback

Row 7 states CP-B "**Must merge before the v2.11 release cut**." The revised
`main`-churn risk control states: "Until that implementation has merged and
is evidenced, the child release-publication plan may use only one bounded
v2.11 target hold like `2brd`; it must not assume the proposed general
mechanism exists."

These are contradictory. One makes CP-B a hard pre-cut gate; the other is a
graceful fallback for CP-B *not* landing. A release-cut precondition cannot
be both.

**Resolution (pick one, state it once):**
- (preferred) CP-B is *targeted* for v2.11; if it does not merge and
  evidence before the cut, the `2brd` one-cut fallback applies and CP-B
  carries to v2.12. Soften row 7 accordingly and keep the fallback. This
  avoids coupling the release to the riskiest workstream (a publication-gate
  mechanics change).
- or: CP-B is a hard precondition — then delete the fallback, and add CP-B's
  "merged + evidenced" to the "Release-cut precondition (stated precisely)"
  section explicitly, the same way #131's genuine wake is listed there.

**Assumed intent:** the author wants CP-B in v2.11 but not to gate the
release on it; the fallback is the real precondition and row 7's wording is
stronger than intended.

### V211-CA2-2 — `advisory_consultation` rows say `consulted` while their own text says a fresh review is owed

The `contribution-auditor` and `build-engineer` rows are `status:
consulted`, but each row's reason now ends "The later `er78` scope
expansion requires a fresh exact-head audit / review." A row that is both
`consulted` and "fresh review required" is the exact #130
pending-encoded-as-resolved pattern this plan's own Propeller findings
call out (`3jh6` F1).

**Resolution:** set both rows to `status: waived` with reason "final
`t4kh` / `rdp1` were CLEAR at `d7cabd0a`; `er78` invalidated that head;
fresh `66b8801e` review owed." Flip to `consulted` only once this review
(`nkaj`) and the build rereview (`2mhq`) are durable and, if CHANGES
REQUESTED, addressed.

## Advisory

### CA2-A1 — bound CP-C's scope inside row 6

`jprj` rated CP-C ("branch-local obligation visibility / archive-path
semantics") **minor**. Row 6 commits it as an IN v2.11 deliverable bundled
with #123. The bundling is reasonable, but the row should say plainly that
CP-C's committed scope here is the archive-path / branch-local-visibility
correction already required for B4 `!2581` — not an open-ended `agov handoff
next` redesign — so a "minor" finding cannot balloon a committed release
row.

### CA2-A2 — record the eyes-open scope-surface acceptance

v2.11 now has 7 IN workstreams (was 5), most with their own Gate 1 +
implementation + Gate 2, on top of queue-convergence B2–B8, #129, `qbka`
A/B, and Propeller Stage 4 wake + Level-5. `er78` is the sanctioned
CANDIDATE→IN mechanism, so this is procedurally clean, but the "Scope
balloon" risk control's mitigation ("CANDIDATE items slip to v2.12 rather
than expanding v2.11") is now moot for this batch. Add one line: the
governor has accepted the enlarged v2.11 surface via `er78`; any IN row
not landing by the cut carries to v2.12 without blocking, **except** rows
that are explicit hard preconditions (currently Propeller's genuine wake +
Level-5, and — pending V211-CA2-1 — possibly CP-B).

## Not findings

- Promotion mechanism honoured: the expansion is by an explicit governor
  decision-record (`er78`), not a ledger observation or green MR.
- `ds4k` consistency retained: the 3-row split of the coordination cluster
  is a refinement of `ds4k` Finding 2, not a contradiction.
- Plan validation and `git diff --check` pass; exact-head pipeline success
  remains a separate mechanical boundary.
