# Exit and decommission

*Status: Released. Current framework version: see [Release information](../../reference/release-info.md). Audience: evaluators and
administrators. Read this page **before** adopting — knowing the way out
is part of deciding to come in.*

Adoption is reversible by design. The framework is files, rules, and
conventions; it has no runtime, no service registration, and no data it
holds hostage. This page summarises the three exit modes and is the canonical
public reference for leaving cleanly.

Figure 1 places the three exit paths — pause, downgrade, remove — as
branches off the same adoption timeline this page is about to walk
backward through, rather than as three unrelated procedures. Reading the
branch point first, before the table below defines each mode in detail,
shows that all three start from the same decision ("stop using some or
all of AGF") and differ only in how much of the governed history and
tooling stays behind afterward. The point to retain: exit is a branch on
the same timeline as adoption, evaluated with the same governed-decision
discipline — not an emergency procedure reached only after something has
gone wrong.

![Adoption timeline branching into freeze, replacement, and decommission exit paths](../../reference/figures/adoption-exit-flow.svg)

*AI-generated figure. Publication remains the responsibility of the human maintainer.*

## Three exit modes

Table 1 compares the three exit modes. Pause is operationally cheap,
downgrade keeps only selected rule guidance, and remove deletes the framework
artifacts completely.

| Mode | What it means | Effort |
|---|---|---|
| **Pause** | Stop enforcing gates; keep files in place | Disable CI jobs and hooks; one commit |
| **Downgrade** | Drop from full governance to rules-only (keep `agf/rules/`, drop gates and coordination) | Keep the selected `agf/rules/` files and remove gate obligations |
| **Remove** | Delete every framework artifact | One atomic removal commit |

Removal effort is one atomic cleanup commit. Its duration has not yet been
measured; it varies with your CI, hooks, and how much project-owned evidence
(`docs/plans/`, `docs/coordination/`, learning records) you choose to keep or
prune first. No numeric exit-time claim is made until a real decommission is
measured from a defined start to a verified clean end state.

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
2. Remove any small assistant loader file that only pointed at `agf/rules/`.
3. Decide whether to keep or remove project-owned evidence in `docs/plans/`
   and `docs/coordination/`. Keeping it in git history is normal; no history
   rewrite is part of exit.
4. Remove hooks (pre-commit entries, assistant settings, `core.hooksPath`).
5. Revert CI jobs the framework added.
6. Commit as a single atomic removal.
7. Verify: grep the repo for `agov`, `agents-govern`, `agf/rules`, and
   assistant settings. Any remaining match should be intentionally kept and
   documented.

## Why this page exists

A governance framework that is hard to leave creates pressure to never
evaluate it honestly. Documenting the exit path next to the entry path is
deliberate: you can adopt in stages, measure whether the gates earn
their overhead in your project, and reverse the decision cheaply if they
do not. The [downstream impact](downstream-impact.md) page itemises what
you would be reversing.
