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
  - docs/reference/rule-trigger-recall/protocol.md
  - docs/reference/rule-trigger-recall/corpus.yaml
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

## Proposal A: two-stage smoke criterion, compared against the baseline

Stage 1 (unchanged): every covered scenario, N=3, on the pre-edit **baseline head** and on the **candidate
head**, under identical conditions (same client version, model id, prompt wording, tools restriction). A scenario
that is 3/3 on both is done.

Stage 2 (bounded, symmetric): if **either** side missed on a scenario in stage 1, run that scenario for **10
more trials on each side** (baseline head and candidate head), so at most 20 extra calls per flagged scenario.
This fixes the asymmetry in the first draft, where a scenario that was already weak at baseline (for example
2/3) could never be judged against a 0/10 candidate.

Exact comparison, applied to the 10-trial counts only (stage 1 trials are kept as evidence, not pooled):

- Let `b` = baseline failures out of 10 and `c` = candidate failures out of 10.
- **Regression** if `c >= 3` **and** `c - b >= 3`. This blocks the edit.
- **Variance or improvement** otherwise: record `b` and `c`, do not block, list the scenario in the MR.

Operating characteristics, exact binomial (two independent 10-trial samples, per-trial failure probability
`p_b` baseline and `p_c` candidate):

| case | chance the edit is blocked |
|---|---|
| no real change, `p_b = p_c = 0.02` | 0.07% |
| no real change, `p_b = p_c = 0.05` | 0.7% |
| no real change, `p_b = p_c = 0.10` | 3.0% |
| no real change, `p_b = p_c = 0.30` (a weak scenario) | 11.0% |
| baseline 0.05, candidate 0.20 | 23.4% |
| baseline 0.05, candidate 0.30 | 49.2% |
| baseline 0.05, candidate 0.50 | 87.8% |
| baseline 0.05, candidate 0.70 | 99.0% |
| baseline 0.30, candidate 1.00 | 99.8% |

A reliable scenario is blocked well under 1% of the time when nothing changed, a badly regressed one
(50%+) about 88-99% of the time, and a weak scenario that gets no worse is blocked about 11%, not
automatically. Moderate regressions (a rise from 5% to 20%) are mostly not caught by smoke; the **claim gate
(n >= 73 per unit, Wilson bound 0.95) measures those and is neither touched nor weakened.** The smoke gate stays
a regression screen, never a claim, and stage 2 counts are never pooled into a rate.

Baseline hygiene: the baseline stage 1 result file for the parent head is kept with the MR, with the Draft-07
validation and malformed controls from `protocol.md`. Hold-outs remain frozen and are used only for evaluation.

## Proposal B: the two weak scenarios

Observed at `49ed24d7`, route selection, 3 trials each:

1. `agent-improvement-proposal-paraphrase` ("I spotted duplicated instructions across two files while
   working and want to suggest cleaning them up."): 0/3, answered `NONE` each time (the earlier complete run
   at `3a99c3c1` scored 2/3). The trigger line in `AGENTS.md` names friction, duplication, confusing workflow,
   avoidable token spend and documentation gaps, but never the *act of suggesting a cleanup*.
2. `merge-request-workflow-paraphrase` ("Can I push this tiny fix straight to main?"): once answered
   `branch-and-worktree-rule.md`, which is always loaded and says "Never commit directly to `main`".
   That is a defensible first read, not a missed trigger.

### Decision on (2), made now: the accept list is NOT changed

`merge-request-workflow-paraphrase` keeps its current `accept` list. Widening it after seeing the miss would
lower a bar that I authored, and the new stage 2 already separates a single stray answer (variance) from a real
regression, so no corpus edit is needed. `corpus.yaml` is not edited for any accept list; its only change under this plan is adding the revealed
unseen paraphrases after reveal (see Acceptance). The scenario is recorded as a known-variance scenario in the MR. If later evidence shows that
`branch-and-worktree-rule.md` is the habitual first read, that is a new proposal.

### Trigger text for (1)

Extend the `AGENTS.md` trigger line and the `INDEX.md` row for `agent-improvement-proposal-rule` to say the
situation includes *suggesting a cleanup or consolidation of duplicated instructions*, the routing wording
the canonical rule uses ("note, review, lesson-proposal or plan"). No rule text changes.

### Unseen paraphrases: sealing, storage and reveal lifecycle

The failing scenario cannot validate its own fix (the edit is made after seeing it fail), so the evaluation uses
**three unseen paraphrases of the same situation, written by coder-tester (Codex)**, 5 trials each = 15 trials.
Lifecycle, in order:

1. **Seal.** Before the trigger edit is written, Codex writes the three prompts to a local file outside the
   repository (not committed, not shared), computes `SHA-256(file bytes + a random 16-byte salt)` and publishes
   **only that hash** in an AGF coordination record. I never read the prompts at this stage.
2. **Edit.** I write and push the trigger edit at exact head `H`. No unseen prompt exists in the repository, in
   any MR text, or in any room message yet.
3. **Reveal.** After `H` is pushed, Codex publishes the file and the salt (as a commit to this MR's branch, or
   a coordination record). Anyone can verify the hash matches the sealed value; a mismatch voids the evaluation.
4. **Evaluate.** Run the 15 trials at `H` under the protocol; also run stage 1/2 for all 52 scenarios against
   the baseline.
5. **Burn.** The revealed prompts are retained as `holdout: false`, `author: coder-tester` corpus scenarios
   marked used. They are never again counted as unseen for a later trigger edit; Codex writes a fresh set then.
   This keeps contaminated prompts from passing as hold-outs.

Pass: at least **13 of 15** (87%). This is a smoke result, fixed here before the edit, not a recall claim.

## Acceptance

- `protocol.md` carries the stage 1 / stage 2 text of Proposal A, and its table is reproducible from the
  binomial formula (script or formula in the MR).
- Baseline stage 1 at the parent head and candidate stage 1 at the candidate head, both complete (52 x 3),
  with Draft-07 validation and the malformed controls retained as in `protocol.md`. Stage 2 (10 + 10 trials) for
  every scenario that missed on either side, with `b` and `c` listed and the regression rule applied exactly
  as written; no scenario classified as a regression.
- The sealed-paraphrase lifecycle is followed in order and the hash verifies; the 15-trial evaluation passes at
  least **13 of 15**. This is a smoke result, not a claim.
- The `merge-request-workflow-paraphrase` accept list is unchanged and the scenario is listed as known variance.
- `corpus.yaml` is changed only after reveal, to add the burned paraphrases as `holdout: false` (a separate
  commit that reuses revealed text verbatim); this plan declares that follow-on and nothing earlier.
- The claim gate is not run and not changed.

## Classification and impact

STRICT: edits `AGENTS.md` and `.claude/rules/INDEX.md` (AI loaders, a downstream-impact trigger path;
red-listed under the earlier delegation, and decided here by the delegate or Human Governor under
`tpam`) and changes a test protocol that later MRs will cite as acceptance evidence.
```
Impact:
  Foundry:          one trigger line and one INDEX row change; one test protocol gains a symmetric two-stage comparison; the revealed unseen paraphrases are added to the corpus after reveal. No accept list changes.
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
- R3 the sealed lifecycle is skipped or a prompt leaks before reveal. Mitigation: the hash is published first and
  verified at reveal; a mismatch voids the evaluation and Codex writes a fresh set.
- R4 N=10 re-runs cost calls: at most 10 per missing scenario per run, bounded by the count of misses.
Rollback: revert the MR. No tag or release is involved.
