---
id: T1-1-phase-b-elixir-validator
status: complete
topic: elixir-coordination-validator
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-06-10"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: local validation tooling only; no secrets, auth, or isolation changes
  - role: contribution-auditor
    status: not-applicable
    reason: internal foundry tooling; not an upstream submission
---

# Plan: T1-1 Phase B — Elixir Validator for Coordination Messages

**Date:** 2026-06-10T23:56:56Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T1-1 Phase A (spec/schemas/coordination-message.schema.json, merged ✓)

---

## Intent

Initialize the Elixir project in `tooling/elixir/` and implement `agov-coord-validate`,
a Mix task that validates coordination message files against the JSON schema defined in
`spec/schemas/coordination-message.schema.json`.

The validator is the first operational artifact of the foundry tooling layer. It makes
the coordination schema machine-enforceable rather than documentation-only.

---

## Affected files

| File | Action |
|---|---|
| `mise.toml` | No change — elixir 1.20.1-otp-29 + erlang 29.0.2 already listed |
| `tooling/elixir/mix.exs` | Create — new Mix project `agov` |
| `tooling/elixir/lib/mix/tasks/agov.coord.validate.ex` | Create — Mix task implementation |
| `tooling/elixir/lib/agov/coord_validator.ex` | Create — validation logic (pure module) |
| `tooling/elixir/test/agov/coord_validator_test.exs` | Create — unit tests |
| `tooling/elixir/test/fixtures/valid_claim.md` | Create — valid fixture |
| `tooling/elixir/test/fixtures/invalid_missing_kind.md` | Create — invalid fixture |
| `tooling/elixir/mix.lock` | Create — locked after `mix deps.get` |
| `.gitignore` | Update — add `tooling/elixir/_build/` and `tooling/elixir/deps/` |

---

## Migration needed

No. This is new tooling with no schema migrations.

---

## Implementation plan

### Step 0 — Install Elixir/Erlang via mise (prerequisite)

```bash
mise install elixir erlang
```

Verify: `mix --version` resolves.

### Step 1 — Initialize Mix project

```bash
cd tooling/elixir
mix new agov --no-sup
```

Edit `mix.exs` to:
- Set `app: :agov, version: "0.1.0"`
- Add dependency: `{:jason, "~> 1.4"}` (JSON parsing) and
  `{:ex_json_schema, "~> 0.10"}` (JSON Schema draft-07 validation)
- Add `escript:` section for optional CLI binary (deferred, Mix task is sufficient)

### Step 2 — Write tests FIRST (TDD: RED phase)

Create test fixtures:
- `test/fixtures/valid_claim.md` — front matter with all required fields, kind: claim,
  artifacts_owned has ≥1 item
- `test/fixtures/invalid_missing_kind.md` — front matter missing `kind` field
- `test/fixtures/invalid_claim_no_artifacts.md` — kind: claim with empty artifacts_owned

Write `coord_validator_test.exs` asserting:
- `CoordValidator.validate_file/1` returns `{:ok, _}` for the valid fixture
- `{:error, errors}` with `errors` being a non-empty list for each invalid fixture
- `{:error, :no_frontmatter}` when a file has no YAML front matter block

Confirm tests fail (RED) before writing implementation.

### Step 3 — Implement CoordValidator (GREEN phase)

`lib/agov/coord_validator.ex`:

1. `validate_file(path)` — read file, extract YAML front matter, parse, validate against schema
2. `extract_frontmatter(content)` — regex: `^---\n(.*?)\n---`
3. `validate_against_schema(data)` — load schema from `spec/schemas/coordination-message.schema.json`
   (path relative to project root, not tooling/elixir/), run `ExJsonSchema.Validator.validate/2`

Schema path resolution: accept via application config or env var `AGOV_SCHEMA_DIR` defaulting
to `../../spec/schemas` relative to the mix project root. Keeps tooling decoupled from cwd.

### Step 4 — Implement Mix task

`lib/mix/tasks/agov.coord.validate.ex` — `Mix.Task`:
- Invocation: `mix agov.coord.validate <file> [<file>...]`
- Reads each file via `CoordValidator.validate_file/1`
- Prints `OK  <file>` or `ERR <file>` with error details
- Exit code 0 if all valid, 1 if any invalid

### Step 5 — Refactor (REFACTOR phase)

- Extract error formatting to a separate function
- Confirm no code duplication with any future validator tasks

---

## Test strategy

| Test tier | What | Where |
|---|---|---|
| Unit | `CoordValidator.extract_frontmatter/1`, `validate_against_schema/1`, valid/invalid fixtures | `test/agov/coord_validator_test.exs` |
| Manual (Mix task) | `mix agov.coord.validate docs/coordination/00003-claude.md` against real coordination files | Run locally, output inspected |

TDD order: fixtures → test → RED confirmed → implementation → GREEN confirmed → refactor.

---

## Risks

| Risk | Likelihood | Mitigation |
|---|---|---|
| `ex_json_schema` draft-07 support gaps | Low | Library is well-maintained and explicitly targets draft-07 |
| YAML parsing in Elixir (no stdlib YAML) | Medium | Use `yamerl` or `yaml_elixir`; add as dependency |
| Schema path resolution in CI / different cwd | Medium | Use env var `AGOV_SCHEMA_DIR` with explicit override in tests |
| Elixir install time (compile from source) | Low | mise.toml already has `compile=false` pattern for Python; apply same for Elixir if needed |

---

## Rollback

Remove `tooling/elixir/` contents (except `.gitkeep`) if the approach fails.
No shared state is modified; the schema in `spec/` is read-only from this tool.

---

## Out of scope (deferred)

- Graph traversal of `references-to` (deferred to a future task)
- Validate entire `docs/coordination/` directory in one invocation (can add after MVP)
- CI integration (separate task)
- `agov-coord-validate` as an escript binary (Mix task is sufficient for now)
