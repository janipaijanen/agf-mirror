---
id: 20261025T183939Z-v211-changelog-and-scope-reconciliation
status: pending-gate1
topic: v211-changelog-and-scope-reconciliation
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      Requested at publication. The plan edits only changelog prose and names no credential, secret or
      cross-project link; the CI `agov validate infosec` job is the mechanical evidence, not a verdict.
  - role: contribution-auditor
    status: pending
    reason: >-
      Requested at publication. An independent (non-author) review is needed because the plan classifies
      adopter-facing changes (BREAKING/ADDITIVE) that the governor will rely on at the release cut.
affected_paths:
  - CHANGELOG.md
  - agf/CHANGELOG.md
depends_on:
  - 20260909T021841Z-v211-release-scope
  - 20260909T203636Z-v211-release-publication
  - 20261024T070435Z-timer-reality-polling-alignment
  - 20261021T220000Z-retire-windows-support
  - 20261023T083302Z-remove-elixir-leftovers
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: v2.11.0 changelog and scope reconciliation (child of the approved v2.11 scope plan)

Requested by codex-foundry in ACS seq 466/468 (option B). Draft by planner-architect. Nothing here is
approved; no changelog, VERSION, release, tag or deploy action happens before a governor-reviewer record.

## Parent and what this plan does not change

Parent: `docs/plans/20260909T021841Z-v211-release-scope.md` (`candidate_release: v2.11.0`, Gate 1 `tsbh`).
This child changes **none** of its release-cut conditions (1-4), its workstreams, its sequencing, or the
separate publication plan `20260909T203636Z-v211-release-publication`. It only reconciles the
`[Unreleased]` changelog text with what is on `main` and records classification evidence. Whether the cut
happens, when, and its tag stay with the governor.

## Problem

Evidence: read-only audit in ACS seq 456/459 over `v2.10.1..origin/main` at `d648d151`; 37 first-parent
merges touch `agf/`, `tooling/` or CI.

- `agf/CHANGELOG.md` `[Unreleased]` holds one entry (Windows retirement) and a stray line, "Adopter docs:
  not applicable — this MR activates the already-documented optional Improvement Propeller only for
  foundry...", that belongs to no entry. Root `CHANGELOG.md` holds two entries (Podman pull routing,
  Windows retirement).
- About 14 shipped changes have no entry and no `Adopter docs:` line (list below).
- Timer-reality (MR !3205, merge `8bfdc25f`) is merged but unclassified in the changelog.

## Scope and changes (implementation, after Gate 1, in one MR)

Edit only `agf/CHANGELOG.md` and root `CHANGELOG.md` `[Unreleased]`. Add one entry per change, in the
classes below, each with `Adopter docs: updated | not applicable | waived <id>` after a check that the
claimed docs exist (see Validation). Classes are the reading from the audit; the governor may change any.

| Class | Change (commit / MR) |
|---|---|
| BREAKING | MCP strict unknown-argument rejection, 57527ede (!3191): unpublished keys now fail with a JSON-RPC error. Migration: remove extra keys; see `adopt/mcp-server.md`. |
| BREAKING | Windows tooling preview retired, c2a73e5e: entry exists; check wording and docs line only. |
| BREAKING | Timer-reality alignment, 8bfdc25f (!3205): poll-and-coordinate skill default idle window 240 to 180 min; copied blanket background-process bans replaced by a pointer to "Scheduling and notification honesty". Migration text is in the !3205 description. |
| ADDITIVE | `agov coord list --ref` and MCP `ref` argument, c1999c8e (!3193) |
| ADDITIVE | Publication holds and `hold-status`, 68003b56; challenge-response rule and priority ordering, cbfa285b; self-announcing supersession, 78fc52a7; `superseded` plan status and legacy migration, e9f7742d, 0b9d21ff |
| ADDITIVE | Plan `advisory_consultation` value `pending`, 4a00dc6e (issue 130); plan `depends_on`, command default still advisory, bcc76b04 |
| ADDITIVE | Improvement Propeller skill, 32fd0134; third-party plugin coexistence doc, 08d3a2ce; governance-mode LIGHT/NORMAL/STRICT, 7efe8867; high-risk-operations domain customization, 5216a2ef |
| EDITORIAL / PREVENTIVE | Elixir release-guard narrowing, 7588a7c5 (five forbidden globs removed from `agf/dist/release-manifest.yaml`; accepted by Gate 1 `dhdh`, test gap recorded in ACS seq 459); coordination schema compile-once, d4cc426a; docs-estate fixes (046cec8a, bfc4d648, 7bd189b9, 4f7b6b95, 2165b579, a507cadd, 52892835) |
| NOT SHIPPED | Coordination and plan records, learnings, DinD removal (7aab12ac, CI only), docs-only Elixir reference cleanup f13f1524 beyond the classified item above: no entry. |

Stray line repair: attach the "Adopter docs" sentence to the Improvement Propeller skill entry if its text
matches that change; if it does not match any entry, remove it. The implementer records which in the MR.

## Classification and impact

Changelog prose only; no code, schema, rule, CI or release-config change. Impact: Foundry: none.
Direct adopters: more accurate upgrade notes; the BREAKING entries make existing behavior changes visible.
Downstream: none. Migration: none for this plan itself (the BREAKING items carry their own). Evidence: audit
ACS seq 456/459; commits above.

## Rule-projection screen

No Pages page or runbook gains or changes normative guidance, so `documentation-rule-projection-rule.md`
does not trigger and no manifest is added. The classes follow `always/downstream-impact-rule.md`.

## Tests and acceptance

- `agov plan validate` on this plan; `agov coord validate` on the handoff.
- After implementation: `agov validate adopter-docs` passes; every new entry has an `Adopter docs:` line; a
  grep that every commit in the table appears by SHA or MR in exactly one entry; `git diff --check`; the diff
  touches only the two changelog files.
- Gate-self-test: not applicable; no gate is introduced or modified.

## Collision and sequencing disposition

Scan at 2026-10-25T18:39Z against `origin/main` and all open MRs: no open MR touches `CHANGELOG.md` or
`agf/CHANGELOG.md`. The parent plan is cited, not edited. Any release-metadata MR (`VERSION`, version
headings) is separate and later; this plan must merge before it so the cut moves `[Unreleased]` complete.

## Risks and rollback

Risk: a wrong class misleads adopters. Mitigation: the governor and the contribution-auditor review the
table; each row cites its commit. Residual: docs-estate items were classed from titles only; the implementer
reads each diff once before writing its line. Rollback: revert the implementation MR; no other artifact moves.

## Explicitly out of scope

`VERSION`, version headings, release notes pages, tags, packages, deploy, namespace checks, the parent
plan's conditions, any code, schema or rule edit, and the closure-evidence pack for old issues.
