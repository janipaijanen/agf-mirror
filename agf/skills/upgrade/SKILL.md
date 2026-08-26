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
- The current framework version will be identified by Step 0, below,
  from the adoption section of `agf/learnings/<codename>.yaml`.

<!-- CUSTOMIZE: adjust paths if your project uses Layout B (subdirectory) -->

## Step 0 — Identify the authoritative adoption record

Before Step 1 reads or presents anything. **This step identifies and
reads only — it performs no version comparison and writes nothing to
any record.**

1. Look for exactly one `.yaml` file in `agf/learnings/` (excluding
   `_template.yaml`) whose `adoption.checkout_role` is already `live`.
   - **If exactly one such record exists:** select it automatically. No
     operator prompt.
   - **Otherwise (zero live records, regardless of how many candidate
     files exist — including exactly one candidate file, whether it
     carries `checkout_role: imported` explicitly or no `checkout_role`
     at all):** enumerate every candidate `.yaml` file in
     `agf/learnings/` (excluding `_template.yaml`), showing each file's
     `checkout_role` if set, and require the human operator to
     explicitly name exactly one as "this checkout's own live adoption
     record." **Do not guess** via file count, alphabetical order,
     most-recently-modified, filename matching a git identity, or any
     other heuristic — even when only one candidate file exists. If the
     operator selects a candidate already marked `checkout_role:
     imported`, that is a legitimate explicit override — Step 3 later
     overwrites it to `live`, exactly as if it had been unmarked.
2. Read the selected record's `adoption.framework_version` field
   exactly as recorded — however stale it may be. **Do not compare it
   to `agf/VERSION`. Do not block on a mismatch. Do not write anything
   to the record.** The recorded value, however stale, is the correct
   starting point for Step 1's changelog range: it represents the last
   version this checkout's adoption process actually reviewed, not
   necessarily the version the tree happens to be at today. Comparing
   or correcting it here would silently truncate the range Gate 1 is
   about to present.

## Step 1 — Extract and inspect

1. Extract the archive to a temporary directory.
2. Read `CHANGELOG.md` in the archive — identify every entry between
   the Step-0-identified candidate's recorded `framework_version` (the
   current version, read exactly as recorded — see Step 0; not
   cross-checked against `agf/VERSION`) and the new version.
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

1. **First, before any archive file is copied:** on the record Step 0
   already identified — never a new selection — write
   `adoption.checkout_role: live` (if not already set) and
   `adoption.upgrade_in_progress: true` plus
   `adoption.upgrade_started_at: "<current RFC3339 timestamp>"`. This
   step performs no new selection decision; it only persists the
   identity Step 0 already established.
2. Copy new and updated files from the archive to local paths.
3. For new rule templates: add `origin: upstream` to frontmatter if not
   already present.
4. For updated agent definitions (non-customized): overwrite in place.
5. For updated agent definitions (customized): skip — user handles manually.
6. Merge new fields from `_template.yaml` into the project's learning
   record if applicable (append only, do not overwrite existing data).

## Step 4 — Update adoption record

In the record Step 0 identified (`agf/learnings/<codename>.yaml`), update:

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

**Immediately after** `adoption.framework_version` is updated above,
**remove** the `adoption.upgrade_in_progress` and
`adoption.upgrade_started_at` fields Step 3 set — this closes the
mid-upgrade suppression window `agov.validate.learnings`'s
version-drift check honors while those fields are present (see
`agf/docs/site/docs/adopt/install-and-layouts.md`, Step 7, for the
full lifecycle and its 24-hour bound).

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
