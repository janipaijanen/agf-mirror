# Should you adopt?

*Status: Released (v2.1 documentation). Audience: evaluators. After
this page you can make an informed adopt / wait / skip decision.*

## What you are deciding

agents-govern is a file-based governance framework for AI-assisted software
development. In plain terms: it is a set of files that tell your AI assistant
how to behave, a review process that a human approves before anything merges,
and a decision log that makes those approvals reconstructible later.

The trade is deliberate: you accept a little ceremony so the human governor —
the person who approves gates and owns project risk — can reconstruct what was
decided and stop unsafe work before it lands. Adopt when that audit trail is
worth more than the overhead.

## The practical test

Use AGF when the cost of an unreviewable AI-generated change is higher than
the cost of pausing for a plan, a review, and a recorded decision.

Do not use AGF because it sounds rigorous. Use it when you need the rigor:
long-lived code, real users, regulated or security-sensitive work, shared team
conventions, or a codebase where "it worked in my session" is not good enough.

## What AGF is / is not

AGF is a repository-local governance framework for AI-assisted software
development. It defines the files and decisions that make agentic work
auditable: roles, gates, plans, coordination records, schemas, validators, and
learning records. No new infrastructure: the rules live in your repository,
your existing AI assistant loads them as instructions, and your existing CI
runs the validators.

AGF is not an agent runner, hosted workflow service, model-safety filter,
security scanner, legal/compliance system, or substitute for human gate
ownership. It works beside your existing assistant, CI, review, and release
tools; it does not replace them.

"No new infrastructure" means no hosted service, daemon, or orchestration
server. It does **not** mean zero human effort. The governor role is real:
someone must read gate evidence, accept or reject risk, and stop work when the
trail is not good enough.

## Adopt now if…

- AI assistants write a significant share of your code, **and** you have
  (or are willing to be) a human governor who approves plans and merges.
- You are a solo maintainer who wants a repeatable review discipline around
  AI-generated work before you approve it yourself.
- You are a small team that needs the same conventions to hold across tools,
  sessions, and contributors.
- Your project will live long enough for an audit trail to matter —
  months, not days.
- You already have quality conventions you want assistants to follow
  consistently, and prompting alone has not held.
- You need reconstructible evidence for internal audit, security review, or
  compliance work. AGF supplies the trail; it does not make the project
  compliant by itself.
- You accept review overhead per change as a deliberate trade for
  reconstructible decisions.

## Wait if…

- You need multi-project or multi-governor federation — that is a
  [hypothesis](../../future/evolution.md), not a shipped capability.
- You need every rule and extension point to be backed by multi-project
  evidence today. Some evidence-limited rules are marked in their rule text.
- Your team has no capacity for the governor role right now. Gates without
  a governor become ceremony; adopt when someone can own the role.

## Skip if…

- No AI assistants in your loop — conventional review and CI serve a
  human-only team better.
- The codebase is exploratory or disposable.
- You expect the AI to self-regulate without a human governor. AGF makes
  human accountability explicit; it does not remove it.
- You expect a hosted service or a runtime. This framework is files,
  rules, and conventions that your existing tools load and enforce.

## What adoption asks of you

Table 1 summarizes the commitments you accept when you adopt the framework.

| Commitment | Size |
|---|---|
| Initial install | Assistant-prepared file setup, human-reviewed rule choices — an afternoon |
| Per-change overhead | Plan + two gates for non-trivial changes; fast track for small ones when no agent holds an active claim (an exclusive reservation in the coordination channel) |
| Governor attention | Often 5-15 minutes for a non-trivial first-trial change; more for high-risk work |
| Ongoing | Learning records when incidents or reusable patterns happen; upgrade hygiene per release |
| Reversibility | Full removal is documented and verifiable — see [exit](../adopt/exit-and-decommission.md) |

The complete itemisation is on the [downstream impact](../adopt/downstream-impact.md)
page. If you read only one more page before deciding, read that one.

If you record assistants working under human, team, or service delegation,
AGF makes that delegation more visible. That visibility is useful for audit
and incident reconstruction, but it also creates privacy and labor risk:
named identifiers, timestamps, git authorship, review comments, and merge
events can be correlated by anyone with repository access. AGF does not ship
native productivity reports, principal ranking, review-depth scores, or
HR-exportable datasets. Treat delegation metadata as governance evidence, use
identity minimization where possible, and run your own privacy, labor, and
purpose-limitation review. These mitigations are not a compliance checklist.

## Next

Decided to try it → [Install the `agf/` directory](../adopt/install-and-layouts.md).
