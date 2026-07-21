---
id: adoption-surface-clarity
status: approved
topic: adoption-surface-clarity
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-20T22:39:21Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Public documentation clarification only; no credentials, access
      control, schema, runtime, or security-boundary change.
  - role: contribution-auditor
    status: waived
    reason: Single-agent planning from a governor-approved documentation
      direction. Gate 2 must still check message-integrity, scope, and
      shipped-versus-planned wording against the cited assessment evidence.
---

# Plan: distinguish the shipped library from an adopter's activated subset

**Date:** 2026-07-20
**Status:** GATE 1 APPROVED — the human governor reviewed the proposed
documentation shape in the active Codex session and directed, "nice. please
implement." The companion Codex-scribed governor decision records that
instruction durably.

## Trigger and problem

The Sol High assessment found that the complete distribution's roles, rules,
skills, coordination kinds, gates, and record types create too large a default
mental model for first adoption. Follow-up discussion established that the
public docs contain real start-small mitigations, but evaluators must assemble
them from several pages and can reasonably conclude that every shipped element
is mandatory.

The documentation must distinguish three different surfaces without claiming
that an unshipped product exists:

1. the complete library included in the release;
2. the subset an adopter activates for a trial or project; and
3. the proposed reduced starter product, which does not currently ship.

This is both a clarity problem and an implementation-honesty boundary. Clearer
copy may explain the current selective-adoption model; it must not imply that a
reduced starter archive, four-kind coordination contract, one-validator path,
or minimal CI bundle has already been delivered.

## Canonical message

`agf/docs/site/docs/start/should-you-adopt.md` will become the canonical home
for a section titled **What ships and what you activate**. It will state:

- releases include the complete governance library;
- installation does not mean activating every role or retaining every
  task-specific rule;
- the existing solo and standard commissioning shapes use two and three active
  roles respectively;
- core rules remain binding, while task-specific rules are selected by
  applicability and inapplicable rules should be removed during commissioning;
- the current coordination schema still exposes all nine message kinds; and
- a reduced starter product is a Level 1 Planned hypothesis pending external
  pilot evidence, not a shipped capability.

A short two-sentence disclosure will be quoted verbatim on the public home,
evaluation landing, and curated dossier, with a link to the canonical section.
This follows `marketing-tone-rule` message integrity: one detailed source,
plus identical attributed summaries rather than drifting paraphrases.

## Affected files

- `agf/docs/site/docs/start/should-you-adopt.md` — canonical installed versus
  activated explanation and shipped/planned boundary.
- `agf/docs/site/index.md` — first-screen disclosure before the next-step link.
- `agf/docs/site/evaluate.md` — require reviewers to distinguish full
  inventory, activated subset, and unshipped starter proposal.
- `agf/docs/site/ai-evaluation-dossier.md` — add the same distinction to the
  curated Flow 04 orientation, not only the generated appendix.
- `agf/docs/site/docs/adopt/install-and-layouts.md` — link commissioning steps
  back to the canonical explanation; retain the existing rule-deletion and
  minimum-role matrix as operational detail.
- `agf/docs/site/future/evolution.md` — list the reduced starter as a hypothesis
  and name the independent-pilot evidence required to promote it.
- `agf/CHANGELOG.md` — record the public adoption-surface clarification.
- `docs/coordination/` — Gate 1 handoff/decision and eventual Gate 2 handoff.

## Explicitly out of scope

- No reduced archive, profile schema, validator mode, CI example, role file,
  rule file, or coordination-message contract is created.
- No inventory number is hard-coded. Evaluators may count the checkout; public
  prose must not introduce another number that drifts as rules or roles change.
- The missing legacy `agf/docs/adopt/intensity-guide.md` reference is a separate
  documentation defect. This change will not cite it or imply it implements the
  starter profile.
- No claim is made that the selective-adoption model has been validated by
  independent pilots.

## Downstream impact and migration

Classification: **EDITORIAL**. This corrects how existing shipped and planned
surfaces are explained. It introduces no governance capability, schema,
operator command, or migration obligation. Adopter documentation is updated
and the changelog will say so explicitly.

## Test strategy

1. `agov plan validate docs/plans/adoption-surface-clarity.md`.
2. Confirm the short disclosure is byte-for-byte identical wherever quoted.
3. `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml`.
4. Inspect the built home, evaluation page, and dossier for the disclosure and
   working canonical link; confirm the dossier's generated appendix still
   contains the source page.
5. `agov validate adopter-docs` and `agov dist validate`.
6. `git diff --check`.

## Risks and controls

| Risk | Control |
|---|---|
| Clearer wording is mistaken for a shipped starter product | Say explicitly that the starter is Level 1 Planned and name what does not ship. |
| Repeated summaries drift | Use one canonical section and one verbatim two-sentence disclosure. |
| Landing pages become another long adoption guide | Keep the disclosure short and link to the canonical page. |
| Selective rules are interpreted as optional governance | Distinguish binding core rules from task-specific rules that are removed only when inapplicable. |
| Counts become stale | Do not hard-code inventory totals. |

## Rollback

Revert the documentation paragraphs, future hypothesis, and changelog entry.
There is no data, schema, tooling, or adopter migration to reverse.
