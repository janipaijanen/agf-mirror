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
