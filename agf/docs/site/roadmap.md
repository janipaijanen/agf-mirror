# Roadmap

*Audience: evaluators, adopters, contributors, and maintainers or AI
assistants locating the authoritative plan and evidence.*

| Field | Value |
|---|---|
| Status | v2.4.10 release publication approved; later release train **approved**, Gate 1 |
| Observed at | 2026-07-31T20:06:24Z |
| Source revision | Portfolio reconciliation follows merged Gate 1 plan MR !858 at `20fc8925`; its approved exact head is `86fb6b3b`. The v2.4.10 release follows approved publication-plan MR !883 at `a8f46743` (merged as `0ba16cbe`). This page's own revision is whatever this file's Git/GitLab history shows as its latest commit. |
| Governed source | [MR !755](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/755), merged at `27716f20`, places the Gate 1-approved SoW on `main`. MR !858 defines the approved portfolio reconciliation. `docs/plans/post-v248-release-batching-amendment.md` records the governor-approved smaller release batches. |
| Authority | Informational projection. Governed plans, recorded gate decisions, changelogs, tags, and release artifacts are authoritative. This page is neither. |

## Current released baseline

**Latest release: `v2.4.10`** — [release page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.4.10), tag `v2.4.10`.

v2.4.10 retains the complete v2.4.9 baseline and resolves issue #92 by making
the repository-local `tooling/agov` wrapper source-compatible with its
checkout. It forwards the authoritative Rust command tree, verifies a
deterministic tooling-source fingerprint, performs at most one verified
release rebuild, fails closed when recovery is unavailable, and reports the
correct wrapper for cross-worktree invocation. Standalone release binaries
retain their existing behavior.

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

The v2.5.0 release train is **Gate 1 approved**. [MR !755](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/755)
(`docs/plans/v250-statement-of-work.md`) is the approved controlling SoW and
merged to `main` at `27716f20`. Read the plan and its coordination trail directly rather than
relying on a summary here — this page intentionally does not reproduce the
release-train stages to avoid a second copy that can drift from the plan.
v2.4.10 is the latest published patch deliverable in that approved train. It includes
the complete v2.4.8 recovery baseline, which superseded the immutable
unpublished v2.4.6 tag and the immutable OCI-only v2.4.7 partial publication
for adoption.

The remaining portfolio is deliberately split rather than accumulated into one
large release:

| Release | Character | Candidate scope |
|---|---|---|
| v2.4.9 | Published patch-only live-surface correction | #93; no new contracted surface |
| v2.4.10 | Published repository-local tooling reliability correction | #92; no command-semantic change |
| v2.5.0 | Authority and acknowledgement contracts | #79, #80, and only necessary #33/#67/#78 slices |
| v2.6.0 | Coordination reliability | #84, #85, #89, and bounded #76 work |
| v2.7.0 | MCP and assistant integration | #86-#88 and #90 |
| Later minor | Separately gated work | #91, #94, and approved backlog themes |

## In scope now

- **v2.5.0 contract core:** upgrade-safe adopter role authority
  ([#79](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/79))
  and actionable acknowledgement obligations
  ([#80](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/80)),
  plus only the minimum #33/#67/#78 compatibility slices those contracts need.
- **Reliability and integration candidates, not yet release promises:**
  #84/#85/#89 route to v2.6.0; #86-#88/#90 route to v2.7.0; #91/#94
  remain later-minor candidates.
  Each requires current-main reproduction, compatibility classification, an
  approved child plan, and Gate 2 evidence. A `v2.5.0` milestone is routing
  metadata, not implementation authority.
- **Issue-board workflow** ([issue #75](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/75)) —
  proposed, Gate 1 plan open in [MR !745](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/745),
  not yet approved.
- **Coordination-visibility performance** ([issue #76](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/76)) —
  proposed, Gate 1 plan open in [MR !746](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/746),
  not yet approved.

Issue #81 remains fixed in the v2.4.8 recovery baseline. Issue #93 is fixed in
the v2.4.9 release: the broken legacy wrapper and dead parity harness are
removed, current tooling instructions use Rust `agov`, and the published
archives are checked for regressions. Issue #92 is fixed in v2.4.10: the
repository wrapper is bound to compatible tooling-source content and forwards
the full Rust parser surface. Issue closure follows anonymous downloaded-asset
and exact-tag source-checkout verification so the durable record can cite
Level 5 evidence.

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
