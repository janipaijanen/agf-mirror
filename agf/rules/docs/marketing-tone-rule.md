---
provenance: inherited-v1x-unverified
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Marketing and Communication Tone
#
# Framework principle: Communication integrity boundary
# See: framework.md — Pattern 7 (Documentation Review Pipeline)

This rule applies to all content written or edited in `docs/**/*.md`, `README.md`, and any other markdown file intended for external readers (business users, administrators, or developers).

## Empathy and audience

Every reader has real work to do and real constraints. Acknowledge their situation before describing what the application offers.

- **Directors and managers** carry responsibility for plans that are hard to keep in sync. They've tried many approaches and know the trade-offs.
- **Employees and staff** follow the plans others set and want visibility into their own workload without chasing someone for an answer.
- **Business-IT and administrators** are responsible for infrastructure that must stay up, stay secure, and stay maintainable.
- **Developers** value honesty, specificity, and technical accuracy over marketing language.

Write for the reader's situation, not for the product's ego.

## Vendor and tool respect

Other tools -- whether enterprise suites, SaaS services, open-source alternatives, or productivity software -- serve real needs for real organisations. Never frame them as failures.

**Forbidden patterns:**
- Naming a vendor or product followed by a weakness, cost criticism, or limitation
- Implying that users of other tools have made a mistake
- Describing a competitor's product as the cause of an organisation's pain

**Allowed patterns:**
- Describing the *situation* or *outcome* the reader may recognise, without naming what caused it
- Stating what the application *does* and *offers*, not what others *lack*
- Acknowledging that the market has mature, established players and that this product occupies a specific focused niche

## Positive framing

State what the application *enables*, not what it eliminates or replaces.

| Instead of | Use |
|---|---|
| "No more spreadsheets" | "One source of truth for planning, tracking, and reporting" |
| "No Excel needed" | "Planning and coordination live in one place" |
| "Replaces your current tools" | "Adds a system of record alongside your existing workflows" |
| "Without the complexity of X" | "Focused on the core problem" |

Describe outcomes: what the director can see, what the manager can act on, what the developer can deploy and maintain.

## Coexistence with other tools

If the application imports or exports data (ODS, CSV, JSON, etc.), frame this as openness, not a concession:

> Your data stays open. Import initial data; export for analysis. Planning, tracking, and reporting stay in one place.

## Tone by audience

| Audience | Tone |
|---|---|
| Directors, managers | Warm, confident, outcomes-focused. Acknowledge the challenge; describe the benefit. No jargon. |
| Employees | Clear, reassuring. Focus on visibility and self-service. |
| Business-IT, admins | Practical, operational. Emphasise maturity: migrations, audit logs, backups built in. |
| Developers (README, plans) | Direct, factual, technical. No marketing language. State what the system does and how. |

## What to avoid

- Superlatives without evidence ("the best", "the only", "unlike anything else")
- Negative comparisons, even implicit ones ("unlike traditional tools", "without the bloat")
- Implying urgency or fear as a sales tactic
- Vendor names in proximity to words like "cost", "complexity", "bloat", "lock-in", or "failure"
- Framing the reader's current situation as a mistake they made

## Message integrity

Enterprise buyers read multiple documents. When the same claim appears twice in slightly different words, a cautious reader spots the inconsistency and raises doubt. The fix is to never allow the drift in the first place.

**Rule:** Every factual or value claim is stated once, in its canonical form. Other documents reference or link to that statement. They do not paraphrase it.

**Forbidden:**
- Stating the same capability in two documents with different numbers, scope, or qualifiers
- Summarising a section that already exists elsewhere rather than linking to it
- Softening or strengthening a claim in a summary vs. the detail page

**Allowed:**
- Linking to the authoritative location
- Quoting verbatim with attribution when a brief restatement is genuinely needed

**Canonical product descriptors:** Define a table of canonical phrases for your project's key concepts (database, auth, deployment, data model, etc.) and use exactly those phrases across all documentation. Never paraphrase them.
