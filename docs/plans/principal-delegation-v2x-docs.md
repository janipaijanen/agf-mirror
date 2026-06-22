---
id: principal-delegation-v2x-docs
status: approved
topic: public-docs
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-06-22T00:13:29Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Docs-only change. No security boundaries, authentication, authorization,
      secrets handling, or schema changes. Convention objects (delegation,
      contestation) are not validator-enforced in v2.x.
  - role: contribution-auditor
    status: consulted
    reason: >
      Coder-tester (Codex) reviewed the Gate 1 plan and returned findings in
      20260622T000407Z-coder-tester-k66a.md. Blocking finding resolved (front
      matter added). Content recommendations incorporated: delegation_scope
      added to minimum convention fields; valid_until promoted to strongly
      recommended; GDPR legal-basis language replaced with privacy/labor review
      wording; schema non-enforcement clarified.
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: principal-bound AI delegation — v2.x docs convention

**Date:** 2026-06-21
**Role:** planner-architect
**Status:** GATE 1 APPROVED — 2026-06-22
**Source:** `docs/proposals/ai-on-behalf-of-human-principal.md`
**Gate 1 handoff:** `docs/coordination/20260621T235800Z-planner-architect-g1hf.md`
**Consultation response:** `docs/coordination/20260622T000407Z-coder-tester-k66a.md`

---

## Intent

Document the principal-bound AI delegation pattern as a v2.x docs-only
convention. No schema change. No new coordination message kinds. The pattern
uses existing `kind: note` messages with optional delegation or contestation
objects; these objects may be syntactically accepted by the coordinator but
are not semantically validated or enforced by tooling in v2.x. They are
documented as a stable but non-enforced convention.

---

## Scope

### In scope

- A new "Delegated AI work" section in one public docs page
  (preferred: `agf/docs/site/docs/concepts/roles-and-gates.md`; alternative:
  a new page under `advanced/`).
- One new FAQ entry in `agf/docs/site/docs/start/adopter-faq.md`.
- A short residual-risk paragraph on the adoption decision path
  (`agf/docs/site/docs/start/should-you-adopt.md`).
- A synthetic example delegation grant record (no real user handles).
- Adopter guidance for team aliases, rotating duty aliases, batch grants, and
  retention limits where privacy risk outweighs audit precision.

### Out of scope

- Schema changes (deferred to v3+ pending adopter evidence).
- New coordination message `kind` values.
- `agov coord validate` semantic enforcement of delegation objects.
- `agov session resume` surfacing referenced grants.
- Tooling commands that aggregate, rank, or compare principal-bound records.

---

## Convention documented

### Delegation grant (human-authored `kind: note`)

Minimum convention fields: `principal_kind`, `identifier_namespace`,
`identifier`, and `delegation_scope`. `valid_until` is strongly recommended
for all grants; required in example usage below. Open-ended delegation without
`valid_until` invites misuse and should be flagged in the docs as a risk.

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
- `delegation_scope` names what the AI may do under the grant. It is part of
  the minimum convention, not optional decoration.
- `valid_until` is set by the human grant author; AI agents do not infer or
  extend it. Grants without `valid_until` should be treated as high-risk.
- `cannot_approve` is advisory defense-in-depth wording; Gate 1 and Gate 2
  authority still rests with the governor-reviewer regardless.

### Contestation (later immutable note)

Contestation of a delegation claim is filed as a subsequent immutable
`kind: note` with `note_type: contestation` that references the original
message. Contested messages are not mutated; the contestation record is the
audit artifact.

### Anti-surveillance and transparency boundaries

The delegation pattern makes AI activity more legible, not less. AGF does not
create surveillance risk that was absent from plain git. Git already records
author, timestamp, changed files, branches, review comments, and merge events.
Structured delegation metadata lowers monitoring cost and increases signal
fidelity for governance purposes. The honest claim is that AGF avoids native
surveillance tooling and makes the tradeoff visible to adopters — not that
it eliminates monitoring risk.

The residual risk: once `identifier` values are in immutable records, they are
discoverable by anyone with repo access. These are privacy, labor, and
purpose-limitation considerations for adopters. Privacy-risk mitigations
documented in the convention:

- Use team aliases (e.g., `@docs-team`) instead of individual handles where
  individual attribution is not required.
- Use rotating duty aliases (e.g., `@on-call-reviewer`) for repeated short
  delegation windows.
- Use batch grants scoped to a sprint or milestone rather than per-task grants.
- Set and document retention limits; state the purpose (governance, audit, and
  safety metadata) when conducting local privacy and labor review.
- Warn adopters in environments with adversarial HR or legal dynamics to
  consider identity minimization before enabling principal-bound delegation.

These are privacy-risk mitigations. They are not a compliance checklist and do
not substitute for local legal, privacy, or labor review.

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
- `agov plan validate` on this file passes before Gate 1 approval.

---

## Risks

| Risk | Mitigation |
|---|---|
| Scope creep into schema | Explicit out-of-scope list in this plan and in the docs section |
| GDPR: real handles in examples | Synthetic-only constraint; reviewer checks |
| Unclear anti-surveillance language | Draft reviewed against `marketing-tone-rule.md` and `verifiable-claims-rule.md`; honest scope claim |
| Adopter misreads convention as required | Convention is optional; docs must state this |
| Adopter misreads mitigations as compliance checklist | Explicit "not a compliance checklist" statement in docs |
| Open-ended delegation without valid_until | Convention recommends valid_until; docs flag open-ended grants as high-risk |

---

## Rollback

Revert the docs commits. No schema or tooling artifacts to unwind.
