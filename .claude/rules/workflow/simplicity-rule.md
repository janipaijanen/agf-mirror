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
`docs/radar.md` meeting that section's own eligibility list below. If
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

A reversible research-maintenance change may skip a separate Gate 1 plan,
contribution audit, and pre-change coordination MR only when **all** of these
conditions hold:

1. Every commit from the freshly resolved target merge base through `HEAD`,
   and the final aggregate diff, changes only the existing files
   `docs/sources.md` and/or `docs/radar.md`. Merge commits are ineligible.
   A temporarily added, reverted, imported, or cherry-picked change to any
   other path is ineligible even when absent from the final diff.
2. The change only adds or corrects a source classification, preserves an
   unverified stub, adds or refreshes a non-binding radar question, updates a
   reasoned `review-by` date, or follows an existing promote/keep/retire radar
   lifecycle.
3. The text is advisory and reversible: it does not adopt a tool or practice,
   promise implementation, assign work, select priority, or grant an external
   source, tool, status, or verdict AGF authority.
4. Material claims are traceable to named URLs or repository evidence. The MR
   identifies pinned revisions and access dates where relevant, distinguishes
   primary, secondary, self-reported, inferred, and unverified evidence, and
   states material limits.
5. A radar entry keeps its controversy or open question, non-binding status,
   source cross-reference where applicable, and `review-by` date.
6. Before writing, the implementer freshly fetches the target, inventories all
   open MRs targeting it and their changed paths, and checks existing entries
   for duplicate identities, batch/allocation conflicts, and edit collisions.
7. The implementer repeats that target and all-open-MR scan immediately before
   Gate 2. The MR records UTC scan time, target SHA, each open MR's IID, head
   SHA, `updated_at`, changed paths, and a disposition for every possible
   overlap. Any relevant state change invalidates the evidence until refreshed.
8. No disqualifier below applies. Uncertainty means full Gate 1.

The fast path is semantic, not merely a filename allowlist. It is automatically
disqualified by any normative rule or authority change; gate, schema,
validator, CI, merge, or release-policy change; implementation commitment;
adoption, integration, dependency, plugin, account, hosted service, data flow,
or runtime trial; roadmap, milestone, release, staffing, assignment, budget, or
priority selection; security, credential, privacy, personal-data, legal,
compliance, or regulatory material; public AGF product, marketing, competitive,
performance, maturity, effectiveness, or compliance claim; governed-decision
rewrite; any other changed path; or ambiguity about fidelity, reversibility,
scope, or ownership.

If the work grows into a disqualifier, stop before committing the expanded
scope and return to the ordinary Gate 1 route. Do not treat work already done as
retroactive approval.

### Required branch, evidence, and review

The fast path still requires a normal branch and MR to the protected target,
the commit-message waiver, source-fidelity evidence, targeted validation, a
green exact-head pipeline, and a binding governor-reviewer Gate 2 decision.
The MR is the evidence carrier and must state the exact paths and head SHA;
sources, revisions, access dates, evidence classes, and limits; the radar
disposition; the eligibility checklist and disqualifier result; the fresh
target/open-MR collision scan; and validation results.

After fetching the target, qualify the complete range fail-closed. Record the
target ref/SHA, merge-base SHA, exact range, every commit SHA, every path for
each commit, every final-tree path, and the procedure exit code. The procedure
must:

1. resolve the target commit and merge base or fail;
2. reject an empty range and every merge commit;
3. run `git diff-tree --no-commit-id --name-only -r` separately for each commit
   and reject every path except `docs/sources.md` and `docs/radar.md`;
4. run `git diff --name-only <merge-base>...HEAD` and reject every other final
   path; and
5. run `git diff --check <merge-base>...HEAD` and print `result=PASS` only when
   all earlier checks succeeded.

Do not rely on a final-tree diff alone: it cannot reveal an extra path added and
later reverted. Run the procedure under `set -euo pipefail` with explicit
non-zero failure exits so a later successful command cannot hide a finding.

Immediately before Gate 2, compare the fresh target and candidate against every
open MR touching either allowed file. Recheck canonical source identities,
batch allocation, radar titles/cross-references, edit regions, registry
vocabulary, and `review-by` values. A duplicate, collision, unclear allocation,
or unresolved owner makes the candidate ineligible. A changed candidate head
requires new validation, pipeline, and exact-head review.

The governor-reviewer may escalate any candidate to full Gate 1 and alone
decides Gate 2 and merge. No separate coordination record is required merely
to start or submit an eligible MR; use coordination when the work creates a
separate downstream obligation, escalation, accepted risk, or decision the MR
and merge do not fully carry.
