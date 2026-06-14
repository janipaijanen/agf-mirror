---
id: tool-authority-boundary-rule
scope: always
applies_to: [llm-agent, human-agent, ci]
evidence: seq-287-seq-289
upstream_candidate: true
strength: MUST
status: active
see_also: [high-risk-operations-rule, downstream-impact-rule]
provenance: foundry-native
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Tool Authority Boundary
#
# Framework principle: Tools provide evidence; named rules grant authority
# See: framework.md — Principle 5 (Escalation over Improvisation)
# Evidence: seq-287 (dogfooding model), seq-289 (decision record)

**Scope:** Every automated check, validator, CI job, MCP server, AI runtime,
evaluator, or local model that produces output bearing on a governance decision.

## The principle

> A tool may block work only when a named rule in the adopter's rule set
> explicitly grants that check blocking authority. All other tool output
> is advisory evidence.

Normative authority lives in framework files (`agf/rules/`, `agf/agents/`),
local adopter rule sets, and explicit role decisions. Tools — validators,
CI jobs, MCP servers, AI runtimes, local models — provide reproducible
evidence over those files and records. Evidence supports decisions; it does
not replace them.

## Three levels of tool output

| Level | Label | Effect | Who decides |
|---|---|---|---|
| Blocking | `severity: blocking` | May halt work if named rule grants this | The named rule |
| Advisory | `severity: advisory` | Surface for human or role review; do not halt work | Reviewer or role |
| Informational | (no severity) | Logged, not surfaced in review | — |

A validator or CI job MUST declare which level its findings carry. Undeclared
output is treated as advisory.

## Rules for validator authors

- **Name the rule that grants blocking authority.** A blocking check without a
  named backing rule is undocumented authority. Document it before shipping.
- **Emit `severity: blocking` or `severity: advisory` on each output line** so
  consumers can triage without reading source code.
- **Preserve exit codes.** Blocking findings exit non-zero. Advisory findings
  do not change the exit code.
- **Do not promote advisory findings to blocking** without a rule update first.

## Rules for agents and reviewers

- Do not treat advisory tool output as a blocker. Route it to the appropriate
  role for decision.
- Do not treat volume of advisory findings as a substitute for a named rule
  or a role decision.
- When a tool blocks work, verify that a named rule grants that authority
  before accepting the block.

## Applies to all tool types

- Validators (`agov coord validate`, `agov plan validate`, custom validators)
- CI jobs and pre-push hooks
- AI runtimes, evaluators, MCP servers (regardless of vendor or speed)
- Local models used as review or scoring instruments

AI runtimes, evaluators, and local models are participants or instruments in
the governance process. They do not hold governance authority by virtue of
their model class, speed, or volume of output. A high-confidence model output
is evidence; it becomes binding only when a named role accepts it through a
documented gate.

## Exemption: emergency authority

A role with documented emergency authority (see `workflow/deputy-governor-rule.md`)
may treat advisory findings as blocking for a time-bounded session. The
delegation artifact MUST name this elevation and its expiry.

## Relationship to dogfooding

In a self-governing repository — where the framework being built is also used
to govern itself — circular authority is a specific risk: a tool that validates
framework files must not itself become the source of framework authority. This
rule is the guard:

- `agf/` is the adopter-facing product and normative framework.
- `tooling/` validates, packages, and exercises the framework.
- The repository is the first adopter and live evidence corpus.
- Role decisions and named rules remain the source of governance authority.

See `docs/adr/20260614-dogfood-authority-boundary.md` for the AGF-NG decision record.
