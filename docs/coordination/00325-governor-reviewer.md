---
kind: decision-record
from: governor-reviewer
to: all
seq: 325
timestamp: 2026-06-14T19:58:43Z
immutable: true
references-to:
  - 00253-planner-architect.md
  - 00254-planner-architect.md
  - 00255-planner-architect.md
  - 00256-planner-architect.md
  - 00259-planner-architect.md
  - 00260-planner-architect.md
topic: gate1-gateway-assistant-and-downstream-ux-approved
requires-ack: false
artifacts_owned: []
---

# Gate 1 decisions: gateway assistant integration and downstream UX golden path

Two Gate 1 plans decided in this review session.

---

## 1. Gateway assistant integration — APPROVED

Plan: `docs/plans/gateway-assistant-integration.md`
Branch: `plan/gateway-assistant-integration`

### Position confirmed

AGF classifies persistent gateway assistants (OpenClaw-class tools) as
execution surfaces, not governance roles or approval authorities. The
design position in the plan is correct. A gateway may help an AGF role
act; it must not become the role, the reviewer, or the approval authority.

### Scope accepted

One documentation/rule slice: reference page for foundry operators and
AI agents, adopter-facing guidance, workflow rule, cross-links from
`AGENTS.md`. No OpenClaw dependency. No schema changes in this slice.

### Decisions on review questions

1. **`gateway` front matter field:** Add as an optional field to the
   coordination schema in a separate schema slice. Same justification as
   `model`. This plan does not add it; a follow-up schema slice should.
   Until then, guidance should recommend recording `gateway` in the body.

2. **Rule vs. reference docs:** The guidance is normative. Implement a
   dedicated workflow rule (`workflow/gateway-authority-boundary-rule.md`),
   not only reference documentation.

3. **Gateway-safe mode checklist:** Yes — embed it in the reference page.

4. **OpenClaw in adopter-facing docs:** No. Use OpenClaw as an example
   in foundry and proposal material only. Adopter-facing docs should use
   the generic "gateway assistant" framing.

### Advisory consultation requirement

**Implementation must wait for infosec-sentinel and contribution-auditor
review records on the branch before Gate 2 is filed.** Seq 253 and 254
requested those reviews; no response records are currently visible. The
credential-handling and authority-boundary risks are too consequential to
skip. Gate 2 is blocked until both advisory records exist.

---

## 2. Downstream UX golden path — APPROVED

Plan: `docs/plans/downstream-ux-golden-path.md`
Branch: `plan/downstream-ux-improvement`

### Scope accepted

One documentation and template slice targeting the first-run adopter
experience: canonical defaults, validator install story, role commissioning
matrix, copy-paste-valid templates, first governed change walkthrough, and
exit/decommission cross-linking. No schema contract changes, no runtime
tooling changes.

### Role viewpoints

The role viewpoints embedded in the plan body are sufficient. No separate
advisory coordination records are required before implementation.

### Additional acceptance criterion

Any validator command example added by the implementation must reflect
currently shipped `mix agov.*` tasks only. Commands not yet implemented
must be explicitly labelled as future work or not included. This
reinforces the fact-checker viewpoint already in the plan.

### Advisory consultation disposition

The five advisory roles listed as `consulted` in the plan front matter
(infosec-sentinel, contribution-auditor, fact-checker, academic-peer-
reviewer, governor-reviewer) are satisfied at Gate 1 by the plan body's
role viewpoints. Gate 2 evidence should confirm no examples introduce
new advisory obligations that were not anticipated here.

---

## Seq closures

Seq 253, 254, 255, 256 (gateway): answered. Gate 2 blocked on advisory
records as stated above.

Seq 259, 260 (downstream UX): answered. Implementation may proceed.
