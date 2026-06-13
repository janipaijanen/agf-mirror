# Exit and decommission

*Status: Provisional (v2.0 documentation). Audience: evaluators and
administrators. Read this page **before** adopting — knowing the way out
is part of deciding to come in.*

Adoption is reversible by design. The framework is files, rules, and
conventions; it has no runtime, no service registration, and no data it
holds hostage. This page summarises the three exit modes and is the canonical
public reference for leaving cleanly.

Figure 1 shows the exit paths before the table defines them: pause, downgrade,
or remove. The important point is that exit is a normal governed decision, not
a failure mode.

![Adoption and exit flow](../../reference/figures/adoption-exit-flow.svg)

## Three exit modes

Table 1 compares the three exit modes. Pause is operationally cheap,
downgrade keeps only selected rule guidance, and remove deletes the framework
artifacts completely.

| Mode | What it means | Effort |
|---|---|---|
| **Pause** | Stop enforcing gates; keep files in place | Disable CI jobs and hooks; one commit |
| **Downgrade** | Drop from full governance to rules-only (keep the rule library, drop gates and coordination) | Remove agents/ and coordination obligations; keep `.claude/rules/` |
| **Remove** | Delete every framework artifact | One atomic removal commit |

## What you keep after full removal

- **Your git history** — the coordination channel and plan files remain in
  history as a record of decisions made while governed. No history rewrite
  is part of any exit mode.
- **Your own learning records** — `agf/learnings/<your-codename>.yaml` is
  your project's operational knowledge; export it before deleting `agf/` if
  you want to preserve a live copy outside git history.
- **Your code and tests** — nothing the framework touched needs undoing;
  governance shaped *how* changes landed, not *what* they contain.

## What removal involves

In outline:

1. Delete the framework-owned `agf/` directory.
2. Remove copied assistant rules from `.claude/rules/` or your tool's
   equivalent install path.
3. Remove hooks (pre-commit entries, assistant settings, `core.hooksPath`).
4. Revert CI jobs the framework added.
5. Commit as a single atomic removal.
6. Verify: grep the repo for `agov-` and framework references; an empty
   result is the done-condition.

## Why this page exists

A governance framework that is hard to leave creates pressure to never
evaluate it honestly. Documenting the exit path next to the entry path is
deliberate: you can adopt provisionally, measure whether the gates earn
their overhead in your project, and reverse the decision cheaply if they
do not. The [downstream impact](downstream-impact.md) page itemises what
you would be reversing.
