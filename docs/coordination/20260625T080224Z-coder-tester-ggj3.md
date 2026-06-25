---
kind: note
from: coder-tester
to: planner-architect
id: 20260625T080224Z-coder-tester-ggj3
timestamp: 2026-06-25T08:02:24Z
immutable: true
model: codex-gpt-5
references-to: []
topic: python-removal-binding-parity-decision
requires-ack: false
artifacts_owned: []
---

# Python removal depends on binding parity / product decision

Jani asked whether AGF-NG has parity with the v1.x Python implementation for
the critical parts, and whether the historical Python code can then be removed
from this repository.

Coder-tester assessment:

- The supported Rust `agov` path covers the current critical day-to-day
  operator surface: coordination validation, plan validation, coordination
  discovery/visibility, handoff/session resume, framework validators,
  distribution validation/build/tag check, version/provenance, usage report,
  and token ROI reporting.
- `./tooling/agov validate` passes on the current checkout, with existing
  warnings only.
- Full v1.x command parity is not exact. Python still has commands/surfaces
  without current Rust equivalents, especially:
  - `agov-validate-binding`
  - `agov-validate-acronyms`
  - `agov-bump`
  - `agov-health`
  - old `agov --capabilities` / contract-changelog style surfaces

The product-critical question is binding.

Why binding can be critical:

- Binding connects concrete artefacts to governed work: which agent/task/review
  path produced this commit, file, release artefact, or sidecar.
- For adopters needing stronger auditability, incident reconstruction, customer
  assurance, procurement review, or regulated/safety-sensitive evidence, this
  closes the gap between "we have governance records" and "this artefact is
  tied to those records."
- Existing contract/compliance docs still describe binding as Layer 3 / Full
  tier material, and some schema descriptions still name the Python validator
  as the contract implementation.

Why binding may not be critical for foundry's immediate day-to-day operation:

- Foundry currently gets practical control from human governor review,
  coordination files, merge requests, Git history, CI, and Rust validators.
- If AGF v2 intentionally ships without Full-tier binding enforcement, the
  Python binding implementation can become historical/deferred instead of
  operational.

Request to planner-architect:

1. Please recommend whether binding remains a critical AGF v2 product promise
   before removing `agents-govern/src/agents_govern/validate_binding.py`.
2. If yes, recommend a Rust parity slice for binding before Python removal.
3. If no, recommend the documentation/schema cleanup needed to explicitly
   retire or defer binding validation before deleting the historical Python
   implementation.
4. Please also classify the non-binding missing Python surfaces
   (`acronyms`, `bump`, `health`, `capabilities/changelog`) as critical,
   deferred, retired, or unnecessary for AGF v2.

Tentative coder-tester recommendation:

- Do not remove `agents-govern/` Python code yet.
- First settle the binding contract stance.
- Then either port binding to Rust or update the contract/docs/schemas to stop
  presenting Python binding validation as current AGF v2 capability.
- After that, remove historical Python in a dedicated cleanup branch/MR.
