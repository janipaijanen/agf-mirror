---
id: 20261101T115616Z-release-content-inventory
status: pending-gate1
topic: release-content-inventory-and-release-plan-options
depends_on:
  - 20261030T160040Z-forge-evidence-rule
  - 20261030T163253Z-pages-structure-and-voice
  - 20261030T210529Z-pages-source-consistency-g1-g5
affected_paths:
  - docs/plans/20261101T115616Z-release-content-inventory.md
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: >-
      Review not yet requested. A release-content plan that includes a BREAKING
      rule needs contribution review before Gate 1; the request goes with the
      Gate 1 handoff for this plan.
  - role: infosec-sentinel
    status: pending
    reason: >-
      Review not yet requested. The plan changes no code, secret, privacy or
      supply-chain surface; the request goes with the Gate 1 handoff so the
      governor can waive or route it.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: release content inventory and release plan options

Governance mode: NORMAL (planning only). This plan edits no shipped rule, page,
loader, changelog, version file or tag. It **names no version number, milestone
or date** and selects no release content. Authority for the drafting: governor
decision `20261101T115418Z-governor-reviewer-gqha`.

Per `docs/roadmap.md`, no later version number or release scope is binding
because it appears in a proposal. A release needs its own governed plan, Gate 1
approval, exact-head Gate 2 decision and explicit tag and publication
authorization. This plan is the input for that choice, not the choice.

## Observation (evidence from Git, read-only)

- Latest stable release: v2.13.1, annotated tag at commit
  `db300c090527817a9d556e13f25d414ab75e09bf` (`docs/roadmap.md`).
- `origin/main` at observation: `75bcbd9608a2411066c8a228e909c9f7909e9a38`.
  Later merges may change the inventory; the plan that follows re-observes it.
- Since the tag, `main` has 58 first-parent merges. About 91 changed paths are
  coordination records and 17 are review evidence.
- **Adopter-facing paths changed since the tag** (`git diff --name-only
  db300c09 origin/main`): 19 files under `agf/docs/site`, two figure files, one
  each under `agf/docs/reference` and `agf/docs/governance`,
  `agf/agents/MANIFEST.example.yaml`, `agf/ADOPTING.md`, `agf/learnings/foundry.yaml`
  and `agf/CHANGELOG.md` (a header edit only). **Nothing under `agf/rules`,
  `agf/schemas`, `agf/templates`, `tooling/` or the CI configuration changed.**
- `agf/CHANGELOG.md` `## [Unreleased]` is empty, so none of the adopter-facing
  work above has a changelog entry yet. Release rule: each adopter-relevant entry
  needs an `Adopter docs: updated | not applicable | waived <id>` line.

## Inventory of candidate content

| # | Candidate | State | Adopter impact | Notes |
|---|---|---|---|---|
| A1 | Pages slices 1 to 4 (who does what, should I adopt, how do I start, how do I work) | Merged | Docs | `6bd6cfb0`, `5faa9d4e`, `2c4ff431`, `5a610485` |
| A2 | Pages communication-authority edits | Merged | Docs | `5b4c3f11` |
| A3 | Source-consistency G1 to G5 and the G4/G5 alignment | Merged | Docs | `e4618ff4`, `b592022d`; plan `20261030T210529Z-pages-source-consistency-g1-g5` |
| A4 | Role-table corrections in install-and-layouts and quick-reference; ADOPTING.md role selection and trigger-role changes; `MANIFEST.example.yaml` skipped-role example | Merged | Docs and an example file | Edits to `agf/ADOPTING.md` and an example manifest change what an adopter reads at setup |
| A5 | Scope supplement S1 and sources/radar batch 43 | Merged | Docs; sources/radar are project-internal | `37828e4a`, `26fe2dc1` |
| A6 | Solo-role calibrated learning record | Merged | Learning record in `agf/learnings/foundry.yaml` | `700f4d72` |
| A7 | Changelog SPDX header repair | Merged | Header only | `4076884d` |
| B1 | Telemetry method page (!3466) | Draft MR, green pipeline | Docs | Not mine; no notes |
| B2 | Pages slice 5, Home rewrite (!3467) | Draft MR, green pipeline | Docs | Not mine; no notes |
| B3 | Code toolbar space plan and CSS (!3471, S2) | Draft MR, Gate 1 draft plan | Docs site CSS | Needs its own Gate 1 |
| C1 | Forge-publication evidence rule (plan on !3426) | Plan awaiting Gate 1; D1 option A | **BREAKING**, major release only | Cannot ship in any v2.x release; needs a separately approved major-release sequence |
| D1 | Queue convergence B2 to B8 | Sequenced in `docs/roadmap.md` | Internal | Not release content unless a plan says so |
| D2 | `affected_paths`-vs-actual-diff drift check | Unclaimed follow-up | Tooling | Needs its own Gate 1 plan |
| D3 | `release-v2.7.0-rc.2` fate | `awaiting-governor` | None | Does not block |
| E1 | 40 plans in `pending-gate1` | Open | Mixed | Mostly old; not triaged here |

## Release lanes

Two independent lanes follow from the evidence. Neither is selected here.

**Lane R-A: docs-only release from current `main`.** Content: A1 to A7, plus any of
B1 to B3 that merge first. No rule, schema, template, tooling or CI change. The
v2.13.1 release (`!3394`) was a docs-focused release and is the precedent for how
the project has classified such a change. Preconditions: changelog entries with
`Adopter docs:` lines; `agov validate adopter-docs`; content-freeze sweep by the
documentation-estate steward; namespace-absence evidence; the standard
release-via-merge-request steps.

**Lane R-M: a major release carrying the BREAKING rule.** Content: C1, and
whatever else the governor chooses to sequence with it. Preconditions: the plan
for C1 reaches Gate 1; a separate, approved major-release plan names scope and
sequence; the migration note exists in the changelog; adopter docs parity.

The lanes do not depend on each other. R-A does not need C1 and must not carry it.

## Decisions for the governor

1. **Lane choice.** Prepare R-A, R-M, both, or neither.
2. **R-A scope.** Release A1 to A7 as they stand, or wait for B1 to B3 to land.
3. **Draft MRs B1 to B3.** Who owns them, and whether they target R-A.
4. **R-M scope.** What, if anything, besides C1 joins a major release.
5. **Version numbers and dates.** Not chosen by this plan. Named only in the
   release plan the governor approves.
6. **The 40 pending-gate1 plans (E1).** Triage later, retire, or leave.

## Out of scope

Any version, date, milestone or tag; any changelog, version-file or rule edit;
implementing C1; the 40 older plans; issues and issue labels (skipped by
instruction); GitLab milestone changes.

## Rollback

Delete or supersede this plan. It changes no shipped artifact.

## Gate 1 request

Approve this inventory and lane structure as the input to a governed release plan,
and answer the six decisions above. Approval of this plan authorizes drafting the
chosen lane's release plan and nothing more: no release, tag, version or date.
