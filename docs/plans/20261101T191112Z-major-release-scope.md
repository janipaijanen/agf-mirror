---
id: 20261101T191112Z-major-release-scope
status: pending-gate1
topic: major-release-scope-carrying-the-forge-evidence-rule
candidate_release: proposed-3.0.0-not-confirmed
affected_paths:
  - docs/plans/20261101T191112Z-major-release-scope.md
depends_on:
  - 20261101T115616Z-release-content-inventory
  - 20261030T160040Z-forge-evidence-rule
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: >-
      Review not yet requested. The scope plan fixes what a BREAKING release
      carries; the request goes with the Gate 1 handoff.
  - role: infosec-sentinel
    status: pending
    reason: >-
      Review not yet requested. The plan changes no code or secret surface; the
      request goes with the Gate 1 handoff so the governor can waive or route it.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Major release scope

Governance mode: NORMAL (planning only). This plan edits no version file, changelog,
tag or shipped artifact. It **proposes** a version and content and asks the governor to
confirm them. It names **no date**. The version, date and content stay governor
decisions: they are excluded from the delegation `20261101T162840Z-governor-reviewer-srra`.

Authority: the Human Governor approved a major release in principle
(`20261101T185810Z-governor-reviewer-7mgm`) and told the session to proceed with release
work. That approval does not choose a version, date or content.

## Baseline (evidence from Git)

- Latest stable release: v2.13.1 (tag commit `db300c090527817a9d556e13f25d414ab75e09bf`).
- `origin/main` at drafting: `d14d59562847ac0bcf0203c9c8f58709baf41595`.
- `agf/CHANGELOG.md` `## [Unreleased]` is empty on `main`. The forge-evidence entry sits on
  Draft MR !3482 only.
- Since the tag, nothing under `agf/rules`, `agf/schemas`, `tooling/` or CI changed on
  `main` (checked in the inventory plan). Adopter-facing changes on `main` are docs, the
  `MANIFEST.example.yaml` example, `ADOPTING.md` role text, figures and one learning record.

## Proposed version

**3.0.0**, because the forge-evidence rule is BREAKING under `downstream-impact-rule.md`
and `agf/framework.md` reserves MAJOR for breaking changes. Confirmation is decision V1.

## IN scope (proposed)

1. **Forge-publication evidence rule** (MR !3482): `forge-publication-evidence-rule.md`,
   its projections and the migration note. Independent review CLEAR at `7da8b01d`. Gate 2
   and merge are separate decisions (S1).
2. **Everything already merged to `main` since v2.13.1** (a standard release ships `main`
   as it stands): Pages slices 1 to 4, communication-authority and source-consistency
   G1 to G5 edits, role-table corrections in `ADOPTING.md`, `install-and-layouts` and
   `quick-reference`, the `MANIFEST.example.yaml` skipped-role example, scope supplement S1,
   the solo-role learning record and the changelog SPDX header repair. Each needs a changelog
   entry with an `Adopter docs:` line, because the Unreleased section is empty.

## Undecided content

Pages Draft MRs !3466 (telemetry method), !3467 (Home rewrite) and !3471 (code toolbar CSS):
include only those that finish independent review and Gate 2 before the freeze (S2).

## OUT of scope

Queue convergence B2 to B8, the `affected_paths` drift check, the `release-v2.7.0-rc.2`
disposition, the 40 old `pending-gate1` plans, the external-AI assessment follow-up, and any
tooling or schema change. None has a plan that targets this release.

## Sequence (each step has its own evidence; this plan approves none)

1. **S1: merge !3482** at a CLEAN exact head with Gate 2 (fresh review if the head changes).
2. **S2: finish or exclude the Pages Drafts.**
3. **Content-freeze sweep** by the documentation-estate steward over the frozen `main`.
4. **Publication plan** (separate Gate 1, like `20261030T064744Z-v2131-release-publication`):
   the version surfaces list, `[Unreleased]` split into `[3.0.0]`, namespace-absence evidence,
   `release-3.0.0` branch and merge request, and tag authorization.
5. **Publication and verification**, including anonymous downloaded-artifact checks.

## Decisions for the governor

- **V1.** Confirm the version number (proposed 3.0.0).
- **S1.** Confirm that !3482 may merge to `main` now. After that, any release cut from `main`
  carries the rule, so no docs-only v2.x release from `main` should precede the major one.
- **S2.** Which Pages Drafts are in, and the freeze point.
- **D.** The release date (not proposed).
- **P.** Who may authorize the tag and publication.

## Risks and rollback

A merged !3482 puts the BREAKING rule on `main` ahead of the release (mitigated by V1/S1
ordering). Rollback of this plan: supersede it. It changes no shipped artifact.

## Out of this plan

No version bump, changelog edit, tag, release, publication or issue closure.

## Gate 1 request

Approve the scope and sequence as input to the publication plan, and answer V1, S1, S2, D
and P. Approval authorizes drafting the publication plan and nothing more.
