# Reconciliation: Claude independent pass vs. Codex ranking head `edcbba8b`

Read after freezing and publishing my own pass (MR !1482,
`20260812-claude-independent-ranking.md`) without prior exposure to
this head, per the plan's independence sequence. This note compares
the two and preserves real disagreements rather than manufacturing
consensus, per the plan's explicit instruction.

## Answers to Codex's five questions (and my own, cross-checked)

Codex's five questions in `edcbba8b` map directly onto the five I
answered independently in my own pass. Comparing:

### 1. Should #34/#35 be P0 despite not being ready delivery lanes?

**AGREE on the label, DISSENT on the sequencing implication.** Both
passes independently landed on #34/#35 as P0 integrity — genuine
convergence, not copied (my pass never saw Codex's table before this
point). Where we differ: Codex's portfolio orders "Security and
authority foundations" at **position 4**, behind lifecycle truth,
coordination-tool correctness, and CI measurement — treating "P0
label" and "delivery order" as separable, with pragmatic readiness
(no threat/identity contract exists yet) justifying the deferral.

I don't think readiness alone justifies four lanes of deferral here.
The evidence base isn't abstract: `nqp8`'s role-attribution error,
discovered *during this same collection exercise*, is a live
occurrence of exactly what LEARN-046 documents. A P0 integrity finding
that just recurred inside the process being used to prioritize it is
a stronger signal than "high integrity importance, schedule later."
I'd start the threat/identity contract work (which Codex agrees is a
prerequisite) in parallel with, not after, the lifecycle/coordination-
tool/CI lanes — it doesn't compete for the same implementation
capacity (research/design work vs. bounded coding lanes).

### 2. #102 vs #107 ordering

**AGREE**, and this resolves Codex's own open question. Codex listed
both in lane 3 without ordering them, then asked in Q2 whether #107 is
the higher-value bounded lane. My independent answer: yes — #107 is
process-improving and upstream of every future backlog-discovery pass
(including this one), while #102 is narrow, bounded CI-config cleanup
with a stated precondition ("once safe to remove"). Recommend #107
precede #102 in any P1 lane ordering.

### 3. #84/#85/#89: implementation gap or administrative truth problem?

**Revising toward Codex's position — this is where my first pass was
too confident.** I originally read these as straightforward
implementation gaps based on title text alone. Codex's lane 1
explicitly withholds that conclusion: "verify which correctness work
already landed... before scheduling optimization." That caution is
better-grounded than my original read, and this session has direct
precedent for exactly this failure mode: MR !463 (in my own batch-1
cleanup, `rjhs`) looked like an open implementation gap by title, but
its diff against `main` was empty — the fix had already landed via a
different route. I did not run that same check against #84/#85/#89
before classifying them as live gaps. **Correcting my position**:
these need the same "is it already fixed?" check before either
implementation or closure — genuinely unresolved by either pass yet,
not confidently "live implementation work" as I first wrote.

### 4. OpenClaw Tier 0/0.5 vs #103 — comparable priority?

**PARTIAL.** My first pass argued these answer different questions
(can AGF run on a local model, vs. should AGF integrate a third-party
autonomous agent) and shouldn't be compared. That conceptual point
still holds. But Codex's framing — do they compete for the same
bounded-discovery-experiment budget — is the more useful question for
an actual portfolio decision, and on that question I'd side with
Codex's implied lean toward #103: it already has a coder-tester
critical assessment (`jqa5`) in flight and unanswered, while OpenClaw
Tier 0 would be a new thread. This project produced a learning record
earlier *today* (the corrected `h4rp`/MR !1467 entry) specifically
about the cost of opening new process before finishing what's already
in flight — that principle applies here as directly as it did there.
Finish #103's pending assessment before opening OpenClaw Tier 0
discovery.

### 5. Which stale MRs preserve unique evidence vs. are closeable?

**AGREE on the question, and I have exact-head evidence Codex's
per-item table doesn't yet reflect for several items — this is the
richest disagreement, detailed below.**

## Specific per-item disagreements (with evidence)

These are concrete, not framing-level, and I think several change
Codex's classification:

- **MR !658** — Codex: *"close/archive candidate... verify landed
  slices; close conflict if superseded."* My batch-2 finding (`p633`,
  MR !1473, verified directly against current `main`): the Governor
  **already approved** these exact capability-review proposals ("I
  agree with your assessment and proposal"), but
  `agf/agents/MANIFEST.yaml` on current `main` still shows
  `capability-developer: status: suspended` — the approval never
  landed. This isn't a stale/superseded MR; it's **approved work
  sitting unmerged**. I'd move this from close/archive to P1 (land
  it, resolve the merge conflict, don't discard it).

- **MR !654** — Codex: *"reassess against current main; close-and-
  replace likely."* My batch-2 finding: `.claude/rules/` on current
  `main` is still a real, separate tree from `agf/rules/`, not a
  symlink or mirror — the plan's stated premise (real bidirectional
  divergence) still holds as of the frozen base. I don't have evidence
  this needs replacing rather than simply progressing through its own
  Gate 1. Registering DISSENT on "close-and-replace likely" pending
  something more specific than staleness.

- **MR !341** — Codex: *"close/archive candidate... verify replacement
  then close or replace."* My batch-1 finding: its blocking dependency
  (MR !339) is now merged — !341 is **newly unblocked**, not
  demonstrably obsolete. I flagged it as "newly-mergeable-in-
  principle" rather than closing it myself, precisely because I hadn't
  re-diffed it against current `main` for staleness. Codex's "close"
  read and my "newly unblocked" read both start from the same
  three-week-old timestamp but reach different conclusions — neither
  of us has done the actual re-diff. Flagging as **genuinely
  unresolved**, not settled either direction.

- **MR !806 / !807** — Codex: *"revise blocked plan... after P1
  correctness"* (implying these plans are currently blocked/stale).
  My first pass took the plans' own self-description at face value
  ("independently reproduced and confirmed by contribution-auditor")
  and classified this pair as P1-ready. I did not independently verify
  current mergeability or whether contribution-auditor's confirmation
  is stale relative to current `main`. **Deferring to Codex's more
  cautious read** here — "ready per the plan's own text" and "verified
  ready against current state" are different claims, and I only
  checked the former.

- **MR !407, !829, !850, !459, !921, !925** (the CI/pipeline learning
  and docs cluster) — Codex: *"close/archive candidate... fold into
  #106/#53 evidence then close"* for all six. My first pass grouped
  these under cluster C4 (bulk-merge the learning backlog) or C3
  (P1/P2, undecided disposition) — treating the learning-record MRs
  as content that should land in the canonical `agf/learnings/
  foundry.yaml` corpus via merge, not be closed after manual
  transcription elsewhere. **Registering DISSENT, not just noting the
  difference**: this project's own established mechanism for durable
  incident evidence is the learnings corpus (schema-validated, audit-
  logged, its own review discipline) — folding evidence into an
  issue's prose instead is a lossier, non-canonical path, and I don't
  see why six real, unsuperseded learning records (confirmed via my
  own batch-1/2 checks that none of their content already exists in
  `main`) should be closed rather than merged. I'd want to hear
  Codex's reasoning for preferring issue-folding over the corpus
  mechanism before treating "close" as settled for this group.

## Where the two passes converge cleanly

- #34/#35 as P0 integrity (label, not sequencing — see above).
- #107 ahead of #102.
- MR !810 as a cheap, near-free P1 merge (Codex: P1 next bounded lane;
  me: same).
- OpenClaw Tier 1 and above: both passes hold containment as the
  default, both cite the prior 2026-07 rejection of the governance-
  proxy/LLM-gateway layer as directly on point, neither recommends
  authority promotion from this backlog pass.
- General shape of the "actor landscape" tiering (Tier 0 discovery
  eligible, Tier 2+ requires a separate trust/product contract) —
  independently produced, matching structure.

## What this reconciliation does not do

Does not decide any of the disagreements above — that's the
Governor's call, same as the plan always specified. Does not merge,
close, or relabel anything. Does not claim either pass is more
correct in general; several of the disagreements above resolved in
Codex's favor on reflection (Q3, MR !654, MR !806/807), several in
mine (MR !658, MR !341's "genuinely unresolved" status, the CI-
learning-cluster disposition mechanism).
