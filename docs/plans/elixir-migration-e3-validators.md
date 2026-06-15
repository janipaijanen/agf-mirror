---
id: elixir-migration-e3-validators
status: gate1-approved
topic: elixir-validator-implementation
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-06-15"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: local validation tooling only; no secrets, auth, or isolation changes
  - role: contribution-auditor
    status: not-applicable
    reason: internal foundry tooling; not an upstream submission
---

# Plan: E3 — Elixir Validator Implementation

**Date:** 2026-06-15T19:14:52Z
**Status:** GATE 1 APPROVED
**Depends on:** E1+E2 complete (seq 357); schemas exist in `agf/schemas/`
**Branch:** `feat/elixir-e3-validators`
**Coder-tester:** Codex

---

## Intent

Implement 7 missing validators as Elixir Mix tasks, replacing the Python
`agov-validate-*` commands in the primary adoption path. After E3 merges,
Gates 6 and 7 can proceed.

`validate_binding` (git commit trailer / sidecar system) is deferred to E4 —
it requires git integration and is not in the primary validation path.

---

## Affected Files

| File | Action |
|---|---|
| `tooling/elixir/lib/agov/validate_learnings.ex` | Create — pure validation module |
| `tooling/elixir/lib/agov/validate_agents.ex` | Create — pure validation module |
| `tooling/elixir/lib/agov/validate_adoption.ex` | Create — pure validation module |
| `tooling/elixir/lib/agov/validate_infosec.ex` | Create — pure validation module |
| `tooling/elixir/lib/agov/validate_contributions.ex` | Create — pure validation module |
| `tooling/elixir/lib/agov/validate_framework_config.ex` | Create — pure validation module |
| `tooling/elixir/lib/mix/tasks/agov.validate.learnings.ex` | Create — Mix task |
| `tooling/elixir/lib/mix/tasks/agov.validate.agents.ex` | Create — Mix task |
| `tooling/elixir/lib/mix/tasks/agov.validate.adoption.ex` | Create — Mix task |
| `tooling/elixir/lib/mix/tasks/agov.validate.infosec.ex` | Create — Mix task |
| `tooling/elixir/lib/mix/tasks/agov.validate.contributions.ex` | Create — Mix task |
| `tooling/elixir/lib/mix/tasks/agov.validate.framework_config.ex` | Create — Mix task |
| `tooling/elixir/lib/mix/tasks/agov.validate.ex` | Create — orchestrator Mix task |
| `tooling/elixir/test/agov/validate_learnings_test.exs` | Create — unit tests |
| `tooling/elixir/test/agov/validate_agents_test.exs` | Create — unit tests |
| `tooling/elixir/test/agov/validate_adoption_test.exs` | Create — unit tests |
| `tooling/elixir/test/agov/validate_infosec_test.exs` | Create — unit tests |
| `tooling/elixir/test/agov/validate_contributions_test.exs` | Create — unit tests |
| `tooling/elixir/test/agov/validate_framework_config_test.exs` | Create — unit tests |
| `tooling/elixir/test/fixtures/valid_learning.yaml` | Create — positive fixture |
| `tooling/elixir/test/fixtures/invalid_learning_pii.yaml` | Create — PII fixture |
| `tooling/elixir/test/fixtures/valid_agent.yaml` | Create — positive fixture |
| `tooling/elixir/test/fixtures/invalid_agent_missing_tier.yaml` | Create — negative fixture |

**Must NOT touch:** `tooling/elixir/test/agov/claim_tracker_test.exs`

---

## Migration Needed

No schema migrations. New Elixir modules only.

---

## Path Resolution Convention

All validators use the same env-var-with-default pattern already established
in the codebase:

```elixir
@default_schema_dir "../../agf/schemas"
@default_learnings_dir "../../agf/learnings"
@default_agents_dir "../../agf/agents"
@default_root "../../agf"

defp schema_dir, do: System.get_env("AGOV_SCHEMA_DIR", @default_schema_dir)
defp learnings_dir, do: System.get_env("AGOV_LEARNINGS_DIR", @default_learnings_dir)
defp agents_dir, do: System.get_env("AGOV_AGENTS_DIR", @default_agents_dir)
defp root_dir, do: System.get_env("AGOV_ROOT", @default_root)
```

Paths are resolved relative to the mix project root (`tooling/elixir/`).

---

## Implementation Order (TDD: RED → GREEN → REFACTOR per validator)

### Step 1 — `Agov.ValidateLearnings` (priority 1)

**Interface:**
```elixir
Agov.ValidateLearnings.validate_file(path :: String.t()) ::
  {:ok, map()} | {:error, [{message :: String.t(), location :: String.t()}]}

Agov.ValidateLearnings.validate_all(dir :: String.t() | nil) ::
  {:ok, [String.t()]} | {:error, [{file :: String.t(), errors :: list()}]}
```

**Checks (in order):**
1. Parse full YAML file (not front matter — learnings are full YAML docs)
2. Validate against `learning.schema.json` using `ExJsonSchema`
3. Semantic checks:
   - PII heuristics on freetext fields (description, text, summary):
     - URLs (`https?://`)
     - Email addresses (`[^@]+@[^@]+\.[^@]+`)
     - Company suffixes (`\b(Inc|Corp|Ltd|GmbH|LLC|Oy|Oyj|Pty|plc)\b`)
     - Internal hostnames (`\b[a-z0-9-]+\.(internal|local|corp|cloud)\b`)
   - Confidence level (if present) must be one of:
     `single-adopter`, `multi-adopter`, `longitudinal`, `industry-corroborated`
   - `multi-adopter` or `longitudinal` confidence requires evidence_count >= 2
     OR length of evidence_refs >= 2

**Mix task:** `mix agov.validate.learnings [DIR]`
- Default dir: `$AGOV_LEARNINGS_DIR` or `../../agf/learnings`
- Output: `OK  file.yaml` / `WARN file.yaml: <message>` / `ERR file.yaml: <message>`
- Exit 0 if no errors; exit 1 if any errors (warnings pass)

**Test fixtures:**
- `test/fixtures/valid_learning.yaml` — minimal valid learning with `project.codename` and one entry
- `test/fixtures/invalid_learning_pii.yaml` — learning with email address in text field

---

### Step 2 — `Agov.ValidateAgents` (priority 2)

**Interface:**
```elixir
Agov.ValidateAgents.validate_file(path :: String.t()) ::
  {:ok, map()} | {:error, [{message :: String.t(), location :: String.t()}]}

Agov.ValidateAgents.validate_all(dir :: String.t() | nil) ::
  {:ok, [String.t()]} | {:error, [{file :: String.t(), errors :: list()}]}
```

**Checks:**
1. Parse full YAML file
2. Validate against `agent.schema.json`
3. Semantic checks:
   - `agent.tier` must be one of:
     `production`, `governance`, `cross-cutting`, `publication-support`, `meta`
   - Skip `MANIFEST.yaml` (not an agent definition)

**Mix task:** `mix agov.validate.agents [DIR]`
- Default dir: `$AGOV_AGENTS_DIR` or `../../agf/agents`
- Same output format as validate_learnings

**Test fixtures:**
- `test/fixtures/valid_agent.yaml` — minimal valid agent with name, id, tier
- `test/fixtures/invalid_agent_missing_tier.yaml` — agent missing the tier field

---

### Step 3 — `Agov.ValidateAdoption` (priority 3)

**Interface:**
```elixir
Agov.ValidateAdoption.validate(root :: String.t() | nil) ::
  {:ok, [warning :: String.t()]} | {:error, [{message :: String.t(), location :: String.t()}]}
```

**Checks (structural, no schema):**
1. `agents/` directory exists at root
2. At least one YAML file in `agents/` (excluding MANIFEST.yaml)
3. Mandatory agents present: `contribution-auditor`, `infosec-sentinel`
   (search by agent `id` field, not just filename)
4. `GOVERNANCE.md` exists at root
5. `agents/MANIFEST.yaml` exists at root

Items 1, 3, 4 are errors. Items 2, 5 are errors. None are warnings.

**Mix task:** `mix agov.validate.adoption [ROOT]`
- Default root: `$AGOV_ROOT` or `../../agf`

---

### Step 4 — `Agov.ValidateInfosec` (priority 4)

**Interface:**
```elixir
Agov.ValidateInfosec.validate_file(path :: String.t()) ::
  {:ok, []} | {:error, [{message :: String.t(), location :: String.t()}]}

Agov.ValidateInfosec.validate_all(dir :: String.t() | nil) ::
  {:ok, [String.t()]} | {:error, [{file :: String.t(), findings :: list()}]}
```

**Scans for (all are errors — blocks publish):**
```
VCS tokens:    ~r/(glpat-|ghp_|gho_|github_pat_|glcbt-|glrt-|xoxb-|xoxp-)[^\s"]{4,}/
DB conn str:   ~r/(postgres|mysql|redis|mongodb|amqp):\/\/[^\s"]*[:@]/
API keys:      ~r/(api[_-]?key|secret[_-]?key|SECRET_KEY|password)\s*[:=]\s*[^\s"]{8,}/i
Private keys:  ~r/BEGIN\s+(RSA |EC |DSA |OPENSSH )?PRIVATE KEY/
Filesystem:    ~r/\/(home|var|opt|srv|etc)\/[a-zA-Z0-9_\/.-]+/
```

**Mix task:** `mix agov.validate.infosec [DIR]`
- Default dir: `$AGOV_LEARNINGS_DIR` or `../../agf/learnings`

---

### Step 5 — `Agov.ValidateContributions` (priority 5)

**Interface:**
```elixir
Agov.ValidateContributions.validate_file(path :: String.t()) ::
  {:ok, [warning :: String.t()]} | {:error, [{message :: String.t(), location :: String.t()}]}
```

**Patterns (all are warnings, not errors — require human review):**

Absence-of-incident bias: freetext fields that claim no failures without
evidence (e.g., "no issues found", "no problems occurred").

Governance relaxation: proposals to weaken, skip, or bypass a rule without
citing evidence (e.g., "we could remove", "we should skip", "bypass the").

Evidence quality: `multi-adopter` or `longitudinal` confidence claimed with
`evidence_count: 1` or empty `evidence_refs`.

**Mix task:** `mix agov.validate.contributions [DIR]`
- Default dir: `$AGOV_LEARNINGS_DIR` or `../../agf/learnings`
- Warnings do not cause exit 1

---

### Step 6 — `Agov.ValidateFrameworkConfig` (priority 6)

**Interface:**
```elixir
Agov.ValidateFrameworkConfig.validate_file(path :: String.t()) ::
  {:ok, map() | :skipped} | {:error, [{message :: String.t(), location :: String.t()}]}
```

**Checks:**
1. Opt-in: if file does not exist, return `{:ok, :skipped}` (not an error)
2. Validate against `framework.schema.json`
3. Cross-check `contract_version` field against `VERSION` file at root

**Mix task:** `mix agov.validate.framework_config [FILE]`
- Default file: `$AGOV_ROOT/framework.yaml` or `../../agf/framework.yaml`

---

### Step 7 — `Mix.Tasks.Agov.Validate` (orchestrator)

`mix agov.validate [ROOT]`

Runs all 6 validators in order:
1. `mix agov.validate.learnings`
2. `mix agov.validate.agents`
3. `mix agov.validate.adoption`
4. `mix agov.validate.infosec`
5. `mix agov.validate.contributions`
6. `mix agov.validate.framework_config`

Prints a summary table. Exit 0 if no errors across any validator.
Exit 1 if any validator produced errors.

---

## Test Strategy

| Tier | What | Where |
|---|---|---|
| Unit | Each validator module in isolation with file fixtures | `test/agov/validate_*_test.exs` |
| Integration | `mix agov.validate` against real `agf/` directory | Manual invocation, inspected |

TDD order per validator: write fixture → write test (RED) → implement (GREEN) → refactor.

Minimum tests per validator:
- 1 positive fixture case (`{:ok, _}`)
- 1 negative fixture case (`{:error, _}` or `{:ok, [warning]}`)

Do not write more test infrastructure than test logic. Use real YAML fixtures
(not mocks) — this is boundary shape testing (ref: boundary-shape-testing-rule).

---

## Risks

| Risk | Mitigation |
|---|---|
| `YamlElixir` / `yamerl` already a dependency? | Check `mix.exs`; add if not present |
| Schema path resolution in CI | Use `AGOV_SCHEMA_DIR` env var; set in CI if needed |
| `learning.schema.json` says "Python validator is contract of record" | E3 makes Elixir the new contract; schema description is stale and will be updated in E5 |
| `agf/learnings/foundry.yaml` has non-standard structure | Test against actual file; adjust checks as needed |

---

## Constraints

- Do NOT modify `tooling/elixir/test/agov/claim_tracker_test.exs`.
- Do NOT modify existing tests; only add new files.
- Gate 1 plan schema requires `advisory_consultation` with infosec-sentinel and
  contribution-auditor entries — verify each new Gate 1 plan file passes
  `mix agov.plan.validate` before committing.
- One branch: `feat/elixir-e3-validators`. File a Gate 2 handoff to
  governor-reviewer when all tests pass.

---

## Rollback

If any validator proves significantly more complex than estimated: file a note
to coordination, split off that validator to a separate E3b branch, and merge
what's ready. Do not block the entire E3 branch on one difficult validator.
