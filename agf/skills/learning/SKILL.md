---
name: record-learning
origin: upstream
description: >
  Create a new learning record entry in the project's learnings YAML.
  Generates a timestamp-plus-random learning ID when IDs are used, finds the
  insertion point, scaffolds all required fields, and validates the result.
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

## Step 3 — Generate an ID if this record uses IDs

If the target learning file uses `id:` fields, generate a timestamp ID with a
random suffix:

```text
YYYYMMDDThhmmssZ-<4-char>
```

Use UTC and a four-character lowercase alphanumeric suffix, for example
`20260621T150000Z-a3fx`. Do not create timestamp-only IDs for new records.

Do not allocate the next `L###` number. Historical `L###` records remain valid,
but new records must not use a shared sequence counter.

When updating an existing learning record, preserve its existing `id:`. Do not
rename historical `L###` or timestamp-only records for consistency. If the old
record needs to be superseded, add a new learning entry with a new suffixed
timestamp ID and link the old entry forward using the local supersession
convention.

If the target learning file does not use IDs, do not invent one unless the user
or local project guidance asks for IDs.

## Step 4 — Gather learning details

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

## Step 5 — Format and insert

Format the entry as YAML with correct indentation (2 spaces for list
items under `learnings:`, 4 spaces for nested fields). Use the exact
field order from `learnings/_template.yaml`.

For multi-line `description` or `recommendation` values, use YAML
folded block scalar (`>`).

Insert at the position identified in Step 2.

## Step 6 — Validate

1. Run `agov validate learnings`
   if the validation command exists in the project.
2. Run `agov validate infosec`
   to screen for information leaks.
3. If either validator reports issues, fix them before finishing.

## Step 7 — Report

Tell the user:
- The learning ID if one was used, otherwise the entry position
- Category and severity
- Whether validation passed
- If `scope: framework`: remind them to submit upstream via GitLab issue
  or MR (see docs/governance/learning-system.md)
