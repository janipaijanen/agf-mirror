---
name: infosec-screen
origin: upstream
description: >
  Run the InfoSec Sentinel screening pipeline: validate-infosec.sh,
  pre-commit hooks, SPDX header verification, and project-specific
  pattern checks.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# InfoSec screen

Run a comprehensive information-leak screening of the project's
governance artifacts. This wraps `validate-infosec.sh` with additional
checks that the script alone does not cover.

## Step 1 — Run the automated sentinel

```bash
uv run agov-validate-infosec
```

If the `--include-md` flag is relevant (project has narrative docs
like `docs/governance/retrospective.md` or `reviews/*.md`), use:

```bash
uv run agov-validate-infosec --include-md
```

Record the output: number of blocks, warnings, and acknowledged findings.

## Step 2 — Run pre-commit hooks (if configured)

If `.pre-commit-config.yaml` exists:

```bash
pre-commit run --all-files
```

Record any failures. Pre-commit hooks may catch issues the sentinel
does not (e.g., trailing whitespace in YAML that breaks parsing,
file-size limits).

## Step 3 — SPDX header verification

Check that every source file carries an SPDX license header.

**Required pattern** (first or second line of file, unless the file
uses YAML frontmatter):

<!-- CUSTOMIZE: adjust the license identifier if your project uses a different license -->

```
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
```

or for scripts/YAML:

```
# SPDX-License-Identifier: CC-BY-SA-4.0
```

**Exception: files with YAML frontmatter** (e.g., `SKILL.md`). The
`---` fence must be on line 1 for parsers to read the frontmatter.
Place SPDX headers immediately after the closing `---` fence:

```
---
name: my-skill
description: ...
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
```

**Files to check:**
- `*.md` — markdown files
- `*.yaml` / `*.yml` — YAML files
- `*.sh` — shell scripts

<!-- CUSTOMIZE: add file extensions for your project's languages -->

Report any files missing the header.

## Step 4 — Project-specific pattern checks

<!-- CUSTOMIZE: add patterns specific to your project -->

Check learning records and governance docs for:

1. **Real names in fictional contexts** — if the project uses fictional
   scenarios (e.g., training exercises, demo data), verify that real
   city names, institution names, or person names have not leaked into
   fictional content.

2. **Codename consistency** — verify that the project's codename is used
   consistently and no real project/product/organisation names appear.

3. **Cross-project references** — check that no learning record
   references another project's codename in a way that creates linkage
   (the sentinel checks this, but manual review catches subtler cases
   like "the same team's other project").

## Step 5 — Summary

Report:

| Check | Status | Details |
|-------|--------|---------|
| validate-infosec.sh | pass/fail | N blocks, N warns, N acked |
| pre-commit hooks | pass/fail/skipped | failures listed |
| SPDX headers | pass/fail | missing files listed |
| Project patterns | pass/fail | findings listed |

If any check failed, list the specific actions needed to resolve.
