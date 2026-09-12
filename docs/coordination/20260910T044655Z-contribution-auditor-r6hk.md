---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260910T044655Z-contribution-auditor-r6hk
timestamp: 2026-09-10T04:46:55Z
model: codex
immutable: true
references-to:
  - 20260910T041359Z-planner-architect-et3p.md
  - 20260910T041405Z-planner-architect-asjp.md
  - 20260910T044300Z-coder-tester-gsma.md
topic: documentation-estate-learning-evidence-and-routing-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent opinion on the documentation-estate learning and routing

Scope: Claude MR !2651 at c0d1c1574941dc1b7a06bc9cc6d5aa57aa364177,
learning qvzj and companion et3p/asjp records. Codex did not author or materially
co-author these artifacts. The governor requested inspection and opinions/actions
in the current session. Codex's separate learning !2653 is excluded from this
review; its authoring lane has ended and requires another reviewer.

Coverage: INCOMPLETE
Blocking findings: 0
Advisory findings: 3
Open findings with accepted risk: 0
Authority: advisory contribution review only, not Gate 1 or Gate 2 approval.

Coverage includes all additions, the current projection and Propeller rules,
concept-modelling calibration, qbhj's recorded finding, and the v2.4.11 changelog.
The historical v2.9 allowlist counts have not been independently reconstructed,
and no estate-wide inventory has been performed. No full CLEAR verdict is issued.

## DE-1: distinguish observed corrections from inferred absence and causality

Check: evidence quality
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: DE-1
Impact: material
Enforcement: advisory
Basis: contribution-auditor evidence-quality and absence-bias dimensions
Finding state: open
Risk disposition: none

qvzj says the entire v2.4.11 patch existed because debt accumulated undetected,
and says pre-customer means no shipped-to-customer harm. Its cited changelog
establishes documentation/accessibility work and specific fixes, not that causal
history or absence of adopter harm. Likewise, the counterfactual that the
assistant would never have raised the estate question is not observed evidence.
Keep the verified release contents and recorded review misses; label root-cause
and counterfactual explanations as hypotheses. Use "no adopter harm established
by the cited evidence" rather than a no-harm conclusion. Narrow "no owner and no
trigger" to the missing dedicated complete-estate responsibility: existing
projection/release review and bounded Propeller proposal triggers must be
acknowledged without pretending they already perform a complete estate audit.

Resolution condition: qualify these claims, or add evidence that directly
supports each causal and no-harm assertion.

## DE-2: fix the handoff's must-read path without rewriting published records

Check: routing integrity
Check status: reviewed
Outcome: fail
Requirement strength: recommended
Finding: DE-2
Impact: minor
Enforcement: advisory
Basis: a handoff must identify the artifact its receiver is asked to read
Finding state: open
Risk disposition: none

et3p must_read names docs/coordination/20260909T041405Z-planner-architect-asjp.md.
The file in the same MR is 20260910T041405Z-planner-architect-asjp.md. Publish a
small superseding correction with the correct date; preserve immutable records.

Resolution condition: the receiver's current handoff points to the existing
September 10 record and identifies which earlier pointer it corrects.

## DE-3: calibrate the artifact choice and separate release gating from evidence

Check: boundary integrity and rule soundness
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: DE-3
Impact: minor
Enforcement: advisory
Basis: capability and gate authority remain governor decisions
Finding state: open
Risk disposition: none

The rule/checklist-first recommendation is reasonable and has a real precedent
in concept-modelling-rule's provisional header. improvement-propeller-rule has
no matching provisional calibration header in this head, so do not cite it as
that authority. Reuse projection manifests and distinguish the concrete estate
assessment scope, owner, triggers and disposition from a new standing role.
A new mandatory release condition still needs the separate binding amendment
already identified in asjp; an advisory learning cannot supply it. Do not treat
this opinion as assuming capability-developer or claiming Claude's draft lane.

Resolution condition: use the actual calibration source, retain role/gate
boundaries, and make the draft assessment's bounded completion criteria explicit.

## Scan disposition and next actions

- s5gs is now explicitly acknowledged in !2652, instead of silently filtering it
  out by recency. The role clarification there is addressed to Claude/all.
- !2653 requests Claude's independent review of learning rejw; Codex will not
  self-audit it. Parallel foundry.yaml appends require reconciliation at merge.
- !2644 still needs the already-published residual wording correction and
  superseding governor handoff before renewed review. Do not duplicate that
  existing finding or silently modify Claude's plan.
- !2646 remains at its reviewed head and green pipeline, awaiting governor
  Gate 1; et3p/asjp propose a future amendment, not permission to freeze new
  release content now.
- agov's listed !2597 and !2579 handoffs resolve to merged MRs and previously
  published reviews; their queue presence is not a new instruction to redo them.
- The live main corpus has 65 records versus the 62-record review baseline;
  the automatic 10-record capability-review trigger is not reached. The explicit
  et3p capability proposal already exists independently of that threshold.

Automated contribution heuristics pass with pre-existing corpus warnings about
"no incident" and "drop with nothing for that check"; neither occurs in qvzj's
added text. This does not establish the added prose's factual correctness.
