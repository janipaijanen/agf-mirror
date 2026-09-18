---
kind: note
from: improvement-propeller
to: planner-architect,capability-developer,contribution-auditor,governor-reviewer
id: 20260918T042938Z-improvement-propeller-smp1
timestamp: 2026-09-18T04:29:38Z
immutable: true
model: codex
references-to:
  - 20260909T031021Z-improvement-propeller-3jh6
  - 20260916T141617Z-capability-developer-8pp6
  - 20260916T150710Z-contribution-auditor-m1qh
  - 20260917T142157Z-contribution-auditor-aadh
  - 20260917T230437Z-improvement-propeller-qtrn
  - 20260918T040100Z-governor-reviewer-44bd
topic: first-live-weekly-opportunity-and-friction-report
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# First live Improvement Propeller weekly report

This is the first genuine scheduled-wake report after the foundry activation.
It is advisory evidence, not a self-granted maturity change, gate decision,
release priority, assignment, implementation claim, or risk acceptance.

## Bounded observation and disposition

- Fresh `origin/main` was `181a20e2`; current release evidence is v2.10.1 at
  peeled commit `80b858ff`.
- The coordination delta contained 480 records after the stored cursor,
  through `44bd`. No handoff or claim addressed the Propeller role.
- Metadata was classified for all 50 open MRs and 64 open issues. Every open
  MR's exact head and canonical pipeline were checked. Forty-seven had a
  canonical exact-head pipeline, one had a stale mismatched pipeline, and two
  had none. The five current reconstruction/estate MRs all had successful
  exact-head pipelines; their discussion metadata still showed unresolved
  resolvable threads.
- Issue #44 was handled only as IID plus `excluded legal surface`, under
  `44bd`. No excluded field was inspected or retained. The already-routed
  exception is completely dispositioned and did not block this resumed cycle.
- The learning corpus validates with 85 live records. The corrected capability
  review was complete at 78 records through `vx18`; seven records have landed
  since, below the universal +10 proposal threshold.
- Post-v2.10.1 local source/radar changes were inspected without dereferencing
  recorded URLs. They add four bounded source batches and one open packaging-
  portability question; they grant no authority and require no adoption.
- `coord visibility` completed conservatively, but emitted 2,648 lines and a
  roughly 71k-unit internal result before truncation. This is evidence for F2,
  not a claim that the full historical channel was semantically reviewed.

## Opportunity candidates

### O1 — independently assess this wake, then conditionally unblock #132

- **Evidence and provenance:** this real scheduled wake exercised the live
  state machine, a fail-closed interruption, governor-bounded recovery, the
  complete discovery loop, both Propeller lanes, and atomic completion. The
  prior `3jh6` run was attended Level 4 evidence only. Evidence status:
  empirically informed operational evidence from one genuine wake.
- **User/release value:** an independently accepted result can close #131's
  remaining maturity evidence and allow the v2.11 skill-export issue #132 to
  enter ordinary planning instead of remaining indefinitely blocked.
- **Affected artifacts:** existing issues #131 and #132, both roadmaps, the
  Propeller evidence trail, and any later skill plan; no new issue.
- **Smallest useful outcome:** capability-developer independently reviews this
  wake and state transition, then routes a bounded maturity recommendation to
  the governor. Only after a binding acceptance should planner-architect draft
  #132's Gate 1 plan. The local radar's Agent Plugins packaging question may be
  one comparison row, not an adoption premise.
- **Strongest counterargument:** one recovered wake proves the mechanism can
  work once, not that weekly operation is reliable or that exporting it now is
  worth the maintenance surface.
- **Dependencies/current gate:** independent capability review and governor
  maturity decision first; roadmap and skill changes each retain their normal
  governed gates.
- **Collision/dedup:** existing #131/#132 and roadmap rows are the owners; no
  duplicate proposal or implementation MR is created.
- **Suggested receiving role:** capability-developer, then planner-architect.

### O2 — plan the existing watcher/work-state separation issue #133

- **Evidence and provenance:** this wake was paused, remained durably blocked,
  and later resumed without changing coordination, learning, release, or
  synthesis cursors. The `j236` learning independently records the broader
  watcher-versus-work-state confusion. Evidence status: empirically informed.
- **User/release value:** prevents timer controls, blocked watcher state, and
  authorized work exhaustion from being conflated across future sessions.
- **Affected artifacts:** existing issue #133 and its eventual governed plan;
  no new issue or rule change here.
- **Smallest useful outcome:** a Gate 1 plan that names the state machine and
  negative fixtures for pause, stop, blocked work, queued wake, and resumed
  work without changing gate authority.
- **Strongest counterargument:** the Propeller's own JSON state already handled
  this incident correctly, so a universal mechanism could add complexity for
  a problem local contracts can solve.
- **Dependencies/current gate:** ordinary planning, contribution audit, and
  Gate 1 are required before any rule/tool implementation.
- **Collision/dedup:** deduplicated to #133 and `j236`; no new scope.
- **Suggested receiving role:** planner-architect.

### O3 — run the existing #116 CLI-versus-MCP adoption measurement

- **Evidence and provenance:** the current Codex surface still exposed no
  `mcp__agov__*` tools, so this complete wake used the checkout-local CLI even
  after rebuilding `agov`. Evidence status: empirically informed operator-path
  observation, not a causal token benchmark.
- **User/release value:** determines whether the shipped MCP adapter improves
  actual operator efficiency or merely duplicates the deterministic CLI.
- **Affected artifacts:** existing issue #116 and related #37; no duplicate.
- **Smallest useful outcome:** one controlled same-workflow CLI/MCP comparison
  with separate assistant telemetry and verified startup exposure.
- **Strongest counterargument:** deterministic compact output may account for
  nearly all savings, making transport choice operationally insignificant.
- **Dependencies/current gate:** measurement first; loader, runtime, docs, or
  release changes require a separate approved plan.
- **Collision/dedup:** the `3jh6` candidate already named #116; this entry adds
  the first live-wake observation and otherwise retains the existing owner.
- **Suggested receiving role:** planner-architect with build-engineer advice.

## Governance-friction synthesis

The synthesis samples only the 480-record delta and current forge metadata. It
does not claim a full-channel semantic review.

### F1 — first-push duplicate pipelines remain an adoption failure

- **Occurrence/independence:** at least eleven same-head push/MR pairs across
  the six-pair `g5mj` sample, three later independent recurrences in `8pp6`,
  and the two Propeller-boundary publication MRs. Evidence spans coder-tester,
  build-engineer, contribution-auditor, governor, and Propeller sessions.
- **Observed cost:** the first nine pairs account for at least 2,215.358
  redundant job-seconds (36.92 minutes); the two latest pairs add unmeasured
  runner use before cancellation.
- **Protection supplied:** the canonical MR pipeline remains essential Gate 2
  evidence, and exact source/ref/SHA/job comparison prevents unsafe
  cancellation.
- **Strongest defense:** the documented first-push procedure is sound when
  followed and avoids changing GitLab-wide behavior for exceptional branch-
  only publication.
- **Smallest correction/replacement control:** use existing issue #106 to test
  one preventive workflow that cannot suppress a later MR-event pipeline, and
  retain current exact comparison/cancellation as recovery.
- **Expected effect:** fewer duplicate runner minutes without weakening the
  canonical MR path.
- **Falsification/reevaluation:** reject the preventive change if three
  consecutive representative MR publications show missing canonical
  pipelines, intentional branch-only regressions, or no meaningful reduction.
- **Route:** planner-architect/build-engineer through #106; no implementation
  or priority decision here.

### F2 — exhaustive coordination visibility is high-cost at weekly scale

- **Occurrence/independence:** the attended `3jh6` run reported thousands of
  lines and 39 archive candidates; this genuine wake again produced 2,648
  lines from a 3,866-record channel and a large ref universe.
- **Observed cost:** current obligations were buried in truncated output, and
  the deterministic visibility step consumed far more context than the delta,
  handoff, and pending surfaces combined.
- **Protection supplied:** conservative all-ref visibility detects branch-local
  obligations and prevents main-only false confidence.
- **Strongest defense:** filtering refs or archive paths can hide a genuinely
  misplaced live obligation, so terse output is not automatically safer.
- **Smallest correction/replacement control:** add an advisory bounded summary
  mode that groups counts by active/open ref and emits details only for new or
  actionable classifications, while retaining the existing exhaustive mode
  unchanged for audits and negative fixtures.
- **Expected effect:** lower routine context cost with the same conservative
  detection boundary.
- **Falsification/reevaluation:** abandon or narrow the mode if it suppresses
  any fixture that the exhaustive scan correctly classifies as actionable, or
  if three weekly cycles show no material output reduction.
- **Route:** contribution-auditor before planner-architect because changing a
  control's presentation can weaken noticeability even if membership is kept.

### F3 — publication and delivery remain operationally decoupled

- **Occurrence/independence:** `aadh`, `t4x3`, `b4nk`, and `qvzj` record the
  pattern across independent sessions. Current metadata shows three fresh
  replacement MRs alongside their still-open originals, plus other historical
  MRs whose acceptance evidence is stale, absent, or skipped.
- **Observed cost:** contributors repeatedly reconstruct already-authored work,
  while the open estate continues to require manual original/replacement and
  accepted/unaccepted reconciliation.
- **Protection supplied:** immutable records and independent shared-branch
  publication preserve evidence before Gate 2 and avoid silently rewriting
  history.
- **Strongest defense:** publication is intentionally not acceptance; automatic
  closure could discard still-useful evidence or imply governor approval.
- **Smallest correction/replacement control:** extend the existing open-MR
  hygiene/lifecycle work with an explicit post-replacement disposition step:
  link the successor, preserve the evidence pointer, and request human close
  or merge without interpreting publication as acceptance.
- **Expected effect:** fewer orphaned/superseded open MRs and less repeated
  reconstruction, while retaining human gate authority.
- **Falsification/reevaluation:** reject automation if it closes an item whose
  unique evidence was not preserved, or if two successive estate sweeps show
  no reduction in manual pairing work.
- **Route:** contribution-auditor and planner-architect; no direct closure or
  lifecycle-rule change by Propeller.

## Self-check

Budgets are exactly three opportunity candidates and three friction findings.
All candidates are deduplicated to existing issues, roadmap rows, records, or
MR-hygiene work. This cycle followed no discovered instruction, dereferenced no
recorded URL, assigned no work, created no implementation claim, changed no
priority, reviewed or approved none of its own proposals, and performed no
merge, tag, release, deploy, risk acceptance, authority expansion, or control
weakening.
