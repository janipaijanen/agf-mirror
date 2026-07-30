---
id: v246-release
status: pending-gate1
topic: v246-release
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      Assigned by `20260729T152543Z-governor-reviewer-hapn` and reactivated
      by `20260730T123406Z-governor-reviewer-thnq`, both of which require
      routing this plan through contribution-auditor before Gate 1.
      `20260730T131402Z-contribution-auditor-17qq` is the prior complete
      release-plan pass; its B1-B5 findings are corrected at this source
      head. `20260730T134315Z-contribution-auditor-gdrb` independently
      confirmed B1-B5 resolved and requested only this consultation-history
      correction before final exact-head confirmation. During release
      execution, contribution-auditor reproduced a release-blocking metadata
      omission: the prior v2.4.5 release changed five version surfaces, while
      this plan authorized only two. This amendment corrects that scope before
      publication.
  - role: infosec-sentinel
    status: consulted
    reason: >
      `hapn`/`thnq` require infosec-sentinel routing before Gate 1. Scope
      is version bump, changelog finalization, release-branch creation,
      and tag/publish workflow only — no new credential handling, schema,
      validator, or trust-boundary change. Flagging specifically for a
      pass on the release-artifact provenance chain (SPDX/license checks,
      clean-archive extraction, `agov dist post-publication-check`) since
      that is the one place this plan touches externally-published,
      immutable output.
---

# Plan: v2.4.6 release (version bump, tag, publication)

## Gate 1 amendment requested

Gate 1 was approved by
`20260730T145733Z-governor-reviewer-3g7k` at exact reviewed head
`d489ed77d479b26ab67b9880d5dcbb20dcad8d3a`. Execution stopped before the
release branch was committed or pushed because a fresh build after changing
`agf/VERSION` still reported `agov 2.4.5`: the Rust workspace version and
generated lockfile were outside the approved two-file scope. The v2.4.5 tag's
own merge commit confirms that a coherent release changes five metadata
surfaces, not two.

Gate 1 amendment is therefore requested to add exactly three mandatory version
metadata files — `tooling/rust/Cargo.toml`, `tooling/rust/Cargo.lock`, and
`agf/templates/AGENTS.md` — while retaining the already-approved
`agf/VERSION` and `agf/CHANGELOG.md` changes. No product behavior or release
content is otherwise added.

## Authorization chain

- `20260729T152543Z-governor-reviewer-hapn` — original assignment: prepare
  a narrowly scoped Gate 1 plan for cutting and publishing v2.4.6 from
  current `main`, after the recovery implementation merged (MR !784,
  merge commit `7146101e`; approved plan MR !774, merge commit
  `59a4ec9e`; Gate 2 decision MR !785, merge commit `e0cde7a3`).
- `20260730T084823Z-governor-reviewer-8rph` — approved MR !804 (CI
  interruptibility implementation) at Gate 2 and separately directed:
  "Resume the already-approved v2.4.6 release work in parallel with
  monitoring... No future v2.5.0 tooling plan is pulled into the v2.4.6
  release scope by this decision."
- `20260730T123406Z-governor-reviewer-thnq` — explicit reactivation,
  confirming `hapn` "remains the complete task contract" and that this is
  "not speculative work." Directs: reconcile against current `main`,
  freeze the exact planning base, route through contribution-auditor and
  infosec-sentinel, and states the future tooling plans (MRs !805–!807)
  and the CI longitudinal assessment (MR !810) are outside v2.4.6 and not
  blockers for this drafting assignment.
- `20260730T145733Z-governor-reviewer-3g7k` — binding Gate 1 approval at
  exact reviewed plan head `d489ed77d479b26ab67b9880d5dcbb20dcad8d3a`.
  The subsequent metadata-only update marks that decision in this file;
  it does not expand the approved release scope.
- `20260730T150606Z-contribution-auditor-tsgk` — release stop and amendment
  request after a clean local build proved that the approved two-file scope
  would package binaries self-reporting 2.4.5 under 2.4.6 filenames and
  provenance expectations.

## Release base

`origin/main` at `b0623ecd3c7a259dcf7091f87d5178f15881cf6f` (fetched and
verified immediately after the original Gate 1 plan and decision record
merged, 2026-07-30T15:03Z). `agf/VERSION` reads `2.4.5`; the last tag is
`v2.4.5`. The local `release-2.4.6` worktree was created from this exact base,
but no release commit or push occurred before the metadata stop condition.
Re-verify the base again after this amendment is approved and merged.

## Reconciliation: every commit since v2.4.5

112 commits exist between `v2.4.5` and this release base. Classification unit:
distinct commit OIDs on this range, classified from the unique path delta
reported by `git diff-tree --no-commit-id --name-only -r <OID>`. This yields
42 commits touching only `docs/coordination/*.md`, 39 pathless structural merge
commits with no unique delta under that method, and 31 commits with another
path. The 31 non-coordination OIDs partition into exactly four non-overlapping
groups: Group A (8), Group B (8), Group C (11), and Group D (4) — 8 + 8 + 11
+ 4 = 31. The earlier 69 figure conflated 37 coordination-only commits with
32 pathless merge commits at the old base; this amendment corrects that
description without changing the already-verified A/B/C OID membership.

### Group A — in scope: the already-approved v2.4.6 recovery patch (8 commits)

Implements `docs/plans/v246-recovery-patch.md` (Gate 1 approved, Gate 2
approved via MR !785, merged via MR !784 at `7146101e`). Already fully
reflected in `agf/CHANGELOG.md`'s `## Unreleased` section:

- Clean-archive validation now runs every Python test shipped in both
  archive formats (issue #81).
- Completed the remaining legacy GitLab Pages URL corrections in
  governance documentation (issue #67 Slice B).
- Restored resolution of legacy 20-character Crockford coordination
  references and deduplicated equivalent resolver results (MR !340).
- Plan validation now distinguishes known inactive advisory roles from
  unknown role names (MR !463).

No changelog drafting needed for this group — only renaming `##
Unreleased` to `## [2.4.6] — <release date>` at implementation time, per
hapn item 3. `MR !739`'s learning-record addition was explicitly excluded
by the recovery-patch plan itself and stays excluded here.

### Group B — reconciliation finding: version-controlled roadmap page (8 commits)

`docs/plans/version-controlled-roadmap-page.md` (Gate 1 approved
`20260728T200855Z-governor-reviewer-331n`, implemented via MR !758/!762,
merged before this release base). Touches adopter-facing distribution
content (`agf/docs/site/roadmap.md`, `agf/docs/build/mkdocs.yml`,
`README.md`) but is **not yet listed in `agf/CHANGELOG.md`'s Unreleased
section** and was not named in `hapn`'s original four-item scope.

Classification: patch-compatible under `contract-stability.md`'s promise
("Fix bugs, clarify documentation, improve error messages" — a new
documentation page with navigation entries adds no validator, exit code,
CLI flag, or schema field). Recommendation: add one `agf/CHANGELOG.md`
entry for it under a new `### Added` subsection of the finalized v2.4.6
section — the current `## Unreleased` section contains only `### Fixed`,
and "added a roadmap page" is an addition, not a fix; it must not be
folded into `### Fixed` merely because that is the only subsection
already present
("Added a version-controlled release roadmap page to the documentation
site, discoverable from the top-level navigation, the documentation
overview, and the repository README.").

This is a reconciliation finding, not a unilateral scope addition — it
requires explicit contribution-auditor and infosec-sentinel confirmation
before Gate 1, since it changes what "the four recovery-patch items" in
`hapn`'s original framing implies. If either reviewer disagrees that this
belongs in v2.4.6, the alternative is an explicit deferral to the next
release with a stated reason, not silent omission either way.

### Group C — explicit exclusion: CI-only work (11 commits, not release content)

`docs/plans/first-push-ci-changes-fallback.md` (6 commits) and
`docs/plans/interruptible-superseded-pipelines.md` (5 commits, including
implementation commit `cb14a740`). Both touch only `.gitlab-ci.yml` and
their own plan files — this project's own CI configuration, not any path
under `agf/dist/release-manifest.yaml`'s distributed content. Neither
changes `agov`/`agov-mcp` binary behavior, framework schemas, or
adopter-facing documentation.

Explicitly and doubly excluded by prior governed decisions:
`20260729T185725Z-governor-reviewer-m5qp` parked v2.4.6 release work
specifically so these CI-mechanism changes could proceed first as
independent, revertible units; `8rph` then resumed v2.4.6 "in parallel"
while confirming no future/adjacent tooling work is pulled into its
scope. No `agf/CHANGELOG.md` entry — this changelog covers the `agf/`
distribution, and `.gitlab-ci.yml` is not part of it.

### Group D — explicit exclusion: release-plan revisions (4 commits)

The four commits after the original planning base that touch a
non-coordination path modify only `docs/plans/v246-release.md`: the initial
release plan and its three correction/approval revisions. They are governance
metadata, are not included by `agf/dist/release-manifest.yaml`, and add no
adopter-visible release content. No changelog entry is required.

## Scope

**In (Gate 1 authorizes exactly these mechanical steps):**

1. Confirm the release base is still current `main` at implementation
   time; re-verify Group A/B/C/D reconciliation and explicitly classify any
   later non-coordination commit before proceeding.
2. Create `release-2.4.6` from that base.
3. On the release branch, change exactly five version-metadata files:
   `agf/VERSION` to `2.4.6`; `agf/CHANGELOG.md` from `## Unreleased` to
   `## [2.4.6] — <implementation date>` with Group B under a new
   `### Added`; `agf/templates/AGENTS.md`'s AGF version comment to `2.4.6`;
   `tooling/rust/Cargo.toml`'s workspace package version to `2.4.6`; and the
   generated `tooling/rust/Cargo.lock` package versions for the five workspace
   crates to `2.4.6`. No dependency version or checksum may change.
4. Open the release MR (`release-2.4.6` → `main`) with a diff limited to
   exactly those five files.
5. Require Gate 2 on the release MR before merge.
6. After merge, tag the resulting `main` merge commit `v2.4.6` and push
   the tag immediately — per `release-via-merge-request-rule.md`, this
   requires no additional confirmation once the MR is merged.

**Out (explicitly, not by omission):**

- Any change beyond the five named version-metadata files. In particular, no
  Rust source, dependency version/checksum, schema, rule, or CI change is in
  scope; every substantive change is already on `main`, separately governed.
- Issue #53 CI-latency work, the interruptibility post-merge monitoring
  tracker, and the parked MR !772 branch-scope rule — per
  `v246-recovery-patch.md`'s own exclusions, still controlling.
- The future-tooling plans in MRs !805–!807 (`agov` coordination/MCP
  reliability, v2.5.0 discovery milestone) and the CI longitudinal
  causality assessment (MR !810) — per `thnq`, explicitly outside this
  plan's scope and not blockers for it.
- Any v2.5.0-rc.1 scope (adopter role authority #79, `requires-ack`
  semantics #80, remaining #67/#78 items) — deferred per the approved
  v2.5.0 Statement of Work, unchanged by this plan.

## Required validation (release-MR Gate 2 evidence)

Per `hapn` item 4 and `release-via-merge-request-rule.md`'s MR checklist,
at the exact release-MR head:

1. `agf/VERSION`, the Rust workspace/package lock versions, the packaged
   `agov`/`agov-mcp` self-reports, and the adopter template version comment all
   match the intended release version exactly (`2.4.6`).
2. `agf/CHANGELOG.md` contains a `[2.4.6]` entry with a real date, not
   backfilled later; every adopter-observable change named (Group A's
   four items, Group B's roadmap-page entry if confirmed).
3. Version references are consistent across agent definitions and
   documentation (`agov validate` umbrella, which includes
   `agov.validate.readme_stats` and `agov.validate.assistant_loader`
   freshness checks).
4. `agov validate adopter-docs` — adopter-facing documentation coverage
   for every release-relevant change.
5. `agov dist validate` — release manifest structural validation.
6. `agov dist build` (or the CI-equivalent `.rust-tooling-package` /
   `agf:dist:release` jobs) produces both `.tar.gz` and `.zip` archives
   that extract cleanly and run every shipped test with zero failures —
   this is the exact clean-archive fixture Group A's recovery patch
   added, now exercised against the real release build, not only a CI
   simulation.
7. SPDX headers and license consistency checks pass across all files.
8. The full required CI matrix is green at the exact release-MR head —
   not a stale pipeline from an earlier commit on the same branch.
9. `agov plan validate docs/plans/v246-release.md` (this file) and
   `agov validate infosec` both pass.

## Post-tag verification (after the tag is pushed)

Per `immutable-tags-rule.md`, the tag is permanent once pushed — these
checks catch a bad release without ever moving or deleting the tag:

1. `agov dist check-tag` — confirms `CI_COMMIT_TAG` matches
   `agf/VERSION` on the tag pipeline.
2. Automatic, not manual: the tag pipeline's `release:verify` CI job
   extracts the packaged `agov` binary from the just-built release
   archive and invokes that binary's own `dist post-publication-check`
   directly against the live GitLab Release API — independent of the CI
   workspace's `dist/` copy. This composed check anonymously discovers
   and downloads every asset linked from the real published Release,
   validates downloaded provenance records against the downloaded bytes,
   and performs the eligible packaged-binary self-report verification
   internally as part of the same composed check. This is the
   authoritative evidence that the publicly reachable artifact matches
   what provenance claims, not merely what CI built.
3. The repository-local `tooling/agov` wrapper's manual allowlist
   currently accepts only `dist validate|build|check-tag` and rejects
   both `dist self-report-check` and `dist post-publication-check` with
   exit 127 (filed separately as issue #92). This release does not
   promise or depend on that wrapper as a manual post-tag path — the
   tag pipeline's `release:verify` job bypasses the wrapper entirely by
   invoking the packaged binary directly, and #92 stays explicitly
   outside v2.4.6 scope.
4. Manual spot-check: the GitLab Release page lists the expected asset
   set — framework source `.tar.gz`, platform `agov` tooling archives,
   their checksum/provenance sidecars, and the
   `agov-oci-<version>-provenance.json` asset — with no missing or extra
   entry. The Release page does not itself list a downloadable OCI image
   asset; the OCI provenance JSON's contents name the immutable image
   reference/digest that the composed check verifies.

## Rollback and stop conditions

**Before the tag (fully reversible):** if any Gate 2 evidence item fails,
do not merge the release MR — fix on the release branch and re-request
Gate 2. If the release MR merges but a defect is found before tagging,
revert the merge commit on `main` (a normal revert, no tag involved yet)
and restart from a corrected release branch.

**At or after the tag (irreversible per `immutable-tags-rule.md`):** a
pushed tag is never deleted, moved, or retagged, even if it turns out to
be broken. If post-tag verification (above) finds a defect:

1. Do not attempt to fix `v2.4.6` in place.
2. Commit the fix to `main` through the normal MR workflow.
3. Bump to the next patch version (`v2.4.7`) and repeat this same
   release workflow for the corrected version.
4. Record the `v2.4.6` defect and its `v2.4.7` fix in the changelog and,
   if the defect reached a published artifact, as a learning record.

**Missing or inconsistent provenance specifically:** if
`post-publication-check` finds a published asset whose downloaded bytes
don't match its provenance record, or an expected asset is absent from
the Release page, treat this as a release-integrity incident: do not
mark the release verified, open a governed incident record immediately,
and do not proceed with any dependent work (e.g., announcing the
release) until resolved.

## Non-goals

- No v2.5.0-rc.1 scope of any kind.
- No change to the `.gitlab-ci.yml` interruptibility or first-push
  mechanisms — those remain independently governed (Group C).
- No new validator, exit code, CLI flag, or schema field — this release
  is patch-SemVer by definition; anything requiring additive surface
  moves to a v2.5.0-rc.1 candidate instead.
