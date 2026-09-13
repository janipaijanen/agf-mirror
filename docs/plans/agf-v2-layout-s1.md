---
id: agf-v2-layout-s1
status: complete
topic: agf-v2-layout
gate1-approved-by: governor-reviewer
gate1-approved-seq: 215
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: directory restructure and documentation; no credentials, auth, network, or secret handling
  - role: contribution-auditor
    status: consulted
    reason: reviewed in 00224-contribution-auditor.md (2026-06-13), verdict recommend-ready-for-gate2
---

**Correction (2026-09-13, qbka Slice B):** status was `gate1-approved`, an
invalid enum spelling; corrected to `complete` since `agf/` is the current,
live canonical framework directory this plan created. `advisory_consultation`
contribution-auditor entry was `status: pending`, also invalid; corrected to
`consulted` — the review had in fact already happened
(`00224-contribution-auditor.md`) but the frontmatter was never updated
afterward. See `docs/plans/20260913T171837Z-qbka-slice-b-legacy-plan-
migration.md` for full evidence.

# Plan: AGF v2.0 Layout S1 — Create `agf/` Landing Directory

## Intent

Create the canonical `agf/` landing directory that downstream adopters receive
when adopting AGF v2.0, and retire the now-superseded working directories in
the same slice.

The `agf/` directory is the single directory an adopter copies into their
project. Everything they need to read, customise, and govern with is under
`agf/`. Everything foundry-specific stays outside it.

## Background

- Governor decision: seq 204 (single-directory layout for v2.0+)
- Directory name approved: seq 209 (`agf/`, visible, no version suffix)
- Implementation constraints: seq 208 (not rename-only; paths root-discovered
  or configurable)
- v1.x preservation: `agents-govern/` MUST remain unchanged (session constraint)
- Gate 1 answers: seq 215 (governor direct, 2026-06-13)

## Hard Constraint

**`agf/` MUST NOT ship `agents-govern/` content in any form.**

`agents-govern/` is the foundry's internal v1.x governance directory. It is
never part of the adopter distribution. When `agents-govern/` is no longer
needed by the foundry it will be retired, but it stays in git history. Any
review that finds `agents-govern/` content inside `agf/` is a BLOCK.

## `agf/` Content Boundary (Gate 1 approved)

| Source | Destination in `agf/` | Note |
|---|---|---|
| `spec/rules-template/` | `agf/rules/` | Full content; source retired in S1 |
| `spec/schemas/` | `agf/schemas/` | Full content; source retired in S1 |
| `agents-govern-2.0/framework.md` | `agf/framework.md` | |
| `agents-govern-2.0/ADOPTING.md` | `agf/ADOPTING.md` | |
| `agents-govern-2.0/agents/` | `agf/agents/` | ALL agent YAMLs; adopters decide commissioning |
| `agents-govern-2.0/docs/` | `agf/docs/` | |
| `agents-govern-2.0/CHANGELOG.md` | `agf/CHANGELOG.md` | |
| `agents-govern-2.0/LICENSE` | `agf/LICENSE` | |

**Not included in `agf/`:**

- `agents-govern-2.0/learnings/` — foundry-specific; adopters start their own
- `agents-govern/` — v1.x foundry internal; MUST NOT appear in `agf/`
- `tooling/` — foundry-internal sources
- `docs/coordination/`, `docs/plans/`, `docs/proposals/` — foundry coordination

## Retirements in S1

Both source directories retire in S1 (Gate 1 Q2 and Q3: retire immediately):

| Directory | Retirement action |
|---|---|
| `spec/rules-template/` | `git rm -r`; `agf/rules/` is the sole canonical source |
| `spec/schemas/` | `git rm -r`; `agf/schemas/` is the sole canonical source |
| `agents-govern-2.0/` | `git rm -r`; `agf/` is the replacement |

`spec/` directory is retired entirely if `rules-template/` and `schemas/` were
its only contents.

## Tooling Impact (required in S1)

Retiring `spec/schemas/` breaks the Elixir tooling default
`AGOV_SCHEMA_DIR=../../spec/schemas`. This path reference must be updated in
the same S1 commit. Options:

- Update default to `../../agf/schemas`
- Or make the path root-discovered (preferred per seq 208 constraint)

The Mix task defaults for `--coord-dir` and `--manifest` reference
`../../docs/coordination` and `../../agents-govern/agents/MANIFEST.yaml`
respectively — these are not affected by S1 retirements.

## Affected Files

| File / Directory | Action |
|---|---|
| `agf/` | Create and populate per content boundary |
| `spec/rules-template/` | `git rm -r` after populating `agf/rules/` |
| `spec/schemas/` | `git rm -r` after populating `agf/schemas/` |
| `agents-govern-2.0/` | `git rm -r` after populating `agf/` |
| `tooling/elixir/lib/agov/` | Update `AGOV_SCHEMA_DIR` default |
| `AGENTS.md` | Reference `agf/` as v2.0 landing dir |

No changes to `agents-govern/`. No coordination schema changes.

## Implementation Sequence

1. Create `agf/` and populate from source directories (verify content match before deleting sources)
2. Update tooling `AGOV_SCHEMA_DIR` default to `../../agf/schemas`
3. Run `mix test` — all tests must pass
4. `git rm -r spec/rules-template/ spec/schemas/ agents-govern-2.0/`
5. Run `mix test` again — confirm no test references the retired paths
6. Update `AGENTS.md`
7. Run `mix agov.coord.visibility --base-branch master` — confirm clean

## Rollback

Before retirement commits are made: delete `agf/`, revert tooling change.
After retirement: restore from git history (`git checkout HEAD~N -- spec/ agents-govern-2.0/`).

## Test Strategy

No new unit tests. Verification checklist:

1. `ls agf/{rules,schemas,agents,docs}/` — all expected subdirs present
2. `diff -rq agf/rules/ <git-show HEAD~1:spec/rules-template/>` — content matches source snapshot
3. `mix test` — 125 tests pass after tooling update and retirements
4. `mix agov.coord.visibility --base-branch master` — no new unbridged obligations
5. Contribution-auditor review before Gate 2 (advisory, required)
