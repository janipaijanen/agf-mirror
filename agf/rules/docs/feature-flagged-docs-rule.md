---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Feature-Flagged Functionality: Documentation Requirements
#
# Framework principle: Documentation Review Pipeline
# See: framework.md -- Pattern 7 (Documentation Review Pipeline)
# Evidence: silver-birch and copper-rapids (validated across Elixir and Python stacks)

**Scope:** Every non-trivial feature flag that adds a user-visible workflow.

## What "non-trivial" means

A flag is non-trivial if it introduces any of the following:
- A setup sequence (admin must configure data before users see anything useful)
- Multiple entry points spread across the navigation
- A relationship between system-wide configuration and per-project behaviour
- A dedicated page or view that is hidden when the flag is off

Simple toggles (show/hide a field, change a label) do not require a dedicated page.

## Required for every non-trivial flag

### 1. A dedicated how-to page

Create `docs/howto/<feature-slug>.md` with this structure -- no section may be omitted:

```
# <Feature name>: setup and usage

## Overview
Two to four sentences: what the feature enables, who benefits, which flags are required.

## Prerequisites
What must be in place before setup begins.

## Step N: <Action>  (repeat for each setup step)
**What:** One sentence.
**Why:** The outcome for the user if this step is done (or skipped).
**How:** Numbered steps. Reference the exact navigation path.
**Verify:** One sentence confirming the step succeeded.

## Where to find results
One subsection per result view.

## Entry points summary
Table: | Task | Location |

## See also
Links to: feature flags index, related workflows.
```

### 2. A one-line entry in feature-flags.md

`docs/feature-flags.md` is the index. Add exactly one line for the flag:
```
See [Feature name](howto/feature-slug/) for step-by-step instructions.
```
Do not expand the inline description. All detail belongs in the dedicated page.

## What NOT to do

- Do not embed a full how-to inside `feature-flags.md` -- link to the dedicated page
- Do not describe the same entry point in two pages with different wording
- Do not omit Prerequisites -- readers arrive from different starting points
