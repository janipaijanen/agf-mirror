# Roadmap

*Audience: evaluators, adopters, contributors, and maintainers or AI
assistants locating the authoritative plan and evidence.*

| Field | Value |
|---|---|
| Status | v2.6.1 published; v2.7.0 retired unused after the abandoned immutable v2.7.0-rc.1 candidate and stopped rc.2 reconstruction; v2.8.0 standard release candidate in construction from `main` |
| Observed at | 2026-08-28T09:05:14Z |
| Source revision | v2.6.1 remains the latest fully published stable release. Binding governor decision `20260828T085508Z-governor-reviewer-j37k` retires v2.7.0, retains the abandoned v2.7 branches and rc.1 tag as evidence, and authorizes v2.8.0 through the standard current-`main` release workflow from exact base `ca808607`. This page's own revision is whatever this file's Git/GitLab history shows as its latest commit. |
| Governed source | `docs/coordination/20260828T085508Z-governor-reviewer-j37k.md` and handoff `20260828T085514Z-planner-architect-rmna.md` govern the v2.8.0 release construction. |
| Authority | Informational projection. Governed plans, recorded gate decisions, changelogs, tags, and release artifacts are authoritative. This page is neither. |

## Current released baseline

**Latest fully published stable release: `v2.6.1`** — [release page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.6.1), tag `v2.6.1` (`f780ac0e`), released 2026-08-17. Constructed directly from `main`'s then-current tip (Path A) rather than a frozen line, correcting v2.6.0's incomplete publication. `v2.5.0` — [release page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.5.0), tag `v2.5.0` (`b9215bd6`), released 2026-08-10 — was the prior fully published stable release. `v2.4.11` — [release page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.4.11), tag `v2.4.11`, released 2026-08-01. `v2.6.0`'s tag exists and its OCI image published, but its GitLab Release and generic package were never created — it is not counted as a fully published release and remains superseded by v2.6.1 for adoption.

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

The v2.8.0 release candidate is being cut from current `main` through the
standard release workflow. It includes #86, #87, #88, and #90 plus all
post-v2.6.1 work and repairs already merged to `main`. The frozen v2.7.0
reconstruction is abandoned because its stale base repeatedly excluded
required repairs; v2.7.0 is retired unused. Publication remains subject to
exact-head Gate 2 evidence and the mandatory pre-tag OCI build and Trivy scan.

The remaining portfolio is deliberately split rather than accumulated into one
large release:

| Release | Character | Candidate scope |
|---|---|---|
| v2.4.9 | Published patch-only live-surface correction | #93; no new contracted surface |
| v2.4.10 | Published repository-local tooling reliability correction | #92; no command-semantic change |
| v2.4.11 | Published documentation and accessibility patch | #97; already-merged !890/!894 content only, no new contracted surface |
| v2.5.0 | Published stable release | Final stable promotion of the v2.5.0 release train |
| v2.6.0 | Tag constructed, incompletely published (OCI only); superseded | #84, #85, #89, bounded #76 work; `release:create` failed on a stale aarch64 asset contract |
| v2.6.1 | Published stable release | Same functional scope as v2.6.0, plus the asset-contract fix that let `release:create` succeed; issues #76/#84/#85/#89 closed |
| v2.7.0 | Retired unused | Frozen reconstruction abandoned; rc.1 remains immutable and failed |
| v2.8.0 | Candidate from current `main` | #86-#88/#90 plus all post-v2.6.1 work and required repairs |
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
- **v2.6.1 coordination reliability — published, closed:** #84, #85,
  #89, and #76 shipped in v2.6.1 (published 2026-08-17, correcting
  v2.6.0's incomplete tag-only construction) and are independently
  post-publication verified (`h584`) — Level 5 Deployable.
- **v2.8.0 current-main release — candidate:** #86-#88/#90 and all other
  post-v2.6.1 changes on `main`; exact-head Gate 2 and pre-tag OCI/Trivy
  evidence remain required. #91/#94 remain later-minor candidates.
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
