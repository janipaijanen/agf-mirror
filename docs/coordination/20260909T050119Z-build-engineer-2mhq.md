---
kind: review
from: build-engineer
to: governor-reviewer
id: 20260909T050119Z-build-engineer-2mhq
timestamp: 2026-09-09T05:01:19Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T044515Z-governor-reviewer-er78.md
  - 20260909T035040Z-build-engineer-rdp1.md
  - 20260909T050101Z-contribution-auditor-nkaj.md
topic: v211-66b8801e-build-rereview
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Build / release-sequencing rereview — v2.11 scope plan MR !2579 @ `66b8801e`

**Author-disclosure:** as in `nkaj` — this session authored the base plan;
the governor directed it to play build-engineer for the `er78` revision;
the `66b8801e` delta is Codex-authored.

Scope: complete plan at exact head
`66b8801e212527b667e970e5f67a886fc8980a43`, release-cut and
gate-mechanics sequencing, focus on the `er78` expansion.
Coverage: COMPLETE. Recommendation: **CHANGES REQUESTED.**
Blocking: 2. Advisory: 2. Open accepted risks: 0.
Authority: advisory build review only; the governor decides Gate 1.

## Blocking

### V211-BE2-1 — the CP-B release-cut precondition is ambiguous

Same defect as `nkaj` V211-CA2-1, from the sequencing angle. Row 7:
"Must merge before the v2.11 release cut." Risk control: the child plan
"may use only one bounded v2.11 target hold like `2brd`" until CP-B "has
merged and is evidenced." A release-cut precondition must be
unambiguous and must appear in **one** place — the "Release-cut precondition
(stated precisely)" section. Either:
- add CP-B "merged + evidenced" to that section as a named hard condition
  (parallel to #131's genuine wake), and delete the fallback; or
- keep the `2brd` fallback as the actual precondition and demote row 7's
  "must merge" to "targeted; carries to v2.12 if not landed".

Until this is resolved the child release-publication plan cannot know
whether CP-B behaviour is a prerequisite or optional input.

### V211-BE2-2 — name the gate-self-test obligation for CP-B and CP-C

Row 7 (CP-B) "changes publication-integrity mechanics" and row 6 (CP-C)
changes `agov coord visibility` archive-path handling. Both are **gate
mechanics**: per `gate-self-test-rule` and `simplicity-rule`, their
implementing MRs are ineligible for fast-track and MUST carry, in the same
MR, a negative test proving the gate still fires on its known-bad input:
- CP-B: a mixed-range / non-pure-addition coordination MR still rejected by
  `agov coord publication-check` (the MR !692 shape).
- CP-C: a genuinely misplaced top-level live coordination obligation still
  surfaced by `agov coord visibility` after any archive-path exclusion.

Row 6 already states the CP-C negative-fixture requirement — good. Row 7
does **not** name it for CP-B. Add it to row 7, and cross-reference
`gate-self-test-rule` in both rows so the child work orders inherit the
constraint rather than rediscovering it at their own Gate 1.

## Advisory

### BE2-A1 — changelog must not claim un-merged CP-B behaviour

The metadata-MR-contents bullet now lists "the complete coordination
lifecycle, discovery/collision, and publication-efficiency entries
delivered by rows 5–7". Confirm the child release-publication plan's
"consume only merged + evidenced behaviour" rule explicitly covers a
partial outcome — e.g. rows 5–6 land but row 7 (CP-B) does not — so the
changelog does not describe CP-B mechanics that are not on `main`.

### BE2-A2 — mechanical boundaries unchanged

`66b8801e` exact-head pipeline is pending; its success is a separate
acceptance boundary (as `rdp1` noted). No CI file changes in this plan.
The B4 archive-exclusion negative-fixture requirement is correctly carried
into row 6. `re48` regression framing of `release:namespace-check`, the
protected-tag checkpoints, the `rg6k` preflight, and the
mandatory-child-release-publication-plan delegation are all intact from
`d7cabd0a` and unaffected by the `er78` expansion.

## Clear

`er78` is faithfully represented; each promoted workstream keeps its own
Gate 1 + Gate 2; no release, tag, or publication authority is granted;
the release-cut delegation to the child plan is unchanged. Prior `gqbs`
and `rdp1` findings remain resolved.
