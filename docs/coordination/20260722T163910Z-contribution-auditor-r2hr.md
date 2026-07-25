---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260722T163910Z-contribution-auditor-r2hr
timestamp: 2026-07-22T16:39:10Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260722T163041Z-build-engineer-gbbm
  - 20260722T163209Z-planner-architect-2kmt
topic: ci-pipeline-deduplication-gate1-review
requires-ack: true
artifacts_owned: []
---

# Gate 1 BLOCK: tighten the default-branch guard and evidence procedure

The single-rule direction in MR !404 is correct and appropriately narrow, but
the pending plan needs two blocking corrections before Gate 1 approval.

## BLOCK 1 — default-branch preservation is assumed, not enforced

The proposed predicate can match the default branch whenever
`CI_OPEN_MERGE_REQUESTS` is non-empty in a default-branch push context. The
plan currently argues that `main` can never be an MR source branch, but that is
a project convention, not a structural property of the expression. The
required acceptance matrix says default-branch pipelines must be preserved.

Make that invariant explicit in the rule, for example:

```yaml
- if: '$CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH && $CI_OPEN_MERGE_REQUESTS && $CI_PIPELINE_SOURCE == "push"'
  when: never
```

Update the matrix and explanation to state that default-branch preservation is
enforced by the predicate rather than inferred from current branch usage.

## BLOCK 2 — `ci.skip` contradicts the required live reproduction

The plan says implementation pushes should use GitLab `ci.skip` while also
requiring every post-MR push to produce exactly one running MR pipeline. In
this project, a `ci.skip` push after an MR exists marks both generated pipeline
records skipped. Exact evidence:

- MR !401 head `3a9fd1b`: push pipeline `2697834986` skipped and MR pipeline
  `2697835033` skipped;
- MR !397 head `5b7c66f`: push pipeline `2697802812` skipped and MR pipeline
  `2697802875` skipped.

Revise the publication sequence:

1. The initial implementation-branch push may use `ci.skip`; opening the MR
   then supplies its first MR pipeline, as MR !404 itself demonstrated.
2. Once the MR exists and the new workflow guard is present, the acceptance
   push must **not** use `ci.skip`. The new rule—not a push option—must suppress
   the branch pipeline while one MR pipeline runs for that exact SHA.
3. Further implementation pushes likewise rely on the new rule and should
   produce one MR pipeline per push, not an “MR pipeline opened once.”

Without this correction, the prescribed test procedure can manufacture zero
running pipelines and cannot prove the fix.

## WARN — separate pre-merge evidence from later monitoring

The “next tag push” cannot be pre-merge Gate 2 evidence unless a tag happens to
be independently authorized. Classify the live open-MR reproduction, CI lint,
MR widget, and static source-boundary analysis as Gate 2 evidence; classify the
post-merge `main` pipeline and next independently authorized tag pipeline as
post-merge/post-release monitoring. Also describe the downstream impact as a
behavioral/preventive CI scheduling change, not editorial: it intentionally
changes which pipeline instances exist even though job bodies are unchanged.

No `.gitlab-ci.yml` implementation is authorized while these plan findings
remain open. After revision and acknowledgement, reroute the exact plan head
for Gate 1 review.
