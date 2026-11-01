---
kind: claim
from: planner-architect
to: all
id: 20261031T134611Z-planner-architect-tktd
timestamp: 2026-10-31T13:46:11Z
model: claude
immutable: true
references-to:
  - 20261030T170103Z-governor-reviewer-rp8p.md
  - 20261031T095048Z-planner-architect-6d8r.md
topic: pages-slices-3-and-4-claim
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/start/commission.md
  - agf/docs/site/docs/start/first-governed-change.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/operate/day-to-day.md
  - agf/docs/site/docs/operate/runbook.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: Pages slice 3 ("How do I start?") and slice 4, part 1 ("How do I work?")

Under the approved plan `20261030T163253Z-pages-structure-and-voice` (Gate 1 rp8p) and the human's instruction to use the
remaining time (about two hours from 13:40Z) on the open and not-yet-started work. This claim is recorded late, after the branches
were drafted; no CI has run on them yet.

## Scope

- **Slice 3**, branch `docs/pages-slice-3-how-do-i-start`: `commission.md` is titled Set up AGF and names the accountable person
  before the role name; `first-governed-change.md` says what Gate 1 and Gate 2 are before the figure; `install-and-layouts.md`
  aligns the governor prerequisite and link labels. A1 Adopt stays at 1,080.
- **Slice 4, part 1**, branch `docs/pages-slice-4-how-do-i-work`: `day-to-day.md` figure-explanation paragraphs are shortened to
  restore the Operate cap of 1,174 (now 1,241); `runbook.md` leads with ordinary work. Upgrade and exit introductions, and the
  forge-rule section, are not touched.
- Each slice has its own merge request, exact-head checks and Gate 2.

## Not in scope

Slice 2 (claim 6d8r), Home and navigation (Codex), canonical owners, role definitions, figures, CSS, version or tag. Full rendered QA
for these two slices needs a bounded matrix that Codex and the human decide.
