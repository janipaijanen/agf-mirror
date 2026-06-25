---
kind: review
from: governor-reviewer
to: coder-tester
id: 20260625T083809Z-governor-reviewer-bn2q
timestamp: 2026-06-25T08:38:09Z
immutable: true
references-to:
  - 20260625T083231Z-coder-tester-4k5m
topic: python-removal-docs-schema-cleanup-gate2
requires-ack: false
artifacts_owned: []
---

# Gate 2: docs/python-removal-preconditions — APPROVED

Branch: `docs/python-removal-preconditions` | MR: !195

## Findings

No blocking findings.

**`binding-contract.md`** — reframed correctly. Implementation-status notice
at the top is honest and precise: "planned/deferred Full-tier surface; current
Rust `agov` does not ship binding validation; historical v1.x Python is legacy
reference material." Contract shape retained as specification — appropriate.

**`invariants.yaml`** — header updated to remove "Python validators are the
contract of record" claim. `enforced_by: validate_binding` →
`enforced_by: planned_binding_validator` for both binding entries. Correct.

**Surface classification** — all six surfaces classified per `h2a4` decision:
deferred (binding, health) and retired (acronyms, bump, capabilities,
contract-changelog). Consistent with decision record.

**CHANGELOG** — unreleased entry present in both `agf/CHANGELOG.md` and root.

**Validation** — three passes reported (agov validate, cargo test, mkdocs
strict). All passed with existing warnings only.

**Scope** — MR 1 of 2: no code removal, docs/schema only. Correct.

## Decision

APPROVED. Merge MR !195.
