# AGF v2 manifest

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

## Provenance

| Field | Value |
|-------|-------|
| Framework | agents-govern (AGF) |
| Version | 2.0.0 (provisional — unreleased) |
| Source | AGF-NG foundry |
| Repository | https://gitlab.com/janipaijanen/agents-govern-framework |
| License | CC-BY-SA-4.0 |
| Layout | B (single `agf/` directory; adopter files stay in `docs/`) |

This manifest describes the `agf/` directory as it stands in the foundry.
AGF v2.0 is provisional and not yet independently packaged. Adopters who
adopt from this repository adopt from the foundry directly.

## Contents

| Path | Purpose | Adopt as |
|------|---------|---------|
| `agents/` | Agent role definitions (one YAML per role) | Install under `agf/agents/` or use in place |
| `agents/MANIFEST.yaml` | Example active-role status file (foundry dogfood) | Replace with your own project's role-status file |
| `rules/` | Governance rules (`always/`, `workflow/`, `docs/`, `ui/`) | Install to your assistant's rules directory; search `<!-- CUSTOMIZE -->` |
| `schemas/` | JSON schemas and invariants for validation | Use with Elixir tooling (`tooling/elixir/`) |
| `templates/` | First-run starter files (coordination, plans, learning) | Ask your assistant to prepare first-run evidence from these templates |
| `skills/` | AI assistant workflow scripts | Install to your assistant's skills directory when supported |
| `project-specific-examples/` | Stack-specific rule examples | Use only when the stack matches |
| `ADOPTING.md` | Step-by-step adoption guide | Follow step by step |
| `framework.md` | Framework theory and reference | Read-only reference |
| `CHANGELOG.md` | v2.0 change history | Reference |
| `VERSION` | Current version string (`2.0.0`) | Reference |
| `LICENSE` / `LICENSE-MIT` | CC-BY-SA-4.0 and MIT dual license | Include in project |
| `docs/governance/` | Governance documents (this file, learning system, retrospective) | Read-only reference |
| `docs/contracts/` | Agent-output binding contract, integration contract, stability promise | Read before enabling full compliance |
| `docs/adopt/` | Compliance profile definitions | Use to choose adoption depth |
| `docs/adr/` | Architecture decision records | Informational |

## Quick start

Use the public installation guide first:
[Install the `agf/` directory](../site/docs/adopt/install-and-layouts.md).
It gives you a pasteable commissioning prompt so your assistant prepares the
file moves, local evidence directories, rule install target, and first learning
record.

After the assistant prepares the diff, review:

- `agf/` exists as the single framework directory;
- your assistant rules directory contains the AGF rules it will load;
- `agf/learnings/<codename>.yaml` exists and does not reveal the real project
  name;
- `docs/plans/` and `docs/coordination/` exist, or your project has documented
  different paths;
- `agf/agents/MANIFEST.yaml` is replaced or clearly documented as your local
  role-status authority, not the foundry dogfood roster;
- remaining `CUSTOMIZE` markers are deliberate and tracked.

For the full 7-step adoption process, see [ADOPTING.md](../../ADOPTING.md).

## Upgrading

When upgrading from a previous version, record the upgrade in your learning record:

```yaml
upgrade_history:
  - date: "<date>"
    from_version: "<previous>"
    to_version: "2.0.0"
    agent_changes: []
    notes: ""
```

## v1.x adopters

If you adopted from agents-govern v1.x, the `agf/` directory is a structural
restructure of the same rules, agents, and schemas. No rule or agent behaviour
changed in v2.0.0. See `CHANGELOG.md` for the full list of path renames.
The v1.x manifest (agents-govern 1.1.50) is no longer part of this distribution.
