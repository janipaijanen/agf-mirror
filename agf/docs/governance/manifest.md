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
| `agents/` | Agent role definitions (one YAML per role) | Copy to `agf/agents/` or use in place |
| `agents/MANIFEST.yaml` | Example active-role status file (foundry dogfood) | Replace with your own project's role-status file |
| `rules/` | Governance rules (`always/`, `workflow/`, `docs/`, `ui/`) | Copy to `.claude/rules/`; search `<!-- CUSTOMIZE -->` |
| `schemas/` | JSON schemas and invariants for validation | Use with Elixir tooling (`tooling/elixir/`) |
| `templates/` | First-run starter files (coordination, plans, learning) | Copy into `docs/coordination/`, `docs/plans/`, `agf/learnings/` |
| `skills/` | AI assistant workflow scripts | Copy to `.claude/skills/` (optional) |
| `project-specific-examples/` | Stack-specific rule examples | Copy if stack matches |
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

```bash
# 1. Create your agf/ directory (copy from foundry or clone)
git clone https://gitlab.com/janipaijanen/agents-govern-framework
cp -r agents-govern-framework/agf/ your-project/agf/

# 2. Copy governance rules into your AI assistant's rules directory
mkdir -p .claude
cp -r agf/rules/ .claude/rules/

# 3. (Optional) Copy skills
cp -r agf/skills/ .claude/skills/

# 4. Create your project learning record (use a codename, not the real name)
mkdir -p agf/learnings
cp agf/templates/learning-record.yaml agf/learnings/<codename>.yaml

# 5. Create working evidence directories
mkdir -p docs/plans docs/coordination

# 6. Replace foundry role-status with your own
cp agf/agents/MANIFEST.yaml agf/agents/MANIFEST.yaml
# Edit to reflect your project's actual roles and who plays them

# 7. Customize rules
#    Search for <!-- CUSTOMIZE --> in .claude/rules/**/*.md
#    Edit to match your stack, locale set, and project conventions

# 8. (Optional) Stack-specific rules
#    cp -r agf/project-specific-examples/python-flask/ .claude/rules/
```

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
