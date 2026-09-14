---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Development Workflow: Think Before You Code
#
# Framework principle: Pre-implementation planning gate
# See: framework.md — Pattern 1 (Pipeline), ADOPTING.md Step 4

This rule applies to every non-trivial task.
**Non-trivial** = creates, modifies, or deletes more than one file,
or touches a service, route, model, migration, or test.

**Exact research-maintenance exception:** a change limited throughout every
commit and its final diff to the existing `docs/sources.md` and/or
`docs/radar.md` may skip this rule's Gate 1 plan only when it satisfies every
semantic, evidence, collision-scan, and disqualifier condition in
`simplicity-rule.md`'s sources/radar fast path. That rule is the canonical
owner. Generic documentation and every ambiguous or expanded change remain
subject to this workflow. Gate 2 and protected-branch MR review are never
skipped.

## Phase 1 -- Clarify First (Socratic)

Before writing any code, surface hidden assumptions.

**Always ask if not already answered in context:**
- What is the user trying to achieve? (intent, not just the literal request)
- Are there simpler alternatives?
- Does this change ripple? (model change -> migration? route change -> tests? service change -> audit log?)
- Does this conflict with an existing rule?

**For new features, additionally ask:**
- Which application state(s) or lifecycle phases apply?
- Audit log needed?
- Translation string needed? (if i18n is used)
- Does it affect tenant isolation or data scoping?
- Which roles should see this? Does it change visibility of an existing view for any role?

Ask at most 3 questions at once. Present options in 1-2 sentences each.
Do NOT proceed to Phase 2 until questions are answered or trivially obvious.

## Phase 2 -- Write a Plan File First

For any change touching >1 module, write a plan file FIRST.

The exact sources/radar exception above is not a module-count loophole; it is
the only multi-file exception owned by `simplicity-rule.md`.

**Location: `docs/plans/<project>-<short-description>.md`**
<!-- CUSTOMIZE: adapt the path prefix to your project's plan directory -->
- MUST be in a VCS-tracked directory so all agents, tools, and reviewers can read it.
- MUST NOT be placed in `.claude/plans/`, `.cursor/`, `.copilot/`, or any
  AI-tool config directory. Those paths are session-scoped and agent-specific;
  plan files are shared governance artifacts.

- Get timestamp from OS (never invent it)
- Include: intent, affected files list, migration needed (yes/no), test strategy, risks, rollback
- If the plan introduces, changes, or removes normative operating guidance in
  an adopter-facing Pages page or runbook, include the six-row projection
  screen from `documentation-rule-projection-rule.md` (normative owner,
  adopter explanation, runbook/situation map, AI landing/loaders,
  validation/drift evidence, distribution).

Do NOT begin implementation without a confirmed plan.

### Affected-paths collision blocking

<!-- Row 6 S4, issue #123: promotes agov plan collision-check from
     advisory-only (Row 6 S3) to MR-blocking for the plan-vs-plan case.
     This is the named rule tool-authority-boundary-rule.md requires
     before agov:governance:plan-collision-gate may block a merge
     request. -->

A plan MAY declare `affected_paths` (see `gate1-plan.schema.json`) as a
machine-checkable, normalised form of the affected-files list required
above. When it does, the `agov:governance:plan-collision-gate` CI job
compares that plan's `affected_paths` against every other live
(`pending-gate1`/`approved`) plan's `affected_paths`. **A plan whose
`affected_paths` overlaps another live plan's, with neither plan's
`depends_on` naming the other, blocks Gate 1** until the overlap is
resolved -- either coordinate and add `depends_on` (see the schema
field's own description for its exact acknowledgment semantics), or
edit one side's `affected_paths` so they no longer overlap.

This blocking check compares **plan-vs-plan only**. A collision against
a local branch's or another open MR's actual changed files is not
blocked by this check -- `agov plan collision-check`, run without
`--plans-only`, still reports those overlaps advisory-only, for a human
reviewer to consider.

## Phase 3 -- Micro-Task Breakdown

Decompose into tasks of 2-5 minutes each. Every task must state:
1. Exact file(s) to create or modify
2. What "done" looks like (a test passing, a route returning 200, a field present)
3. Which task it depends on

Present the list, get confirmation, then start.

## Phase 4 -- Implement in Task Order

Per task:
1. Write the test FIRST (see tdd-rule)
2. Write the minimum code to pass it
3. Verify the task is done before moving to the next

Never skip ahead. Never implement two tasks simultaneously unless using
parallel tool calls for genuinely independent work streams.

## Phase 5 -- Self-Review Gate Before Any Commit

Run the full systematic-code-review-rule checklist before proposing any commit.
No exceptions, no shortcuts.
