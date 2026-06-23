---
name: upgrade-agents-govern
origin: upstream
description: >
  Gated upgrade workflow for adopting a new agents-govern release.
  Extracts the archive, compares upstream vs local, applies changes
  surgically, updates the adoption record, and verifies infosec.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Upgrade agents-govern

Perform a gated upgrade of the agents-govern framework in this project.
Follow every step in order. **Do not skip a gate.**

## Prerequisites

- The new release archive (`.tar.gz` or `.zip`) is available locally.
- The current framework version is recorded in the adoption section of
  `learnings/<codename>.yaml`.

<!-- CUSTOMIZE: adjust paths if your project uses Layout B (subdirectory) -->

## Step 1 — Extract and inspect

1. Extract the archive to a temporary directory.
2. Read `CHANGELOG.md` in the archive — identify every entry between the
   current version and the new version.
3. Read `docs/governance/manifest.md` for the new version's provenance.

Present the changelog entries to the user.

### GATE 1 — User confirms changelog review

> **Wait for explicit user confirmation before continuing.**
> The user may decide not to upgrade after reviewing changes.

## Step 2 — Compare upstream vs local

For each category, compare the archive contents against the project's
current files:

| Category | Archive path | Local path | Compare how |
|----------|-------------|------------|-------------|
| Agent definitions | `agents/*.yaml` | <!-- CUSTOMIZE: e.g., agf/agents/*.yaml --> `agents/*.yaml` | diff field by field; flag `customized: true` agents |
| Rule templates | `rules/` | `agf/rules/` | diff; keep project-local loader files separate |
| Scripts | `scripts/` | <!-- CUSTOMIZE --> `scripts/` | diff |
| Learning template | `learnings/_template.yaml` | `learnings/_template.yaml` | diff; check for new fields |
| Framework docs | `framework.md`, `ADOPTING.md`, etc. | local copies | diff |

**For agent definitions marked `customized: true`** in the adoption record:
show the diff but do not overwrite. Present the upstream changes so the
user can decide what to merge manually.

**For rule files with `origin: project`** in frontmatter: skip entirely.
These are project-specific and must not be touched.

Also inspect copied assistant-rule or assistant-skill trees such as
`.claude/rules/` and `.claude/skills/` when they exist. If they contain
AGF-provided files copied from an older release, replace those copies with a
loader reference to `agf/`. If they contain intentional project-specific
overrides, move those overrides outside copied AGF rule or skill trees before
continuing the upgrade.

Present a summary table: files unchanged / updated / new / skipped.

### GATE 2 — User confirms comparison summary

> **Wait for explicit user confirmation before applying changes.**

## Step 3 — Apply changes

1. Copy new and updated files from the archive to local paths.
2. For new rule templates: add `origin: upstream` to frontmatter if not
   already present.
3. For updated agent definitions (non-customized): overwrite in place.
4. For updated agent definitions (customized): skip — user handles manually.
5. Merge new fields from `_template.yaml` into the project's learning
   record if applicable (append only, do not overwrite existing data).

## Step 4 — Update adoption record

In `learnings/<codename>.yaml`, update:

```yaml
adoption:
  framework_version: "<new version>"
  framework_commit: "<new commit from docs/governance/manifest.md>"

upgrade_history:
  - date: "<today>"
    from_version: "<old version>"
    to_version: "<new version>"
    agent_changes: [<list of updated agent IDs>]
    notes: "<summary of what changed>"
```

## Step 5 — Verify

1. Run `agov validate infosec`
   — ensure no new findings.
2. Run `agov validate learnings` — ensure learning
   record is still valid.
3. Run `agov validate`, `agov plan validate` on changed plan files, and
   `agov coord validate` on changed coordination files. If a needed Rust
   command is missing, report the tooling gap and record any manual check in
   the MR.
4. If pre-commit hooks are configured, run them.

## Step 6 — Summarise

Output a summary:

- Previous version → new version
- Files updated (count by category)
- Files skipped (customized agents, project-origin rules)
- New files added
- Validation results
- Any manual actions remaining (customized agent merges)
