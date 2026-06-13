# Proposal: Elixir-First Tooling Direction

## Status

Proposed.

## Decision

AGF-NG tooling should converge on Elixir/Mix. Python should not be a strategic
tooling surface for new AGF-NG work.

This means:

- New validators, generators, handoff tools, learning tools, release checks,
  and session-continuity commands should be implemented as `mix agov.*` tasks
  under `tooling/elixir/`.
- New documentation should teach Mix invocation first.
- Existing Python tooling in imported or transitional framework material is
  migration debt, not the direction of travel.
- Python may still appear in stack-specific examples when the downstream
  project itself is Python, but those examples are not AGF-NG framework
  tooling.

## Rationale

The root AGF-NG project already uses Elixir/Mix for coordination, plan, and
handoff tooling. Continuing to add Python surfaces would split the operational
model and make adoption harder to explain:

- one command family is easier for humans and AI agents to remember;
- one dependency/runtime story is easier to install in CI;
- one validator implementation language reduces drift between schemas,
  semantic checks, and coordination tooling;
- Elixir is already where the governance state-machine behavior lives.

The framework should not ask adopters to install Python for AGF-NG tooling if
the project has chosen Elixir as its tooling runtime.

## Migration Principles

1. **No new Python tooling.** Any new command proposed after this decision
   should be a Mix task unless the governor explicitly grants an exception.
2. **Retire Python by surface, not by deletion.** Existing Python commands
   should be replaced by equivalent Mix tasks before docs stop mentioning
   them or release archives stop shipping them.
3. **Keep schemas language-neutral.** JSON Schema and YAML artifact formats
   remain portable. Elixir owns validation semantics and command behavior.
4. **Preserve adopter exits.** Removing Python tooling from the framework
   must not scatter files outside the single extracted distribution directory.
5. **Document transitional status honestly.** While Python references remain
   in `agents-govern-2.0/`, label them as legacy/transitional rather than
   current AGF-NG direction.

## Candidate Migration Slices

| Slice | Goal | Notes |
|---|---|---|
| E1 | Inventory Python tooling surfaces | List `pyproject.toml`, `src/agents_govern/`, scripts, docs, CI references, and release manifest entries. |
| E2 | Map Python CLI commands to Mix tasks | Produce command matrix: current `agov-*` command, target `mix agov.*`, owner module, test fixtures. |
| E3 | Implement missing validators in Elixir | Prioritize learning, agent, infosec/anonymization, contribution, and adoption validators. |
| E4 | Replace distribution and schema generators | Move release/build helpers and schema generation into Mix tasks or language-neutral build steps. |
| E5 | Update docs and release manifests | Replace `uv run agov-*` and Python package references with Mix equivalents. |
| E6 | Remove Python package from shipped framework | Delete or archive Python tooling only after Mix replacements pass in CI. |

## Immediate Documentation Rule

Until migration is complete, documentation should avoid introducing new
Python-based examples for framework tooling. Prefer:

```bash
cd tooling/elixir
mix agov.<domain>.<action> ...
```

For adopter-facing v2 documentation, the desired eventual form is:

```bash
cd agents-govern/tooling/elixir
mix agov.validate.learnings learnings/<codename>.yaml
```

The exact distribution path may change during E2/E4; do not freeze it in
public docs until the packaging plan is approved.

## Open Questions

- Should the distributed framework contain `tooling/elixir/` at the root of
  the extracted `agents-govern/` directory, or should Mix tooling remain in a
  top-level `tooling/` directory only in the AGF-NG source repository?
- Should compatibility shims exist for old `agov-*` command names, and if so,
  for how many releases?
- Which Python validators are still the contract of record today, and which
  Elixir modules already cover equivalent behavior?

## Non-Goals

- This proposal does not remove Python stack examples.
- This proposal does not rewrite all validators in one change.
- This proposal does not change artifact schemas.
- This proposal does not require downstream projects to use Elixir for their
  product code.
