---
id: 20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases
status: pending-gate1
topic: recall-smoke-criterion-and-weak-paraphrases
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: Review requested. Edits loader trigger text and a test protocol; no credentials, schemas or runtime code.
  - role: contribution-auditor
    status: pending
    reason: Review requested before Gate 1.
affected_paths:
  - AGENTS.md
  - .claude/rules/INDEX.md
  - docs/reference/rule-trigger-recall/corpus.yaml
  - docs/reference/rule-trigger-recall/protocol.md
  - docs/reviews/rule-projection/20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases.yaml
depends_on:
  - 20261023T140050Z-rule-trigger-recall-corpus
  - 20261023T080431Z-instruction-context-followup
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T045214Z-remove-elixir-references
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260921T040743Z-assistant-rule-drift-resolution
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: a smoke criterion that separates regression from variance, and the two weak paraphrases

Required follow-up named in Codex's disposition of MR !3176 (ACS seq 222): "prepare a separate governed
plan before this corpus is used as a blocking trigger-change gate; address both weak paraphrases
and propose a smoke criterion that separates regression from model variance without weakening
the claim gate. Do not edit frozen hold-outs." Draft by planner-architect for review by
coder-tester (Codex). Nothing here is approved. Separation of duties: I authored the scenarios
in the corpus under test, so Gate 1 and Gate 2 of this plan are decided by the other delegate or
the Human Governor.

## Why the current smoke bar cannot work as written

The approved bar is zero misses across the 60 high-consequence trials at N=3. Two complete runs
on the same triggers gave 60/60 (head `3a99c3c1`) and 59/60 (head `49ed24d7`). The arithmetic
explains it. If a rule is picked correctly with probability 0.99 per trial, the chance that
60 trials show at least one miss is 1 - 0.99^60 = **45%**; at 0.995 it is 26%; at 0.98 it is 70%.
A bar that fails nearly half the time for a 99%-reliable rule detects luck, not regression, and
would block every legitimate trigger edit at random.

## Proposal A: two-stage smoke criterion

Stage 1 (unchanged): every covered scenario, N=3, the exact head under test. A scenario with 3/3 passes
is done.
Stage 2 (new, only for a scenario that missed in stage 1): re-run that scenario alone for **10 more**
trials under the same conditions. Classify:

- **Regression** if at least **3 of the 10** fail **and** the same scenario passed 3/3 on the pre-edit
  baseline run. This blocks the edit.
- **Variance** otherwise: record it, do not block, and list the scenario in the MR.

Operating characteristics (exact binomial, 10 trials), per-trial failure probability p:

| true failure rate p | P(at least 3 of 10 fail) = chance the edit is blocked |
|---|---|
| 0.02 | 0.1% |
| 0.05 | 1.2% |
| 0.10 | 7.0% |
| 0.30 | 61.7% |
| 0.50 | 94.5% |

So a reliable rule is blocked about 1% of the time, a badly regressed one (50%) about 95% of the time,
and a moderately regressed one (30%) about 62%. Moderate regressions are what the claim gate (n >= 73
per unit, Wilson bound 0.95) exists to measure; this plan **does not touch the claim gate**. The
smoke gate stays a regression screen, never a claim, and stage 2 results are never pooled into a rate.

Baseline: before an edit, run stage 1 on the exact parent head and keep that result file. A candidate's
miss counts toward "regression" only for a scenario that was 3/3 at the baseline, so an old weak
scenario cannot block an unrelated edit. Hold-outs remain frozen and are used only for evaluation.

## Proposal B: the two weak scenarios

Observed at `49ed24d7`, route selection, 3 trials each:

1. `agent-improvement-proposal-paraphrase` ("I spotted duplicated instructions across two files while
   working and want to suggest cleaning them up."): 0/3, answered `NONE` each time (the earlier complete run
   at `3a99c3c1` scored 2/3). The trigger line in `AGENTS.md` names friction, duplication, confusing workflow,
   avoidable token spend and documentation gaps, but never the *act of suggesting a cleanup*.
2. `merge-request-workflow-paraphrase` ("Can I push this tiny fix straight to main?"): once answered
   `branch-and-worktree-rule.md`, which is always loaded and says "Never commit directly to `main`".
   That is a defensible first read, not a missed trigger.

Changes proposed:

- **Trigger text for (1).** Extend the `AGENTS.md` trigger line and the `INDEX.md` row for
  `agent-improvement-proposal-rule` to say the situation includes *suggesting a cleanup or consolidation of
  duplicated instructions*, the wording the canonical rule's table uses for routing ("note, review,
  lesson-proposal or plan"). No rule text changes.
- **Do not tune on the failing scenario.** The edit is made after seeing that scenario fail, so its score
  after the edit is contaminated. Evaluation uses **three new, unseen paraphrases** of this situation written
  by coder-tester (not by me, not read when editing the trigger), 5 trials each = 15 trials, plus the
  existing scenarios as a regression check under Proposal A.
- **Accept list for (2).** Add `agf/rules/workflow/branch-and-worktree-rule.md` to the `accept` list of
  `merge-request-workflow-paraphrase` only, with this plan as the recorded reason. This corrects a scenario
  I authored; it is a corpus-definition change, not a trigger change, and it is disclosed as a post-hoc
  adjustment. Hold-outs are not touched. If the governor-reviewer prefers, leave the accept list as is and
  record the scenario as a known-variance scenario instead.

## Acceptance

- Protocol text for Proposal A is written and reviewed; its table is reproducible from the binomial formula.
- Baseline stage 1 at the parent head and candidate stage 1 at the new head, both complete (52 x 3), with
  the Draft-07 validation and the malformed controls retained as in `protocol.md`.
- The 15-trial fresh-paraphrase evaluation passes at least **13 of 15** (87%); this is a smoke result, not
  a claim, and the exact count is fixed here before the edit is made.
- No other covered scenario that was 3/3 at baseline is classified as a regression under stage 2.
- The claim gate is not run and not changed.

## Classification and impact

STRICT: edits `AGENTS.md` and `.claude/rules/INDEX.md` (AI loaders, a downstream-impact trigger path;
red-listed under the earlier delegation, and decided here by the delegate or Human Governor under
`tpam`) and changes a test protocol that later MRs will cite as acceptance evidence.
```
Impact:
  Foundry:          one trigger line and one INDEX row change; one test protocol gains a stage; a scenario accept list grows by one path.
  Direct adopters:  none; not under agf/.
  Downstream:       none.
  Migration:        not required.
  Evidence:         MR !3176 runs at 3a99c3c1 (60/60) and 49ed24d7 (59/60); Codex 8q4h and the 222 disposition.
```
Adopter docs parity: not applicable.

## Risks and rollback

- R1 Goodhart: tuning a trigger to a scenario inflates its score. Mitigation: unseen paraphrases written by
  another author; the edit's evaluation never uses the failing scenario.
- R2 the 3-of-10 threshold passes a moderate regression. Mitigation: documented in the table; the claim gate
  stays the instrument for moderate effects.
- R3 the post-hoc accept change looks like lowering the bar. Mitigation: disclosed here, limited to a rule that
  states the same obligation, reversible, and optional for the governor-reviewer.
- R4 N=10 re-runs cost calls: at most 10 per missing scenario per run, bounded by the count of misses.
Rollback: revert the MR. No tag or release is involved.
