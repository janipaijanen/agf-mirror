---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Simplicity: The Primary Design Goal
#
# Framework principle: Proportional governance overhead
# See: framework.md — Pattern 6 (Fast Track), Principle 7 (Technology Neutrality)

Complexity is the primary source of bugs, maintenance cost, and unreadable code.
Every addition of complexity requires justification. Simplicity does not.

## The Simplicity Test

Before proposing any design, implementation, or abstraction, ask:
> "Is there a simpler way to achieve the same outcome?"

If yes: propose the simpler way. Explain why it is sufficient.
If no: explain why the complexity is necessary (not just useful).

## In code

**Prefer:**
- A function over a class, when the function is sufficient
- A direct DB query over a service layer, when there is no reuse or business logic
- A plain dict or dataclass over a class hierarchy
- An existing utility over a new one
- Solving the actual problem over solving a generalised version of it

**Refuse (unless explicitly justified):**
- Abstractions with only one implementation
- Base classes with only one subclass
- Configuration options that no one will ever change
- Caching before a performance problem is measured and confirmed
- Generic systems when a specific solution would take 10% of the effort
- "Future-proofing" for requirements that do not exist

## In architecture

Work within the existing architecture. Do not propose:
- Frontend frameworks unless the user asks
- Microservices or service decomposition for a monolith
- Message queues or event-driven architecture for problems solvable with a function call
- New infrastructure components without a concrete, measured need
- API gateways or versioning for an internal tool

The existing architecture is intentional.
Propose departures only when the existing architecture provably cannot solve the problem.

## In tests

A test that is hard to understand is a test that will be wrong.
- Keep fixtures minimal (only the data the test actually needs)
- Keep assertions specific (not "response contains something")
- Prefer one assert per test over one test per feature
- Delete a test before keeping a broken or misleading one

## The over-engineering warning signs

Stop and reconsider if you find yourself:
- Adding a parameter "in case we need it later"
- Creating a helper function used in exactly one place
- Writing more test infrastructure than test logic
- Introducing a pattern the rest of the codebase does not use
- Reaching for a design pattern by name before understanding the problem

## Calibrating workflow overhead to task size

Not every task needs the full multi-phase workflow from development-workflow-rule.

| Task type | Workflow required |
|-----------|------------------|
| Typo, comment, doc update | Edit directly, commit |
| Single-file bug fix with obvious cause | Fix -> test -> commit |
| New field on existing model | Plan note + migration + test + commit |
| New route or service | Full plan file + TDD + review checklist |
| Cross-cutting change (>3 modules) | Full workflow + worktree |
| Refactor of existing code | Characterisation tests first, then refactor |

When in doubt: choose the lighter process. Upgrade if the task grows.

**Fast-track waiver:** When using the lighter process (skipping the plan file), record the waiver in the commit message: what task, why low-risk. If the task turns out larger than expected, stop and write a plan -- do not continue without one. Not eligible: PII, security boundaries, architecture, data model changes.

**Fast-track commits must not include a version bump, CHANGELOG entry, or git tag.** These are work commits -- they accumulate on the main branch until the next release. Tagging a fast-track commit causes version creep.
