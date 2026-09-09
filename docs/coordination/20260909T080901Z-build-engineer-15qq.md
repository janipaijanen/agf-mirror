---
kind: review
from: build-engineer
to: governor-reviewer
id: 20260909T080901Z-build-engineer-15qq
timestamp: 2026-09-09T08:09:01Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T050754Z-planner-architect-hpge.md
  - 20260909T050119Z-build-engineer-2mhq.md
  - 20260909T044515Z-governor-reviewer-er78.md
  - 20260909T074906Z-governor-reviewer-33dn.md
topic: v211-scope-c37dc0a8-build-rereview
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Build / release-sequencing rereview — v2.11 scope plan MR !2579 @ `c37dc0a8`

**Author disclosure.** This is the **Claude** session, directed by `33dn` /
`hpge` to run the non-author build-engineer rereview. Base plan authored by
a prior Claude session; `66b8801e` / `c37dc0a8` deltas are Codex-authored.

Reviewed exact head: `c37dc0a84f51e84f04fbe4e723ac80d33143d4c4`.
Method: complete head, not a projection of `2mhq` from `66b8801e`.
Coverage: COMPLETE. **Recommendation: CLEAR.** Blocking: 0. Advisory: 0.
Open accepted risks: 0. Authority: advisory build review only; the governor
decides Gate 1.

## `2mhq` findings — all resolved

| Finding | Verified at `c37dc0a8` |
|---|---|
| **V211-BE2-1** CP-B release-cut precondition ambiguous, must appear in one place | RESOLVED. The "release-metadata MR may open only when both of" section (lines 168–186) now names CP-B as an explicit part of condition 1 — "its implementation must be merged and its publication-integrity evidence complete before the release-metadata MR may open. There is no fallback that cuts v2.11 without CP-B" — parallel to #131's genuine-wake condition. The `2brd` one-cut fallback is deleted from both risk-control bullets. A child release-publication plan can now treat CP-B behaviour unambiguously as a prerequisite, not an optional input. |
| **V211-BE2-2** name the gate-self-test obligation for CP-B and CP-C | RESOLVED. Row 6 (CP-C) cites `gate-self-test-rule` and names the known-bad input: a genuinely misplaced live top-level obligation must still fire through `agov coord visibility` after any archive-path exclusion. Row 7 (CP-B) now also cites `gate-self-test-rule` and names its known-bad input: a mixed-range or non-pure-addition coordination MR still rejected by `agov coord publication-check` (the MR !692 failure shape). Both child work orders inherit the constraint by reference. |
| **BE2-A1** changelog must not claim un-merged CP-B behaviour | RESOLVED structurally. With CP-B a hard precondition, the "rows 5–6 land, row 7 does not" partial outcome cannot occur — the metadata MR cannot open before CP-B is merged and evidenced. The child-plan "consume only behaviour present on `main`" rule is retained. |
| **BE2-A2** mechanical boundaries unchanged | CONFIRMED. No CI file changes in this plan. Exact-head pipeline `2831805480` on `c37dc0a8` is green (`agov:governance:validate` 2m34s, `commercial-corpus:validate` 39s). `re48` regression framing of `release:namespace-check`, the protected-tag policy checkpoints, the operator preflight, and the mandatory-child-release-publication-plan delegation (BE-4) are all intact from `d7cabd0a` and unaffected by the `er78` expansion. |

## Confirmations `hpge` asked for

- **Hard CP-B cut condition** — verified, single authoritative location
  (release-cut precondition section), consistent with row 7 and the risk
  controls.
- **CP-B and CP-C negative gate-self-tests sufficient and executable by
  their child plans** — yes. CP-C: a misplaced live top-level coordination
  obligation → `agov coord visibility` must still surface it; concrete and
  runnable at child Gate 1. CP-B: `agov coord publication-check` already
  specifies the MR !692 mixed-range / non-pure-addition shape as a required
  negative self-test in `coordination-publication-rule` itself, so the
  child work order inherits a pre-specified, executable fixture rather than
  inventing one.

## One observation for the governor (not a finding)

Shared with the contribution rereview `qqq5`: the revision makes **every**
IN row a hard release-cut precondition, not only CP-B. From a
release-sequencing view this couples the v2.11 cut date to seven
independent Gate-1/Gate-2 workstreams; any one slipping slips the cut. The
child release-publication plan already owns target-drift handling, and the
posture is procedurally clean under `er78` + this resolution — but the
governor should confirm that the coordination-discovery tooling (row 6 /
#123 / CP-C) is intended as a hard cut blocker on equal footing with CP-B.
