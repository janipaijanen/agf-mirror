---
id: rule-enforcement-page
status: pending-gate1
topic: rule-enforcement-page
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Public documentation only. The page describes existing authority and
      enforcement boundaries; it adds no credentials, runtime access,
      security control, or executable policy.
  - role: contribution-auditor
    status: consulted
    reason: >
      The page will consolidate normative and implementation claims that are
      currently distributed across rules and public documentation. The plan
      requires every blocking claim to cite a shipped rule, command, CI
      mechanism, or explicit maturity limitation, and prohibits converting
      recommendations into unsupported capability claims.
---

# Plan: canonical Rule enforcement concept page

**Date:** 2026-07-21
**Status:** Pending Gate 1

## Trigger and problem

The public documentation explains enforcement in several places, especially
`contracts-and-validators.md`, `install-and-layouts.md`, `position.md`, and
`tool-authority-boundary-rule.md`. Readers can reconstruct the model, but there
is no canonical page answering the direct question: when is an AGF rule merely
normative guidance, and when does a validator, hook, CI job, branch policy, or
human gate mechanically stop work?

The absence causes two opposite errors: treating every `MUST` statement as a
hard technical control, or treating AGF as prompt-only guidance even when an
adopter has wired required CI and branch protection. The user requested a
public **Rule enforcement** page and asked that the surrounding pages remain
aligned.

## Outcome

Add `agf/docs/site/docs/concepts/rule-enforcement.md` as the canonical public
explanation of AGF's enforcement model. Put it under **Concepts**, adjacent to
**Learning loop and evidence**. Preserve the division of responsibility:

- **Rule enforcement** defines terminology, authority, enforcement modes, and
  the rule-to-gate implementation sequence.
- **Learning loop and evidence** explains how evidence creates, strengthens,
  weakens, or retires normative rules.
- **Contracts and validators** remains the command, schema, exit-code, CI, and
  policy-profile implementation reference.
- **Install and layouts** remains the commissioning procedure.
- **Position** remains the runtime boundary statement.
- **Tooling conformance** remains the shipped-command inventory.
- **Future evolution** remains the home for unimplemented enforcement depth.

## Page structure

1. **What “enforcement” means in AGF** — distinguish normative obligation from
   technical prevention.
2. **Five terms** — normative rule strength, advisory evidence, blocking
   authority, structural enforcement, and human gate decision.
3. **Authority chain** — evidence informs a rule; a named rule, contract, CI
   policy, or governor decision grants blocking authority; tooling implements
   the check; negative tests prove it fires; branch/release policy makes the
   result unavoidable.
4. **Enforcement matrix** — instruction-only, human-gated, validator-backed,
   structurally required, and runtime control, with current AGF examples and
   explicit maturity.
5. **How to implement a blocking rule** — name the authority, choose the
   narrowest check, emit clear severity/exit semantics, wire it into CI or a
   hook, require it through repository policy, add a reproducible negative
   test, and retain a documented human override/escalation path.
6. **What ships now** — plan and coordination schema validation, selected
   artifact validators, adopter-docs validation, coordination visibility, CI
   sketches, and human gates; do not imply that every rule has a validator.
7. **Known limits** — no runtime interception, no shipped semantic
   plan-to-diff proof, no complete rule-to-validator coverage, and no shipped
   binding-traceability layer.
8. **Relationship to the learning loop** — evidence can justify creating,
   strengthening, weakening, or retiring both a rule and its enforcement.

The page will include one compact diagram or ordered flow only if it makes the
authority chain clearer than prose; the implementation should prefer a small
text flow over a new generated image unless visual review shows a diagram is
materially better.

## Alignment edits

- `agf/docs/build/mkdocs.yml` — add **Rule enforcement** under Concepts.
- `agf/docs/site/docs/index.md` and
  `agf/docs/site/docs/start/framework-map.md` — expose the page in the main
  documentation maps.
- `agf/docs/site/docs/concepts/evidence-and-learnings.md` — link enforcement as
  the next stage after evidence becomes a normative rule.
- `agf/docs/site/docs/concepts/position.md` — keep the short runtime-boundary
  statement and link to the canonical enforcement model.
- `agf/docs/site/docs/advanced/contracts-and-validators.md` — keep operational
  detail and link its terminology to the canonical page.
- `agf/docs/site/docs/adopt/install-and-layouts.md` — link commissioning of
  blocking CI to the canonical model.
- `agf/docs/site/reference/tooling-conformance.md` — link shipped tooling
  claims to the enforcement model without duplicating its definitions.
- `agf/docs/site/ai-evaluation-dossier.md` and `agf/docs/site/evaluate.md` — add
  the page to enforcement-related reading tracks and require evaluators to
  distinguish normative, advisory, blocking-capable, and structurally required
  surfaces.
- `agf/docs/site/future/evolution.md` — link planned deeper enforcement back to
  the current-state page.
- `agf/CHANGELOG.md` — record the new public concept page.

MR !286, which renames the evidence page/navigation label to **Learning loop
and evidence**, should merge before implementation so the new page links use
the settled canonical label without merge churn.

## Explicitly out of scope

- No new validator, hook, CI job, branch-protection setting, schema, rule, or
  runtime control.
- No claim that all `MUST` rules are mechanically enforced.
- No change to which current findings are advisory or blocking.
- No new enforcement maturity vocabulary; use the existing AGF five-level
  capability labels where a capability claim needs qualification.
- No generated enforcement-coverage percentage. The repository does not yet
  have complete rule-to-check metadata from which such a number could be
  derived honestly.

## Downstream impact and migration

Classification: **EDITORIAL**. The page consolidates and cross-links existing
authority and implementation boundaries. It changes no adopter artifact,
command, schema, exit code, or required policy. No migration is required.

## Verification

1. `agov plan validate docs/plans/rule-enforcement-page.md`.
2. `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml`.
3. Check all enforcement capability claims against current Rust `agov --help`,
   `agov validate --help`, `.gitlab-ci.yml`, and the named normative rules.
4. Search public docs for `advisory`, `blocking`, `normative`, `structural`, and
   `runtime` to confirm no edited page contradicts the canonical definitions.
5. Verify every link to **Learning loop and evidence** uses the settled label
   while retaining the existing URL.
6. Render the new page and inspect the terminology table/flow at desktop and
   narrow viewport widths.

## Risks and controls

| Risk | Control |
|---|---|
| A `MUST` rule is described as mechanically unavoidable | Separate normative strength from implementation mode in the first screen and matrix. |
| CI/tooling is treated as an independent governor | Require every blocking example to name its authority source. |
| Planned semantic enforcement is presented as shipped | Put known limitations beside the current shipped-surface list and use maturity labels. |
| Cross-page summaries drift | Make this page canonical; keep other pages task-specific and link rather than restate full definitions. |
| The new page becomes a generic CI tutorial | Limit examples to AGF authority, contracts, gates, and adopter commissioning. |

## Rollback

Remove the page, navigation entry, cross-links, and changelog line. No runtime,
schema, policy, or adopter data change requires reversal.
