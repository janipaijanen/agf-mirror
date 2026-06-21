# Adopter FAQ

*Status: Provisional (v2.0 documentation). Audience: people deciding whether
to commission AGF in a real repository. This page answers the questions that
usually appear after the idea makes sense, but before the first governed
change feels safe.*

## Do we need AGF, or just better prompts?

Use better prompts first. If that is enough, stop there.

AGF is for the moment when polite instructions stop carrying the weight:
"write tests", "do not push to main", "explain risky migrations", "leave a
review trail". The framework turns those expectations into files, roles,
gates, validators, and merge evidence that survive across sessions and tools.

If the code is throwaway, AGF is probably too much. If the code will live for
months and AI assistants are making real changes, the structure starts to earn
its keep.

## Should we commission the tooling too?

Usually yes, but not on day one.

Start with the framework files and one governed change. Once the loop is real,
add the `agov` tooling so agents and CI stop re-reading the same governance
files by hand. The tooling is not the framework's authority; it is the
mechanical helper that validates plans, coordination entries, learning records,
and handoffs.

The practical path is:

1. Copy `agf/`.
2. Run one small governed change.
3. Add `agov coord validate` and `agov plan validate` to CI.
4. Add more checks only when the team understands what each one is allowed to
   block.

For setup details, see [Tooling and token savings](../adopt/tooling-and-tokens.md)
and [Contracts and validators](../advanced/contracts-and-validators.md).

## Does AGF replace our code review?

No. It gives your review a spine.

Humans still approve plans, accept risk, and decide what gets merged. Agents
can draft plans, implement changes, run tests, prepare reviews, and collect
evidence. AGF's job is to keep those responsibilities visible so the assistant
does not become its own planner, implementer, reviewer, and approver in one
smooth blur.

## Who is the governor?

The governor is the human who can say yes, no, wait, or stop.

In a solo project, that is you. In a team, it might be a tech lead, maintainer,
security owner, or rotating reviewer. The title matters less than the authority:
someone must own Gate 1 plan approval, Gate 2 merge approval, and the decision
to relax or tighten governance.

If nobody is willing to be governor, do not adopt AGF yet. Gates without a
governor become ceremony.

## When should we write a learning record?

Write one when the project learns something the next agent, reviewer, or
adopter should not have to rediscover.

Good triggers:

- an incident or near-miss;
- a repeated coordination failure;
- a rule that prevented real damage;
- a rule that was too heavy and caused workarounds;
- a validator that caught something a human review missed;
- a pattern that happened twice and should become guidance.

Do not write a learning for every ordinary task. The learning file is not a
diary. It is the place where operational evidence becomes durable.

## How do we draft a learning?

Start from the shipped template:
`agf/templates/learning-record.yaml`.

Copy it to your adopter-owned learning file:
`agf/learnings/<your-codename>.yaml`.

Keep the first draft simple:

- what happened;
- why it mattered;
- which agents or humans were involved;
- what evidence proves it happened;
- what should change, if anything;
- how confident you are.

Use the project codename at the boundary. Do not include customer names,
private paths, secrets, domain-revealing identifiers, or chat transcripts that
contain sensitive context.

For the model behind learnings, see
[Evidence and learnings](../concepts/evidence-and-learnings.md).

## Do downstream adopters get a learning template?

Yes. The starter templates include:

- `agf/templates/learning-record.yaml`
- `agf/templates/private-codename-mapping.example`

The private codename mapping is deliberately an example. Keep your real mapping
outside git unless your own governance says otherwise. Upstream should see your
codename, not your private project identity.

## When should a learning go upstream?

Send it upstream when it would improve AGF for other adopters.

Good upstream candidates:

- a rule failed in a way others could hit;
- a rule prevented a meaningful failure;
- the documentation led you into confusion;
- tooling blocked or missed something important;
- a repeated pattern suggests a new validator, template, or example.

Keep local-only learnings local when they are specific to your stack, your
organisation, or private project facts. The framework needs evidence, not your
confidential context.

## How do we push a learning upstream?

Today, treat it like a normal contribution:

1. Anonymize the learning. Use only the project codename and neutral technical
   descriptions.
2. Validate it locally with the AGF tooling when available. At minimum, run the
   general validator (`agov validate`) before sending the record outside your
   repository. If you are using the Elixir reference tasks, run the learning,
   infosec, and contribution validators before opening the upstream request.
   From `tooling/elixir/`, those checks are:
   `mix agov.validate.learnings`, `mix agov.validate.infosec`, and
   `mix agov.validate.contributions`.
3. Open an upstream issue or merge request in the
   [AGF project](https://gitlab.com/janipaijanen/agents-govern-framework) with
   the anonymized learning or a summary derived from it.
4. Include the proposed framework change: rule, docs, validator, template, or
   "no change yet, evidence only."
5. State whether the evidence is a single incident, a repeated pattern inside
   one adopter, or corroboration of an existing upstream rule.

The upstream acceptance process is still maturing. That is okay. The important
thing is not to wait for perfect machinery before recording evidence safely.

## Can one AI assistant play several roles?

Yes, if it plays them sequentially and records the handoffs.

This is the validated core for AGF today: one human governor, one repository,
and one or more AI sessions filling roles over time. The same assistant may
draft the plan as `planner-architect`, later implement as `coder-tester`, and
later prepare a self-review. It must not approve its own gates. The governor's
decision remains human.

The coordination channel is what keeps the roles from dissolving into chat
memory. Each role leaves files for the next role to read.

## Can several AI assistants work at once?

Yes, carefully.

Give each active assistant a role, a branch, and a clear artifact boundary.
Make them communicate through `docs/coordination/`, not private chat. Require
claims before non-trivial work. Before every commit or push, have the agent
check the branch it is on.

The framework already supports this operating style because the shared medium
is git. It does not require a shared agent runtime.

## Can several humans and several AI assistants use AGF together?

Probably, but AGF v2 has not validated the full organisational pattern yet.

The safe version is modest. Run it as a pilot, not as an organisation-wide
policy system:

- one repository;
- one governor at a time;
- clear human backup for the governor;
- one coordination channel;
- explicit role claims;
- normal code-owner or maintainer review layered on top of Gate 2;
- one branch owner per active branch, whether that owner is human or AI;
- human developers filing the same kind of handoffs as AI agents when their
  work creates obligations for someone else.

What we do not know yet is how AGF should scale across multiple governors,
multiple teams, or many repositories without creating confusion about who owns
the final decision. Treat that as a pilot area. If you try it, write learning
records. That is exactly the kind of evidence the framework needs.

## How should a mixed human and AI team avoid collisions?

Use the same boring habits for everyone.

- Claim non-trivial work before starting it.
- Keep one concern per branch.
- Name the branch owner in the coordination channel.
- Before pushing, check the branch and recent commits.
- If a human takes over an AI branch, file a note saying so.
- If an AI takes over a human branch, make the human's current state durable
  first: changed files, tests run, open risks, and the next expected action.

AGF does not make parallel work safe by itself. It gives the team a place to
make ownership visible before the collision happens.

## What should our first week look like?

Keep it small.

Day 1: read [Should you adopt?](should-you-adopt.md), [Downstream impact](../adopt/downstream-impact.md),
and [Exit and decommission](../adopt/exit-and-decommission.md).

Day 2: copy `agf/`, pick a codename, and customize only the rules that plainly
need local values.

Day 3: run [Your first governed change](first-governed-change.md) on a small,
real task.

Day 4: add the first CI validators for plans and coordination messages.

Day 5: review friction honestly. Keep what helped. Remove or defer what became
empty ceremony.

## What should we avoid in the first week?

Avoid turning adoption into a platform project.

Do not start with ten agents, custom schemas, a full CI matrix, and upstream
contribution automation. Start with one governed change and one human governor.
Then add structure where the project actually feels pain.

AGF works best when it makes accountability lighter to practice, not heavier to
perform.
