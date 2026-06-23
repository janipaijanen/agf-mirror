---
name: health
origin: upstream
description: >
  Periodic governance health check. Runs all validators (adoption,
  agents strict, contributions, infosec) and produces a summary.
  Designed for pre-commit, weekly CI, or on-demand use.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governance health check

Run all governance validators and produce a summary. Use this
periodically to catch drift before it accumulates.

## When to run

- Before releases (catches issues pre-tag-check would find)
- After upgrading the framework (verifies upgrade didn't break anything)
- Weekly or monthly (catches slow drift in agent topology, learning
  quality, and adoption conformance)
- After adding or modifying agents (verifies communication symmetry)

## Step 1 — Run the available aggregate checks

```bash
agov validate
```

Full `agov health` summary coverage is deferred. The available
aggregate validation surface runs:

1. **Adoption conformance** — mandatory agents present, rules exist,
   GOVERNANCE.md exists, tests exist, pre-commit configured
2. **Agent definitions** — structural validity and tier checks
3. **Contribution audit** — absence-bias detection, governance
   relaxation detection, evidence quality in learnings
4. **InfoSec screening** — credential leaks, PII, cross-field
   re-identification, cumulative leakage

## Step 2 — Review findings

The output ends with one of:

- **HEALTHY** — all checks passed, no action needed
- **HEALTHY with warnings** — review flagged items, decide if action needed
- **UNHEALTHY** — errors found, must fix before next release

For each finding, the relevant validator prints specific guidance
on what to fix.

## Step 3 — Fix and re-run

Fix any errors. Re-run `agov validate` to verify.

Warnings are advisory — review each one and decide whether to act.
Not every warning requires a change (e.g., ownership overlap between
academic reviewers is expected).

## Recommended cadence

<!-- CUSTOMIZE: adjust cadence to your project's release rhythm -->

| Project stage | Recommended cadence |
|---------------|-------------------|
| Active development | Every commit (pre-commit hook) |
| Maintenance | Weekly or before each release |
| Dormant | Monthly (catches stale references) |

To add to pre-commit:

```yaml
# .pre-commit-config.yaml
- id: agov-validate
  name: Governance health check
  entry: agov validate
  language: system
  pass_filenames: false
  always_run: true
```
