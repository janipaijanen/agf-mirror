---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260621T235800Z-planner-architect-g1hf
timestamp: 2026-06-21T23:58:00Z
immutable: true
references-to:
  - 20260621T235747Z-planner-architect-3rn7.md
topic: principal-delegation-v2x-docs-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/principal-delegation-v2x-docs.md
  - docs/coordination/20260621T235800Z-planner-architect-g1hf.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Gate 1 plan for principal-bound AI delegation — v2.x docs convention.
    - Approve or reject the scope (affected files, in/out-of-scope list, convention shape).
    - If approved, coder-tester may proceed with implementation.
  must_read:
    - docs/plans/principal-delegation-v2x-docs.md
    - docs/proposals/ai-on-behalf-of-human-principal.md
  do_not_touch:
    - agf/rules/
    - agf/schemas/
    - agf/agents/
---

# Gate 1 handoff: principal-bound AI delegation — v2.x docs convention

## Plan summary

Plan at `docs/plans/principal-delegation-v2x-docs.md`.

**Docs-first, no schema change.** Three public docs pages receive new content:

| File | Change |
|---|---|
| `roles-and-gates.md` | New "Delegated AI work" section |
| `adopter-faq.md` | New FAQ entry |
| `should-you-adopt.md` | Short residual-risk paragraph |

**Convention core:**

- Human-authored `kind: note` with `note_type: delegation-grant` records a
  principal delegation. AI does not self-attest.
- AI messages acting under a delegation reference the grant via `references-to`.
- `identifier_namespace` + `identifier` are explicitly authored by the human
  grant source, not inferred from git metadata.
- Contestation is a later immutable `kind: note` with `note_type: contestation`.
- Anti-surveillance boundaries, transparency tradeoff, and adopter mitigations
  (team aliases, rotating duty aliases, batch grants, retention limits) are
  documented.

**Explicit out-of-scope:** schema change, new `kind` values, validator
enforcement, tooling aggregate reports, HR-exportable datasets.

## Gate 1 questions for governor-reviewer

- [ ] Is the docs-first scope correct, or should the schema extension happen
      in this same slice?
- [ ] Is the three-page file list complete, or does another existing page also
      need a mention?
- [ ] Is the convention shape (human authors, AI references, no AI self-assertion)
      the right model? Any constraint to add or remove?
- [ ] Are the anti-surveillance boundary statements appropriately scoped for
      public docs, or too detailed / not detailed enough?
- [ ] Any concerns about the GDPR framing or the transparency tradeoff language
      before it appears in public docs?
