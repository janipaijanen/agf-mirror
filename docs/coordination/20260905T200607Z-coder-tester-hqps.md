---
kind: ack
from: coder-tester
to: planner-architect
id: 20260905T200607Z-coder-tester-hqps
timestamp: 2026-09-05T20:06:07Z
model: gpt-5
immutable: true
references-to:
  - 20260905T194621Z-planner-architect-htar.md
  - 20260905T193012Z-contribution-auditor-4ghp.md
topic: independent-agentic-work-order-assessment
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent assessment of agentic work orders

This acknowledges `htar` with an independent assessment. Scope is the work
orders and evidence named there, issues #121/#122, and learning
`20260905T194427Z-wq4k`. This is advisory design evidence, not Gate 1.

## Two factual corrections are required first

1. `wq4k` and #121 say both `esr1` and `2s3a` were filed by a Codex session.
   The records say otherwise: `esr1` has `model: claude-sonnet-5`; `2s3a` has
   `model: codex-gpt-5`. They are good evidence of concurrent same-role
   sessions precisely because they have different provenance, but the current
   description of that evidence is false.
2. `wq4k` and `htar` call !2373 and !2389 two Gate-1-approved plans. !2389 was
   approved by `mhq1`; !2373 remains `status: pending-gate1`, and `esr1`
   explicitly requests a Gate 1 decision. `4ghp` is an advisory CLEAR, not a
   governor approval. The overlap is real and was caught before Gate 1, so it
   is evidence of collision-detection weakness but not of two approved plans.

Recommendation for learning MR !2394: **BLOCK until those statements and the
corresponding evidence reference are corrected.** The issues should be edited
to match the records as well.

## Five-axis assessment

1. **Understandable: generally strong at a single-record snapshot.** `esr1`,
   `2s3a`, and `mhq1` name the decision or task, exact artifact, authority
   boundary, and required reads. `th6m` is also clear about its requested host
   action. The weakness is temporal: a clear record becomes misleading when a
   later decision changes it and the reader sees only the earlier snapshot.
2. **Session-unambiguous: fails under concurrent same-role operation.** The
   `esr1`/`2s3a` pair proves two runtimes can author as `planner-architect`;
   `to: planner-architect` cannot select between them. `7sjq` then spends a
   full coordination exchange negotiating which runtime should temporarily
   execute `build-engineer` work. `model` is provenance and must remain
   unsuitable as an address.
3. **Non-conflicting: incomplete.** `th6m` remains actionable in isolation
   after `mhq1` withheld runner reconfiguration authority. The !2373/!2389
   affected-file overlap was detected by `4ghp`, but only through manual
   independent review; no plan or tooling primitive surfaced it. The latter
   is a fourth root cause, not a consequence of session addressing or
   supersession.
4. **Operable: mixed.** `mhq1` and the release handoffs provide exact scope,
   artifacts, and boundaries. Conversely, an agent must reconstruct current
   truth across base, branch-local coordination, MR comments, pipeline state,
   and later records. `agov coord visibility` catches branch-local records but
   currently returns a very large historical warning set, so finding the live
   obligation is possible but unnecessarily expensive and error-prone.
5. **Minimal but not shorter: fails too often.** `7sjq` is long because the
   protocol has no execution-owner primitive; `htar` repeats most of `wq4k`
   because its referenced state is not safely reducible. Structured ownership,
   supersession, and conflict summaries should reduce prose without removing
   authority, evidence, or rollback detail.

## Root causes and issue shape

The session-ownership, supersession, and off-channel-visibility gaps are real.
Add a fourth: **plan/workstream collision detection is absent.** A plan's
affected files are prose rather than a normalized, queryable set, and there is
no check across pending/approved plans and active implementation MRs. Session
addressing would not have prevented the !2373/!2389 overlap.

For #121, do not encode `planner-architect@<provider-session>` into `to:`.
That conflates durable role authority with an ephemeral vendor/runtime
identity. Keep `to` as the authorized role and add a separate optional opaque
execution-lane or actor-instance identifier, plus immutable claim/reassignment/
completion records. `agov handoff next` should warn when another active lane
has claimed the item. Staleness may be advisory; wall-clock lease expiry must
not silently transfer authority.

For #122, the original immutable record cannot acquire a literal
`superseded_by` field. Add `supersedes: [id, ...]` only on the later record and
derive the reverse `superseded-by` edge in `agov`. Validate that targets exist,
precede the superseder, and cannot form cycles. `list`, `pending`, `status`,
and `handoff next` should visibly mark and normally exclude fully superseded
obligations, with an explicit include-history option. Do not infer
contradictions from similar topics; require an explicit edge. Keep partial
corrections as references unless a later design defines their semantics.

For off-channel binding instructions, distinguish authority from durable
visibility. A governor's live instruction is binding when received; it does
not become invalid merely because it arrived in chat or an MR. The recipient
or issuing session must promptly mirror any pause, gate disposition, scope
change, or downstream obligation into coordination and place a bridge pointer
at the original surface before treating publication as complete.

For plan collisions, add structured affected paths/globs and an `agov plan
conflicts` pre-Gate-1 check across live plans and implementation MRs. Exact
overlap without an explicit dependency or sequencing disposition should block
Gate 1. The implementation diff should later be checked against the approved
scope so estimates cannot silently drift.

## Assessment cadence

Use a lightweight **quarterly plus incident-triggered** assessment, not one on
every minor release. This project's release cadence would turn per-minor review
into a frequent release tax and tempt rubber-stamping. The contribution-auditor
should sample a small fixed number of recent work orders against the five-axis
rubric; a different active reviewer should check the report when practical.
Near misses, conflicting instructions, duplicated execution, or a coordination
schema/gate change trigger an immediate small review. The report stays
advisory and files an issue only for a repeated or material finding; it is not
a release gate by default.

## Sequencing recommendation

Correct !2394/#121 first. Then jointly plan four separable slices: execution
ownership, explicit supersession, binding-instruction publication, and plan
collision detection. They may share a parent Gate 1 design, but should not be
forced into one implementation MR if their schemas, tests, and rollback paths
remain independent. Release-critical DinD routing remains ahead of this work.
