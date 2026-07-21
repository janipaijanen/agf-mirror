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

## What ships and what you activate

> AGF releases ship the complete governance library — every role, rule,
> skill, schema, and template. In the maintainer's own deployments, the full
> rule set has run unpruned from day one, without a manual curation step,
> and has worked well as shipped; curating the rule set down is optional,
> not a documented prerequisite. A reduced starter package does not
> currently ship.

The release includes the role, rule, skill, schema, template, and documentation
libraries so each project can make its own governed choices. Treat that
inventory as available framework content, not as a roster that must all be
active on day one — and also not as a roster you must prune before it is
safe to use. The maintainer's own projects have evidence only for the
unpruned path: the full rule set left active and running without incident.
Pruning down to a smaller subset is supported and configurable, but it is
not itself validated by any maintainer deployment to date.

For a solo trial, the documented minimum is `coder-tester` plus the human
`governor-reviewer`. A standard governed project adds `planner-architect`.
Activate advisory and specialist roles when the work creates the need they
address — this incremental, roles-added-as-needed pattern is what the
maintainer's own projects have followed. Core rules remain binding;
workflow, documentation, UI, and other task-specific rules apply when their
subject is present. If a rule clearly does not and will not apply to the
project, removing it during commissioning avoids teaching assistants that
loaded instructions are optional — but doing so is a choice, not a required
step; leaving the full rule set active has also worked without incident in
the maintainer's own deployments.

For a first, still-incomplete answer to *which specific rules and roles you
can safely remove and what each removal costs, if you choose to prune*, see
the [adopter applicability catalog](../adopt/applicability-catalog.md) — it
classifies a first slice of 8 governed artifacts (of 58 total) with an
activation condition and a removal consequence for each.

The current coordination schema still exposes the full message-kind vocabulary,
and the release provides the general validator and CI surfaces. A smaller
starter product — three roles, two gates, four message kinds, one validator,
and one CI example — is **Level 1 Planned**, not a current installation option.
It remains a hypothesis until independent pilots show which parts reduce
onboarding friction without removing the evidence needed for human gate
decisions. See
[Where v2 could evolve](../../future/evolution.md#reduced-first-adoption-product).

The [installation guide](../adopt/install-and-layouts.md#step-4-review-rule-loading-and-customization)
shows how to select applicable rules and roles, for adopters who choose to
prune. Evaluators should judge both the complete distributed surface and
whatever smaller configuration a given adopter actually runs; neither
measurement substitutes for the other, and the smaller configuration is not
guaranteed to exist.

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
- You need org-wide, centrally enforced policy across many repositories
  before a local pilot. The validated core is one repository with one
  accountable governor. Larger enterprise use should begin with project-local
  governors, explicit branch protection, and a documented escalation path
  rather than assuming AGF already supplies federation.
- You need every rule and extension point to be backed by multi-project
  evidence today. Some evidence-limited rules are marked in their rule text.
- Your team has no capacity for the governor role right now. Gates without
  an engaged governor become lightweight ceremony; the framework is designed
  for human accountability and does not substitute for it. Adopt when you, or
  a designated owner, can realistically provide gate decisions.

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
| Mechanical installation | Assistant-prepared file setup — no more than ten minutes in the maintainer's own deployments; empirically informed maintainer evidence, not yet independent-adopter validation |
| Initial commissioning | Human-reviewed rule choices and your first governed change — separate, variable work with no numeric estimate |
| Per-change overhead | Plan + two gates for non-trivial changes; fast track for small ones when no agent holds an active claim (an exclusive reservation in the coordination channel) |
| Governor attention | Often 5-15 minutes for a non-trivial first-trial change, more for high-risk work — a maintainer-informed planning estimate, not an independently validated benchmark |
| Ongoing | Learning records when incidents or reusable patterns happen; upgrade hygiene per release |
| Reversibility | Full removal is one atomic cleanup commit — see [exit](../adopt/exit-and-decommission.md); its duration is unmeasured and varies with your CI, hooks, and retained evidence |

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

### Privacy, labor, and provider-boundary checklist

Before using AGF beyond a small pilot, answer these locally:

- Who may read coordination records, plans, learning records, and MR comments?
- Are individual names required, or would duty handles, team aliases, or
  sprint-scoped identifiers preserve enough audit value with less monitoring
  risk?
- How long are coordination and review records retained outside ordinary git
  history, for example in CI logs, issue trackers, chat exports, or analytics?
- Does local policy forbid using governance records for productivity ranking or
  labor surveillance?
- Which AI assistants send prompts to hosted provider APIs, and what are their
  prompt logging, training, and retention settings?
- What must be redacted before external model calls: secrets, customer names,
  private paths, incident details, schema names, or domain-revealing examples?
- Which secret-scanning or DLP controls run before prompts, logs, or learning
  records leave the local environment?

AGF can make these questions visible; it does not answer them for your
jurisdiction or employment context. Treat this checklist as risk spotting, not
legal, labor, privacy, or compliance advice.

For a broader set of review questions — not specific to privacy or labor,
and applicable to any governance design, not only AGF — see
[Philosophical lenses on AI governance](../concepts/philosophical-lenses.md).

## Next

Decided to try it → [Install the `agf/` directory](../adopt/install-and-layouts.md).
