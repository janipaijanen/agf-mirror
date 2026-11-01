---
id: 20261101T195927Z-v300-release-publication
status: approved
gate1_approved_by: 20261101T200727Z-governor-reviewer-gksm
gate1_approved_at: "2026-11-01T20:07:27Z"
topic: v300-major-release-publication-through-the-standard-main-path
candidate_release: v3.0.0
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent review `20261101T200725Z-contribution-auditor-krtk` returned CLEAR at
      42eaba5c with two advisory findings (A1, A2), applied in the approval follow-up. An AI
      reviewer; Gate 1 is the Human Governor's, `20261101T200727Z-governor-reviewer-gksm`.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      The same independent review covered the supply-chain and security lens and returned CLEAR
      (workspace-only lockfile changes, namespace and tag controls as v2.13.1). It is an AI
      review, not the CI infosec-sentinel.
affected_paths:
  - AGENTS.md
  - CHANGELOG.md
  - agf/ADOPTING.md
  - agf/CHANGELOG.md
  - agf/VERSION
  - agf/agents/MANIFEST.yaml
  - agf/agents/academic-peer-reviewer.yaml
  - agf/agents/academic-visual-reviewer.yaml
  - agf/agents/build-engineer.yaml
  - agf/agents/capability-developer.yaml
  - agf/agents/coder-tester.yaml
  - agf/agents/contribution-auditor.yaml
  - agf/agents/deployment-engineer.yaml
  - agf/agents/documentation-estate-steward.yaml
  - agf/agents/documentation-writer.yaml
  - agf/agents/fact-checker.yaml
  - agf/agents/governor-reviewer.yaml
  - agf/agents/improvement-propeller.yaml
  - agf/agents/independent-implementation-reviewer.yaml
  - agf/agents/infosec-sentinel.yaml
  - agf/agents/planner-architect.yaml
  - agf/agents/web-ux-reviewer.yaml
  - agf/docs/governance/governance.md
  - agf/docs/governance/manifest.md
  - agf/docs/site/roadmap.md
  - agf/learnings/foundry.yaml
  - agf/templates/AGENTS.md
  - agf/templates/learning-record.yaml
  - docs/plans/20261101T195927Z-v300-release-publication.md
  - docs/roadmap.md
  - tooling/rust/Cargo.lock
  - tooling/rust/Cargo.toml
depends_on:
  - 20261101T191112Z-major-release-scope
  - 20261030T064744Z-v2131-release-publication
  - 20261030T160040Z-forge-evidence-rule
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260916T002012Z-roadmap-refresh-post-v2101-issue130
  - 20261021T220000Z-retire-windows-support
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T045214Z-remove-elixir-references
  - 20261023T080431Z-instruction-context-followup
  - 20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases
  - 20261024T070435Z-timer-reality-polling-alignment
  - 20261025T183939Z-v211-changelog-and-scope-reconciliation
  - 20261025T192020Z-supersedes-adopter-docs
  - 20261027T045944Z-record-only-publication-termination
  - 20261027T230355Z-v212-release-freeze-corrections
  - 20261027T235807Z-v212-release-publication
  - 20261029T204751Z-v213-release-freeze-preparation
  - 20261029T220500Z-v213-release-publication
  - 20261030T144439Z-header-repair-plan
  - 20261030T163253Z-pages-structure-and-voice
  - 20261030T210529Z-pages-source-consistency-g1-g5
  - 20261031T224819Z-solo-role-learning-append
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Publish AGF v3.0.0 through the standard main-branch path

Governance mode: STRICT (release, tag and publication mechanics are a high-risk area).
Planning only. This plan edits no version file, changelog, tag or shipped artifact.

**Authority.** Scope and sequence: `20261101T191112Z-major-release-scope` (Gate 1
`20261101T192401Z-governor-reviewer-qhjr`). Version 3.0.0, target date 2026-11-08 and tag
authority: `20261101T191657Z-governor-reviewer-pr6j`. **The tag and publication are the human
governor's acts.** Releases, tags, versions and dates are excluded from the delegation
`20261101T162840Z-governor-reviewer-srra`, which also expires (2026-11-06T16:00:00Z) before
the target date. **This plan's Gate 1, its metadata Gate 2 and the tag therefore need the
Human Governor** (or a new delegation naming them). The session does not self-approve any of
them.

This plan reuses the mechanics of `20261030T064744Z-v2131-release-publication` and records
only what differs. Where this plan says "as v2.13.1", read that plan's section.

## Candidate and freeze

- Base: `origin/main` at the freeze; observed at drafting `2968f25f92035962b7808fc548b1cfc478f3eb29`.
- Previous release: v2.13.1 (tag commit `db300c090527817a9d556e13f25d414ab75e09bf`).
- Proposed freeze point: **2026-11-05T12:00:00Z** (decision S2). Pages Draft MRs !3466, !3467
  and !3471 are in only if their independent review and Gate 2 finish by then; the rest defer.
- Content: the forge-evidence rule (BREAKING, MR !3482, merged `00ee06f6`) plus everything on
  `main` at the freeze. No tooling, schema, CI or dependency change since v2.13.1.
- At the freeze: re-run the checks below, re-scan open MRs and claims, and run the content-freeze
  sweep by the documentation-estate steward. Any later product change requires a re-freeze.

## Exact release-truth inventory

Observed at `2968f25f`; re-run and reconcile at the freeze, and classify new history rather
than blanket-replacing `2.13.1`.

**A. Current-version fields, changed to `3.0.0` in the metadata MR only:**

| Path | Observed lines | Count |
| --- | --- | --- |
| `AGENTS.md` | 4 | 1 |
| `agf/ADOPTING.md` | 344 | 1 |
| `agf/VERSION` | 1 | 1 |
| `agf/agents/MANIFEST.yaml` | 11 | 1 |
| 16 agent files `agf/agents/<role>.yaml` (`framework_version`), excluding MANIFEST | one per file | 16 |
| `agf/docs/governance/governance.md` | 5 | 1 |
| `agf/docs/governance/manifest.md` | 11, 38, 76 (line 76 is the `to_version` template example) | 3 |
| `agf/learnings/foundry.yaml` | 38 | 1 |
| `agf/templates/AGENTS.md` | 3 | 1 |
| `agf/templates/learning-record.yaml` | 8 | 1 |
| `tooling/rust/Cargo.toml` | 14, 20, 21, 22 | 4 |
| `tooling/rust/Cargo.lock` | 13, 36, 48, 61, 72 | 5 |

Same boundaries as v2.13.1: metadata only; no change to role status or `played_by`, rule
bodies or historical learning entries. `Cargo.toml` changes the workspace version and three
internal dependency versions. Regenerate `Cargo.lock` through Cargo and allow exactly the five
workspace package version changes (agov, agov-core, agov-coord, agov-validators, agov-mcp);
leave external dependencies and checksums byte-identical. Literal check: `git diff -U0
tooling/rust/Cargo.lock` shows exactly five changed lines, each `version = "2.13.1"` becoming
`version = "3.0.0"`. Any other lock change stops for diagnosis. The release MR has the A paths plus the two changelogs.

**B. Latest-published fields, unchanged until verified publication** (then a separate
status-sync MR): `agf/docs/governance/manifest.md` line 18 and the tagged date;
`agf/docs/site/roadmap.md` lines 15, 17, 23, 27; `docs/roadmap.md` lines 10, 12, 17, 25.
Do not copy v2.13.1's job count or evidence into the new release.

**C. Historical and external occurrences** stay unchanged. Do not alter README, derived
distribution manifests, older releases or historical records.

## Changelog text proposed for review

`agf/CHANGELOG.md` `## [Unreleased]` becomes `## [3.0.0] — <release date>`. The date is filled
at the metadata MR from the governor's confirmed date; the target is 2026-11-08.

- **BREAKING (major release only): merge request and issue text cites durable evidence.** The
  existing Unreleased entry for `forge-publication-evidence-rule.md`, with its migration note,
  moves under `### Changed` and the words "This entry is not for any v2.x release" are replaced
  by "Released in 3.0.0". It is the only BREAKING item.
- Documentation and role-text entries for every adopter-facing group merged since v2.13.1, each
  with an `Adopter docs: updated (<paths>)` line. The paths are enumerated from all `agf/docs/**`
  and `ADOPTING.md` changes in `git diff db300c09 <freeze commit>`, plus the example manifest, the
  governance manifest page, the roadmap page and the solo-role learning record.
- Root `CHANGELOG.md` records the repository-level release line only.

## Separate gates and publication controls

As v2.13.1, with these differences:

1. **Gate 1 (this plan):** approved by the Human Governor after independent contribution and
   InfoSec review of the exact head. It authorizes the enumerated metadata and changelog edits,
   the bounded evidence refresh and the later B-only status sync. It grants no merge or tag
   authority.
2. **Release branch:** `release-3.0.0` from the exact reviewed `main` base, in a clean isolated
   worktree under `.worktrees/`. Edit only the A paths and the changelogs.
3. **Namespace preflight and pre-push:** `agov release namespace-check --phase preflight`, and
   again `--phase pre-push` bound to the merge commit, for all four namespaces (Git tag, GitLab
   Release, generic package, OCI tag). Derivation of the package name and OCI reference as
   v2.13.1.
4. **Metadata MR:** first push with `git push -o ci.skip`, then open the MR and verify exactly
   one canonical MR pipeline. No commit-message skip markers. Exact-head success and an
   independent metadata, security and content review.
5. **Metadata Gate 2:** the Human Governor, naming the exact head. Merge without squash; prove
   first parent is the reviewed target, second parent the approved source, and the tree the
   tested metadata tree.
6. **Tag:** the **Human Governor** creates the one annotated `v3.0.0` tag on the exact merge
   commit and pushes it once, after the exact protected-tag rule is in place (access level 40)
   and re-read. Never move or recreate it.
7. **Post-publication:** verify the tag pipeline, the GitLab Release, the generic package, the
   OCI image, checksums and provenance, with anonymous downloads. Then the B-only status-sync MR.

## Acceptance and evidence limits

Acceptance follows the v2.13.1 matrix; the job matrix is derived from the CI rules frozen at the
release base. Counts above are observations at `2968f25f`, not a replacement for the named
field boundaries. Reviews by AI agents are not the CI `infosec-sentinel`. Evidence status:
*Empirically informed* (a repeat of a verified release path); nothing in the major-version bump
changes a tool, a schema or a command.

## Failure handling

Any drift in the frozen base, a nonempty namespace, an ambiguous API result, an unexpected lock
change or a failed exact-head check stops the release and returns to the governor. A published
tag is never deleted or moved; a fix ships as a new version.

## Out of scope

Tooling, schema, CI and dependency changes; any content not on `main` at the freeze; issues;
the post-release source scans; and the choice of a different version, date or authority.
