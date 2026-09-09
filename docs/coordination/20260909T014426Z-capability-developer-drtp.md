---
kind: note
from: capability-developer
to: governor-reviewer
id: 20260909T014426Z-capability-developer-drtp
timestamp: 2026-09-09T01:44:26Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T013812Z-capability-developer-jprj.md
  - 20260908T183242Z-planner-architect-ds4k.md
  - 20260906T191236Z-governor-reviewer-trd6.md
topic: improvement-propeller-advancement-steps
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Improvement Propeller — proposed advancement steps

Capability-developer proposal at governor direction (2026-09-09, "propose
propeller advancement steps"). Agent phase/maturity/manifest evidence is
capability-developer routing per `improvement-propeller-rule.md`. **Advisory
only** — proposes an ordered path; assigns nothing, approves no gate,
changes no manifest, activates nothing. Each step returns for its own
binding decision.

## Baseline (measured `main` `f0805cd8`, 2026-09-09)

| Property | Value |
|---|---|
| Agent def | `improvement-propeller.yaml` — `version 0.1.0`, `phase 4`, `framework_version 2.10.0` |
| Rule | `improvement-propeller-rule.md` — `strength: MUST`, `status: active` (every operative clause conditioned on activation) |
| Manifest | `improvement-propeller: status: planned` |
| Maturity | Level 4 Validated — shipped in v2.10 (RUW-3), deliberately inactive |
| Wake mechanism | none armed |
| Cursor / state | none initialized |
| Owning issues | #131 (activation + first genuine wake), #132 (skill export) |

The rule states plainly: "Without [the activation] decision the role remains
planned/inactive. A manual invocation may validate behavior but is not
evidence of a scheduled Level 5 deployment." `jprj` (this session's
coordination-protocol analysis) is exactly such a manual invocation of the
slow-governance-friction lane — it validates that the *method* works on
live data; it is not activation and not Level 5 evidence.

## Proposed steps

### P-0 — fix the activation-contract inputs (governor decision; blocks P-2+)

The rule requires a governor decision naming: the player and its active
project role; project-local equivalents of sources / radar / roadmap /
issues+MRs / release evidence / coordination / claims; a real supported
wake mechanism and cadence (or the truthful absence of one); an ignored
cursor/state path; and the starting cursors (last capability-review
learning cursor+count, coordination cursor, last weekly run, last
synthesis/release baseline). Codex proposed most of these in the Stage-4
parameters exchange (`gakp` / `c4mt` era). P-0 is the governor confirming
or revising that parameter set.
- **Owns:** #131 Gate 1.
- **Maturity:** no change (still Level 4).
- **Needs:** governor decision-record.

### P-1 — attended full-cycle dry-run (behavior validation, no manifest flip)

The named player runs **one complete Propeller cycle** — both lanes — over
the real configured evidence surfaces, producing a genuine deduplicated
candidate set (≤3 opportunity + ≤3 friction) with every required per-item
field, plus a self-check transcript showing it touched nothing forbidden
(no assign / prioritize-as-directive / implement / approve / merge / claim
/ URL-dereference). Explicitly labelled a dry-run.
- **Owns:** #131 (evidence step within its Gate 1 scope).
- **Maturity:** Level 4 → Level 4 with attended-cycle evidence.
- **Needs:** nothing new to *start* (the rule permits a manual/human-
  requested cycle); the output is Gate-1 evidence, not a gate.
- **Rationale:** proves the full cycle, not just one lane (`jprj` covered
  one lane on one surface), before any manifest change.

### P-2 — manifest activation + first genuine scheduled wake (this is #131's core; Level 4 → Level 5)

Only after P-0 + P-1: governor approves the manifest status change
(`improvement-propeller: status: planned → active` — an agent status
change, governor-approval-required per AGENTS.md). Initialize the
cursor/state path at the P-0 baselines. Arm the real wake mechanism (the
proposed weekly Mon 09:00 UTC heartbeat). Then **wait for the first
unattended wake to fire and produce a candidate set with no human in the
loop.** That unprompted firing is the Level 5 Deployable proof — a manual
invocation dressed as a wake does not count (rule, verbatim).
- **Owns:** #131.
- **Maturity:** Level 4 → Level 5 (on the genuine first wake).
- **Needs:** governor manifest-status approval + confirmation the wake
  mechanism is real and supported.

### P-3 — sustained-operation confirmation (Level 5 is not a one-shot)

Two to three consecutive genuine weekly wakes, each producing a bounded
deduplicated set, each self-verifying it stayed inside the contract, none
creating a control-weakening or priority-setting action, overflow held in
ignored local state (never an issue dump). Observation, not a gate.
- **Owns:** #131 closure evidence.
- **Maturity:** Level 5 confirmed.
- **Needs:** nothing — it is elapsed-time evidence.

### P-4 — export the method as a skill (#132; gated on P-3)

After sustained genuine operation: export the Propeller *method* as a
shipped skill (`agf/skills/improvement-propeller/` or similar) so a
governor/operator can invoke assess-and-propose on demand for a specific
concept or content and feed the result back to AGF — the use the governor
described earlier ("a skill which I can use to assess certain concepts,
contents, and feed them back"). Its own Gate 1.
- **Owns:** #132.
- **Maturity:** new skill artifact at its own maturity level.
- **Needs:** its own Gate 1; **must not start before P-3** — exporting a
  method that has not demonstrated sustained safe unattended operation is
  premature.

### P-5 — version + maturity-claim housekeeping (folded; after P-2 and again after P-3)

Bump `improvement-propeller.yaml` `version` from `0.1.0` to reflect
"activated"; update the queue-convergence roadmap owner
(`docs/roadmap.md`, once B1 lands) and `agf/docs/site/roadmap.md` from
"Level 4 Validated, inactive" to the real state. Fold into P-2's / P-3's
implementation MR rather than a standalone plan.

## Safety invariants across all steps

- No step self-approves. P-0/P-1(evidence)/P-2/P-4 each need a governor
  Gate 1 or manifest-status decision; P-1 and P-3 outputs are evidence, not
  gates.
- P-1..P-3 cycles run **only** under the named player, never a borrowed
  role.
- Any Propeller candidate that would weaken a control routes to
  contribution-auditor before the governor (existing escalation trigger,
  unchanged).
- The universal 10-to-15-learning capability-review trigger stays owned by
  `universal-polling-and-autonomous-continuation-rule.md`; the Propeller
  consumes it, never redefines it.
- Propeller cannot count its own repeated statements as corroboration and
  cannot review or approve its own output (rule, verbatim).

## Sequencing summary

`P-0 → P-1 → P-2 → P-3 → P-4`, with `P-5` folded into P-2/P-3.
#131 spans P-0..P-3; #132 is P-4. Nothing here is time-critical against the
qbka / #129 lifecycle freeze — the Propeller path is independent of that
sequence.

This proposal proposes and routes only. The governor decides whether and
when P-0 begins.
