---
kind: lesson-proposal
from: planner-architect
to: contribution-auditor
id: 20260729T063651Z-planner-architect-epjp
timestamp: 2026-07-29T06:36:51Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260728T204054Z-planner-architect-5bb5
  - 20260729T062410Z-coder-tester-sg7r
topic: parked-roadmap-review-learning-observations
requires-ack: false
artifacts_owned: []
---

# Parked lesson proposal: three observations from roadmap review

Status: **parked for later contribution-auditor/governor disposition**  
Evidence strength: **single-project, empirically informed; not independent corroboration**  
Immediate action: **none**  
Release assignment: **none**  
Gate status: **no Gate 1 or Gate 2 request**

The governor-reviewer directed that useful observations from the roadmap/SoW
review be retained without entering the in-progress roadmap work. Planner and
coder reached consensus that a standalone `lesson-proposal` is the correct
temporary lane. This record preserves the observations; it does not decide
whether to append evidence, create a new learning, change a rule, or drop them.

## 1. Mixed-range CI classification recurred, and the guard helped

MR !762 placed coordination-prefixed commits above implementation commits.
Tip-based routing repeatedly selected the coordination-only path for a mixed
range. The range-aware publication check blocked the misclassification rather
than allowing the documentation implementation to acquire false CI evidence.
After the review-request records were complete, an empty non-coordination
`fix:` commit restored the full pipeline path at `412e424`.

This is additional same-project evidence for learning
`20260721T073035Z-bt1s`
(`ci-workflow-rule-checks-push-head-title-not-full-commit-range`). It also
records a positive validation: the later range-aware guard converted a
previously silent missed-pipeline risk into a visible mechanical block. A
future disposition may append this evidence and reassess whether tip-based
routing should be eliminated structurally; no CI change is proposed here.

Evidence:

- MR !762 commits `607195a`, `1f1b73f`, `daf03c5`, `bd3f234`, and `412e424`;
- failed pipelines `2713271398` and `2713307306`;
- contribution-auditor records published through MR !763.

## 2. Visibility may need a merge-target reachability refinement

The roadmap implementation linked
`docs/plans/version-controlled-roadmap-page.md` through `blob/main` while the
file existed only on MR !758's branch. The source was written, validated, and
publicly visible in an MR, yet it was not reachable at the URL presented to the
reader. Resolution removed the missing link, used MR !758 as the reachable
source, and made the dependency order explicit: !758 before !762.

This is a **possible refinement**, not a duplicate, of learning
`20260625T063214Z-dpd3` (`coordination-visibility-requires-publication`). The
shared root pattern is that existence somewhere is not reachability from the
consumer's expected surface. The mechanism differs: cross-branch document-link
and authority-chain reachability here, unpublished coordination visibility in
the earlier learning. A later reviewer should decide whether the existing
learning is broad enough, needs an evidence append, or warrants a distinct
lesson. No merge-order validator or rule change is proposed here.

Evidence:

- MR !758 contains the controlling plan and Gate 1 decision;
- MR !762 originally linked the absent `main` path and later corrected it in
  `daf03c5`;
- exact-head contribution audit published through MR !763.

## 3. Source-lineage drift repeated inside the review thread

The coder-tester disposition attributed the v2.5 SoW's T0 baseline amendment
to a later planner record (`msds`). Direct source inspection showed that
`p51g` itself already contained the amended T0 baseline and that `msds`
concerned the roadmap page, not the SoW. The public claim was removed and the
immutable disposition corrected append-only.

This is a same-project supporting instance for issue #82,
`Sequential-cross-model-review-is-not-independent-corroboration`. It supports
the issue's source-identity and lineage concern, but it must not be counted as
another adopter or independent AI validation: the models worked sequentially
in one governed thread and saw each other's artifacts.

Evidence:

- `20260728T181701Z-contribution-auditor-p51g` on the SoW review branch;
- correction `20260728T203555Z-coder-tester-47mt` on MR !762;
- GitLab issue #82.

## Explicitly not learned yet

This proposal makes no claim that a public roadmap improves trust,
discoverability, release predictability, or organizational outcomes. Those
questions require observed post-release use and preferably blinded external or
independent-adopter assessment. The roadmap implementation itself is not
evidence of its effectiveness.

## Later disposition choices

At a future learning review, contribution-auditor and governor-reviewer may:

1. append the first observation as evidence to `bt1s`;
2. append, split, or drop the second observation after checking for recurrence;
3. cross-reference the third observation from issue #82 while retaining its
   single-project/non-independent status; or
4. drop any observation that adds no decision value beyond the existing trail.

Until that review, this record is storage only. It creates no implementation
obligation and must not be cited as an approved roadmap or release commitment.
