---
kind: handoff
from: capability-developer
to: planner-architect
id: 20260912T100503Z-capability-developer-r6r7
timestamp: 2026-09-12T10:05:03Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260912T095349Z-governor-reviewer-smmp
  - 20260912T100054Z-coder-tester-rkr8
topic: independent-plan-reviewer-challenge-proposal
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Challenge, improve, or reject the independent-plan-reviewer proposal below.
  must_read:
    - agf/agents/planner-architect.yaml
    - agf/agents/contribution-auditor.yaml
    - agf/agents/independent-implementation-reviewer.yaml
    - docs/coordination/20260912T100054Z-coder-tester-rkr8.md
  do_not_touch:
    - agf/agents/MANIFEST.yaml
    - feat/cpb1-multi-record-coordination-publication
    - codex/cpb2-publication-hold-plan
---

# Proposal for Claude to challenge: independent-plan-reviewer

Governor explicitly requested this exchange: "I second that role proposal.
propose it to claude to be challenged, or improved or rejected." This is
authorization to propose and consult directly, not approval to implement or
activate the role. The direct consultation is the human-requested exception
to capability-developer's default governor-only routing. Codex publishes the
proposal through the existing coordination lane; no new MR or role file.

**Type:** bounded new-role trial, with boundary-adjustment or rejection as
equally valid alternatives. **Maturity:** Level 1 Planned. **Evidence status:**
Empirically informed for the local problem; effectiveness/cost of the proposed
role is untested. **Corroboration:** one project, foundry; independent reviewers
within that project are not independent adopter corroboration.

## Problem and evidence

- CPB1-SEC-1 in the appointed security review at !2636 head
  `1da3584d3a5e89aad50b61e9e165743da87242ae`: implementation obeys the approved
  merge-base-only design, but that design misses copies introduced on an
  advanced target. Implementation conformance alone did not establish design
  adequacy. rkr8 records the finding; the full security report is forthcoming.
- Learning `20260830T085245Z-p2xk`: repeated semantically stale projection
  instructions survived plan edits and structural validation, then were found
  by independent review. Learning `20260906T104942Z-ev5m`: an approved release
  plan's broad exclusion hid mutable metadata and caused release validation
  failure. These are different instances, still from the same project.
- rgb5/ghha show a plan requiring consultations whose corrected-head ownership
  was not staffed. Review should test whether prerequisites can actually be
  fulfilled, not just whether consultation fields exist.
- Existing planner-architect authors plans/contracts. Contribution-auditor
  screens plausibility, evidence, precedent and boundary integrity but its
  reads contract does not explicitly cover exhaustive technical plan semantics.
  Independent-implementation-reviewer compares delivered code to approved plans
  at Gate 2. There is overlap, but no equally explicit pre-Gate-1 contract.

## Proposed smallest trial

Name the assignment **independent-plan-reviewer**. Primary verb: review.
It owns a read-only advisory report for one named plan revision, not the plan.
Start after a concrete draft exists and affected specialist questions can be
identified; finish before governor Gate 1 approval. Early targeted specialist
consultation may happen during drafting. Rerun after material design changes,
including approved-plan amendments, rather than treating old clearance as current.

For the trial, select up to three substantial plans by governor decision. No
universal new gate and no automatic retroactive review of the whole MR estate.
Use a distinct non-author session/model where available; disclose provenance
and any approved fallback. A reviewer must not silently review its own plan.

Review questions:
1. Would implementing this design as written satisfy the user's intended
   outcome, including explicit non-goals and success criteria?
2. Are state transitions, conflicting inputs, failures, concurrency, target
   drift, authority and trust boundaries defined and coherent?
3. Are dependencies, artifact owners, migration/rollback and specialist
   consultations complete and staffed within existing authority?
4. Do acceptance tests distinguish correct behavior from plausible shortcuts,
   and do projections describe the same final contract?
5. Is there a materially simpler design that meets the same requirements?

The reviewer requests named security/build/domain input when needed; it does
not impersonate those specialists or treat their narrow reports as whole-plan
clearance. It cannot edit the plan, approve a gate, activate a role, merge,
release, enlarge scope, or replace the human's product intent with its own.

Report: exact plan commit/blob and dependent artifacts; author/reviewer
provenance; scope/coverage; consolidated findings with concrete counterexamples
and resolution conditions; advisory CLEAR/BLOCK and unresolved uncertainty.
Incomplete coverage cannot be CLEAR. CLEAR is evidence for the governor,
not permission to implement. After corrections, inspect the delta and affected
dependencies and reconcile all previous findings, widening the pass when needed.

## Footprint, migration, and risks

Target one consolidated report per round using existing Git coordination and
MR bridges. Reuse deterministic validation and specialist evidence; do not
repeat full context ingestion or create status-only MRs. Record input/output
tokens where available, elapsed time, review rounds, actionable findings,
false positives, and post-approval design defects/rework. Do not claim causal
savings from three plans; compare observed cost and quality with a disclosed
baseline and retain uncertainty.

Migration for trial: no adopter change, no permanent manifest entry, no new
service. A later approved standing-role plan would own the role YAML,
communication symmetry, manifest/loader/docs projections and lifecycle.
Risks: redundant auditing, another queue, false confidence, excessive review
latency, reviewer redesigning instead of reviewing, and weak independence.
Reject or narrow the trial if it mostly duplicates existing reports or costs
more than the value of unique defects caught; keep human risk judgment explicit.

## Claude: please challenge the proposal, not merely acknowledge it

Compare: (A) bounded new reviewer role; (B) explicit independent plan-review
assignment under an existing role with clarified boundaries; (C) no new role,
repair existing consultation/execution discipline. Which is the minimum change
that solves the evidenced gap? Identify the strongest objection, missing
scope, duplicated duties, staffing constraints, proposed activation triggers,
stop/retirement criteria, and cheaper alternatives. Recommend KEEP / REVISE /
REJECT with reasons and concrete edits. Rejection is a successful consultation.

Please respond on this shared branch (fetch/ff first) or an existing MR bridge,
without opening a new coordination MR or delaying current estate corrections.
This consultation may use your planner-architect judgment; it is a challenge
to a Codex-authored proposal, not an instruction to supply formal gate approval.

## Evidence accounting and next step

Scoped human-triggered analysis, not a completed periodic capability review.
No learning baseline/cursor is reset. Main was 4fe0f389. Parsed category-field
counts (not independently revalidated learning counts): bootstrap foundry 18,
product foundry 70, harbor-workshop 1. No matching independent-plan-reviewer
definition/plan/proposal found on main. Agent validation passed for 16 role
definitions; versions: ten 0.1.0, three 0.2.0, one 0.3.2, two 0.4.0.
Recent role changes include estate-steward and Propeller activation; this
proposal must justify additional footprint rather than infer need from growth.

Proposal disposition: submitted for challenge. No definitions, rules or plans
changed. After Claude's response, synthesize one revised recommendation for
governor decision. Do not add this proposal as a RUW release prerequisite.
