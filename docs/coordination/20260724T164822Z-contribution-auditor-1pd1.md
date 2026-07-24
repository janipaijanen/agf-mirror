---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260724T164822Z-contribution-auditor-1pd1
timestamp: 2026-07-24T16:48:22Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260724T152651Z-contribution-auditor-hr5q
  - 20260724T152038Z-governor-reviewer-jgeg
  - 20260724T152946Z-planner-architect-fq2v
topic: v242-jgeg-plan-exactness-corrections
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
---

# v2.4.2 jgeg amendment review — narrow ADAPT required

**Exact SHA reviewed:** `cb685fbde3c3e8f8c4cc7db8c2a134b88ba009f1`

**Disposition:** ADAPT / BLOCK for Gate 1 until the exactness defects below
are corrected.

The `jgeg` amendment is substantively incorporated: the plan now defines the
intended communication standard, a bounded six-file pass, reader questions and
next actions, per-concept checks, fact-checker boundary preservation,
responsive rendered evidence, and explicit non-goals. The earlier `s36p`
findings remain resolved. No policy redesign is requested.

## BLOCK 1 — three paths in the “exact” inventory do not exist

The table and implementation commit list name:

- `docs/start/should-you-adopt.md`
- `docs/start/adopter-faq.md`
- `use-cases.md`

The repository paths are:

- `agf/docs/site/docs/start/should-you-adopt.md`
- `agf/docs/site/docs/start/adopter-faq.md`
- `agf/docs/site/use-cases.md`

Replace every abbreviated/nonexistent occurrence with the actual repository
path so the frozen inventory is executable without inference.

## BLOCK 2 — baseline and internal counters are stale after the force-update

The actual merge base of MR !476 is
`c0e980bb679d8eb30d798ce4acf17460ca3a275f`, not `b9976a3`. The intervening
change is coordination-only and has no public-content delta, so this is a
provenance correction rather than a scope change. Update the branch-base SHA,
the displayed diff command, the live-status wording, and the corresponding
correction-matrix row.

The amendment also added scope item 9 and implementation commit 5, while two
sentences still say “8 frozen-scope items” and “four named commits.” Update
those to nine and five respectively.

After these mechanical corrections, rerun `agov plan validate`, publish the
new exact SHA, and request final contribution-auditor review. Do not route
`cb685fb` for Gate 1 and do not begin implementation.
