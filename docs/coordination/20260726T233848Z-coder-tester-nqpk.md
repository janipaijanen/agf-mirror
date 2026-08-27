---
kind: note
from: coder-tester
to: planner-architect
id: 20260726T233848Z-coder-tester-nqpk
timestamp: 2026-07-26T23:38:48Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T233427Z-coder-tester-p1qh
topic: downstream-localization-and-commercial-terminology
requires-ack: true
artifacts_owned: []
---

# Reserved identity must not impose AGF vocabulary on adopters

The governor added a material requirement to the pre-v2.5.0 extension model:
downstream projects, especially commercial customers, must be able to name,
localize, brand, and present framework rules and operating concepts in language
that fits their organization. The reserved namespace recommendation stands,
but only for stable machine identity and provenance. It must not reserve the
customer-facing vocabulary.

## Refined contract

Separate four layers that must not be collapsed into one filename or title:

1. **Canonical identity and provenance** — stable upstream identity such as
   `agf:rule:human-gate-authority`, its source version, and integrity metadata.
   This is used for validation, upgrades, audit exchange, and conflict checks.
2. **Semantic contract** — the authority, safety, and behavioral guarantees the
   identity denotes. A normal customization or localization must preserve this
   contract.
3. **Adopter parameters** — project-owned values at declared customization
   points: role assignments, approval names, branch names, required reviewers,
   escalation routes, thresholds, and other permitted policy choices.
4. **Presentation profile** — adopter-owned display labels, descriptions,
   translations, commercial terminology, branding, documentation headings, and
   tool aliases. These may differ completely from AGF's default vocabulary while
   retaining an explicit one-to-one mapping to the canonical identity.

Examples:

```text
canonical: agf:rule:human-gate-authority
customer label: Executive Change Approval
fi-FI label: Johdon muutoshyväksyntä

canonical: agf:role:governor-reviewer
customer label: Accountable Product Owner

canonical: agf:command:compact
customer alias: /project-closeout
```

The canonical identity need not dominate normal customer-facing output. Product
UI, generated manuals, and everyday commands may lead with the adopter label.
Diagnostic, audit, interchange, and upgrade output must retain enough canonical
identity to prove what the label maps to, for example:
`Executive Change Approval [agf:rule:human-gate-authority]` in expanded or audit
views.

## What remains prohibited

- A display label or localized filename must not create a second semantic
  identity accidentally.
- A local alias must not map to two canonical entities, and a reserved upstream
  alias must not be repurposed for a different entity.
- Translation or rewriting must not silently weaken the semantic contract.
  A materially different meaning is an explicit deviation/fork, not
  localization.
- Upgrades must not overwrite the adopter's terminology, locale catalog,
  branding, or permitted parameters.
- AGF's English titles must not be described as mandatory commercial language.

## Design consequences for the bounded plan

- Define an adopter-owned terminology/presentation profile outside `agf/`, with
  locale-aware labels and one-to-one canonical mappings.
- Make tool aliases first-class mapped interfaces rather than independent rule
  sources. Multiple local aliases may map to one canonical command when
  unambiguous.
- Define provenance for translated or restated normative text. The plan must
  decide whether v2.5.0 supports only labels/descriptions or also reviewed
  semantic-equivalent rule projections; it must not imply equivalence can be
  established mechanically.
- Add negative validation fixtures for alias collisions, reserved alias reuse,
  orphan labels, and one local label mapped to incompatible identities.
- Add upgrade tests proving customer terminology and permitted parameters are
  preserved while the underlying upstream artifact advances.
- FAQ examples should include a branded commercial vocabulary and at least one
  non-English locale, not only developer-centric renaming.

This improves the namespace proposal: AGF owns its canonical identifiers and
guarantees; the adopter owns its organizational language and permitted policy
choices. Namespace safety and customer freedom are complementary, not opposing,
requirements.

Please include this distinction in the independent challenge and any subsequent
Gate 1 plan. Do not implement from this note, and do not turn AGF vocabulary into
a customer-facing naming mandate.
