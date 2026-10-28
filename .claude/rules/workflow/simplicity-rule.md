<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Simplicity: The Primary Design Goal
#
# Framework principle: Proportional governance overhead
# See: framework.md — Pattern 6 (Fast Track), Principle 7 (Technology Neutrality)

Canonical owner: `agf/rules/workflow/simplicity-rule.md`. This foundry-local
loader projection mirrors that rule; the canonical file resolves conflicts.

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

## Governance mode: LIGHT / NORMAL / STRICT

Not every task needs the same process weight. "Fix two paragraphs" and "add
a new route" do not warrant the same ceremony, and neither of those warrants
the ceremony a change to gate mechanics or a genuinely high-risk area needs.
Every task is classified into exactly one of three modes before work starts.

The classification is a heuristic self-check the acting session runs, not a
mandatory human pre-declaration -- but any human or coordination record may
explicitly declare a mode, and an explicit declaration overrides the
heuristic result when they disagree (a human declaring STRICT on a change
the heuristic would call LIGHT always wins; the reverse requires the
disqualifier list below to genuinely not apply).

**Step 1 -- check STRICT disqualifiers first (if any apply, mode is STRICT,
full stop):**
- Touches any [`always/high-risk-operations-rule.md`](../always/high-risk-operations-rule.md)
  named area.
- Touches any [`always/downstream-impact-rule.md`](../always/downstream-impact-rule.md)
  trigger path **and** is classified BREAKING under that rule's Parnas test.
- Is gate mechanics per [`always/gate-self-test-rule.md`](../always/gate-self-test-rule.md)
  (introduces or modifies a git hook, validator, CI check, pre-commit
  script, or MR-blocking rule) -- always STRICT regardless of diff size.
  At diff level a weakening and a fix look identical, so "it's one line" is
  not evidence that a gate change is low-risk -- it is exactly the shape a
  weakening takes. A revert of a gate change is still a gate change.
- PII, security boundary, architecture, or data-model change.
- Public AGF product/marketing/competitive/performance/maturity/compliance
  claim, or a governed-decision rewrite.

**Step 2 -- if none of the above apply, check LIGHT eligibility:**
cosmetic/local-impact task -- typo, comment, ordinary doc update, a
single-file bug fix with an obvious cause, a new field on an existing model,
or a narrow reversible research-maintenance edit to `docs/sources.md`/
`docs/radar.md` meeting the eligibility list in the canonical rule's research-maintenance section. If
eligible: skip the separate Gate 1 plan file; record the fast-track waiver
in the commit message (what task, why low-risk). If the task turns out
larger than expected, or grows mid-flight into a STRICT or NORMAL shape,
stop and write a plan -- do not continue on the lighter path past the point
it fits.

**Fast-track commits must not include a version bump, CHANGELOG entry, or
git tag.** These are work commits -- they accumulate on the main branch
until the next release. Tagging a fast-track commit causes version creep.

**Step 3 -- otherwise, NORMAL:** full plan file + TDD + review checklist for
a new route/service; full workflow + worktree for a cross-cutting change
touching more than three modules; characterisation tests first for a
refactor of existing code. Full Gate 1 plan -> governor-reviewer approval ->
implementation -> Gate 2 review -> merge, exactly as
[`development-workflow-rule.md`](development-workflow-rule.md) and
[`systematic-code-review-rule.md`](systematic-code-review-rule.md) already
require.

When in doubt: choose the lighter mode that still satisfies the applicable
branch and review rules, and upgrade the moment the task grows past it.

*Note for adopters extending this classification:* a category naming
contractual, compliance, legal, or externally-binding deliverables as an
additional STRICT trigger was proposed and deliberately deferred (governor
decision `20260920T115342Z-governor-reviewer-3es7`) pending a fast-follow
plan that first customizes `always/high-risk-operations-rule.md` for the
adopting project's own domain, so that a new substantive obligation doesn't
ride in alongside this rename-and-consolidate pass. Projects with real
contractual/compliance deliverables should treat that as open follow-up
work, not as already covered here.

## Sources/radar research-maintenance fast path (a LIGHT-tier example)

The full eligibility list, disqualifiers, and the required branch, evidence and
review procedure for this exception are owned by the canonical rule
`agf/rules/workflow/simplicity-rule.md`. Read that section before using the
exception. Any uncertainty or disqualifier means full Gate 1; Gate 2 and the
protected-branch MR stay mandatory.
