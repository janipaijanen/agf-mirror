---
id: agf-v2-layout-s1
status: pending-gate1
topic: agf-v2-layout
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: directory rename and documentation; no credentials, auth, network, or secret handling
  - role: contribution-auditor
    status: pending
    reason: this is an adopter-facing distribution change; contribution-auditor should review before Gate 2
---

# Plan: AGF v2.0 Layout S1 — Create `agf/` Landing Directory

## Intent

Create the canonical `agf/` landing directory that downstream adopters receive
when adopting AGF v2.0. This supersedes `agents-govern-2.0/` as the v2.0
distribution artifact.

The `agf/` directory is the single directory an adopter copies into their
project to adopt the framework. Everything an adopter needs to read, customise,
and govern with is under `agf/`. Everything foundry-specific (learnings,
internal tooling, CI configuration) stays outside it.

## Background

- Governor decision: seq 204 (single-directory layout for v2.0+)
- Directory name approved: seq 209 (`agf/`, visible, no version suffix)
- Implementation constraints: seq 208 (not a rename-only; paths must be
  root-discovered or configurable)
- v1.x preservation: `agents-govern/` must remain unchanged (session constraint)

## Proposed `agf/` Content Boundary

| Source | Destination in `agf/` | Rationale |
|---|---|---|
| `spec/rules-template/` | `agf/rules/` | Adopters copy this to `.claude/rules/` or equivalent |
| `spec/schemas/` | `agf/schemas/` | Coordination message schemas |
| `agents-govern-2.0/framework.md` | `agf/framework.md` | Core framework document |
| `agents-govern-2.0/ADOPTING.md` | `agf/ADOPTING.md` | Adoption guide |
| `agents-govern-2.0/agents/` | `agf/agents/` | Agent definitions |
| `agents-govern-2.0/docs/` | `agf/docs/` | Framework documentation |
| `agents-govern-2.0/CHANGELOG.md` | `agf/CHANGELOG.md` | Version history |
| `agents-govern-2.0/LICENSE` | `agf/LICENSE` | License |

**Not included in `agf/`:**

- `agents-govern-2.0/learnings/` — foundry-specific; adopters start their
  own `learnings/` from scratch
- `tooling/` — foundry-internal; adopters use published releases, not sources
- `agents-govern/` — v1.x release; stays unchanged
- `docs/coordination/`, `docs/plans/` — foundry coordination; not adopter content

## Affected Files

| File / Directory | Action |
|---|---|
| `agf/` | Create; populate per content boundary above |
| `agents-govern-2.0/` | Retain as-is (source of truth until superseded by `agf/`); deprecation is a future slice |
| `AGENTS.md` | Add `agf/` reference; note it as the v2.0 landing dir |
| `README.md` (if present) | Update to reference `agf/` for v2.0 adoption |
| `docs/coordination/README.md` | No change (foundry coordination, not adopter-facing) |

No schema changes. No changes to `agents-govern/`. No tooling changes in S1.

## What S1 Does NOT Include

- Renaming or removing `agents-govern-2.0/`
- Updating tooling defaults (Mix task `--coord-dir`, `--manifest` defaults
  still point at `../../agents-govern/` and `../../docs/coordination`; these
  are a separate slice)
- Publishing or distribution CI changes
- Migrating existing adopters from Layout A/B
- Removing `spec/rules-template/` or `spec/schemas/` (they remain as the
  source; `agf/rules/` and `agf/schemas/` are copies managed in sync)

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| `agf/rules/` and `spec/rules-template/` diverge | Define `agf/` as the canonical; `spec/rules-template/` either becomes a symlink or is retired in S2 |
| Windows path limit hit by `agf/rules/workflow/...` | Audit deepest paths; current deepest is ~55 chars below the landing dir root — within budget |
| Adopters confuse `agf/` (v2.0) and `agents-govern-2.0/` (working copy) | Document clearly in AGENTS.md and `agf/ADOPTING.md` |
| `agents-govern-2.0/` references in coordination history | No action needed; historical references are not updated retroactively |

## Gate 1 Questions

**Q1 — Content boundary:** Is the proposed content boundary correct?
Specifically: should `agf/agents/` contain all agent YAML files from
`agents-govern-2.0/agents/`, or only the role-definition files (excluding
foundry-specific agents like `contribution-auditor` if those are
foundry-internal only)?

**Q2 — Sync vs. copy:** Should `agf/rules/` be a copy of `spec/rules-template/`
(two canonical sources to maintain) or should `spec/rules-template/` be retired
immediately and `agf/rules/` become the sole source? S1 recommendation: copy
first, retire in S2 once `agf/` is validated.

**Q3 — `agents-govern-2.0/` retirement:** Should `agents-govern-2.0/` be
deprecated immediately (S1) or retained until `agf/` is validated and the
distribution CI is updated? S1 recommendation: retain; retirement is S2.

## Rollback

Delete `agf/`. No persisted data or schema changes. No CI changes. Fully
reversible.

## Test Strategy

No unit tests for a directory-creation slice. Verification:

1. `ls agf/` — confirm all expected subdirs present.
2. `diff -rq agf/rules/ spec/rules-template/` — confirm content matches source.
3. `mix agov.coord.visibility --base-branch master` — confirm scanner sees
   no new unbridged obligations introduced by this change.
4. Contribution-auditor review before Gate 2 (per advisory_consultation above).
