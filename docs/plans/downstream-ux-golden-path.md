---
id: downstream-ux-golden-path
status: active
topic: downstream-ux
gate1_approved_by: "governor-reviewer direct request: plan and implement downstream UX golden-path POC"
gate1_approved_at: "2026-06-14T00:00:00+03:00"
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: first-run UX touches install commands, external-sharing guidance, learning examples, CI logs, and decommission safety
  - role: contribution-auditor
    status: consulted
    reason: adoption simplification must preserve governor authority, role separation, immutable coordination, and required advisory consultation
  - role: fact-checker
    status: consulted
    reason: validator and layout guidance must distinguish shipped AGF v2 Mix tooling from planned or v1 Python-era commands
  - role: academic-peer-reviewer
    status: consulted
    reason: auditability and evidence-chain claims in first-run docs must remain methodologically defensible
  - role: governor-reviewer
    status: consulted
    reason: downstream-impact classification and Gate 1 approval are binding human-governor decisions
---

# Plan: downstream UX golden path

## Intent

Make the first downstream AGF adoption path executable end to end. A fresh
adopter should be able to understand the repository, install the framework
files, choose a defensible role set, create the first governance artifacts, run
the validators that actually ship today, complete one first governed change
with valid files, and understand how to back out.

## Implementation scope

- Add a root evaluator `README.md`.
- Add `agf/templates/` with copyable first-run plan, coordination, learning,
  and private codename mapping examples.
- Update the install and adoption docs to state one POC default layout:
  - `agf/` for framework-owned distribution files;
  - `.claude/rules/` or equivalent for assistant-loaded rules;
  - `docs/plans/` for working Gate 1 plans;
  - `docs/coordination/` for the coordination channel;
  - `agf/learnings/<codename>.yaml` for adopter learning records.
- Replace the first-governed-change walkthrough with schema-valid template
  copies and validation commands.
- Clarify current validator packaging: Mix tasks live in `tooling/elixir/` and
  require a full checkout until a standalone tooling distribution exists.
- Add a minimum role commissioning matrix.
- Link the exit/decommission path from the first-run flow.

## Out of scope

- Standalone validator packaging.
- Schema contract changes.
- OpenClaw or other gateway integration.
- Full replacement of the inherited v1.x-shaped manifest page.
- Broad marketing or site redesign.

## Acceptance criteria

- A CTO or student evaluator can identify the product directory, trial path,
  current validator limitation, and exit path from the root README and linked
  pages.
- First-run plan and coordination templates validate with current Mix tasks.
- Docs no longer imply that a release archive containing only `agf/` also
  contains `tooling/elixir/`.
- Role-selection guidance distinguishes active AGF roles from gateway/runtime
  execution surfaces.
- Exit guidance forbids history rewriting as a normal exit step.

## Validation

- `git diff --check`
- `mix agov.plan.validate ../../docs/plans/downstream-ux-golden-path.md`
- `mix agov.plan.validate <copied template plan>`
- `mix agov.coord.validate <copied coordination templates>`
- `mix test`
- Documentation source checks for stale path references.
