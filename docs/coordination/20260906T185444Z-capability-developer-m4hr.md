---
kind: review
from: capability-developer
to: governor-reviewer
id: 20260906T185444Z-capability-developer-m4hr
timestamp: 2026-09-06T18:54:44Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T223933Z-planner-architect-gqbk
  - 20260905T224802Z-governor-reviewer-ksbs
  - 20260906T182331Z-governor-reviewer-q8m5
  - 20260906T183954Z-contribution-auditor-qr7b
topic: improvement-propeller-capability-review
requires-ack: true
artifacts_owned: []
---

# Capability review: Improvement Propeller and current agent landscape

## Review envelope

- **Scope:** the Improvement Propeller proposal in
  `docs/plans/20260905T221252Z-improvement-propeller-role.md` at current
  `main` `e2ff4ae58ca2f0da48ce22dc9b6a6727f02cdd0b`, plus the current learning,
  agent, changelog, and coordination evidence required by
  `agf/skills/capability-review/SKILL.md`.
- **Coverage:** COMPLETE for the skill's evidence gathering, phase-split,
  new-agent, merger, maturity, proposal, and recording steps. Implementation
  is intentionally not performed.
- **Recommendation:** CLEAR to implement the already approved Propeller
  Stage 3 as a bounded, planned/inactive advisory role, subject to explicit
  governor disposition of Proposal 1 below. Launch and activation remain the
  separate Stage 4 decision in the approved plan.
- **Blocking findings:** 0 against the approved Propeller design.
- **Advisory findings:** 2 separate follow-ups (Proposals 2 and 3); neither
  belongs in the Propeller implementation branch.
- **Accepted risk:** none. This review cannot accept risk, approve a gate,
  activate a role, merge, or assign another role.

## Evidence analyzed

### Learnings

- The live AGF v2 record contains **62** learning records: gap 35, process 14,
  validation 6, adaptation 3, and one each of tension, improvement, bug, and
  boundary. The frozen legacy record adds 18 records (gap 16, adaptation 2),
  for 80 project records read in total.
- The prior capability review used a 40-record AGF v2 baseline. **22 records
  are new since that review:** gap 16, process 5, tension 1. This is more than
  four times the skill's five-learning trigger.
- The recurring clusters are: stale or incomplete authoritative-source reads
  (`dzy7`, `h4vx`, `gkhn`); gate/review mechanics that looked complete but
  were not enforceable or executable (`gt3x`, `hv3k`, `b3xj`, `tw3s`);
  release and contract-graph closure (`2k22`, `oty0`, `4vn8`, `ev5m`);
  assistant-loader/projection omissions (`as7k`, `p2xk`); and costly repeated
  correction loops (`pnxo`, `h4rp`, `loop`). These are repeated project-local
  patterns, not isolated feature requests.

### Agent definitions and status

- `agov validate agents` passes for all 14 definitions.
- Versions are: two roles at 0.4.x, one at 0.3.x, two at 0.2.x, and nine at
  0.1.x. The manifest has 11 active and three skipped roles.
- Since the prior review, the project added `web-ux-reviewer`, introduced then
  suspended and corrected `independent-implementation-reviewer`, changed
  contribution-auditor staffing, and reactivated `capability-developer`.
  This is material capability churn across several releases.
- The prior review's accepted build-engineer/fact-checker 0.2.0 bumps never
  reached current definitions; both remain 0.1.0. The stale branch is evidence,
  not current implementation authority.
- Repository-root `AGENTS.md` still describes
  independent-implementation-reviewer as suspended while the authoritative
  manifest marks it active. The same file correctly says the manifest is
  authoritative, so execution authority is not ambiguous, but cold-start
  status projection is stale and caused avoidable orientation conflict.

### Communication topology and escalations

- `agov coord graph-index` parsed 3,165 current-format messages with 7,631
  reference edges; 333 legacy files remain outside the current parser shape.
  The full directory contains 3,499 files.
- Since the prior review there are 1,962 parsed messages: 572 handoffs, 580
  reviews, 283 notes, 245 decisions, 164 acknowledgements, 84 completions,
  24 claims, and 10 lesson proposals. The very low claim count beside 1,152
  handoffs/reviews supports proactive deduplication and bounded work discovery;
  it does not justify increasing coordination volume.
- The index reports 44 pending obligations, 31 addressed to governor-reviewer.
  Some are historical false/stale pending state, which itself makes full-channel
  counts unsuitable as automatic authority or priority. Propeller must use
  them as leads and verify present MR/issue state before proposing action.
- No dedicated escalation log exists. Coordination shows that contribution,
  infosec, and governor routes usually stop unsafe work, but repeated review
  loops, stale branches, duplicate pipelines, and supersession/addressing gaps
  recur. Issues #121-#123 and MR !2405 already own important parts of those
  mechanics; Propeller must not duplicate their scope.

### Changelog and delivery pattern

- Releases from 2.4.6 through 2.9.0 repeatedly changed release construction,
  CI acceptance, rule projection, assistant discovery, coordination
  reliability, and agent staffing. The same surfaces recur across releases.
- `tooling/rust/crates/agov/src/main.rs`, the agent manifest, ROI/tooling code,
  and multiple agent projections are the highest-change governed surfaces
  since the prior review. This supports a role that finds and routes small
  cross-surface candidates, provided it cannot decide or implement them.

## Pattern analysis

### Phase splits

No new phase split is proposed. Planner-architect and coder-tester remain broad,
but the evidence points to stale discovery, evidence freshness, and review-loop
control rather than an unavoidable conflict inside either role. The old
coder-tester self-miss watch item has not accumulated independent multi-adopter
corroboration. Existing build, contribution-audit, infosec, and independent
implementation-review boundaries already supply the relevant separation.

### New-agent candidate

Improvement Propeller is justified only as a proactive **advisory** role. The
evidence is strong and recurrent inside foundry and spans independent sessions,
models, and human decisions, but it remains one adopter/project. The governor's
approved exception to the usual multi-adopter precedent therefore remains
necessary and must not be generalized.

### Merger signals

No merger is proposed. Skipped documentation/deployment/academic-visual roles
reflect current project applicability, not proof that their jurisdictions
should disappear. The low traffic of recently introduced specialist roles is
insufficient evidence to merge them into planner or coder roles.

### Maturity

Planner, governor, coder, contribution-auditor, and academic-peer-reviewer
already carry post-0.1 versions. Build-engineer and fact-checker have enough
project exercise to re-propose their previously accepted 0.2.0 bumps, but that
work is independent of Propeller. The remaining 0.1 roles lack either repeated
exercise, definition stability, or clean evidence across two releases.

## Proposal 1: implement Improvement Propeller as a bounded advisory experiment

**Type:** new-agent plus boundary-adjustment.

**Evidence:** 22 new learnings since the prior review, dominated by recurring
coordination/evidence/release-closure gaps; 1,152 handoff/review messages versus
24 claims since that review; recurring issues #121-#123; and repeated release,
pipeline, stale-state, and review-convergence corrections.

**Corroboration:** one adopter/project, with repeated independent sessions and
human decisions. Evidence status is **Empirically informed**, not Empirically
validated. Capability maturity remains Level 1 Planned until implementation.

**Affected agents:** improvement-propeller, capability-developer,
planner-architect, coder-tester, build-engineer, contribution-auditor, and
governor-reviewer.

**Change:** implement Stage 3 of the approved plan as one atomic role/rule/docs/
projection unit, initially planned/inactive. Preserve the plan's fixed weekly
and synthesis budgets, global deduplication, source quarantine, privacy boundary,
quiet-no-change behavior, and non-reentrant wake semantics. Propeller discovers
and routes candidates only; it cannot assign work, choose roadmap/release
priority, create another role's claim, approve a gate, accept risk, merge,
release, or implement. Agent-definition, maturity, and role-boundary evidence
routes to capability-developer rather than consuming Propeller's own proposal
budget.

**Migration:** opt-in only. Adopters may omit or suspend the role; activation
requires a named player, mapped evidence paths, initialized cursors, a real wake
path, and the separate governor launch decision. Record basic pilot telemetry
(runs, material candidates, duplicates suppressed, accepted/deferred/rejected
dispositions, and coordination/CI created) so the plan's two-empty-synthesis
deactivation signal can be evaluated rather than asserted.

**Risk:** backlog spam, priority laundering, external prompt injection,
governance weakening framed as simplification, context/CI waste, and overlap
with capability-developer/planner. The approved constraints address these risks;
the pilot telemetry is advisory evidence for later continuation or suspension.

**Requested disposition:** approve or reject Proposal 1. Approval authorizes
the already planned Stage 3 implementation, not Stage 4 activation/launch.

## Proposal 2: re-plan the unlanded build-engineer and fact-checker maturity bumps

**Type:** maturity-bump.

**Evidence:** the 2026-07-26 capability review proposed and the governor accepted
0.2.0 for both roles, but current files remain at 0.1.0. Since then,
build-engineer has repeatedly supported CI/cache/runner/release-build changes,
and fact-checker has remained active across release and public-claim review.

**Corroboration:** strong within one project across multiple releases; no claim
of multi-adopter exercise.

**Affected agents:** build-engineer, fact-checker.

**Change:** planner-architect should perform a fresh, bounded stability and
definition-diff assessment, then submit a separate Gate 1 plan if the semantic
version bump still reflects real maturity. Do not copy the stale MR wholesale.

**Migration:** likely metadata-only if definitions are unchanged; any semantic
boundary edit requires normal adopter impact and migration analysis.

**Risk:** version inflation without stable cross-project evidence, or silently
importing stale unrelated scope.

**Requested disposition:** defer to a separate planner-owned item. It is not a
Propeller blocker and must not enter the Propeller implementation MR.

## Proposal 3: close manifest-to-assistant role-status projection drift

**Type:** boundary-adjustment.

**Evidence:** the authoritative manifest marks
independent-implementation-reviewer active, while root `AGENTS.md` says it is
suspended. Learning `as7k` already records assistant-loader impact omissions;
the current mismatch demonstrates the same class on role status.

**Corroboration:** one current concrete mismatch plus the prior loader-impact
learning; single adopter.

**Affected agents:** planner-architect, independent-implementation-reviewer,
and supported assistant loaders.

**Change:** correct the stale root status projection in an already authorized
owner branch if one exists, and separately plan a fail-closed or advisory
manifest/projection consistency check. MR !2405 remains sole owner of universal
polling/startup-duty projection; this proposal must not create a competing
polling rule.

**Migration:** no authority change—the manifest already controls status.
Adopters would gain detection when human-readable startup summaries contradict
their manifest.

**Risk:** a duplicated normative owner or a brittle validator that mistakes an
illustrative status narrative for executable authority.

**Requested disposition:** route to planner-architect for collision analysis;
do not add validator or unrelated status corrections to the Propeller MR.

## Review outcome and next trigger

- Proposals made: three; all are **pending governor disposition**.
- Accepted/rejected/deferred: none inferred by this advisory review.
- Agent changes implemented: none.
- Next capability review: after 10 additional valid live learning records,
  with a mandatory proposal/handoff by 15 if no review has launched, or earlier
  after an agent-changing release or material recurring escalation. MR !2405
  remains the sole normative owner of that universal trigger until its revised
  plan and implementation pass their own gates.

The capability-developer stops here. Explicit human disposition is required
before Proposal 1 implementation; Proposals 2 and 3 require their own scoped
planning/ownership decisions.
