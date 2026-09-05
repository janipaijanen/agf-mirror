---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260905T154555Z-contribution-auditor-3tbq
timestamp: 2026-09-05T15:45:55Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T150157Z-planner-architect-esqr.md
  - 20260905T144657Z-planner-architect-2s3a.md
topic: roi-workflow-baseline-plan-review-clear
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review evidence envelope (advisory authoring aid; not a gate decision)

Scope: MR !2372 / `docs/plans/20260905T144657Z-roi-workflow-baseline-repair.md`
  at exact head `38d027dcc2280c1bfbfba92103c52144dea1a3d8`
Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 2
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1

---

# Claude contribution-auditor review of the ROI workflow-baseline repair plan

Requested by `esqr` (the governor's Gate 1 approval of MR !2372 was made
conditional on this review despite the plan's own
`advisory_consultation: contribution-auditor: waived` entry — this record
supersedes that waiver's assumption with an actual review).

## Bounded question

Is restoring the documented automatic savings baselines for `cold-start`,
`session-resume:<role>`, and `next-handoff:<role>` correctly scoped, tested,
and safe for v2.9.0?

## Verification performed

Read the plan, `2s3a`, `esqr`, `docs/reference/tooling-token-roi.md`, and the
current `tooling/rust/crates/agov-core/src/roi.rs`. Confirmed line by line:

1. **The discrepancy is real.** `docs/reference/tooling-token-roi.md:88-92`
   states automatic collection records "workflow-baseline savings estimates
   for" `agov session resume` and `agov handoff next`. `workflow_baseline()`
   (`roi.rs:866-891`) matches only `coord-validate`, `plan-validate`,
   `validate:umbrella`, and the `validate:` prefix; `cold-start`,
   `session-resume:*`, and `next-handoff:*` all fall to `_ => None`, so those
   records carry no `baseline_source` / `approx_saved_*` fields. The plan's
   problem statement is accurate.
2. **The numbers match the governed doc.** The plan's 3,000–5,000 manual-token
   band for `cold-start` / `session-resume:<role>` and 1,000–3,000 for
   `next-handoff:<role>` are exactly the "Manual context estimate" column of
   the "Live Baseline" table (`tooling-token-roi.md:189-190`). The plan
   introduces no new number.
3. **The source anchor resolves.** `#live-baseline` corresponds to the
   `## Live Baseline` heading (`tooling-token-roi.md:165`), mirroring the
   existing `#validation-baseline` → `### Validation Baseline` pattern.
4. **Validation mappings are preserved** by scope item 2 and acceptance
   criterion 5, including the `#validation-baseline` source string.
5. **MCP characterization is correct.** `transport`/`adapter`/`tool` are
   resolved separately from `workflow_baseline()`; enabling `agov-mcp` cannot
   fix this because baseline selection lives in `agov-core`. Acceptance
   criterion 7 (CLI/MCP identical baseline arithmetic) covers this.
6. **Plan schema:** `agov plan validate` → OK (blocking severity).
7. **MR diff is plan-only:** two files (the plan and `2s3a`), 134 insertions,
   no code touched — correct for a Gate 1 plan MR.

## Scope / test / safety assessment

- **Scope:** correctly narrow — one function plus regression tests plus a
  deferred CHANGELOG entry. Local aggregate telemetry only. No new collected
  field, prompt, credential, network call, or persistence destination
  (infosec waiver stands on its stated grounds).
- **Tests:** the eight acceptance criteria target the one stated risk
  (over-assigning a documented baseline to an unrelated workflow name)
  directly — exact `cold-start` match, arbitrary role suffix on each of the
  two bounded prefixes, zero-saturation subtraction, unknown-workflow stays
  unestimated, full workspace suite. Adequate.
- **Safety for v2.9.0:** forward-only. Records written before the repaired
  binary is built are untouched; no historical JSONL rewrite; no retroactive
  report recalculation. Rollback is a plain revert of the implementation MR
  before release publication. Low risk.

## Advisory findings (non-blocking; recorded, not gating)

**A1 — `next-handoff` band generalized from a governor-reviewer-specific doc
row.** `tooling-token-roi.md:190`'s label reads "Find next *governor-reviewer*
handoff"; the plan applies the 1,000–3,000 band to every
`next-handoff:<role>`. This is a defensible reading of the doc's own
`<role>` placeholder and the band is conservative, but the doc row could be
generalized in wording later so the two never drift. Not a Gate 1 blocker.

**A2 — the doc's "Estimated saving" column is illustrative only.** The plan
maps the *manual* band to `min/max_tokens` and lets `saved_tokens()` subtract
*measured* output + prompt overhead at runtime, exactly as the existing
validation baselines behave. Internally consistent; noting it so a later
reader does not expect the 2,500–4,500 "Estimated saving" figure to appear
verbatim in records.

## Recommendation

**CLEAR.** No blocking findings. The plan is a faithful implementation-to-
documentation repair, correctly scoped, adequately tested, and safe for the
v2.9.0 line. The two advisory items above are wording-drift hygiene, not
conditions on Gate 1. Gate 1 remains the governor-reviewer's decision; this
record only removes the conditional review blocker attached in `esqr`.
