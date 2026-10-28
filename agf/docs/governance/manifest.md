# AGF v2 manifest

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

## Provenance

| Field | Value |
|-------|-------|
| Framework | agents-govern (AGF) |
| Version | 2.12.0 |
| Source | AGF-NG foundry |
| Repository | https://gitlab.com/janipaijanen/agents-govern-framework |
| License | CC-BY-SA-4.0 |
| Layout | B (single `agf/` directory; adopter files stay in `docs/`) |

This manifest describes the `agf/` directory as it stands in the foundry.
AGF v2.11.0 is the latest published, independently verified stable release
(tagged 2026-10-26). Adopters should install releases from the GitLab Releases
page and verify the published checksums and provenance rather than
treating an arbitrary repository checkout as a release artifact.

## Contents

| Path | Purpose | Adopt as |
|------|---------|---------|
| `agents/` | Agent role definitions (one YAML per role) | Install under `agf/agents/` or use in place |
| `agents/MANIFEST.yaml` | This foundry's own local role-status authority (adopter-owned path; never packaged in release archives) | Not present in a release archive — start from `agents/MANIFEST.example.yaml` instead |
| `agents/MANIFEST.example.yaml` | Neutral role-status template shipped in release archives | Copy to `agents/MANIFEST.yaml` and edit each role's `status`/`played_by` before wiring manifest-driven tooling |
| `rules/` | Governance rules (`always/`, `workflow/`, `docs/`, `ui/`) | Keep under `agf/rules/`; point your assistant at this canonical copy |
| `schemas/` | JSON schemas and invariants for validation | Use through the shipped Rust `agov` validators |
| `templates/` | First-run starter files (coordination, plans, learning) | Ask your assistant to prepare first-run evidence from these templates |
| `skills/` | AI assistant workflow scripts | Keep under `agf/skills/`; use local wrappers only when your tool requires them |
| `project-specific-examples/` | Stack-specific rule examples | Use only when the stack matches |
| `ADOPTING.md` | Step-by-step adoption guide | Follow step by step |
| `framework.md` | Framework theory and reference | Read-only reference |
| `CHANGELOG.md` | v2.0 change history | Reference |
| `VERSION` | Current version string (`2.12.0`) | Reference |
| `LICENSE` / `LICENSE-MIT` | CC-BY-SA-4.0 and MIT dual license | Include in project |
| `docs/governance/` | Governance documents (this file, learning system, retrospective) | Read-only reference |
| `docs/contracts/` | Agent-output binding contract, integration contract, stability promise | Read before enabling full compliance |
| `docs/adopt/` | Compliance profile definitions | Use to choose adoption depth |
| `docs/adr/` | Architecture decision records | Informational |

## Quick start

Use the public installation guide first:
[Install the `agf/` directory](../site/docs/adopt/install-and-layouts.md).
It gives you a pasteable commissioning prompt so your assistant prepares the
file moves, local evidence directories, optional assistant loader, and first
learning record.

After the assistant prepares the diff, review:

- `agf/` exists as the single framework directory;
- your assistant can read the canonical AGF rules from `agf/rules/`;
- `agf/learnings/<codename>.yaml` exists and does not reveal the real project
  name;
- `docs/plans/` and `docs/coordination/` exist, or your project has documented
  different paths;
- `agf/agents/MANIFEST.yaml` exists as your local role-status authority —
  copied from the shipped `agf/agents/MANIFEST.example.yaml` template and
  edited, not this foundry's own dogfood roster;
- remaining `CUSTOMIZE` markers are deliberate and tracked.

For the full 7-step adoption process, see [ADOPTING.md](../../ADOPTING.md).

## Upgrading

When upgrading from a previous version, record the upgrade in your learning record:

```yaml
upgrade_history:
  - date: "<date>"
    from_version: "<previous>"
    to_version: "2.12.0"
    agent_changes: []
    notes: ""
```

## v1.x adopters

If you adopted from agents-govern v1.x, the `agf/` directory is a structural
restructure of the same rules, agents, and schemas. No rule or agent behaviour
changed in v2.0.0. See `CHANGELOG.md` for the full list of path renames.
The v1.x manifest (agents-govern 1.1.50) is no longer part of this distribution.
