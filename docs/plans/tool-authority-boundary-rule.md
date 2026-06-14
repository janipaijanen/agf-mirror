---
id: tool-authority-boundary-rule
status: pending-gate1
topic: tool-authority-boundary
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: Tool authority boundaries affect which automated failures may stop work and must not let validators or AI runtimes silently escalate advisory findings into binding security or governance decisions.
  - role: contribution-auditor
    status: consulted
    reason: The change makes an adopter-facing governance rule and repository ADR; auditability, role authority, and blocking-versus-advisory semantics must remain explicit.
  - role: fact-checker
    status: consulted
    reason: Documentation and validator output must distinguish currently enforced checks from advisory evidence and proposed future enforcement.
---

# Plan: Tool Authority Boundary Rule

## Intent

Implement the planner-architect direction from seq 289: make the AGF
tool-authority boundary explicit before adding more automation.

The governing principle is:

> A tool may block work only when a named rule in `agf/rules/` or the adopter's
> local rule set explicitly grants that check blocking authority. All other tool
> failures are advisory evidence.

This preserves AGF's dogfooding model:

- `agf/` is the adopter-facing product and normative framework;
- `tooling/` validates, packages, builds, summarizes, and exercises the
  framework;
- this repository is the first adopter and live evidence corpus;
- role decisions and named rules remain the source of governance authority.

## Background

Seq 287 proposed the dogfooding/tooling authority frame. Seq 289 accepted the
frame and requested a Gate 1 plan before implementation. The accepted targets
are:

- `agf/rules/always/tool-authority-boundary-rule.md`;
- `docs/adr/YYYYMMDD-dogfood-authority-boundary.md`;
- one existing validator updated to emit `advisory` versus `blocking` output;
- positive and negative fixture coverage for that validator.

The plan intentionally keeps `agf/` and `tooling/` in one repository for now.
It clarifies authority; it does not split the codebase.

## Rule Scope

Add `agf/rules/always/tool-authority-boundary-rule.md` as an adopter-facing
rule. The rule should state:

- normative authority lives in framework files, local adopter rules, and role
  decisions;
- tools provide reproducible evidence over those files and records;
- tool output is advisory unless a named rule grants blocking authority;
- a blocking check must name the rule that gives it blocking authority;
- AI runtimes, reviewers, evaluators, MCP servers, and local models are
  participants or instruments, not governance authorities by brand or speed;
- validator authors must document whether a finding is advisory or blocking.

## ADR Scope

Add `docs/adr/YYYYMMDD-dogfood-authority-boundary.md` recording this project's
decision to adopt the rule for AGF-NG itself.

The ADR should record:

- the three-role dogfooding model: `agf/` as product, `tooling/` as
  implementation layer, repository as first adopter;
- why circular authority must be avoided;
- why the rule belongs in `agf/rules/always/` rather than only in `AGENTS.md`;
- how this interacts with high-clockspeed deliberation: agents and validators
  create evidence, synthesis lifts durable outputs, governor-reviewer decisions
  remain binding.

## Validator Slice

Update one existing validator to expose whether its findings are advisory or
blocking. The preferred first candidate is the coordination-message validator
because the coordination schema already grants hard validation authority over
message front matter and handoff shape.

Implementation should:

- preserve existing exit-code behavior for current blocking failures;
- add a machine-readable or plainly parseable severity field using the
  convention `severity: advisory | blocking`;
- name or document the rule/schema that grants blocking authority;
- add positive and negative fixture coverage;
- avoid changing unrelated validators in the same slice.

If implementation discovery shows that the plan validator is a cleaner first
candidate, coder-tester may switch to the plan validator only if the Gate 2
handoff explains the reason and preserves the same acceptance criteria.

## Documentation Scope

- Reference the new rule from public/tooling docs where validator authority is
  described.
- Add a one-line reminder to `AGENTS.md` for agents adding automation:
  tools may block only when a named rule grants blocking authority.
- Avoid claiming that all existing validators already report severity until
  they do.

## Out of Scope

- Splitting `agf/` and `tooling/` into separate repositories.
- Reworking every validator to report severity in this first slice.
- Changing gate authority or governor-reviewer approval rules.
- Treating tool output, AI model output, or evaluator volume as a substitute for
  synthesis and role decisions.
- Building MCP, OpenCode, OpenClaw, local-model, or dashboard integrations.
- Enforcing Public Corner / Bulletin Board lift rules in tooling.

## Acceptance Criteria

- The new adopter-facing rule exists at
  `agf/rules/always/tool-authority-boundary-rule.md`.
- The rule includes the canonical blocking-authority statement from seq 289.
- The ADR records AGF-NG's dogfooding authority model and cites seq 287 and seq
  289.
- One existing validator reports or emits `severity: advisory | blocking` for
  its findings without weakening current blocking behavior.
- Positive and negative tests cover the selected validator's severity output.
- Docs state which part is currently implemented and do not imply that every
  validator already has severity metadata.
- `AGENTS.md` includes only a short reminder, not the normative rule body.

## Validation

- `git diff --check`
- `mix agov.plan.validate ../../docs/plans/tool-authority-boundary-rule.md`
- `mix agov.coord.validate ../../docs/coordination/<gate1-handoff>.md`
- `mix test`
- Direct positive and negative tests for the selected validator's severity
  output.
- Documentation search for stale claims that all tooling is already severity
  annotated.

## Risks

- Severity labels could become decorative if they do not name the rule that
  grants blocking authority.
- Too broad a first implementation could churn multiple validators and obscure
  the authority model.
- A tool might be perceived as the decision-maker even when it only exposes a
  blocking failure; docs should keep role authority visible.
- Adding an ADR directory on this branch may need reconciliation with other
  branch-local ADR work if it exists elsewhere.
