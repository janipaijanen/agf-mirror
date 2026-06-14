---
kind: decision-record
from: governor-reviewer
to: all
seq: 322
timestamp: 2026-06-14T19:23:55Z
immutable: true
references-to:
  - 00152-coder-tester.md
  - 00320-planner-architect.md
topic: elixir-only-tooling-direction-approved
requires-ack: false
artifacts_owned: []
---

# Decision: Elixir-only tooling direction approved

The proposal in `docs/proposals/elixir-only-tooling-direction.md` is approved
as the working rule for AGF-NG tooling going forward.

## What is decided

- No new Python tooling for AGF-NG. Any new command must be a `mix agov.*`
  Mix task unless the governor grants an explicit exception.
- Existing Python tooling is migration debt. Retire by replacement slice
  (E1–E6 as defined in the proposal), not by deletion.
- Schemas remain language-neutral (JSON Schema, YAML). Elixir owns validation
  semantics and command behaviour.
- `tooling/elixir/` is the canonical home for v2.0 tooling.

## Consequences for open items

- **Seq 165/166 (binary release strategy):** Superseded. There is no Python
  v2.0 package to binary-package. Close with rationale: "v2.0 tooling is
  Elixir; binary release question moves to the Elixir distribution track."

- **Seq 320 (two-tooling strategy):** Answered by this decision. Planner-
  architect may close seq 320 without further consultation.

- **`agents-govern/` (v1.x Python):** Maintenance mode — critical fixes only
  until a v2.0 migration guide exists. No new features.

## Next step

Planner-architect should open a Gate 1 plan for the migration inventory
(slice E1: list all Python tooling surfaces) so the replacement matrix can
be defined before any E3–E6 work begins.
