---
kind: completion
from: planner-architect
to: all
seq: 6
timestamp: 2026-06-10T23:06:13Z
immutable: true
references-to:
  - 00003-planner-architect.md
  - 00004-planner-architect.md
  - 00005-planner-architect.md
topic: T1-1-coordination-kind-schema
requires-ack: false
artifacts_owned: []
---

# Completion: T1-1 Phase A — coordination message kind schema

## What was done

T1-1 Phase A is complete on branch `feat/T1-1-kind-schema`.

### Schema (`spec/schemas/coordination-message.schema.json`)

- Nine `kind` values defined as enum
- Per-kind constraints via `if/then`:
  - `claim`: `artifacts_owned` non-empty
  - `completion`: `artifacts_owned` empty
  - `ack`: `references-to` non-empty
  - `handoff`: `handoff` object required
- `from` and `to`: role identifiers (not model names), `minLength: 1`
- `model`: optional, LLM instance, provenance only
- `session`: optional, UUIDv7, audit only

### Coordination README (`docs/coordination/README.md`)

- Schema reference added
- Front matter template updated with three-field sender identity
- Role identifier convention documented

### Design decisions recorded

- 00004: full discussion of model-name vs role vs role@model designs
- 00005: decision record — three-field sender identity

## Verification

All three pre-existing messages (seq 1–3) validated against schema
constraints using bespoke Python validator. All passed.

## Remaining risk

- `from` values in seq 1–3 use `claude` and `human` (pre-convention).
  Valid under the schema (minLength: 1 satisfied) but do not follow
  the role-identifier convention. Immutable — cannot be corrected.
- No JSON Schema library available; validation uses bespoke Python
  script. Phase B Elixir validator will enforce the schema properly.

## Phase B

Separate task. Requires:
1. Elixir project initialisation in `tooling/elixir/`
2. YAML front matter parser
3. Schema validator against `spec/schemas/coordination-message.schema.json`
4. `agov coord validate` command

Awaiting Gate 2 for this branch before Phase B planning begins.
