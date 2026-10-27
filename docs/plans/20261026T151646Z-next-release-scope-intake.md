---
id: 20261026T151646Z-next-release-scope-intake
status: pending-gate1
gate1_approved_by: 20261027T183717Z-governor-reviewer-a8d6
gate1_approved_at: "2026-10-27T18:37:17Z"
topic: next-release-scope-and-rolling-intake
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      This amendment selects release scope, version and a documentation-only
      base. It changes no credential path, security policy, executable content,
      artifact publication, tag or deployment mechanism. Every admitted
      workstream and the eventual publication child plan retain their own
      infosec consultation requirements.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Review 20261027T215147Z-contribution-auditor-rajn covers exact plan head
      1c131572e22e44b4428100d83ea46fd44378e3b9 and returns CLEAR after the
      sole stale-state finding was resolved. This later metadata-only head
      records that review without changing version, base, scope, candidate
      disposition, authority, or acceptance criteria.
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
content gathering. This plan is the single rolling owner for
that release's scope. It starts immediately after verified v2.11.0 publication
at annotated tag `v2.11.0` (`d732e372f35bacd1f37981bb89031041d35121e4`,
peeled commit `c1a13f8cdf626a9c10df5ddc82cc8cf6ebefb48a`).

Governor decision `20261027T183717Z-governor-reviewer-a8d6` approved the prior
exact plan head. This final amendment returns the plan to `pending-gate1`
because it proposes the exact v2.12.0 freeze base and closes the last landed
content disposition. The earlier decision does not authorize this head.

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
workstream retains its own Gate 1 and exact-head Gate 2. Until a binding
governor decision approves this exact head, this plan grants no implementation,
merge, tag, package, release, or deployment authority.

## Why a rolling intake is needed

The current evidence surfaces answer different questions and are not a
complete release scope by themselves:

- changelogs record landed work but cannot represent prospective work or choose
  release scope by themselves;
- the open queue remains nontrivial and mostly unmilestoned, including old
  `park-radar` items that must not become release candidates by age or
  visibility;
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
release-content-freeze sweep, reconcile both changelogs and record each landed
adopter-visible row in its applicable canonical changelog, choose the semantic
version from actual compatibility impact, and
derive a separate release-publication child plan. The child plan owns metadata,
namespace, protected-tag, pipeline, artifact, and anonymous-verification steps.

## Landed delta since v2.11.0

The three-file post-publication status sync merged as `0fa27e48`; it records
v2.11.0 publication and does not create a new product headline. MR !3098 source
head `6fccd783116bef86a908b4e53eadf9fd32d67b14` then merged as `dc5523f6`,
correcting stale command references in three shipped `agf/skills` files. That
adopter-visible correction is landed but is not an `IN` commitment: it merged
without the required pre-merge Gate 2 decision and before its plan MR !3248
merged. Correction record `20261026T180956Z-governor-reviewer-pbgd` at
`2ccba663967767c7ede3359f1203bb89afddac3c` merged through MR !3258 as
`e5548cf6e9b5372e374aa3368b6e98ae3800553c`; it records the chronology without
retroactively authorizing the merge. The remaining post-tag changes through
`0d76ea53` (the only change after `a52767d4` is bridge note
`20261027T053114Z-planner-architect-tks3`, MR !3278). Radar correction MR !3281 merged as
`5ebb03be` under Gate 2 record `1rdd` (MR !3286) and radar-closure proposal MR !3282 merged as
`914495f3`. The coexistence plan merged as `58ac4aec` (MR !3275) under Gate 1 and Gate 2 record
`p47g` (MR !3288, merged `108bc189`). The Pages homepage plan merged as `47716594` (MR !3276)
under Gate 1 record `djnt` (MR !3290, merged `0e6e2604`). This amendment merged as `25326561`
(MR !3283). The eight park-radar merge requests were closed 2026-10-27 under decision `5nmt`
(MR !3291, merged `c245ce38`) and completion record `gpn3` (MR !3293); see DEFER. The
final reconciliation is recorded in
`docs/reviews/next-release-freeze-evidence/20261027-read-only-summary.md`, merged
at candidate base `9af411aa1186f454a5801100dcddf258d588d414`. At that base MR
!3098's retained skill-command correction is the only landed change requiring
a new v2.12 changelog entry or disposition, reconciled in `agf/CHANGELOG.md`;
the root changelog intentionally has no entry. The post-v2.11 status sync in
`agf/docs/governance/manifest.md` and `agf/docs/site/roadmap.md` is separately
inventoried as a no-changelog publication-status update. Plans, coordination
records, radar entries and verbatim radar
carrier files (`docs/radar.md`, `docs/proposals/radar/`, merged through MR !3274
as `208f1d5a1031d49ac5b6056738a928c39dd0cb88`), and the Phase A measurement
report (`docs/reviews/context-delivery-measurement/`, merged through MR !3271 as
`9ef652c7e5b1a29f2841b2d26b4fcc9c07b7252f`) do not add another adopter-visible
release item.

Timer-reality polling alignment is not part of this delta. Its implementation
commit `324322ef26b3efaeb2cd5a545c6d95cd64e4ba0e` is already an ancestor of the
v2.11.0 peeled commit and the v2.11.0 changelog records it.

## Freeze candidate

Proposed semantic version: **v2.12.0**. Proposed exact implementation base:
`9af411aa1186f454a5801100dcddf258d588d414`. The candidate contains one bounded
documentation slice, in this order. A minor version is proposed because the two
items add adopter-facing documentation and navigation surfaces without a
breaking compatibility change. If approved, the candidate would freeze that
scope; it does not authorize a tag, publication or deployment, which require
the later release-publication child plan and decisions.

1. `20261027T052500Z-tooling-coexistence-discoverability` (materially corrected
   plan status approved, renewed Gate 1 `g2ar` on `main`). Implementation first:
   README pointer, one adopter page, one navigation line and one projection manifest.
2. `20261027T052118Z-gitlab-pages-homepage-modernization` (MR !3276; plan status
   approved, Gate 1 `djnt` on `main` via MR !3290). Implementation only after item
   1 merges. Its Gate 2 requires an independent rendered exact-head web-UX review,
   in addition to the plan's own acceptance evidence.

The exact-tag claim recheck at the proposed base confirms Ponytail v5.3.2 commit
`16352ac229eb08a9598dcae48fe9f2b342567d86` and Caveman v4.0.1 peeled commit
`d262ab3882952a59312ff3537cd86d894523c5ab`; the corrected evidence table and
advisory reviews are in the coexistence plan. This amendment stays
`pending-gate1` until a non-author review of this exact head and a new durable
governor decision names this base, v2.12.0 and both ordered IN items. Each
implementation still needs its own exact-head Gate 2.

## Retained landed change

- MR !3098's skill-command correction is retained in the whole-tree release.
  It is reconciled in `agf/CHANGELOG.md`; correction record `pbgd` preserves the
  chronology without claiming retroactive Gate 2 authority.

## CANDIDATE seed inventory

These rows are decision inputs, not commitments.

| Item / source | Potential adopter-visible outcome | Evidence and current gate | Compatibility and docs | Disposition | Promotion or exit condition |
|---|---|---|---|---|---|
| MR !3066 / issue #137, distribution-scan measurement | Evidence on suspected leading-`**` re-walk cost | Exact head `f7c1c8ec379bc61fa1b8e1fe4ea42fc4f1731367`; corrected measurement-first Gate 1 plan, independent review clear, green pipeline; `awaiting-governor` | Measurement only; no optimization claim or compatibility effect yet | CANDIDATE | Governor approves the measurement plan; optimization requires later evidence and its own plan |
| Issue #128, cadence register / year-clock | One discoverable view of recurring governance obligations | Explicit prior-release candidate; no current Gate 1 plan | Likely documentation/governance surface; owner and projection boundaries unresolved | CANDIDATE | Governor chooses plan or deferral after checking overlap with roadmap and role cadences |
| Issue #133, watcher idle versus work exhaustion | Mechanically distinct watcher-idle, inbox-idle, and queue-exhausted states | Recurrent operational failure model; issue carries `plan`; no Gate 1 plan | Could affect rules, skills, and operator docs; classification unknown | CANDIDATE | A bounded Gate 1 identifies the canonical owner and acceptance surface |
| MR !3097 evidence, `rule.schema.json` drift | Resolve unenforced schema/corpus disagreement without false compliance | Proposal head `6a8c773d24628bd623ba4536175f11c02188d772` merged as `40f9e8d127df9cba441e663ea070614378ddbcfd`; evidence reports broad nonconformance, but no migration-versus-schema decision or Gate 1 exists | Potentially high-risk schema/rule migration | CANDIDATE | Governor chooses migration versus schema correction, then approves a bounded plan |
| Issues #119 and #138, instruction-context cost | Measure residual context cost without weakening governance, then decide whether any delivery change is warranted | Phase A execution and its report are complete. The Phase A plan `20261026T155400Z-context-delivery-measurement` (Gate 1 `20261026T174028Z-governor-reviewer-snh2`) keeps its approved front-matter status: its report is on `main` at `9ef652c7e5b1a29f2841b2d26b4fcc9c07b7252f` with outcome `inconclusive` under the plan's rule (Claude Code showed material cost on the foundry checkout, none on the clean adopter install; the Codex client was not assessable); no product implementation exists | Phase A changed no loader, rule, schema, package or tool; any later delivery change can be behavior-sensitive and needs its own Gate 1 and recall checks | CANDIDATE | Governor reads the report and decides: close #138 as complete, name a residual scope, or ask for a Codex-client measurement path; no product work self-promotes from the measurement |
| Issues #111 and #113, upgrade/ignore safety | Detect governance loss or excluded provenance during adoption upgrades | Learning issues only; no bounded Gate 1 plan | Security- and data-loss-sensitive trust boundary | CANDIDATE | A plan defines detection, recovery, and false-positive controls; infosec consultation required |
| Issue #136, stale worktree inventory | Safe report-first visibility into stale branches/worktrees | Report-first direction only; explicitly no deletion authorization | Foundry tooling/operator docs; destructive action excluded | CANDIDATE | Gate 1 limits scope to inventory/reporting and defines later deletion authority separately |
| Issue #105, advisory sensitive-material scanner | Optional warning for arbitrary-file sensitive material | Unmilestoned backlog; no plan | Infosec-sensitive; advisory/blocking boundary must be explicit | CANDIDATE | Infosec-reviewed Gate 1 demonstrates need, scope, and false-positive handling |

## DEFER and exclusions

- The old `park-radar` MRs !459, !923, !967, !998, !1000, !1451, !1718, and
  !1722 were closed 2026-10-27 under decision `5nmt` and completion record `gpn3`
  (MR !3293). Their unique content is preserved as radar entries and carrier files
  on `main` (MR !3274, merged `208f1d5a`). They are not release candidates.
- Issue #129 remains withdrawn. Semantic/linked-data work needs a fresh
  governor decision before it can re-enter intake.
- Issue #137 does not admit an optimization until the measurement work proves
  the hypothesis.
- Issue #134 remains blocked on named capacity-one hardware/operator evidence.
- MR !2679 was closed without merge on 2026-10-27 under governor decision
  `20261027T214011Z-governor-reviewer-s8n7` (published through MR !3308). Its
  distribution-scan slice is superseded by !3066; the builder-image trial was
  not selected. The source branch and history remain available for reference.
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
- The plan names one candidate version and exact base but grants no
  implementation, metadata, tag, publication or deployment authority before
  the binding governor decision. It adds no tooling or schema.
- The freeze review can account for every merged change since `v2.11.0`, both
  selected `IN` rows, and every named `CANDIDATE` disposition without relying
  on stale milestones.

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
