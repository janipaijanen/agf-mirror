---
id: agf-v2-python-cutoff
status: approved
topic: agf-v2-python-cutoff
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-06-14T20:10:15Z
gate1_approved_seq: 335
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: removing Python-era validator guidance changes which security and credential checks adopters are told to run, and must not weaken `.env` or external-artifact screening claims
  - role: contribution-auditor
    status: consulted
    reason: the change affects adopter-facing promises, release contents, migration guidance, and the boundary between AGF v2 product and v1.x bootstrap reference
  - role: fact-checker
    status: consulted
    reason: the cleanup must distinguish true AGF v2 tooling from v1.x/Python historical evidence and from Python used only as MkDocs build infrastructure
---

# Plan: AGF v2 Python cutoff

## Intent

Remove Python tooling as an AGF v2 adopter-facing product premise.

AGF v2 tooling is Elixir/Mix-based. The `agents-govern/` Python line is a
bootstrap/v1.x maintenance reference, not the v2 product surface. AGF v2 docs,
rules, skills, contracts, and release guidance should stop telling adopters to
use `uv run agov-*`, `agov-validate-*`, Python package entry points, or Python
binary distribution paths as the v2 way forward.

## Decision Context

Seq 325 corrects the earlier tooling-distribution premise:

- no Python tooling path for AGF-NG v2.0;
- no Python binaries, installers, package distribution, or SBOM roadmap for
  v2.0 tooling;
- v1.x Python is maintenance-only for critical/security fixes and narrow
  compatibility repairs;
- the correct future planning target is Elixir tooling distribution.

This plan converts that decision into a scoped cleanup of adopter-facing AGF v2
content.

## Scope

Audit and update AGF v2 product-facing files outside `agents-govern/`:

- `agf/ADOPTING.md`
- `agf/docs/contracts/*.md`
- `agf/docs/site/**/*.md`
- `agf/rules/**/*.md`
- `agf/skills/**/*.md`
- `agf/agents/**/*.yaml`
- `tooling/agov`
- `docs/reference/session-resume.md`
- any current plan or coordination reference that has become operational
  guidance rather than historical record

Required cleanup:

1. Replace v2 instructions that invoke Python validators with the current
   Elixir/Mix path or the repo-local `agov` launcher where it delegates to
   Elixir.
2. Mark remaining Python examples as domain examples only, such as a
   downstream project using Python/Flask, not AGF tooling requirements.
3. Remove or rewrite v2 claims that "the framework ships Python validators" or
   that Python validator modules are the AGF v2 contract of record.
4. Update skills that still direct AGF v2 operators to `uv run agov-health`,
   `uv run agov-validate-*`, or Python release commands, unless the skill is
   explicitly labeled v1.x/bootstrap-only.
5. Update role/manifests/checklists that still use `agov-validate-*` as the
   validator-first instruction for AGF v2.
6. Update `tooling/agov` defaults so AGF v2 role/schema paths prefer
   `agf/agents/MANIFEST.yaml` and `agf/schemas`, while preserving an explicit
   bootstrap override path for foundry self-governance if still needed.
7. Add a grep-style negative validation checklist for AGF v2 release readiness:
   v2 product docs must not contain unqualified `uv run agov-*`,
   `agov-validate-*`, Python package distribution, or Python binary packaging
   instructions.

## Out of Scope

- Deleting or rewriting `agents-govern/`.
- Removing Python from the v1.x maintenance line.
- Replacing MkDocs merely because it runs on Python. MkDocs is documentation
  build infrastructure, not AGF v2 governance tooling. A future docs-toolchain
  decision may revisit it separately.
- Removing project-specific examples for Python applications, such as
  `agf/project-specific-examples/python-flask/`, provided they are clearly
  examples for downstream projects and not AGF tooling requirements.
- Implementing standalone Elixir tooling distribution formats such as escript,
  Mix archive, Burrito release, container image, installer, signing, SBOM, or
  registry publishing. Those belong in the follow-on Elixir tooling
  distribution plan.
- Editing immutable historical coordination records except by adding later
  corrective records.

## Acceptance Criteria

- AGF v2 adopter-facing documentation no longer instructs adopters to run
  Python validators as the v2 path.
- AGF v2 skills and rules no longer present `uv run agov-*` or
  `agov-validate-*` as current v2 tooling.
- Any remaining Python mentions in `agf/` are either:
  - downstream stack examples;
  - historical/v1.x provenance;
  - MkDocs/docs-infrastructure notes; or
  - explicitly marked deferred/non-v2.
- `tooling/agov` delegates only to the Elixir implementation for v2 commands
  and uses AGF v2 defaults where possible.
- `agents-govern/` remains untouched except for separately approved learning
  records.
- The cleanup branch includes a validation note listing the exact search
  commands used to prove the cutoff.

## Validation

Before Gate 2:

```bash
git diff --check
agov plan validate docs/plans/agf-v2-python-cutoff.md
agov coord validate docs/coordination/<handoff>.md
cd tooling/elixir
mix compile --warnings-as-errors
mix test
mix agov.session.resume --base-branch main
```

Negative search checks should include at least:

```bash
rg -n "uv run agov|agov-validate|PyInstaller|Python binary|Python package" agf tooling docs/reference
rg -n "agents-govern/agents/MANIFEST.yaml|../../agents-govern/agents/MANIFEST.yaml" tooling agf docs/reference
```

Findings are allowed only when the surrounding text clearly labels them as
historical v1.x, bootstrap-only, docs infrastructure, or downstream project
examples.

## Follow-On

After this cutoff lands, write the correct Gate 1 plan for AGF v2 Elixir
tooling distribution. That plan should evaluate source/Mix invocation, Mix
archives, escripts, Burrito-style releases, containers, or other Elixir-native
paths without resurrecting Python as the v2 distribution strategy.
