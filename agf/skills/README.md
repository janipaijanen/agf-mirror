<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Skills (Reference Implementation)

Reusable AI coding assistant workflows that automate common
agents-govern adoption tasks.

> **Two-layer design.** The agents-govern framework is
> technology-agnostic at the conceptual level (agent taxonomy,
> jurisdiction model, governance principles). These skills are the
> **reference implementation layer** — they can be exposed through Claude Code
> or another assistant, but the canonical workflow definitions stay under
> `agf/skills/`.

## Available skills

| Skill | What it automates |
|-------|-------------------|
| [`upgrade/`](upgrade/SKILL.md) | 6-step gated upgrade: extract → compare → apply → track → verify → summarise |
| [`learning/`](learning/SKILL.md) | Find next ID, gather fields, format YAML, insert at correct position, validate |
| [`infosec-screen/`](infosec-screen/SKILL.md) | Run validate-infosec.sh + pre-commit + SPDX check + project-specific patterns |

## Adoption

Ask your assistant to read the skills it needs from `agf/skills/`. If your
tool needs slash-command wrappers or another local command surface, create
small project-local wrappers that point to the canonical `agf/skills/*/SKILL.md`
files. Do not copy upstream skill definitions into a second source-of-truth
directory.

Search for `<!-- CUSTOMIZE -->` comments in each skill and adapt paths
and patterns to your project.

## The `origin:` convention

Each skill declares its provenance in YAML frontmatter:

```yaml
---
name: upgrade-agents-govern
origin: upstream        # from agents-govern distribution
description: ...
---
```

When creating project-specific skills, set `origin: project`:

```yaml
---
name: deploy-staging
origin: project         # created by adopter, project-specific
description: ...
---
```

**Why this matters.** The upgrade skill uses `origin:` to distinguish
skills it should compare against upstream (and potentially update) from
skills it must leave untouched. Without this field, project-specific
skills risk being overwritten during framework upgrades.

`origin:` is an agents-govern convention used by the upgrade skill — it
is not interpreted by Claude Code or any other tool.

### Promotion criteria

A skill is included in the distribution when it meets these conditions:

| Criterion | Why |
|-----------|-----|
| **Validated in at least one project** | Used for real work, not theoretical |
| **Generalizable** | Applies to any adopter, not domain-specific |
| **Anonymized** | No real project/product names |
| **Self-contained** | Works with `<!-- CUSTOMIZE -->` markers, no implicit dependencies |
| **Has `origin: upstream`** | Distinguishable from project-specific skills |

Skills validated by two or more independent projects are considered
stable. Skills validated by one project (like these initial three) are
marked as such and may evolve based on additional adopter feedback.

## Repo-only files

Not every file in the repository ships to adopters. Files that support
the framework's own development (e.g., internal-only skills, dev
scripts, repo-specific rules) can be excluded from the distribution
by adding their path to [`.distignore`](../.distignore) at the repo root.

`dist.sh` copies directories wholesale, then strips anything matching
`.distignore` patterns before creating the archive. This means you can
keep repo-only files alongside distributable ones without a separate
directory structure.

**Example:** to keep a skill repo-only, add it to `.distignore`:

```
skills/internal-only/
```

The same mechanism works for rules, scripts, or any other file in a
shipped directory.

## Evidence

These skills were generalized from iron-gate's adoption experience
(2026-04-02 to 2026-04-12):

- **upgrade**: built during v0.23.1 → v0.24.0 upgrade (33 files across
  4 categories)
- **learning**: built after manually scaffolding 30+ entries with
  identical boilerplate
- **infosec-screen**: built after validate-infosec.sh alone missed
  project-specific patterns
