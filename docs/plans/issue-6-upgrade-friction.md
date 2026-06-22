---
id: issue-6-upgrade-friction
status: pending-gate1
topic: upgrade-friction
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      The change would inspect assistant-loader and tool-specific configuration
      directories such as .claude/. The first slice must report drift without
      exposing secrets, local handles, or private prompt material.
  - role: contribution-auditor
    status: consulted
    reason: >
      The change alters adopter upgrade expectations and `agov validate`
      semantics. It must preserve the advisory-versus-blocking boundary and
      avoid presenting partial validation as complete governance proof.
---

# Gate 1 Plan: Issue #6 Upgrade Friction

**Issue:** GitLab #6, "Upgrade friction from 2.1.0 to 2.1.4"  
**Author:** planner-architect / Codex draft  
**Date:** 2026-06-22

## Intent

Reduce adopter confusion during AGF 2.1.x upgrades when a project has stale
copied assistant files outside `agf/`, and make optional `agov` validation
honest about which governance records it does and does not validate.

The incident in issue #6 shows two separate failure modes:

- copied tool-specific assistant files under `.claude/` can drift from
  canonical `agf/` rules and skills;
- when optional tooling is used, `agov validate` can pass aggregate framework
  checks while direct `agov plan validate` or `agov coord validate` still fail
  on project-owned governance records.

## Proposed First Slice

Implement a narrow Rust tooling and docs change:

1. Add advisory drift detection for copied assistant rule/skill trees:
   - detect `.claude/rules/` when `agf/rules/` exists;
   - detect `.claude/skills/` when `agf/skills/` exists;
   - compare matching files by content hash;
   - warn when files are missing, extra, or differ from the canonical `agf/`
     counterpart;
   - do not read or report `.claude/settings.local.json`, prompt transcripts,
     logs, caches, or secrets.
2. Surface the drift check from `agov validate` as advisory output, not a
   blocking failure.
3. Update `agov validate` output to state coverage explicitly for teams that
   choose to use the optional CLI:
   - framework/adoption records validated;
   - plan and coordination files are not validated by the umbrella command;
   - use `agov plan validate docs/plans/*.md` and
     `agov coord validate docs/coordination/*.md` for those records when
     `agov` is installed, or perform the equivalent manual front-matter/schema
     review when operating without tooling.
4. Update adoption/loader/upgrade docs to make the migration path concrete:
   - remove copied `.claude/rules/` and `.claude/skills/` trees unless they
     are intentionally project-owned;
   - keep only a small loader that points at `agf/`;
   - after upgrade, run the umbrella validator and explicit plan/coord
     validators if the optional CLI is available; otherwise document the
     manual checks performed for copied assistant files, plans, and
     coordination records.

## Out of Scope

- A full `agov adoption drift` or `agov upgrade` command.
- Automatic rewriting or deletion of `.claude/` files.
- Blocking CI status for drift warnings.
- Validation of all historical `docs/plans/` and `docs/coordination/` files
  from `agov validate`; historical repositories may contain pre-contract
  records that are intentionally not clean.
- General support for every assistant vendor directory in the first slice.

## Why Advisory First

Copied `.claude/` files may be intentional local loaders, local project rules,
or old migration debris. A blocking check would create false positives unless
AGF first has a recorded ownership convention for those directories. Advisory
warnings are enough to prevent the specific false confidence from issue #6
without turning tool output into a new approval authority.

## Acceptance Criteria

- A repository with `.claude/rules/foo.md` differing from
  `agf/rules/foo.md` receives an advisory warning from `agov validate`.
- A repository with `.claude/rules/foo.md` missing while `agf/rules/foo.md`
  exists receives an advisory warning.
- A repository with only a small loader file, and no copied `.claude/rules/`
  or `.claude/skills/` tree, receives no drift warning.
- `agov validate` output includes a short coverage note that plan and
  coordination validation are separate commands when the optional CLI is used.
- Documentation tells 2.1.x upgraders how to migrate away from copied
  assistant rule/skill trees without making the `agov` binary a prerequisite
  for AGF adoption.
- Tests cover drift warnings, no-warning loader-only layout, and the coverage
  note.

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| False positives for intentionally project-owned `.claude/` files | Advisory warnings only; docs explain how to keep project-owned local rules distinct from copied AGF files. |
| Secret or prompt exposure while scanning `.claude/` | Only inspect `.claude/rules/` and `.claude/skills/`; report paths and hash equality, not file contents. |
| Umbrella validation becomes too noisy | Keep the output compact: one coverage note plus summarized drift warnings. |
| Scope creep into upgrade automation | Defer dedicated `agov adoption drift` or `agov upgrade` until this first slice has adopter feedback. |

## Validation Plan

- `cargo test -p agov-validators`
- focused `cargo test -p agov`
- `cargo run --quiet -p agov -- validate`
- `./tooling/agov plan validate docs/plans/issue-6-upgrade-friction.md`
- `./tooling/agov coord validate <handoff>`
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict`

## Gate Request

Approve this as a first implementation slice for issue #6, or redirect it to a
docs-only patch if the governor-reviewer wants to avoid new validator behavior
until more adopter upgrade evidence exists.
