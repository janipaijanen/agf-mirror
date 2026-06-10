---
name: record-learning
origin: upstream
description: >
  Create a new learning record entry in the project's learnings YAML.
  Finds the next insertion point, scaffolds all required fields, and
  validates the result.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Record a learning

Add a new learning entry to the project's learning record.

<!-- CUSTOMIZE: set the path to your project's learning record -->

## Step 1 — Locate the learning record

Find the project's `learnings/<codename>.yaml` file. If multiple YAML
files exist in `learnings/`, ask the user which one to use.

Read the file. Identify the `learnings:` section.

## Step 2 — Find the insertion point

New entries go at the **end** of the `learnings:` list, immediately
**before** the `sentinel_ack:` block (if present). If there is no
`sentinel_ack:` block, append after the last learning entry.

Count existing entries to confirm the position.

## Step 3 — Gather learning details

Ask the user for (or derive from conversation context):

| Field | Required | Notes |
|-------|----------|-------|
| `date` | yes | Today's date (YYYY-MM-DD) |
| `category` | yes | One of: `gap`, `overkill`, `validation`, `adaptation`, `tension` |
| `scope` | yes | `framework` (may inform upstream changes) or `project` (local only) |
| `title` | yes | One-line summary |
| `description` | yes | What happened, in enough detail to be useful |
| `agents_involved` | yes | Which agents were relevant (list) |
| `framework_section` | no | Which framework.md section is relevant |
| `business_impact.type` | yes | `prevented_loss`, `enabled_revenue`, `cost_overhead`, or `neutral` |
| `business_impact.description` | yes | Qualitative impact description |
| `recommendation` | no | What should change in the framework, if anything |
| `severity` | yes | `critical`, `significant`, `minor`, or `informational` |

## Step 4 — Format and insert

Format the entry as YAML with correct indentation (2 spaces for list
items under `learnings:`, 4 spaces for nested fields). Use the exact
field order from `learnings/_template.yaml`.

For multi-line `description` or `recommendation` values, use YAML
folded block scalar (`>`).

Insert at the position identified in Step 2.

## Step 5 — Validate

1. Run `uv run agov-validate-learnings learnings/<codename>.yaml`
   if the validation script exists.
2. Run `uv run agov-validate-infosec learnings/<codename>.yaml`
   to screen for information leaks.
3. If either validator reports issues, fix them before finishing.

## Step 6 — Report

Tell the user:
- The learning ID (position in the list, e.g., "entry #31")
- Category and severity
- Whether validation passed
- If `scope: framework`: remind them to submit upstream via GitLab issue
  or MR (see LEARNINGS.md)
