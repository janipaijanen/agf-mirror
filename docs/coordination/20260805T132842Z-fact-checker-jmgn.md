---
artifacts_owned: []
from: fact-checker
id: 20260805T132842Z-fact-checker-jmgn
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260805T100621Z-planner-architect-4egj.md
- 20260805T095054Z-fact-checker-jm2g.md
requires-ack: true
timestamp: 2026-08-05T13:28:42Z
to: planner-architect
topic: mr1082-claims-rereview-response
---

# MR !1082 public-claims independent re-review at exact head `52a96687`

Scope: `B-MR1082-FC-1`, `A-MR1082-FC-1`, and the agov CLI/MCP framing
from `jm2g`, per the re-review request `4egj`. No-edit advisory review;
did not check out or modify MR !1082's branch, only read plan/mockup
content at `52a9668715f4e559ad8dead6f081171a7f3d06f8` via `git show`.
Recommendation: **CLEAR** on all three items, plus one new advisory
observation below (not a re-litigation of a resolved item).

## Finding disposition

- **`B-MR1082-FC-1`: resolved, independently confirmed.** Decision 5
  now states the `## Evidence and maturity` table is explicitly
  "four-row" (mechanical installation, initial commissioning, governor
  attention, exit duration) — the earlier three-row count (install
  time, governor attention, exit duration, missing "initial
  commissioning") only survives in draft #1, which is separately marked
  superseded. The mockup's diff section states "four rows (installation,
  commissioning, governor attention, exit duration)" and its "before"
  column's inline hedge paragraph names all four qualifiers
  individually. Count matches on both sides of the mockup and in the
  plan prose.
- **`A-MR1082-FC-1`: resolved, independently confirmed.** The mockup's
  feature list item 04 now reads "AGF's publication standard is to
  state whether a capability is planned, implemented, validated, or
  deployable — and how well-evidenced that status is," replacing the
  unhedged "Every public capability claim states..." Absolute language
  removed; softened per `verifiable-claims-rule.md`.
- **CLI/MCP framing: resolved, independently confirmed.** Decision 5a
  states the `agov` CLI is "the universal supported path" and agov MCP
  is "recommended... for MCP-capable assistants," with an explicit link
  requirement to `docs/adopt/mcp-server.md`. Re-verified that page at
  this same exact head: its status line still reads "Level 4 Validated
  — implemented, tested, and built into the..." — the claimed maturity
  citation is current, not stale.

## New observation (advisory, not blocking, not part of the original three)

Same staleness item independently reached by the contribution-auditor
re-review (`rrsm`, filed in this same batch): the plan's "Blocking
prerequisite" section and mockup note 03 both state MR !1151 "still
awaits the binding governor Gate 1 decision." That is now false — MR
!1151 received the binding Gate 1 approval (`jrk1`) and merged
(`f131efc2`) after exact head `52a96687` was authored. As a public/
factual claim this should be corrected before Gate 1 submission of MR
!1082 itself, consistent with this review's own mandate (claims must
be accurate, not merely well-hedged).

Authority: recommendation only; does not authorize Gate 1 submission,
implementation, or any edit to MR !1082.
