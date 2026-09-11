---
id: 20260911T044602Z-quantized-model-mechanical-drafting-pilot
status: pending-gate1
topic: quantized-model-mechanical-drafting-offline-pilot
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      The harness is offline and produces no live coordination records
      (see Scope). If a hosted small-model baseline is used (Step 2), no
      adopter-identifying content, credentials, or repository secrets may
      be sent to it — only the genericized synthetic task corpus defined
      in this plan. A fully local/quantized model run has no network
      egress at all. InfoSec must confirm the corpus-genericization step
      (Step 1) actually removes project-identifying detail per
      `external-anonymization-rule.md` before any hosted-baseline call,
      and that the harness has no path to write to `docs/coordination/`
      on the real repository.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Confirms the "mechanical/templated drafting only" boundary is
      enforced by the task corpus design (Step 1), not by the model's own
      judgment; that zero live coordination records are produced (this is
      an evaluation harness, not a pilot on the real channel); and that
      the decision thresholds (Step 5) are stated before results exist, so
      they cannot be adjusted after the fact to fit whatever the numbers
      show.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Quantized/small-model mechanical-drafting pilot — offline evaluation harness (Gate 1)

## Provenance

- Governor directive `pnqk` (2026-09-11): assess the operating model to
  (a) minimize LLM token use, (b) minimize footprint enough to run on
  consumer GPUs with efficient/quantized models.
- Preliminary assessment (`gmpm`, MR !2688): names one bounded, safe
  experiment as a first step, independent of the RUW trigger — a
  quantized/small-model pilot restricted to **capability tier 2**
  (templated, mechanically-verifiable coordination-record drafting),
  since `agov coord validate` already gives an objective, zero-cost
  accept/reject signal for that task class.
- This plan is that experiment, detailed to Gate 1.

## Objective (the question this experiment answers)

**Does a quantized, small, open-weight model — sized to run on
consumer-GPU-class hardware — produce schema-valid *and*
semantically-correct coordination-message drafts for the mechanical/
templated task class, at an acceptable failure rate, and at meaningfully
lower token/compute cost than the current frontier-model baseline?**

This is a capability-maturity question (Level 1 Planned → Level 3
Instantiated evidence, per `implementation-honesty-rule.md`), not a
production-readiness one. A positive result justifies a **separate**,
later Gate 1 for any live pilot; it does not itself authorize one.

## Scope — offline evaluation harness only

**Zero live coordination records are produced by this experiment.**
Every model output is written to a throwaway evaluation directory
(`tmp/quantized-pilot/`, gitignored, never `docs/coordination/`) and
scored by the harness. No commit, branch, MR, or coordination record
from this experiment's outputs enters the real repository or channel.
This makes the safety property independent of model quality: even a
completely wrong output cannot reach governed state.

**In scope (capability tier 2 only):** drafting an `ack`, a routing
`note`, a `completion`, or a simple `review` record from a fully-specified
trigger (an existing record to reference, a known recipient, a known
outcome to state) — the class of task where correctness is checkable
against the trigger, not a matter of judgment.

**Out of scope:** Gate 1/2 plan authorship, cross-cutting review findings,
any record whose content requires inference not present in its trigger,
and anything that would write to the real channel.

## Step 0 — environment interrogation (required before any model is chosen)

Per `environment-interrogation-rule.md`: this plan does not assume where
the harness runs. Before Step 2, build-engineer confirms and records:

- Whether a consumer-GPU-class machine (or a GPU-equipped CI runner) is
  actually available to this project, and its VRAM/compute class. **None
  was found in the current sandboxed development container** (no
  `nvidia-smi`, no local quantized-inference tooling present) — the
  harness cannot assume it can run here.
- Which local-inference stack is installable in the actual target
  environment (e.g. `llama.cpp`, `ollama`, `vllm`) and its resource
  footprint.
- Whether running the harness requires new infrastructure (a
  self-hosted runner, a local workstation) or can reuse existing CI
  capacity — and the cost/scheduling impact either way.

If no consumer-GPU-class environment is available at all, this
experiment is blocked on infrastructure, not on the plan — report that
as the concrete blocker rather than substituting a cloud GPU (which would
not answer the "consumer GPU" question the directive asks).

## Step 1 — build the task corpus (mechanical-only, genericized)

Derive 20-30 synthetic tasks from this session's own real coordination
history, each reduced to: **(a)** a fully-specified trigger (what
happened, who it's for, what record kind is expected), **(b)** the
objective acceptance test (schema validity + a semantic check — see Step
3), **(c)** genericized per `external-anonymization-rule.md` (codename
only, no adopter-identifying detail) since Step 2 may call a hosted
baseline. Draw examples across: ack, routing note, completion,
simple review. Exclude anything from this session that required
judgment beyond restating the trigger (e.g. do not include QC-1's
review-finding content, which needed reasoning, not restatement).

## Step 2 — candidate models

- **Primary candidate**: a quantized, open-weight model in a size class
  plausibly consumer-GPU-deployable (indicatively 7-14B parameters,
  4-8 bit quantization — exact model/quantization selected at
  implementation time based on Step 0's findings, not fixed here).
- **Fast sanity baseline** (optional, cheap, run first): a smaller hosted
  model already available to this project (if any) as an early signal
  before standing up local infrastructure — this does not answer the
  "consumer GPU" question but can catch a badly-designed task corpus
  before spending setup effort on local inference.
- **Frontier baseline**: the current session's own model, run once over
  the identical corpus, for the token-cost and accuracy comparison point.

## Step 3 — scoring (both checks required; neither substitutes for the other)

1. **Schema validity** — `agov coord validate` on the raw output,
   verbatim, no post-processing. Pass/fail, objective.
2. **Semantic correctness** — a human or a stronger-model reviewer
   confirms the schema-valid output actually states the right kind,
   recipient, references, and content for its trigger. This is the check
   that catches "schema-valid but wrong" (the risk this plan's InfoSec
   row and the preliminary assessment both flag) — a validator alone is
   not sufficient evidence of correctness.

Record, per candidate model: first-attempt schema-pass rate,
semantic-correctness rate among schema-valid outputs, tokens generated
per accepted output, and (for the quantized candidate) wall-clock
inference time on the Step-0 hardware.

## Step 4 — comparison

Report all three candidates' numbers side by side. No candidate's result
is adjusted or re-scored after seeing another candidate's numbers.

## Step 5 — decision thresholds (stated now, before any result exists)

- **Schema-pass rate below 90%** on first attempt → capability gap is
  reliability, not cost; report as Level 2 Surface-defined for this task
  class, no live-pilot recommendation.
- **Schema-pass rate ≥ 90% but semantic-correctness rate below 95%** among
  schema-valid outputs → report the specific failure pattern (e.g.
  right-shape-wrong-recipient); no live-pilot recommendation without a
  named mitigation for that pattern.
- **Both ≥ 90%/95%** → Level 3 Instantiated evidence; recommend, as a
  **separate future Gate 1**, a narrowly-scoped live pilot with a human
  or strong-model reviewer gating every output before publication (never
  unsupervised on first deployment).
- **No consumer-GPU-class environment available at all (Step 0 blocker)**
  → report the infrastructure gap; the capability question stays
  unanswered, not answered negatively.

## Risks and controls

- **A wrong output looks plausible and passes casual review.** Control:
  semantic-correctness scoring is against the trigger's objective
  acceptance test, not a reviewer's general impression; genericized
  corpus makes triggers easy to check against ground truth.
- **Corpus accidentally leaks project-identifying detail to a hosted
  baseline.** Control: Step 1's genericization is InfoSec-reviewed before
  Step 2's hosted call, per `external-anonymization-rule.md`.
- **Harness output mistaken for a live record.** Control: `tmp/`
  gitignored path only; no code path in this experiment writes to
  `docs/coordination/`.
- **Decision thresholds relaxed after seeing disappointing numbers.**
  Control: thresholds are fixed in this Gate-1-approved plan text before
  Step 2 runs.

## Rollback

Trivial — the entire harness lives under `tmp/`, is gitignored, and
touches no tracked file. Deleting the harness directory fully reverts the
experiment. No schema, rule, or role changed by this plan.

## Test and acceptance strategy

The harness itself needs no test beyond: it correctly invokes `agov
coord validate` on each candidate output and records the pass/fail; a
small hand-checked subset (5 tasks) confirms the semantic-scoring step
agrees with a human reader before scoring the full corpus. Acceptance =
governor reviews the Step-4 comparison table and the Step-5 threshold
outcome; no further Gate 2 is needed for the experiment itself since it
produces no shipped artifact — only its *findings* feed the next decision
(a live-pilot Gate 1, or closing the question for now).

## Decision requested at Gate 1

Approve (a) the offline-only scope with zero live coordination records
produced, (b) Step 0's environment-interrogation-first requirement (no
assumption of available GPU hardware), (c) the task-corpus design
(mechanical/templated only, genericized), (d) the two-part scoring
(schema validity AND semantic correctness, neither sufficient alone),
(e) the fixed decision thresholds. Owners: build-engineer (harness +
Step 0 infrastructure finding), capability-developer (Step 3-5 capability
read and report). Governor decides any follow-on live pilot separately.
