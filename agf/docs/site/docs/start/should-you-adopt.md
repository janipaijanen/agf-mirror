# Should you adopt?

*Status: Provisional (v2.0 documentation). Audience: evaluators. After
this page you can make an informed adopt / wait / skip decision.*

## What you are deciding

agents-govern is a file-based governance framework for AI-assisted software
development. It gives AI coding agents explicit role definitions, Gate 1
(plan approval before implementation), Gate 2 (review before merge), a
coordination channel (the append-only decision log), and learning records
(structured incident and pattern notes).

The trade is deliberate: you accept a little ceremony so the human governor,
the person who approves gates and owns project risk, can reconstruct what was
decided and stop unsafe work before it lands. Adopt when that audit trail is
worth more than the overhead.

## What AGF is / is not

AGF is a repository-local governance framework for AI-assisted software
development. It defines the files and decisions that make agentic work
auditable: roles, gates, plans, coordination records, schemas, validators, and
learning records.

AGF is not an agent runner, hosted workflow service, model-safety filter,
security scanner, legal/compliance system, or substitute for human gate
ownership. It works beside your existing assistant, CI, review, and release
tools; it does not replace them.

## Adopt now if…

- AI assistants write a significant share of your code, **and** you have
  (or are willing to be) a human governor who approves plans and merges.
- Your project will live long enough for an audit trail to matter —
  months, not days.
- You already have quality conventions you want assistants to follow
  consistently, and prompting alone has not held.
- You accept review overhead per change as a deliberate trade for
  reconstructible decisions.

## Wait if…

- You need multi-project or multi-governor federation — that is a
  [hypothesis](../../future/evolution.md), not a shipped capability.
- You want v2.0's structure specifically: v2.0 is Provisional; v1.x is
  canonical. Waiting for v2.0 promotion is reasonable.
- Your team has no capacity for the governor role right now. Gates without
  a governor are theatre; adopt when someone can own the role.

## Skip if…

- No AI assistants in your loop — conventional review and CI serve a
  human-only team better.
- The codebase is exploratory or disposable.
- You expect a hosted service or a runtime. This framework is files,
  rules, and conventions that your existing tools load and enforce.

## What adoption asks of you

Table 1 summarizes the commitments you accept when you adopt the framework.

| Commitment | Size |
|---|---|
| Initial install | Copy files, customize rules — an afternoon |
| Per-change overhead | Plan + two gates for non-trivial changes; fast track for small ones |
| Ongoing | Learning records when incidents happen; upgrade hygiene per release |
| Reversibility | Full removal is documented and verifiable — see [exit](../adopt/exit-and-decommission.md) |

The complete itemisation is on the [downstream impact](../adopt/downstream-impact.md)
page. If you read only one more page before deciding, read that one.

## Next

Decided to try it → [Install the `agf/` directory](../adopt/install-and-layouts.md).
