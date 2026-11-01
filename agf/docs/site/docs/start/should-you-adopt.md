# Should you adopt?

*Status: Released. Current framework version: see [Release information](../../reference/release-info.md). Audience: evaluators. After
this page you can make an informed adopt / wait / skip decision.*

## What you are deciding

agents-govern is a file-based governance framework for AI-assisted software
development. In plain terms: it is a set of files that tell your AI assistant
how to behave, a review process in which, by default, a human approves before anything merges,
and a decision log that makes those approvals reconstructible later.

The trade is deliberate: you accept a little ceremony so the human governor —
the person who approves gates and owns project risk — can reconstruct what was
decided and stop unsafe work before it lands. Adopt when that audit trail is
worth more than the overhead.

The simplest test is practical: picture the next risky change your assistant
might make. Would you benefit from approving its written plan before work
starts, approving the result before merge, and keeping both decisions in the
repository? AGF calls those checkpoints Gate 1 and Gate 2, and the durable
decision log the coordination channel.

By default a human approves each gate. AGF also describes one narrow, default-off,
provisional deputy route for some Gate 2 decisions. The human governor keeps
ultimate authority. See
[approval authority and optional deputies](../concepts/roles-and-gates.md#approval-authority-and-optional-deputies).

## The practical test

Use AGF when the cost of an unreviewable AI-generated change is higher than
the cost of pausing for a plan, a review, and a recorded decision.

Do not use AGF because it sounds rigorous. Use it when you need the rigor:
long-lived code, real users, regulated or security-sensitive work, shared team
conventions, or a codebase where "it worked in my session" is not good enough.

## Adopt now if…

- AI assistants write a significant share of your code, **and** you have
  (or are willing to be) a human governor accountable for plans and merges.
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
- You expect a hosted service or an agent runtime. AGF supplies files,
  rules, and conventions for your existing tools, with optional local
  tooling for summaries and validation.

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

## Understand the basics

AGF is a set of files in your repository. They tell an AI assistant how to plan,
build, and report its work. AGF defines two approval checkpoints. By default, a
human approves at both.
[Roles in plain terms](../concepts/roles-and-gates.md#roles-in-plain-terms)
explains who does what. The [framework map](framework-map.md) shows the parts.
[What AGF is / is not](#what-agf-is-is-not) gives the longer statement.
AGF ships sixteen role definitions, and a solo project starts with three responsibilities. See
[Which roles does my project need?](../concepts/roles-and-gates.md#which-roles-does-my-project-need).

## Cost and accountability

[What adoption asks of you](#what-adoption-asks-of-you) summarizes the
commitments. [Downstream impact](../adopt/downstream-impact.md) itemises files,
CI jobs, and obligations. [Licensing](../adopt/licensing.md) states the terms.
[Exit and decommission](../adopt/exit-and-decommission.md) states how to leave.
The human governor stays accountable for gate decisions.

## Technical fit

AGF works beside your repository, CI, review, and assistant tools. It adds no
hosted service. [AGF and agent runtimes](../concepts/position.md) explains how it
relates to assistants and other frameworks, and where its enterprise boundary
lies. The [applicability catalog](../adopt/applicability-catalog.md) is an
incomplete pilot. For the artifacts it covers, it gives an activation condition
and a removal consequence. It is not a validated pruning guide.
[Evidence and learnings](../concepts/evidence-and-learnings.md) states the
evidence limits.

## Assistant workflows

Loading rules does not assign a role or grant approval authority. An assistant
can play eligible roles only through an authorized assignment and within their
limits. The human governor stays accountable for approvals; see
[Roles in plain terms](../concepts/roles-and-gates.md#roles-in-plain-terms).
Parallel work needs separate work boundaries, claims for non-trivial tasks, any
required recorded and acknowledged allocation, and recorded handoffs through
`docs/coordination/`. See
[one assistant, several roles](adopter-faq.md#can-one-ai-assistant-play-several-roles)
and [several assistants at once](adopter-faq.md#can-several-ai-assistants-work-at-once).
[Assistant loader configuration](../adopt/assistant-loader-configuration.md)
covers setup. [Third-party tooling](../adopt/third-party-plugin-coexistence.md)
covers coexistence limits.

## Selected use cases

- **Solo maintainer with AI assistants doing most implementation.** The gates give
  the maintainer recorded checkpoints.
- **Small team writing existing conventions down for assistants.** Assistants
  receive the same written rules across sessions. Consistency is the intended
  benefit, and adherence still needs review.
- **Audit or compliance pressure.** AGF supplies a decision trail. It does not
  certify compliance.

The [use cases page](../../use-cases.md#credible-use-cases) keeps the detailed examples.

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

For a solo trial, the documented minimum is `planner-architect`, `coder-tester`
and the human `governor-reviewer`. One assistant may play both `planner-architect`
and `coder-tester`. A standard governed project may use separate players for them.
Activate specialist roles when their trigger applies; the
[roles page](../concepts/roles-and-gates.md#which-roles-does-my-project-need)
lists the triggers. This incremental, roles-added-as-needed pattern is what the
maintainer's own projects have followed. Core rules remain binding;
workflow, documentation, UI, and other task-specific rules apply when their
subject is present. If a rule clearly does not and will not apply to the
project, removing it during commissioning avoids teaching assistants that
loaded instructions are optional — but doing so is a choice, not a required
step; leaving the full rule set active has also worked without incident in
the maintainer's own deployments.

For a first, still-incomplete answer to *which specific rules and roles you
can safely remove and what each removal costs, if you choose to prune*, see
the [adopter applicability catalog](../adopt/applicability-catalog.md). It is an
incomplete pilot classification that gives an activation condition and a removal
consequence for each artifact it covers. Its current coverage is stated on that
page.

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

## Next

Decided to try it → [Set up AGF](commission.md).
Want to check the terms first → [Licensing](../adopt/licensing.md).
Want concrete examples → [Use cases](../../use-cases.md).
Still unsure about overhead → [Downstream impact](../adopt/downstream-impact.md).
Recognized a non-fit → stop here; AGF is not a prerequisite for responsible AI
use.
