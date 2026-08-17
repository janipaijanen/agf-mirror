# Roadmap

*Audience: evaluators, adopters, contributors, and maintainers or AI
assistants locating the authoritative plan and evidence.*

| Field | Value |
|---|---|
| Status | v2.4.11 published; v2.5.0 (stable) published; v2.6.0 tag constructed but incompletely published (OCI only — GitLab Release and package never created, `release:create` failed on a stale aarch64 asset contract); v2.6.1 Gate 1 approved, correcting that failure, release-MR construction in progress |
| Observed at | 2026-08-17T03:47:48Z |
| Source revision | Portfolio reconciliation follows merged Gate 1 plan MR !858 at `20fc8925`; its approved exact head is `86fb6b3b`. The v2.4.11 release published via MR !903, Gate 1-approved by `docs/coordination/20260801T171157Z-governor-reviewer-hek5.md`. The v2.5.0 (stable) release published via approved publication-plan `docs/plans/20260809T194404Z-v250-final-release-publication.md` (MR !1349), Gate 1-approved by decision-record `docs/coordination/20260809T200252Z-governor-reviewer-h4wn.md`. The v2.6.0 release was constructed from a frozen `reconstruct/v2.6` line (not `main`) per `docs/plans/20260813T051840Z-v260-release-publication.md`; its tag object `445a3875` (peeled `4890dfe9`) exists and its OCI image published, but its `release:create` job failed — diagnosed in `20260815T193127Z-coder-tester-pj71.md` and fixed on `main` by MR !1610 (`9ad75651`). The v2.6.1 release corrects this by constructing directly from `main` (Path A), per `docs/plans/20260816T174141Z-v261-release-publication.md` (MR !1657), Gate 1-approved by decision-record `docs/coordination/20260817T034748Z-governor-reviewer-ps6t.md`, following binding path selection `docs/coordination/20260816T212149Z-governor-reviewer-ram5.md`. This page's own revision is whatever this file's Git/GitLab history shows as its latest commit. |
| Governed source | [MR !755](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/755), merged at `27716f20`, places the Gate 1-approved SoW on `main`. MR !858 defines the approved portfolio reconciliation. `docs/plans/post-v248-release-batching-amendment.md` records the governor-approved smaller release batches. `docs/plans/20260816T174141Z-v261-release-publication.md` is the approved v2.6.1 publication plan. |
| Authority | Informational projection. Governed plans, recorded gate decisions, changelogs, tags, and release artifacts are authoritative. This page is neither. |

## Current released baseline

**Latest fully published stable release: `v2.5.0`** — [release page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.5.0), tag `v2.5.0` (`b9215bd6`), released 2026-08-10. Published through the protected maintenance-line workflow as a metadata-only promotion of the exact `v2.5.0-rc.2` functional candidate — zero functional, schema, documentation-content, migration, test, dependency, CI, or artifact-layout change from RC2. `v2.4.11` — [release page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.4.11), tag `v2.4.11`, released 2026-08-01 — was the prior stable release. `v2.6.0`'s tag exists and its OCI image published, but its GitLab Release and generic package were never created — it is not counted as a fully published release. `v2.6.1` is Gate 1 approved and its release-MR construction is in progress; this line updates again once the `v2.6.1` tag and full release artifacts actually exist.

v2.4.10 retains the complete v2.4.9 baseline and resolves issue #92 by making
the repository-local `tooling/agov` wrapper source-compatible with its
checkout. It forwards the authoritative Rust command tree, verifies a
deterministic tooling-source fingerprint, performs at most one verified
release rebuild, fails closed when recovery is unavailable, and reports the
correct wrapper for cross-worktree invocation. Standalone release binaries
retain their existing behavior.

v2.4.11 is a documentation and accessibility patch: it corrects issue #97's
stale commissioning-prompt version pin and Visual Studio Code compatibility
prose, and adds a visible AI-generation disclosure, descriptive alternative
text, and a strict-build accessibility check to every public figure embed in
`README.md` and the public documentation site. It adds no command, schema,
or adopter-facing obligation semantic. Issue #97 is closed: v2.4.11's
published assets and site passed post-publication verification.

A significant limitation in v2.4.5 was fixed in the v2.4.8 baseline:
[issue #81](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/81)
documents that the shipped `v2.4.5` archive's operations-runbook test failed
when run from a freshly extracted `agf-2.4.5.tar.gz` (`Ran 7 tests / FAILED
(errors=4)`, all four from fixtures absent from the release manifest). v2.4.8
ships the missing fixtures and validates the complete clean-archive test suite.
The immutable `v2.4.6` predecessor contains that correction but its tag
pipeline exhausted runner disk during the mandatory Windows cross-build, so it
never produced a GitLab Release, generic package, or OCI release. The immutable
`v2.4.7` predecessor removed that cross-build and published its OCI image, but
a downstream check still required the absent optional Windows ZIP; it produced
no generic package or GitLab Release. v2.4.8 completes the publication recovery
with Windows retained only as optional manual preview evidence.

## Active release train

The v2.6.1 release is **Gate 1 approved** under
`docs/plans/20260816T174141Z-v261-release-publication.md` (MR !1657). It
constructs directly from `main` (Path A, per binding decision `ram5`),
carrying every commit `main` has accumulated since peeled v2.5.0 —
including the four v2.6.0-targeted feature commits (#76, #84, #85, #89,
merged 2026-08-05) and the release-asset-contract fix (`9ad75651`, merged
2026-08-16) that v2.6.0's frozen construction line never received. Read
the plan and its coordination trail directly rather than relying on a
summary here — this page intentionally does not reproduce the
release-construction stages to avoid a second copy that can drift from
the plan. v2.5.0 remains the latest **fully published** stable
deliverable until v2.6.1's tag, GitLab Release, package, and OCI image
are independently verified.

The remaining portfolio is deliberately split rather than accumulated into one
large release:

| Release | Character | Candidate scope |
|---|---|---|
| v2.4.9 | Published patch-only live-surface correction | #93; no new contracted surface |
| v2.4.10 | Published repository-local tooling reliability correction | #92; no command-semantic change |
| v2.4.11 | Published documentation and accessibility patch | #97; already-merged !890/!894 content only, no new contracted surface |
| v2.5.0 | Published stable release | Final stable promotion of the v2.5.0 release train |
| v2.6.0 | Tag constructed, incompletely published (OCI only) | #84, #85, #89, bounded #76 work; `release:create` failed on a stale aarch64 asset contract |
| v2.6.1 | Gate 1 approved; release-MR construction in progress | Same functional scope as v2.6.0 (already on `main`), plus the asset-contract fix that lets the release-create job succeed |
| v2.7.0 | MCP and assistant integration | #86-#88 and #90 |
| Later minor | Separately gated work | #91, #94, and approved backlog themes |

## In scope now

- **v2.5.0-rc.1 contract core (Gate 1-approved, release execution in
  progress):** upgrade-safe adopter role authority
  ([#79](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/79))
  and actionable acknowledgement obligations
  ([#80](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/80)),
  plus the minimum #33/#67/#78 compatibility slices those contracts needed
  (#33 and #67 stay open, unmilestoned; #78 closes with a full disposition
  note). Per the governor's Gate 1 scope-amendment decision, three further
  already-merged, already-Gate-2-approved surfaces ship in the same
  candidate: learning-record `status`/`supersedes`, the
  `governance-file-layout-rule.md` correction, and issue #101's
  `agov ci cache-topology-check` command.
- **v2.6.1 coordination reliability (Gate 1 approved; release-MR
  construction in progress):** #84, #85, #89, and #76 — already merged to
  `main` (2026-08-05) and carried by every commit since; v2.6.1's Gate 1
  approval authorizes their first full publication, correcting v2.6.0's
  incomplete tag-only construction. These items remain Level 4 Validated
  until post-publication verification establishes Level 5 Deployable
  evidence. #86-#88/#90 route to v2.7.0; #91/#94 remain later-minor
  candidates. A `v2.5.0` milestone is routing metadata, not implementation
  authority.
- **Issue-board workflow** ([issue #75](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/75)) —
  proposed, Gate 1 plan open in [MR !745](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/745),
  not yet approved.

Issue #81 remains fixed in the v2.4.8 recovery baseline. Issue #93 is fixed in
the v2.4.9 release: the broken legacy wrapper and dead parity harness are
removed, current tooling instructions use Rust `agov`, and the published
archives are checked for regressions. Issue #92 is fixed in v2.4.10: the
repository wrapper is bound to compatible tooling-source content and forwards
the full Rust parser surface. Issue #97 is fixed and closed in the published
v2.4.11 release, following anonymous downloaded-asset and exact-tag
source-checkout verification so the durable record cites Level 5 evidence.

The complete evidence-backed routing is maintained in the
[issue disposition ledger](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/portfolio/post-v248-issue-disposition.md)
and [MR disposition ledger](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/portfolio/post-v248-mr-disposition.md).

## Parked, radar, and excluded

- **Post-v2.5 triggered backlog:** #31, #32, #34-#36, #38-#49, the residual
  #76 performance scope, and #77. These are considered and deliberately not
  smuggled into v2.5.0. Their triggers are recorded individually in the issue
  ledger; #44 is a legal/entity track rather than an engineering release item.
- **Radar:** #59-#62 and #83 remain Level 1/theoretical watch surfaces with no
  release commitment.
- **Learning intake:** #96 contributes single-adopter evidence for three-way
  upgrade comparison. It informs #79 but does not independently expand the
  approved v2.5.0 release contract.

This page intentionally summarizes rather than duplicates the per-item
ledgers. The ledgers are the complete routing projection; governed plans,
decisions, merged commits, and releases remain authoritative.

## How status changes

This page is manually maintained. It changes only when:

- a published Gate 1 plan or governor decision changes what is proposed or
  approved;
- a merged change or released artifact changes what is delivered;
- a coder-tester implementation or release MR updates it directly to match
  shipped scope.

Every update refreshes the `Observed at` field above and cites its source
artifact. An inaccurate roadmap is documentation drift, not a cosmetic issue,
and is handled through the project's normal adopter-documentation review.
