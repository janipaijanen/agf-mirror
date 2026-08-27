---
kind: decision-record
from: governor-reviewer
to: all
id: 20260824T163113Z-governor-reviewer-36rr
timestamp: 2026-08-24T16:31:13Z
model: human-via-codex
immutable: true
references-to:
  - 20260824T162316Z-contribution-auditor-4t37.md
  - 20260824T155816Z-contribution-auditor-abnr.md
topic: review-convergence-circuit-breaker
requires-ack: false
artifacts_owned: []
---

# Governor decision: stop unbounded clean-room review loops

The human governor approved this convergence directive in the active Codex
session after reviewing the MR !1847 autopsy. It is effective immediately for
MR !1847 / issue #115 and MR !1965 / issue #112, and is the default for future
review loops unless a stricter named rule requires otherwise.

This decision does not waive any known material finding, approve Gate 1 or
Gate 2, authorize implementation beyond an approved plan, or approve any
separate role, CI-scope, release, deployment, or merge decision.

## Convergence protocol

1. Use one COMPLETE independent review of a candidate, followed by one
   consolidated correction batch and one targeted resolution review.
2. The targeted review checks the recorded findings, the correction diff, and
   explicitly affected invariants. It is not another full clean-room audit.
3. After three blocked review cycles, stop automatic fresh-review cycling.
   The governor must choose one of: simplify the design, split scope, accept a
   specifically recorded risk, abandon/rewrite the candidate, or explicitly
   authorize another full review.
4. Assign one independent reviewer to an exact head. Do not run parallel
   competing reviews of the same exact head without explicit governor
   authorization.
5. A blocking finding must identify a concrete failure against a named
   mandatory requirement and have material gate impact. Advisory or
   non-material improvements go to a follow-up issue/backlog unless the
   governor explicitly promotes them.
6. Keep the canonical plan and MR description focused on current operative
   truth. Detailed review history belongs in immutable coordination records;
   use compact references in the plan instead of appending round-by-round
   audit diaries.
7. During targeted verification, a concern outside the correction delta is
   escalated to the governor for disposition rather than automatically
   starting another clean-room round.

## MR !1847 disposition

Round 14 review `20260824T162316Z-contribution-auditor-4t37` is the final full
clean-room review. Its material blocker `CA-NNP-R14-1` remains open and must be
fixed: invalid/escaping repository paths must remain distinguishable from a
legitimately absent planned path and must never be suppressed by
`--allow-planned`.

The planner-architect may prepare one consolidated correction batch. The same
round-14 reviewer then performs targeted resolution verification only. There
is no automatic round-15 full audit. Any newly discovered issue outside that
targeted scope returns to the governor under point 7.

## MR !1965 disposition

The four findings in
`20260824T155816Z-contribution-auditor-abnr` remain open. Resolve the separate
role-authority and CI-scope/build-owner questions through their binding
decision surfaces, then prepare one consolidated correction batch covering
all resulting work. Perform one targeted final verification of those findings
and the correction delta; do not restart an unbounded complete-audit sequence.

## Rationale

MR !1847 accumulated 43 blockers and 6 advisories through rounds 1–13 while
its plan grew from 309 to 2,167 lines. About 906 lines preceded the operative
design, largely because review history was copied into the artifact. The
reviews exposed real defects, but repeated full audits of an expanding,
duplicated text surface created additional drift and prevented convergence.
This directive preserves independent review and material blockers while
adding a bounded path to delivery.
