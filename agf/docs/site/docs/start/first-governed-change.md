# Your first governed change

*Status: Provisional (v2.0 documentation). Audience: developers who have
completed [install and layouts](../adopt/install-and-layouts.md). After
this page you have taken one real change through both gates and seen every
moving part once.*

!!! warning "Inherited from agents-govern v1.x"
    Not yet verified under the AGF v2 layout and tooling. Treat as
    background context only until updated. Adopters may verify and
    report evidence upstream.

Pick a small, real change — a genuine bug fix or a modest feature, not a
toy. The walkthrough below is the full ceremony; at the end you will know
which parts the fast track lets you skip for small work.

Figure 1 shows the path you are about to walk: plan, Gate 1, implementation,
Gate 2, merge, and completion.

![Gate pipeline](../../reference/figures/gate-pipeline.svg)

## 1. Check the fast track first

Before any ceremony, ask: does this change qualify for fast track?
(≤3 files, no shared interface / data model / security boundary change,
no conflict with another agent's claim, no scope uncertainty.) If yes, the
plan and Gate 1 are waived — note the waiver in the commit message and
jump to step 4. For this first walkthrough, choose something *just* big
enough not to qualify, so you see Gate 1 once.

## 2. Write the plan

Create a plan file: intent, affected files, migration needed (yes/no),
test strategy, risks, rollback. Timestamp from the OS, never invented.
Keep it short — a plan that takes longer to write than the change takes to
implement is miscalibrated.

For example, use a concrete file such as
`docs/plans/fix-login-error-message.md`:

```markdown
# Plan: Fix login error message

Intent: make the failed-login message actionable without exposing account
existence.
Affected files: `lib/auth/login.ex`, `test/auth/login_test.exs`
Migration: no
Tests: failing login-message test, then full auth test file
Risks: wording may leak whether an account exists
Rollback: revert the branch
```

## 3. Gate 1 — plan approval

File a coordination entry handing the plan to the reviewer role
(`kind: handoff`, referencing the plan). The governor — or a reviewing
agent the governor has authorized — approves, amends, or rejects with a
`decision-record`. Nothing is implemented before this lands. The point is
not bureaucracy; it is that redirection is cheap before code exists and
expensive after.

The handoff is another concrete file, for example
`docs/coordination/00012-coder-tester.md`, whose front matter says
`kind: handoff`, `to: governor-reviewer`, and points to the plan in
`references-to`.

## 4. Branch, then implement test-first

```
git checkout -b fix/what-was-wrong
```

Never on main. Write the failing test, make it pass, refactor — the TDD
rule in your rules directory is binding on the assistant, which is the
reason this step happens even when the human is in a hurry.

## 5. Self-review

Run the systematic code-review checklist before proposing the commit:
spec compliance, domain rules, security, tests, quality, i18n/a11y,
documentation, commit hygiene. BLOCK items stop the commit; everything
else is judgment.

## 6. Gate 2 — review before merge

Push the branch, open a merge request, and file a coordination handoff to
the reviewer with the evidence: tests green, checklist run, diff scoped to
the plan. The reviewer files findings (`kind: review`); blocking findings
go back to step 4. Approval is a `decision-record` naming the branch and
any conditions.

## 7. Merge and close the loop

Merge via the MR (never a direct push to main), then file a
`completion` entry referencing the approval. The chain —
plan → approval → implementation → review → approval → merge → completion —
is now reconstructible from files alone. That chain is the product.

Name the completion file the same way as every other coordination entry, such
as `docs/coordination/00015-coder-tester.md`, and reference the Gate 2
approval so the chain closes visibly.

## What you just bought

Six months from now, "why was this merged?" is answered by reading five
small files in `docs/coordination/`, not by archaeology. Multiply by every
non-trivial change, and that is the audit trail the framework exists to
produce.

## Next

- Mental model behind the ceremony: [roles and gates](../concepts/roles-and-gates.md)
- Daily rhythm without the training wheels: [day-to-day operation](../operate/day-to-day.md)
