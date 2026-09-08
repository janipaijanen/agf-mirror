# Roadmap

*Audience: evaluators, adopters, contributors, and maintainers or AI
assistants locating the authoritative plan and evidence.*

| Field | Value |
|---|---|
| Status | v2.10.0 prepared as the next stable release; v2.9.0 remains the latest published release until tag verification completes |
| Observed at | 2026-09-08T08:47:49Z |
| Source revision | v2.10.0 is being constructed through the standard current-`main` workflow from frozen content base `2d0fdc00`; the future immutable tag and verified release artifacts become publication authority only after the release MR passes Gate 2 and the tag pipeline succeeds. |
| Governed source | `docs/plans/20260907T045703Z-v210-release-plan.md`, governor approval `docs/coordination/20260907T082727Z-governor-reviewer-kg6s.md`, scope amendment `docs/coordination/20260907T151325Z-governor-reviewer-2emq.md`, and preflight-sequence amendment `docs/coordination/20260908T083406Z-governor-reviewer-rg6k.md` govern v2.10.0 publication together with the future exact-head release Gate 2 record. |
| Authority | Informational projection. Governed plans, recorded gate decisions, changelogs, tags, and release artifacts are authoritative. This page is neither. |

## Current released baseline

**Latest fully published stable release: `v2.9.0`** — use the
[GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.9.0)
for its verified artifacts. v2.7.0 was retired unused after its abandoned
release candidate; the immediately preceding version was the prior stable
release.

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

v2.10.0 is the active standard-path release candidate from reviewed current
`main`. Its content base is `2d0fdc00`: universal polling, the token-ROI repair,
the planned/inactive Level 4 Improvement Propeller, and the four-namespace
release guard. v2.9.0 remains the published stable baseline until v2.10.0's
release MR, immediate pre-push namespace check, tag pipeline, and anonymous
artifact verification all succeed.

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
| Previous stable | Published stable release | Gate-change enforcement and the first integrated A1/A2 release-safety work |
| v2.9.0 | Published stable release | All reviewed work merged after the preceding release, including release-safety and runner-routing improvements |
| v2.10.0 | Active release candidate | Autonomous development operation: universal polling, ROI repair, Level 4 Improvement Propeller, and fail-closed release namespace proof |
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
- **Previous current-main release — published:** #86-#88/#90 and all other
  post-v2.6.1 changes in that release are Level 5 Deployable.
- **v2.9.0 current-main release — published:** all reviewed changes merged
  after the preceding release, with exact-head Gate 2 and pre-tag OCI/Trivy evidence.
  #91/#94 remain later-minor candidates.
- **v2.10.0 autonomous-development release — publication pending:** the four
  mandatory RUW packages are merged. Improvement Propeller is deliberately
  Level 4 and inactive; issue #132 owns post-release activation and genuine
  wake-path proof. Release metadata, exact-head Gate 2, the tag, and public
  artifact verification remain outstanding.
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
