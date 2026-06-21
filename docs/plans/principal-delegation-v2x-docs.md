<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: principal-bound AI delegation — v2.x docs convention

**Date:** 2026-06-21  
**Role:** planner-architect  
**Status:** awaiting Gate 1 approval  
**Source:** `docs/proposals/ai-on-behalf-of-human-principal.md`  
**Gate 1 handoff:** `docs/coordination/20260621T235800Z-planner-architect-g1hf.md`

---

## Intent

Document the principal-bound AI delegation pattern as a v2.x docs-only
convention. No schema change. No new coordination message kinds. The pattern
uses existing `kind: note` messages with optional delegation or contestation
objects; the schema may not validate those objects today — they are documented
as an opaque content convention, not a validator-enforced schema extension.

---

## Scope

### In scope

- A new "Delegated AI work" section in one public docs page
  (preferred: `agf/docs/site/docs/concepts/roles-and-gates.md`; alternative:
  a new page under `advanced/`).
- One new FAQ entry in `agf/docs/site/docs/start/adopter-faq.md`.
- A short residual-risk warning on the adoption decision path
  (`agf/docs/site/docs/start/should-you-adopt.md`).
- A synthetic example delegation grant record (no real user handles).
- Adopter guidance for team aliases, rotating duty aliases, batch grants, and
  retention limits where privacy risk outweighs audit precision.

### Out of scope

- Schema changes (deferred to v3+ pending adopter evidence).
- New coordination message `kind` values.
- `agov coord validate` enforcement of delegation objects.
- `agov session resume` surfacing referenced grants.
- Tooling commands that aggregate, rank, or compare principal-bound records.

---

## Convention documented

### Delegation grant (human-authored `kind: note`)

```yaml
kind: note
from: governor-reviewer
to: coder-tester
id: <id>
timestamp: <utc>
immutable: true
references-to: []
topic: delegation-grant-<branch>
requires-ack: true
artifacts_owned: []
note_type: delegation-grant
delegation:
  principal_kind: human | service | team
  identifier_namespace: "gitlab.com/example-group/example-project"
  identifier: "@example-member"
  delegation_scope:
    - "prepare patch"
    - "run tests"
  valid_until: "<utc>"
  cannot_approve:
    - "Gate 1"
    - "Gate 2"
    - "release tag"
```

Key constraints documented in the convention:

- The grant MUST be human-authored. An AI agent MUST NOT write a delegation
  grant asserting a principal on its own behalf.
- AI messages that act under a delegation reference the grant through
  `references-to`. They do not re-state or infer the principal.
- `identifier` is the human-readable handle from the authority named in
  `identifier_namespace`. It is not derived from git committer, email, or
  SSH key metadata.
- `valid_until` is set by the human grant author; AI agents do not infer or
  extend it.
- `cannot_approve` is advisory defense-in-depth wording; Gate 1 and Gate 2
  authority still rests with the governor-reviewer regardless.

### Contestation (later immutable note)

Contestation of a delegation claim is filed as a subsequent immutable
`kind: note` with `note_type: contestation` that references the original
message. Contested messages are not mutated; the contestation record is the
audit artifact.

### Anti-surveillance and transparency boundaries

The delegation pattern makes AI activity more legible, not less. This is the
transparency tradeoff: AGF does not create surveillance risk that was absent
from plain git. Git already records author, timestamp, changed files, branches,
review comments, and merge events. Structured delegation metadata lowers
monitoring cost and increases signal fidelity for governance purposes.

The residual risk: once `identifier` values are in immutable records, they are
discoverable by anyone with repo access. GDPR and purpose-limitation framing
applies. Documented adopter mitigations:

- Use team aliases (e.g., `@docs-team`) instead of individual handles to
  reduce personal-data exposure where individual attribution is not required.
- Use rotating duty aliases (e.g., `@on-call-reviewer`) for repeated short
  delegation windows.
- Use batch grants scoped to a sprint or milestone rather than per-task grants.
- Set retention limits and document the stated purpose (governance, audit, and
  safety metadata) to support a defensible GDPR legal basis.
- Warn adopters in environments with adversarial HR or legal dynamics to
  consider identity minimization before enabling principal-bound delegation.

### What AGF will not build natively

Documented as explicit out-of-scope boundaries:

- Aggregate principal productivity reports.
- Per-person review-depth scores.
- HR-exportable datasets.
- Commands that rank or compare AI usage across named humans.

---

## Affected files

| File | Change |
|---|---|
| `agf/docs/site/docs/concepts/roles-and-gates.md` | New "Delegated AI work" section |
| `agf/docs/site/docs/start/adopter-faq.md` | New FAQ entry: "Can an assistant work on behalf of a human?" |
| `agf/docs/site/docs/start/should-you-adopt.md` | Short residual-risk paragraph in commitment section |

---

## Migration needed

No. Docs-only change.

---

## Test strategy

- `mkdocs build --strict` passes cleanly.
- No broken internal links introduced.
- No real user handles in examples (synthetic only).
- `agov coord validate` on the Gate 1 handoff file.

---

## Risks

| Risk | Mitigation |
|---|---|
| Scope creep into schema | Explicit out-of-scope list in this plan and in the docs section |
| GDPR: real handles in examples | Synthetic-only constraint in this plan; reviewer checks |
| Unclear anti-surveillance language | Draft reviewed against `marketing-tone-rule.md` and `verifiable-claims-rule.md` |
| Adopter misreads convention as required | Convention is optional; docs must state this |

---

## Rollback

Revert the docs commits. No schema or tooling artifacts to unwind.
