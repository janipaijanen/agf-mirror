---
id: 20261026T151646Z-next-release-scope-intake
status: approved
gate1_approved_by: 20261026T153847Z-governor-reviewer-s72h
gate1_approved_at: "2026-10-26T15:38:47Z"
topic: next-release-scope-and-rolling-intake
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      This plan records release-intake evidence and sequencing only. It
      authorizes no implementation, credential access, CI change, artifact
      publication, version choice, tag, or deployment. Every admitted
      workstream and the eventual publication child plan retain their own
      infosec consultation requirements.
  - role: contribution-auditor
    status: consulted
    reason: >-
      A delegated non-author contribution-auditor reviewed the exact revised
      draft and returned CLEAR with zero blocking, advisory, or accepted-risk
      findings. The first pass found two blocking ambiguities (the boundary of
      evidence-only refreshes and overstated collision coverage) plus one
      lifecycle advisory; all were corrected before the final rereview. The
      durable review record is published in the same Gate 1 review cycle.
affected_paths:
  - docs/plans/20261026T151646Z-next-release-scope-intake.md
depends_on:
  - 20260903T043752Z-queue-convergence-and-roadmap-authority
  - 20260909T195116Z-post-v211-queue-closeout-work-order
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Next release scope and rolling intake

## Purpose and authority

The governor asked on 2026-10-26 to plan the next release and to formalize
content gathering. This version-neutral plan is the single rolling owner for
that release's scope. It starts immediately after verified v2.11.0 publication
at annotated tag `v2.11.0` (`d732e372f35bacd1f37981bb89031041d35121e4`,
peeled commit `c1a13f8cdf626a9c10df5ddc82cc8cf6ebefb48a`).

This plan reuses the proven `IN` / `CANDIDATE` / `DEFER` vocabulary from the
v2.11 scope plan. It is not a new project roadmap, queue ledger, milestone,
schema, or release tool:

- `docs/roadmap.md` continues to own project sequencing;
- the queue-convergence issue/MR ledgers continue to own evidence-backed
  tracker dispositions;
- both `[Unreleased]` changelogs record landed adopter-visible changes, not
  prospective work; and
- GitLab labels, milestones, issue titles, and MR titles remain inputs, never
  scope authority.

Only a governor decision may promote a row from `CANDIDATE` to `IN`, remove an
`IN` row, choose a version, freeze scope, or authorize a release cut. Every
workstream retains its own Gate 1 and exact-head Gate 2. This plan grants no
implementation, merge, tag, package, release, or deployment authority.

## Why a rolling intake is needed

The current evidence surfaces answer different questions and are not a
complete release scope by themselves:

- both `[Unreleased]` changelogs are empty after v2.11.0 and cannot represent
  prospective work;
- the open queue contains 12 MRs and roughly 54 issues, mostly unmilestoned,
  including old `park-radar` items that must not become release candidates by
  age or visibility;
- active GitLab milestones still include stale v2.5/v2.6/v2.7 records; and
- merged work cannot be excluded from a standard whole-tree release merely by
  omitting it from a named scope list.

The smallest adequate control is therefore one manually reconciled table in
this plan. No schema or `agov` command is added during the first release cycle.
Automation is reconsidered only if the pilot records a recurring omission or
material reconciliation cost.

## Intake and reconciliation loop

Refresh this plan after a meaningful merge group, a governor scope request,
or a release-cadence review. Each refresh re-reads, from the immutable prior
tag forward:

1. every merged path and commit since `v2.11.0`;
2. both `[Unreleased]` changelog sections;
3. approved or active plans whose implementation is not merged;
4. current issue/MR/TODO disposition ledgers and triggered radar rows; and
5. current GitLab issue/MR state, exact heads, gates, and binding coordination
   decisions.

For each candidate, record the source, adopter-visible outcome, current gate
and evidence, compatibility/documentation impact, disposition, and the exact
promotion or exit condition. Plan status fields alone are insufficient when
live GitLab state or a later binding decision differs.

Every refresh uses a reviewed MR. An evidence-only correction is limited to
an exact SHA, pipeline/job state, tracker state, timestamp, or durable evidence
reference whose update does not change the row's meaning. Adding or removing a
candidate, reinterpreting readiness/compatibility, changing a promotion or exit
condition, creating or removing an `IN` commitment, choosing a version, or
freezing scope is a material amendment: return the plan to `pending-gate1`,
obtain non-author review, and publish a new governor decision for the exact
head.

At freeze, reconcile the selected whole tree, run the documentation-estate
release-content-freeze sweep, convert landed adopter-visible rows into both
changelogs, choose the semantic version from actual compatibility impact, and
derive a separate release-publication child plan. The child plan owns metadata,
namespace, protected-tag, pipeline, artifact, and anonymous-verification steps.

## Landed delta since v2.11.0

There is currently no next-release product change. Since the tag, only the
v2.11.0 completion record and the three-file post-publication status sync have
merged. Both `[Unreleased]` sections remain empty.

## IN

None. An approved workstream plan or green MR does not self-promote into the
release.

## CANDIDATE seed inventory

These rows are decision inputs, not commitments.

| Item / source | Potential adopter-visible outcome | Evidence and current gate | Compatibility and docs | Disposition | Promotion or exit condition |
|---|---|---|---|---|---|
| Timer-reality polling alignment (`20261024T070435Z-timer-reality-polling-alignment`) | Polling and wake instructions match observable mechanism lifetimes | Gate 1 approved; implementation not yet merged | Classified BREAKING for the poll-and-coordinate skill; owner rule, skill, templates, and adopter pages already scoped | CANDIDATE | Governor promotes after rechecking live collisions and readiness; otherwise defer |
| MR !3098, stale commands in shipped `agf/skills` | Shipped skills use current Rust `agov` commands | Exact head `6fccd783116bef86a908b4e53eadf9fd32d67b14`, green pipeline, binding disposition `awaiting-governor`; normal Gate 1 still required | Documentation/skill correction; impact classification required by its plan | CANDIDATE | Approve a normal Gate 1 and then merge at Gate 2, or defer/close |
| MR !3066 / issue #137, distribution-scan measurement | Evidence on suspected leading-`**` re-walk cost | Exact head `f7c1c8ec379bc61fa1b8e1fe4ea42fc4f1731367`; corrected measurement-first Gate 1 plan, independent review clear, green pipeline; `awaiting-governor` | Measurement only; no optimization claim or compatibility effect yet | CANDIDATE | Governor approves the measurement plan; optimization requires later evidence and its own plan |
| Issue #128, cadence register / year-clock | One discoverable view of recurring governance obligations | Explicit prior-release candidate; no current Gate 1 plan | Likely documentation/governance surface; owner and projection boundaries unresolved | CANDIDATE | Governor chooses plan or deferral after checking overlap with roadmap and role cadences |
| Issue #133, watcher idle versus work exhaustion | Mechanically distinct watcher-idle, inbox-idle, and queue-exhausted states | Recurrent operational failure model; issue carries `plan`; no Gate 1 plan | Could affect rules, skills, and operator docs; classification unknown | CANDIDATE | A bounded Gate 1 identifies the canonical owner and acceptance surface |
| MR !3097 evidence, `rule.schema.json` drift | Resolve unenforced schema/corpus disagreement without false compliance | Exact head `6a8c773d24628bd623ba4536175f11c02188d772`; coordination proposal only; evidence reports broad nonconformance; no acknowledged design choice or Gate 1 | Potentially high-risk schema/rule migration | CANDIDATE | Publish/acknowledge the evidence, choose migration versus schema correction, then approve a plan |
| Issues #119 and #138, instruction-context cost | Lower always-loaded instruction cost without weakening governance | Measured context pressure and Claude usage report; #138 may instead be complete at the accepted threshold | Loader/rule splitting can be behavior-sensitive; recall evidence required | CANDIDATE | First reconcile #138 as close or residual scope; any canonical compression/split needs fresh Gate 1 and recall checks |
| Issues #111 and #113, upgrade/ignore safety | Detect governance loss or excluded provenance during adoption upgrades | Learning issues only; no bounded Gate 1 plan | Security- and data-loss-sensitive trust boundary | CANDIDATE | A plan defines detection, recovery, and false-positive controls; infosec consultation required |
| Issue #136, stale worktree inventory | Safe report-first visibility into stale branches/worktrees | Report-first direction only; explicitly no deletion authorization | Foundry tooling/operator docs; destructive action excluded | CANDIDATE | Gate 1 limits scope to inventory/reporting and defines later deletion authority separately |
| Issue #105, advisory sensitive-material scanner | Optional warning for arbitrary-file sensitive material | Unmilestoned backlog; no plan | Infosec-sensitive; advisory/blocking boundary must be explicit | CANDIDATE | Infosec-reviewed Gate 1 demonstrates need, scope, and false-positive handling |
| MR !2679, builder-image trial slice | Evidence on a bounded builder-image experiment | Exact head `b3c7eaff6ea0502087fc9dd0f7f73bc41ddf8ee8`; draft, green, `awaiting-governor`; its distribution-scan slice is superseded in readiness by !3066 | Research only until evidence supports a product change | CANDIDATE | Governor isolates a still-useful trial slice or closes/supersedes it |

## DEFER and exclusions

- Old `park-radar` MRs !459, !923, !967, !998, !1000, !1451, !1718, and
  !1722 remain post-v2.11 queue-closeout work, not release candidates.
- Issue #129 remains withdrawn. Semantic/linked-data work needs a fresh
  governor decision before it can re-enter intake.
- Issue #137 does not admit an optimization until the measurement work proves
  the hypothesis.
- Issue #134 remains blocked on named capacity-one hardware/operator evidence.
- A generic `release::unmilestoned-backlog` label, an old milestone, or absence
  from this table never promotes an item.
- Anything not named `IN` or `CANDIDATE` is unexamined, not silently rejected.

## Acceptance for this process plan

- `agov plan validate` passes.
- `agov plan collision-check --base-ref origin/main --plans-only
  --changed-only --fail-on-unacknowledged-collision` reports zero
  unacknowledged collisions among assessable plans. Its coverage is explicitly
  incomplete while historical plans lack `affected_paths`; it does not prove
  whole-corpus non-overlap. Reviewers must also inspect the unassessed authority
  owners and this plan's `depends_on` disposition manually.
- Non-author contribution review confirms the table does not promote backlog,
  duplicate roadmap/ledger authority, or overstate issue/MR readiness.
- The plan contains no candidate version, date commitment, implementation
  authority, metadata edit, tag instruction, or new tooling/schema.
- A later freeze can account for every merged change since `v2.11.0`, every
  selected `IN` row, and every named `CANDIDATE` disposition without relying on
  stale milestones.

## Risks and rollback

- **Scope inflation:** a long candidate list may look committed. Control:
  explicit `CANDIDATE` semantics and governor-only promotion.
- **Duplicate authority:** release intake may drift from roadmap or queue
  ledgers. Control: those remain inputs/owners for their domains; this plan owns
  release scope only and edits none of them.
- **Stale evidence:** issue titles and plan statuses may outlive live state.
  Control: exact-head/GitLab/coordination re-read on every refresh and freeze.
- **Process overhead:** manual reconciliation may cost more than it saves.
  Control: one-release pilot; add automation only from measured recurrence.

Rollback is to supersede this plan by governor decision and return to explicit
one-off release scoping. No product, artifact, tag, or tracker state changes as
a result of this plan alone.

## Lifecycle closure

After this release is published and independently verified, mark this plan
`complete`. The following release starts a new version-neutral scope/intake
plan from that immutable tag. This plan never becomes a perpetual or
cross-release authority.
