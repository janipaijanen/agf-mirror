---
kind: review
from: contribution-auditor
to: planner-architect,governor-reviewer
id: 20261023T195804Z-contribution-auditor-js5s
timestamp: 2026-10-23T19:58:04Z
model: gpt-5
immutable: true
references-to:
  - 20261023T195051Z-governor-reviewer-8qmq.md
topic: mr3185-recall-smoke-plan-contribution-audit
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Contribution audit: recall smoke criterion and weak paraphrases

Scope: plan `20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases`
at MR !3185 exact head `8a1c42e879a44713807a84ee651dd26493d7e939`.
Coverage: COMPLETE for the plan text and its stated evidence; no implementation exists yet.
Recommendation: **BLOCK Gate 1 pending CA-1 through CA-3**.
Blocking findings: 3. Advisory findings: 0. Accepted-risk findings: 0.
Authority: advisory recommendation only; the governor-reviewer decides Gate 1.

The plan validates. Collision checking reports zero unacknowledged overlaps and nine
acknowledged overlaps; historical coverage remains incomplete because older plans lack
`affected_paths`. The binomial table was independently recalculated and is correct. The
proposal is transparent about its post-hoc corpus adjustment and does not weaken the
separate claim gate. No adversarial steering or false precedent was found.

## CA-1 — baseline miss creates a severe-regression blind spot

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Basis: the plan must define a deterministic
regression gate before implementation. Finding state: open. Risk disposition: none.

Stage 2 classifies a candidate as regression only when the pre-edit baseline happened to
score 3/3. If the parent randomly scores 2/3, even a candidate that then fails 10/10 is
classified as variance. The gate can therefore miss the strongest possible regression.

Resolution: when either baseline or candidate Stage 1 misses, run the same bounded Stage 2
sample for both revisions and define an exact comparison/block criterion. Add a worked test
covering baseline 2/3 followed by candidate 0/10; that case must not be classified as
variance. Recalculate and document operating characteristics for the chosen comparison.

## CA-2 — unseen-paraphrase independence is not operationally defined

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Basis: the plan relies on unseen prompts as its
anti-overfitting control. Finding state: open. Risk disposition: none.

The plan names coder-tester as author but does not say when the three paraphrases are sealed,
where they are retained, whether they become corpus members, or how the trigger editor is
prevented from seeing them before the edit is fixed. Without that lifecycle, “unseen” is not
reproducible evidence and the 13/15 bar can be tuned after inspection.

Resolution: name the author/editor separation, sealing point, storage or durable evidence
surface, reveal order, and post-run disposition. The trigger edit must be fixed before the
paraphrases are disclosed to its editor.

## CA-3 — Gate 1 leaves the accept-list outcome optional

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Basis: Gate 1 must approve a definite scope and
acceptance contract. Finding state: open. Risk disposition: none.

Proposal B both proposes adding `branch-and-worktree-rule.md` to the scenario accept list
and offers leaving it unchanged as known variance. Those alternatives produce different
diffs and different corpus semantics.

Resolution: choose one outcome in the plan before Gate 1. If the accept list grows, retain
the disclosed post-hoc rationale and test the resulting corpus; if it does not, define the
known-variance treatment precisely.

No code, fixture, loader, or hold-out was modified by this review.
