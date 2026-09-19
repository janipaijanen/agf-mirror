---
id: 20260919T064133Z-issue134-slice-a-serial-role-pilot
status: pending-gate1
topic: issue134-slice-a-manual-capacity-one-pilot
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Ran `agov validate infosec`: OK, no findings. This plan proposes
      no code and authorizes no execution (Step 0 found the required
      hardware absent -- see below); the security-relevant content is
      the negative-scenario list itself (prompt injection, secrets in
      handoffs, forged provenance), which is carried verbatim from the
      already-accepted proposal, not new exposure introduced here.
  - role: contribution-auditor
    status: pending
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: issue #134 Slice A — manual capacity-one serial-role pilot

**Depends on:** #131 (closed Level 5) satisfies RUW's precondition for
this post-RUW work. Accepted design direction: MR !2975,
`docs/proposals/serial-role-execution-on-capacity-constrained-runtimes.md`,
independent review `20260918T151040Z-planner-architect-bemh` (CLEAR),
governor decision `20260918T151813Z-governor-reviewer-jk3j` (confirms
the #103 extension/child disposition). This plan is filed per handoff
`20260919T061216Z-coder-tester-d4pj`.

**Note on a stale citation upstream of this plan:** the proposal's
"Question and verdict" section cites
`20260918T150612Z-planner-architect-qs8r` as "Claude's no-addition
response." That record was independently corrected the same day
(`20260918T151040Z-planner-architect-bemh`, `qs8r` does not reflect an
actual Claude assessment) — the correction itself is what produced the
real review (`bemh`) this plan's own dependency chain relies on. This
plan does not rely on `qs8r`'s content, only on `bemh`'s and `jk3j`'s.
Flagged here rather than silently relied upon; fixing the proposal
document's own stale citation is a separate small edit, out of this
plan's scope.

## Intent

Define the bounded Gate 1 contract for issue #134's Slice A: the
smallest reversible empirical trial of serial governed role execution
on a runtime that can keep only one model resident at a time. This
plan does **not** itself authorize execution — see "Step 0" below,
which found the required hardware absent from every environment this
session has access to. It defines the exact procedure, evidence
schema, negative-scenario checklist, and decision thresholds so that
execution can proceed the moment a named operator supplies the missing
precondition, without a second planning round.

This plan changes no schema, rule, tool, or role definition. `agov`
gains no new command. No AGF scheduler, GPU daemon, runtime mutex,
downloader, or hidden-state snapshot mechanism is proposed (explicitly
forbidden by the issue and the proposal's "Post-RUW pilot" step 1 and
"Slice B" text).

## Step 0 — environment interrogation (required before any execution)

Per `environment-interrogation-rule.md` and the quantized-model-pilot
precedent (`docs/plans/20260911T044602Z-quantized-model-mechanical-drafting-pilot.md`,
which found the same gap for a lighter-weight offline harness): checked
this session's own environment for the required capacity-one,
multi-model-swap hardware.

- `nvidia-smi`, `/dev/nvidia*`: absent.
- Local inference stack (`ollama`, `llama-cpp`): absent.
- Available memory: 3.8GB total, no swap — well below what even a
  small quantized model plus a host OS needs, let alone sequential
  planner/coder/reviewer model swaps.
- This is the sandbox this Claude session runs in. Codex's own runtime
  environment has not been independently confirmed here, but Slice A's
  design (issue text: "On one capacity-one machine... a pinned planner
  model in a fresh role session... a pinned coder model... a distinct
  reviewer model in a fresh session") describes real local model
  hosting with actual load/unload cycles — categorically different
  from either assistant session's own cloud-hosted execution, where
  "the model" is the session's own backing model, not a locally loaded
  and unloaded artifact.

**Finding:** no environment this plan can currently name meets Slice
A's hardware precondition. Per the quantized-pilot precedent's own
Step 5 handling of this exact case: **report the infrastructure gap;
the capability question stays unanswered, not answered negatively.**
This plan does not fabricate or assume hardware access to make
progress look faster than it is.

**What this means for Gate 1:** this plan requests approval of the
*procedure and evidence contract* below (Steps 1-6), not of execution.
Execution remains blocked until the governor names an actual operator
and machine meeting the precondition — their own hardware, a
volunteering adopter, or dedicated infrastructure the project decides
to acquire. That naming is a separate governor decision this plan does
not make.

## Scope — Slice A only

Slice B (conditional adapter assessment, second runtime shape) is
explicitly out of scope, per the issue text: "Only if Slice A
demonstrates value and stable cross-runtime fields." This plan does not
pre-authorize Slice B.

**In scope:** the manual, no-code, single-machine pilot exactly as
specified by issue #134's "Slice A" section and the proposal's
"Post-RUW pilot" section (both read together below — the issue is the
governed scope statement; the proposal supplies the negative-scenario
detail the issue references by cross-section, not by duplicating it).

**Out of scope, explicitly:**

- Any schema change, new `agov` command, or role/rule modification.
- Widening MR !2689's approved quantized mechanical-drafting pilot
  (issue text, explicit).
- Widening issue #121's concurrent same-role session addressing (this
  profile is serial, one model resident at a time, never concurrent).
- Any AGF-owned scheduler, GPU daemon, runtime mutex, model downloader,
  or hidden-state snapshot mechanism (issue text and proposal, explicit).
- A live pilot on the real coordination channel producing governed
  state before Step 1 below is satisfied. Checkpoints published during
  the pilot are real coordination records (unlike the quantized
  pilot's `tmp/`-only harness) — see "Coordination-record boundary"
  under Step 2.

## Step 1 — precondition: name the operator and machine

Before any execution step below runs, the governor names, in a
separate coordination decision: the exact machine (or the exact
adopter/session volunteering one), its actual VRAM/RAM class, and the
exact pinned models intended for the planner/coder/reviewer roles.
This plan does not select models or hardware — Slice A's own text
requires "pinned" artifacts, and pinning requires a real target
environment to pin against.

## Step 2 — the pilot procedure (carried from issue #134 / proposal, unchanged)

1. Use a pinned planner model in a fresh role session.
2. Publish a bounded durable checkpoint, then unload it.
3. Load a pinned coder model and resume only from governed shared
   evidence (not private scratch state carried over from the planner
   session).
4. Publish/checkpoint the exact implementation state, then unload it.
5. Load a distinct reviewer model in a fresh session and review the
   exact head without inheriting private author scratch state.
6. Keep human Gate 1 and Gate 2 authority unchanged throughout — no
   step in this procedure grants, approves, or bypasses a gate.

**Coordination-record boundary.** Unlike the quantized-model pilot
(offline, `tmp/`-only, zero live records), this pilot's checkpoints
and handoffs ARE real coordination records by design — the pilot is
testing whether AGF's actual handoff/checkpoint mechanism supports
this pattern, not a synthetic harness. Each checkpoint/handoff record
published during the pilot MUST:

- be a normal, schema-valid coordination record (no special
  pilot-only format);
- state plainly in its body that it is part of the issue #134 Slice A
  pilot, so a reader encountering it later understands its context
  without needing this plan;
- reference this plan and the pilot's own claim record; and
- not itself claim any Gate 1/Gate 2 decision, role activation, or
  authority beyond what its kind normally carries.

This keeps the pilot's own evidence trustworthy (real records, not a
sandboxed simulation) while ensuring nothing it produces is mistaken
for more authority than a normal coordination record has.

## Step 3 — required measurements (carried from issue #134, unchanged)

Record, per role transition: exact hardware; model/runtime/tokenizer/
configuration digests; context length; load and unload latency; peak
host RAM/VRAM; wall time; crash-recovery result (if exercised); output
result; and governance/protocol violations. **Measure output quality
separately from governance conformance** — a governance violation with
good output, and clean governance with poor output, are different
findings and must not be collapsed into one score.

## Step 4 — required negative scenarios (carried verbatim from the proposal)

All of the following must be exercised, or explicitly marked
not-exercised-and-why in the pilot's completion record — silently
skipping one is not an acceptable disposition:

1. Two launchers attempt capacity-one acquisition; exactly one
   succeeds.
2. An old worker resumes after lease expiry; fencing rejects its
   mutation.
3. A process crashes before checkpoint and after checkpoint but before
   the next model loads; recovery truth differs and no lost work is
   invented.
4. Unload reports success while a process or cache remains; the next
   activation is blocked.
5. A model or handoff forges its role or provenance; runtime evidence
   wins and authority remains unchanged.
6. The same model/session is relabelled as reviewer; independence is
   rejected.
7. The same base weights under another alias or quantization are
   presented as a different model; the result is rejected or recorded
   as unknown according to the approved independence policy.
8. A checkpoint has a changed SHA, omitted dirty file, local-only
   commit, or failed integrity check; resume remains incomplete.
9. A handoff contains prompt injection or a secret; it remains
   untrusted and is not copied into privileged instructions or durable
   logs.
10. A Git claim and runtime lease disagree; execution fails closed and
    requires reconciliation.
11. A floating model tag or digest mismatch is detected; activation is
    denied by the downstream runtime.
12. Repeated switching starves higher-priority ready work; the run
    measures and reports the behavior without overriding governed
    priority.
13. The human Gate is absent; queue or runtime success cannot
    substitute for it.

Since this pilot has no adapter and no automated fencing mechanism
(Slice A is manual, no-code, per issue text), several of these
scenarios (1, 2, 7, 10, 11) are exercised as **manual procedural
tests** (the human operator deliberately attempts the forbidden action
and records whether the manual process — not software — catches it),
not as automated enforcement tests. State explicitly, per scenario,
whether it was exercised as an automated check, a manual procedural
test, or not-exercised-and-why.

## Step 5 — decision thresholds (stated now, before any result exists)

Per the proposal's "Decision points after the pilot," carried
unchanged:

- **Manual checkpoints and current handoff/session-continuity
  mechanisms prove sufficient** → publish the adopter recipe (extends
  issue #103) and stop. No further slices, no new schema.
- **Stable, cross-runtime-portable fields emerge from Slice A alone**
  (still requires a second runtime shape before treating the envelope
  as portable, per the proposal — that confirmation is Slice B's job,
  not this plan's) → the Slice A completion record names them as
  candidates; no schema or adapter is proposed by this plan.
- **High swap cost, state leakage, unreliable fencing, supply-chain
  burden, or no measurable governance benefit** → keep the scenario
  downstream; record the failed assumptions; do not build the
  mechanism.
- **A single successful run** does not establish a universal hardware
  or performance claim (proposal, explicit) — the completion record
  states this limit regardless of how clean the result looks.

Any decision to proceed toward Slice B, an additive schema, or an
optional adapter contract is a **separate future Gate 1**, per the
proposal's own text ("Only then consider a Gate 1 plan..."). This plan
does not pre-approve that step.

## Risks and controls

- **Hardware/infrastructure gap treated as a soft blocker and
  quietly skipped.** Control: Step 0 above states the finding plainly
  and Step 1 makes operator/machine naming an explicit precondition,
  not an implementation detail glossed over.
- **Pilot checkpoints mistaken for a live governance decision or role
  activation.** Control: Step 2's coordination-record boundary
  requires each record to self-identify as pilot content and forbids
  it from claiming gate/role authority.
- **A single clean run over-generalized into a hardware or performance
  guarantee.** Control: Step 5's explicit no-generalization statement,
  required in the completion record regardless of outcome.
- **Negative scenarios silently skipped because Slice A has no
  automated fencing to exercise them against.** Control: Step 4
  requires an explicit per-scenario disposition (automated / manual
  procedural / not-exercised-and-why), not a blanket "N/A."
- **Scope creep into Slice B or an adapter design mid-pilot.**
  Control: Step 5's explicit "separate future Gate 1" requirement for
  anything beyond Slice A's own completion record.

## Rollback

Trivial at the plan level: this plan authorizes no execution while
Step 0's blocker stands, so there is nothing to roll back yet. Once
Step 1's precondition is satisfied and execution begins, rollback is
whatever the downstream operator's own environment supports (deleting
local model artifacts, discarding the manual pilot's working state);
AGF-side rollback is limited to superseding the pilot's coordination
records the normal append-only way (a new record stating the pilot
was abandoned or its findings superseded) — no schema, rule, or role
change is made by this plan to roll back.

## Test and acceptance strategy

No automated test is proposed — Slice A is a manual, no-code pilot by
design (issue text, explicit). Acceptance is: the governor reviews the
Step 4 negative-scenario disposition table and the Step 3 measurements
in the pilot's completion record, confirms Step 5's thresholds were
applied without post-hoc adjustment, and decides whether to close at
the adopter recipe, greenlight Slice B, or keep the scenario
downstream. No further Gate 2 is needed for this plan itself since it
authorizes no shipped artifact — only the eventual pilot completion
record and any follow-on Gate 1 it recommends.

## Role and gate boundary

planner-architect (this session) files this plan per the `d4pj`
handoff. Execution, once Step 1's precondition is satisfied, belongs
to whichever role/operator the governor names as owning the actual
machine — likely coder-tester for the governed-record side and the
named downstream operator for the runtime side, per the proposal's
Responsibility boundary table. capability-developer reads the Step 5
outcome for any maturity-level implication. Gate 1 and Gate 2 approval
remain governor-reviewer's sole authority throughout. No release, tag,
or deploy authority is requested or implied.

## Decision requested at Gate 1

Approve: (a) the Step 0 environment-interrogation finding and its
consequence (procedure approved, execution blocked pending Step 1);
(b) the Slice-A-only scope boundary; (c) the coordination-record
boundary in Step 2; (d) the unchanged negative-scenario list in Step 4
and its per-scenario automated/manual/not-exercised disposition
requirement; (e) the fixed decision thresholds in Step 5. Separately:
name the operator and machine satisfying Step 1's precondition, or
explicitly park this pilot pending one becoming available.
