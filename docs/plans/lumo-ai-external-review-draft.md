---
id: lumo-ai-external-review-draft
status: complete
topic: lumo-ai-external-review
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Drafting an external review response does not change security boundaries,
      schemas, rules, or executable behavior.
  - role: contribution-auditor
    status: consulted
    reason: >
      The draft reformats third-party evaluation feedback into the framework's
      prescribed External Review structure and should preserve uncertainty and
      evidence boundaries accurately.
---

# Lumo.ai External Review Draft

This file is a draft for human review before submission as a GitLab External
Review issue. It is not a Gate 1 implementation plan and should not be posted
without governor-reviewer approval.

**Reviewer type:** external AI (Lumo.ai)
**Reviewer name / handle:** Lumo.ai
**Date:** 2026-06-22
**Framework version:** v2.1 documentation / repository `agf/VERSION` 2.1.1
**Output label:** fit-assessment

## Scope reviewed

**Basis:** docs-only

**Sources consulted:** published `evaluate.md`, `index.md`,
`reference/tooling-conformance.md`, and adoption guidance including
`docs/start/should-you-adopt.md`, as represented in the review. The reviewer
did not read `agf/rules/`, `agf/schemas/`, or `agf/agents/*.yaml`.

## Identity check

- [x] Confirmed. The review identified the canonical repository and treated
  agents-govern as a file-based governance framework, not an agent runtime,
  SDK, hosted service, or ML/model governance product.

## Strengths

- **Clear repository-local operating model.** The review correctly described
  agents-govern as file-based, Git-native, and infrastructure-light: framework
  files live in the adopter repository, assistants load them as instructions,
  and existing CI can run validators. This matches `agf/docs/site/index.md`
  and `agf/docs/site/evaluate.md`.

- **Correct tooling maturity reading.** The review recognized the Rust `agov`
  command surface as deployable adopter tooling and distinguished it from
  future or development-only surfaces. This aligns with
  `agf/docs/site/index.md` and
  `agf/docs/site/reference/tooling-conformance.md`.

- **Honest scope and uncertainty.** The review explicitly disclosed that it
  had not read the rule files, schemas, or agent YAML definitions. That
  behavior follows the uncertainty guidance in `agf/docs/site/evaluate.md` and
  should be treated as a strength rather than a weakness.

- **Useful adoption framing.** The review's pilot-testing guidance is
  consistent with `agf/docs/site/docs/start/should-you-adopt.md`: adopt when
  the audit trail and human gates are worth the overhead, wait when evidence
  or organizational capacity is not yet sufficient, and skip when the project
  does not need AI-governance ceremony.

- **License reading was accurate.** The review correctly identified the
  framework content as CC BY-SA 4.0 and the tooling as MIT, matching the
  release and licensing information exposed from the public documentation.

## Risks / gaps / inconsistencies

- **Role names were simplified too far for a formal review.** The review used
  generic labels such as "Implementer", "Reviewer", and "Governor". Those are
  understandable first-pass descriptions, but a submitted review should use the
  canonical framework role names: `planner-architect`, `coder-tester`, and
  `governor-reviewer`. The public role overview is
  `agf/docs/site/docs/concepts/roles-and-gates.md`; the authoritative role
  contracts live under `agf/agents/*.yaml`.

- **The gate timing should not be described as post-hoc CI enforcement.** The
  review's "only CI-based post-hoc validation" framing is inaccurate. Gate 1
  is before implementation starts, and Gate 2 is before merge. CI and
  validators provide evidence, but the gates are pre-emptive checkpoints, not
  after-the-fact audits. See
  `agf/docs/site/docs/concepts/roles-and-gates.md`.

- **The review did not follow the requested External Review output format.**
  The substantive findings are useful, but `agf/docs/site/evaluate.md` asks
  reviewers to include a label, scope reviewed, identity checkbox, strengths,
  risks/gaps, improvement proposals, evidence basis, and uncertainty. This
  draft restructures the findings into that format.

- **Issue creation may be confusing for anonymous reviewers.** The External
  Review link redirects anonymous users to GitLab sign-in. That is reasonable
  GitLab behavior, but `agf/docs/site/evaluate.md` should say that a free
  GitLab account is required to create an issue.

- **EU AI Act, sustainability, and large-fleet scalability remain fair external
  perspective questions.** The framework does not claim to be a compliance
  product, a multi-maintainer foundation, or a fleet-scale orchestration
  platform. The review is right to flag those questions for adopters who need
  explicit legal traceability, organizational continuity evidence, or
  operation across hundreds of agents.

## Improvement proposals

- **Clarify login requirement for external reviews.**
  Change `agf/docs/site/evaluate.md` under "Submit your review" to state that
  creating an issue requires a free GitLab account. This prevents external
  reviewers from interpreting the sign-in redirect as a broken link.

- **Prefer canonical role names in submitted external reviews.**
  When this draft is posted, replace generic role labels with
  `planner-architect`, `coder-tester`, and `governor-reviewer`, and mention
  that those names come from the public roles page and the shipped role YAML
  files.

- **Clarify gate timing in the submitted review.**
  Replace "post-hoc CI-based enforcement" with a more precise statement:
  agents-govern has no runtime agent-to-agent coordination layer; Gate 1 is
  a pre-implementation planning checkpoint, Gate 2 is a pre-merge review
  checkpoint, and CI/validators provide evidence for those checkpoints.

- **Preserve the legitimate gap observations without overstating them.**
  EU AI Act mapping, single-developer sustainability, and large-fleet
  scalability should be framed as adopter-fit considerations, not factual
  defects in the current framework claims.

## Evidence basis

- `agf/docs/site/evaluate.md` defines the review checklist, output format,
  canonical repository identity, and submission path.
- `agf/docs/site/index.md` describes agents-govern as a file-based framework,
  states the released v2.1 documentation line, and lists the deployable Rust
  `agov` command surface.
- `agf/docs/site/reference/tooling-conformance.md` identifies the
  platform-specific Rust `agov` binary as the adopter-facing release
  implementation.
- `agf/docs/site/docs/start/should-you-adopt.md` explains the adopt / wait /
  skip decision framing and explicitly states that AGF is not a hosted service,
  legal/compliance system, runtime, or security scanner.
- `agf/docs/site/docs/concepts/roles-and-gates.md` names the core roles and
  describes Gate 1 as pre-implementation and Gate 2 as pre-merge.

## Uncertainty

The external review was docs-only. It did not inspect `agf/rules/`,
`agf/schemas/`, or `agf/agents/*.yaml`, so claims about the exact rule library,
schema structure, or role YAML contents should be verified against those files
before being treated as implementation-level findings.
