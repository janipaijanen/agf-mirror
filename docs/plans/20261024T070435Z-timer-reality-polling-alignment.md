---
id: 20261024T070435Z-timer-reality-polling-alignment
status: pending-gate1
topic: timer-reality-polling-alignment
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: Review requested. Edits normative polling and scheduling wording; no credentials, schemas or runtime code. Checks that the continuation-deadline override cannot be read as extra authority.
  - role: contribution-auditor
    status: pending
    reason: Review requested before Gate 1 (normative rule edit, adopter-facing skill and docs).
affected_paths:
  - agf/rules/always/universal-polling-and-autonomous-continuation-rule.md
  - agf/skills/poll-and-coordinate/SKILL.md
  - agf/rules/workflow/branch-and-worktree-rule.md
  - agf/docs/site/docs/operate/poll-and-coordinate.md
  - agf/docs/site/docs/adopt/assistant-loader-configuration.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/templates/AGENTS.md
  - AGENTS.md
  - docs/reference/foundry-polling-and-bridge-procedure.md
  - docs/reference/agent-cold-start-habits.md
  - docs/reference/operator-timer-evidence.md
  - docs/reviews/rule-projection/20261024T070435Z-timer-reality-polling-alignment.yaml
depends_on:
  - 20260905T210255Z-universal-agent-autonomous-polling
  - 20260823T093120Z-poll-and-coordinate-skill
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260921T040743Z-assistant-rule-drift-resolution
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T045214Z-remove-elixir-references
  - 20261023T080431Z-instruction-context-followup
  - 20261023T155559Z-agov-strict-args-ref-list
  - 20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: align polling and timer wording with how wake mechanisms actually behave

Source: Codex governor review (ACS seq 342) of my timer-reality inventory (ACS seq 338-339, read-only
evidence on `main` at `c1999c8e`). Governor stance in seq 286: the documentation-estate-steward drafts
the inventory and proposal; a Gate 1 plan is needed because normative rule text changes. Draft by
planner-architect. Nothing here is approved, and no rule, skill, loader or document is edited before Gate 1.
Separation of duties: Gate 1 is decided by a governor-reviewer record, not by me.

## Problem

Seven documents restate one convention (`15mins/180min`, "internal attention schedule, no external timers")
in their own words. They say nothing about session boundaries, what survives a session end, how a
supervised background process that notifies the assistant is classified, or what an operator may do to keep
a poll running past the idle rule. Two statements are wrong or inconsistent today:

- `agf/skills/poll-and-coordinate/SKILL.md` forbids "a background shell timer, `sleep`, cron job, or
  detached process" without exception, although a harness-supervised monitor does notify the assistant; and
  its default `idle_window_minutes` is 240 while every other file says 180.
- No page states that a poll running in a session stops when the session ends, or that durable unattended
  wake needs an external scheduler with evidence.

The canonical owner is `agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`, section
"Scheduling and notification honesty". The other copies must point to it.

## Scope and changes

1. **Owner rule, capability-neutral.** In the owner section: a mechanism counts as a real wake or
   notification mechanism when it can notify the assistant and its lifetime is observable; state that any such
   mechanism is bounded by the session and by its own declared expiry; require the agent to report each
   replacement (identifier, start, expiry, delivery result) and to re-arm before expiry while the poll is meant
   to continue. No client name and no numeric client limit appears in the rule.
2. **Durability.** Durable unattended wake requires an external scheduler with Level 5 evidence (see
   `docs/reference/maturity-vocabulary` usage in AGENTS.md); anything else is session-bound.
3. **Quiet tiers.** A schedule that polls less often when quiet reduces load and adds no durability.
4. **Operator-set continuation deadline.** A human or governor may set an absolute UTC time before which the
   idle stop does not apply. It is a bounded override of the idle stop only: it grants no authority, changes no
   gate, and ends at its stated time or at the session end, whichever is first.
5. **Pointers, not copies.** Replace the duplicated normative explanations in the six non-owner files with a
   one-sentence statement and a link to the owner section, where the source wording is preserved by quoting
   the owner rather than paraphrasing it.
6. **Consistency fix.** `idle_window_minutes` default 180, matching every other file; the skill's blanket ban
   is replaced by the owner's capability-neutral test.
7. **Non-normative evidence surface.** New `docs/reference/operator-timer-evidence.md` records current
   client-specific observations (for example the approximately 30-minute cap and the re-arm duty seen with the
   Claude Code monitor on 2026-10-23/24), each with date, client version, evidence status (Empirically
   informed or Theoretical) and a note that it is not a rule and may change.

## Classification and impact

Mode: **STRICT** (normative rule edit under `agf/rules/`, adopter-facing skill and templates).

```
Impact:
  Foundry:          wording and pointers in 7 documents plus one new reference page; no tooling change.
  Direct adopters:  BREAKING under the Parnas test for the skill: its blanket prohibition of background timers
                    is replaced by a capability test, so assistants may now rely on a supervised notifying
                    monitor where they were told not to. ADDITIVE for the rule (new duties: declared expiry,
                    re-arm reporting, operator deadline).
  Downstream:       adopters' agent behavior toward polling can change; no artifact shape changes.
  Migration:        adopters who copied the old wording keep working; the migration note says: replace any copy
                    of the "never a background process" sentence with the pointer, and set idle_window_minutes
                    to 180 if they relied on the 240 default.
  Evidence:         inventory in ACS seq 338-339; the 240/180 mismatch is verifiable by grep on main c1999c8e.
```
Adopter docs parity: updated (the same plan edits the adopter pages and template).

## Rule-projection screen (`documentation-rule-projection-rule.md`)

Six-row manifest at `docs/reviews/rule-projection/20261024T070435Z-timer-reality-polling-alignment.yaml`: owner = the universal polling rule
(updated); adopter explanation = `operate/poll-and-coordinate.md` (pointer); runbook = the same page plus
`assistant-loader-configuration.md` (pointer); AI loaders = `AGENTS.md`, `agf/templates/AGENTS.md`
(pointer); validation = `agov plan projection-check` and a grep check that no other file restates the
"no background timer" sentence; distribution = all `agf/` files ship, `docs/reference/*` and root `AGENTS.md`
are foundry-local.

## Tests and acceptance

- `agov plan validate` and `agov plan projection-check` (strict, no `--allow-planned`) pass at the exact head.
- Grep shows exactly one normative statement of the scheduling test (the owner) and pointers elsewhere.
- No remaining mention of a numeric client cap in any file under `agf/`.
- `idle_window_minutes` default reads 180 everywhere it appears.
- Collision check (changed-only) reports zero unacknowledged plan overlaps.
- Negative check: a grep for the removed blanket prohibition sentence finds nothing under `agf/`.
- Gate 2 evidence includes the diff stat and the migration note in the MR description.

## Collision and sequencing disposition

A full plans-only collision check (not changed-only) reports ten overlaps with this plan. Each overlapped plan
is already `approved` and its change is merged on `main`; none is a competing live proposal. This plan builds on
the merged text, so each is declared in `depends_on`. The reason per overlap:

| Plan | Overlapping path | Why this plan depends on it |
|---|---|---|
| `work-item-steward-gate1`, `row5-s3-priority-ordering-and-challenge-response`, `row6-s4-issue123-collision-check-blocking-promotion`, `issue130-pending-advisory-consultation-status` | `AGENTS.md`, `agf/templates/AGENTS.md` | each earlier added or changed loader wording; this plan changes only the polling-timer sentence and leaves their text intact |
| `instruction-context-reduction`, `instruction-context-followup`, `remove-elixir-references`, `recall-smoke-criterion-and-weak-paraphrases` | `AGENTS.md`, `docs/reference/agent-cold-start-habits.md`, `docs/reference/foundry-polling-and-bridge-procedure.md` | they moved or trimmed loader text and trigger lines; this plan edits the moved timer text where it now lives and does not touch trigger lines |
| `assistant-rule-drift-resolution` | `agf/rules/**` | it set the rule-drift handling; this plan edits two named rule files only and follows its handling |
| `agov-strict-args-ref-list` | `agf/docs/site/docs/concepts/coordination-protocol.md` | it added a `coord list --ref` paragraph; this plan edits the timer paragraph of the same page and keeps the `--ref` paragraph |

Implementation re-runs the changed-only collision check against the then-current `main` and re-reads the merged
text of each file before editing it. A new overlap with a live (not merged) plan returns to Gate 1.

## Risks and rollback

- R1 wording drift while replacing copies: mitigated by quoting the owner and by the grep acceptance.
- R2 the continuation override read as authority: mitigated by the explicit "grants no authority" sentence and
  the infosec consultation.
- R3 client-specific facts leaking back into normative text: mitigated by acceptance criterion on numeric caps.
Rollback: revert the implementation MR. No tag, release or schema is involved.

## Explicitly out of scope

Tiered-cadence implementation or any ACS server change, changing the `15mins/180min` default, new tooling,
any change to the delegation or deputy-governor rules.
