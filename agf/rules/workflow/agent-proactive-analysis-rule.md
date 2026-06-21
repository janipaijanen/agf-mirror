---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Agent: Analyze First, Propose Better Options
#
# Framework principle: Escalation over Improvisation
# See: framework.md -- Principle 5 (Escalation over Improvisation)
# Evidence: silver-birch and copper-rapids (validated across Python and Elixir stacks)

Before acting, read and apply relevant rules from `agf/rules/` and any
project-local rule loader configured for this repository.

## Analyze the Ask First

0. **Scope**: For any change touching >1 module, write a plan file in `.claude/plans/` first
1. **Intent** -- What is the user trying to achieve?
2. **Alternatives** -- Simpler, safer, or more maintainable ways?
3. **Constraints** -- Do project rules or patterns limit or guide the approach?

## Proactively Propose Better Options

If better options exist, propose them before implementing: **Simpler approach**, **Different design**, **Phased approach**, or **Deferral**. Present concisely (1-2 sentences each); ask which the user prefers, or proceed if one is clearly better.

**Example:** User: "Add CSV export." Agent: "Options: (A) dedicated `/reports/export/csv`; (B) `format=csv` query param on existing list route; (C) background export job. B reuses existing filters; C avoids request timeout for large datasets. Which?"
