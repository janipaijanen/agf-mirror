> **Status: Provisional.** agents-govern **v1.x is canonical**; this v2.0
> directory is a side-by-side restructure under evaluation. Adopt v1.x if
> you need stability today.

# agents-govern 2.0

Governance for AI-assisted software development. agents-govern defines
roles with bounded authority for AI coding agents, places review gates
before implementation (Gate 1) and before merge (Gate 2), records every
handoff and decision in an append-only coordination channel, and turns
operational incidents into structured learning records that feed the
framework's evolution. It is files, rules, and conventions — not a
runtime: your AI assistant loads the rules, your CI runs the validators,
and a human governor holds final authority.

v2.0 is a structural release: the same framework content as v1.x,
reorganised so that the repository root stays small, governance documents
live under `docs/governance/`, contracts under `docs/contracts/`, adoption
material under `docs/adopt/`, and a buildable documentation site under
`docs/site/`. Behavioural changes to rules or agents are out of scope for
the restructure; see [CHANGELOG.md](CHANGELOG.md).

## Is this for you?

The framework earns its overhead when AI assistants write a significant
share of your code, the project lives long enough for an audit trail to
matter, and someone can own the governor role. It is the wrong tool for
throwaway code, human-only teams, or anyone expecting a hosted service.
The documentation site has an honest decision page:
[should you adopt?](docs/site/docs/start/should-you-adopt.md), and the two
pages worth reading before any commitment:
[downstream impact](docs/site/docs/adopt/downstream-impact.md) and
[exit and decommission](docs/site/docs/adopt/exit-and-decommission.md).

## Quick start (developers)

```bash
# Validate the governance artifacts
pip install .            # or: uv sync
agov-validate-agents
agov-validate-learnings

# Build the documentation site (strict — broken links fail)
pip install mkdocs==1.6.1
mkdocs build -f docs/build/mkdocs.yml --strict
```

Adoption itself is documented step by step in [ADOPTING.md](ADOPTING.md);
the distribution inventory is in
[docs/governance/manifest.md](docs/governance/manifest.md).

## Layout

| Path | Contents |
|---|---|
| `agents/` | Twelve agent role definitions (YAML) + manifest |
| `rules-template/` | Governance rule library to copy into your assistant config |
| `schemas/` | JSON Schemas and invariants for validation |
| `learnings/` | Upstream learning records + adopter template |
| `docs/governance/` | Governance charter, learning system, manifest, retrospective, disclaimer, anonymization notice |
| `docs/contracts/` | Binding, integration, and stability contracts |
| `docs/adopt/` | Compliance profile, decommission guide |
| `docs/site/` | Documentation site sources (MkDocs) |
| `docs/build/` | Site build configuration |
| `docs/reference/` | One-page quick reference |
| `src/`, `pyproject.toml` | Validator and distribution tooling (`agov-*`) |
| `skills/`, `scripts/`, `project-specific-examples/` | Assistant skills, helpers, stack-specific rule sets |

`framework.md` is the theory and reference document; `framework.yaml` is
its machine-readable companion; `ADOPTING.md` is the adoption guide.

## License

Framework content is CC BY-SA 4.0 ([LICENSE](LICENSE)); the Python
tooling is MIT (see `pyproject.toml`). Security policy:
[SECURITY.md](SECURITY.md). Contributions: [CONTRIBUTING.md](CONTRIBUTING.md).
