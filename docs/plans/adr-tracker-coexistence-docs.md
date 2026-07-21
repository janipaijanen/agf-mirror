---
id: adr-tracker-coexistence-docs
status: approved
topic: public-docs
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-21T18:13:37Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Docs-only concept page. No security boundaries, authentication,
      authorization, secrets handling, or schema changes.
  - role: contribution-auditor
    status: consulted
    reason: >
      Critical pre-Gate-1 review is recorded in
      20260721T173317Z-contribution-auditor-qd8r; its content boundaries are
      incorporated into this plan.
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: document AGF/ADR/tracker coexistence

**Date:** 2026-07-21

## Trigger

The governor asked how AGF's iterative-incremental, coordination-channel-based
workflow fits alongside enterprise roadmap/ticketing tools (GitLab Boards,
GitHub Projects, Jira, Asana, Trello). The answer was worked out
conversationally and the governor asked for it to be written up as a concept
page, explicitly requesting Asana and Trello be named alongside the more
commonly discussed tools.

This is documentation-only, additive work. It does not touch schema, tooling,
or governance rules.

## What this page claims (and does not)

The page documents a **coexistence pattern**, not an integration feature.
It must not claim AGF "integrates with" or "supports" any named tracker —
no schema field, validator, or webhook ships with this change. Per
`implementation-honesty-rule`, the pattern described is Level 1 (Planned/
conceptual) unless and until a real adopter or the foundry itself actually
runs it, at which point the page should be revised to cite that evidence.

## Canonical framing (three axes, not two)

`adopter-faq.md`'s existing "How do ADRs fit with AGF?" section already
states the canonical two-way split: "ADRs explain why the architecture is
the way it is; coordination records explain how a specific governed change
moved from proposal to merge." This page adds the third axis without
restating the first two — it links to that section rather than paraphrasing
it, per the message-integrity requirement in `marketing-tone-rule`.

- **Why** (ADR) — existing, linked, not restated.
- **How** (coordination channel + Gate 1/Gate 2) — existing, linked, not
  restated.
- **When / in what order** (roadmap or ticketing tool) — new content this
  page adds.

The tracker axis is conditional. A Gate 1 plan does not need a ticket unless
the adopter's own prioritization process requires one. Absence of a ticket is
therefore not evidence that an AGF plan is ungoverned.

An active coordination claim records ownership and prevents conflicting work;
it is not itself a work-in-progress limit or a capacity policy. The page may
explain how claims and tracker state can be kept consistent, but must preserve
that distinction.

Any automation from git or merge-request events into a tracker must be labelled
as an optional adopter convention. This change ships no tracker integration,
state synchronizer, or reliability guarantee.

## Affected files

- `agf/docs/site/docs/concepts/roadmap-and-tracker-coexistence.md` (new)
- `agf/docs/build/mkdocs.yml` (nav entry, Concepts section)
- `agf/docs/site/docs/start/adopter-faq.md` (one cross-link from the existing
  ADR section, no content change otherwise)
- `agf/docs/site/docs/concepts/position.md` ("Next" list gains one link)

## Vendor-neutrality check

Every named tool (GitLab Boards, GitHub Projects, Jira, Asana, Trello) is
described only by what it does, never in proximity to a criticism, cost
complaint, or implied deficiency, per `marketing-tone-rule`. No tool is
named as a cause of any team's pain; only situations/outcomes are described.

## Verification

1. `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml`
2. `agov validate adopter-docs`
3. `git diff --check`
4. Manual re-read against `marketing-tone-rule`'s forbidden-pattern list
   (vendor name near "cost"/"complexity"/"bloat"/"lock-in"/"failure";
   negative comparisons; unearned superlatives).
5. Confirm the new page is reachable from `mkdocs.yml` nav (no orphan page).
6. Confirm the page does not require a ticket, equate claims with WIP limits,
   or imply that tracker synchronization ships with AGF.

## Risks and controls

| Risk | Control |
|---|---|
| Reads as "AGF has a Jira integration" when it does not | Explicit "what this page is not" statement near the top; no schema/code changes bundled in |
| Duplicates the ADR-vs-coordination claim with different wording | Link to `adopter-faq.md`, do not restate |
| Implies AGF requires or prefers one tracker over another | List all five tools with parallel, equal treatment; no ranking |
| Conflates collision prevention with capacity control | State that claims record ownership; WIP limits remain a separate team policy |
| Makes optional tracker automation sound implemented | Label synchronization as an adopter convention and state that no integration ships |

## Rollback

Revert the new page, the nav entry, and the two cross-links. No other
surface depends on this page.
