# Adopter FAQ

*Status: Released (v2.1 documentation). Audience: people deciding whether
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

## What are the hard rules?

Most of AGF is meant to be adapted: roles, local commands, review cadence, CI
shape, and which specialist agents you actually use. A small set of rules is
hard because breaking them destroys trust in the record.

The big one is immutability:

- filed coordination messages are never edited; corrections are new messages;
- pushed git tags are never moved, deleted, or recreated; a broken release is
  fixed by a new version;
- historical evidence is preserved rather than rewritten to make the story
  cleaner.

This may feel strict until the first failure. Then it becomes calming: everyone
can trust that the trail says what happened, not what someone later wished had
happened.

## Should we commission the tooling too?

Usually yes, but not on day one.

Start with the framework files and one governed change. Once the loop is real,
add the `agov` tooling so agents and CI stop re-reading the same governance
files by hand. The tooling is not the framework's authority; it is the
mechanical helper that validates plans, coordination entries, learning records,
and handoffs.

The practical path is:

1. Ask your assistant to commission `agf/` and the first local evidence files.
2. Review the diff and remaining `CUSTOMIZE` markers.
3. Run one small governed change.
4. Add `agov coord validate` and `agov plan validate` to CI.
5. Add more checks only when the team understands what each one is allowed to
   block.

For setup details, see [Tooling and token savings](../adopt/tooling-and-tokens.md)
and [Contracts and validators](../advanced/contracts-and-validators.md).

## Does AGF replace our code review?

No. It gives your review structure.

Humans still approve plans, accept risk, and decide what gets merged. Agents
can draft plans, implement changes, run tests, prepare reviews, and collect
evidence. AGF's job is to keep those responsibilities visible so the assistant
does not become its own planner, implementer, reviewer, and approver in one
smooth blur.

## How do ADRs fit with AGF?

Keep Architecture Decision Records. AGF does not replace them.

Use ADRs for durable architecture choices: technology selection, data model
shape, integration boundaries, security posture, and decisions that future
maintainers will ask about. Use AGF coordination messages for the work trail:
who proposed the change, who reviewed it, what gate decision landed, and which
branch or merge request carried the implementation.

For downstream adoption, ask your AI assistant to create your own `docs/adr/`
directory and copy the ADR template pattern from `agf/docs/adr/`. Then make ADR
discipline part of the governed loop:

- a Gate 1 plan references any proposed or accepted ADR that shapes the work;
- the planner-architect drafts or updates ADRs for significant design choices;
- affected humans or agents are consulted before the ADR is accepted;
- the governor-reviewer approves the gate, not the author of the ADR alone;
- Gate 2 verifies that the implementation still matches the accepted decision;
- if the architecture changes later, write a new ADR that supersedes the old
  one instead of rewriting history.

The useful split is simple: ADRs explain why the architecture is the way it is;
coordination records explain how a specific governed change moved from proposal
to merge.

## Who is the governor?

The governor is the human who can say yes, no, wait, or stop.

In a solo project, that is you. In a team, it might be a tech lead, maintainer,
security owner, or rotating reviewer. The title matters less than the authority:
someone must own Gate 1 plan approval, Gate 2 merge approval, and the decision
to relax or tighten governance.

If nobody is willing to be governor, do not adopt AGF yet. Gates without a
governor become ceremony.

## Can AI act as a governor?

Not as the validated AGF core today.

An AI can help the governor: summarize a plan, challenge a merge request, check
whether evidence is missing, draft a decision-record skeleton, or compare the
work against the rules. Those are advisory tasks. They can make the human
governor faster and better prepared, but they do not move risk out of human
hands.

AI-as-governor has not been tested enough to recommend as an AGF operating
mode. A project may experiment with stronger AI delegation only as a local
extension, with a human explicitly accepting the risk, defining what the AI is
allowed to approve, and staying accountable for the outcome. If nobody human is
prepared to bear that risk, the project does not have a governor.

## How much governor time should we expect?

Budget real human attention. AGF has no server to operate, but it does add a
human decision role.

For a small first trial, a useful planning range is:

| Work shape | Typical governor attention |
|---|---|
| Trivial typo or comment | Usually none beyond normal review |
| Small low-risk change using fast track | 1-3 minutes to confirm the waiver fits |
| Non-trivial governed change | 5-15 minutes across Gate 1 and Gate 2 |
| High-risk change | As long as the risk review needs; do not optimize this to a fixed number |

These are planning estimates, not service-level promises. The governor is not
reading every file in every session. They are checking the plan before work
starts, deciding whether the evidence is enough before merge, and pulling work
into deeper review when something feels wrong.

If those minutes are unavailable, wait. A gate that nobody has time to read is
worse than no gate because it teaches the team to treat approval as theatre.

## When should we write a learning record?

Write one when the project learns something the next agent, reviewer, or
adopter should not have to rediscover.

Good triggers:

- the first AGF commissioning run;
- an incident or near-miss;
- a repeated coordination failure;
- a rule that prevented real damage;
- a rule that was too heavy and caused workarounds;
- a validator that caught something a human review missed;
- a pattern that happened twice and should become guidance.

Commissioning is a useful first learning because it tests the adoption
promise in the smallest real setting: did the assistant keep AGF-owned files
inside `agf/`, create only a small loader if needed, leave clear decisions for
the governor, and produce validation output you could understand?

Do not write a learning for every ordinary task. The learning file is not a
diary. It is the place where operational evidence becomes durable.

Use this default threshold:

| Do this | When |
|---|---|
| Do not record | Routine implementation friction that stayed inside the normal plan, review, or test loop. |
| Record locally | Commissioning, repeated friction, a useful rule catch, a rule that caused workarounds, or a governance near-miss. |
| Escalate, then record | Secret exposure, irreversible data or release damage, authority-boundary violations, or anything the governor must stop immediately. |

When in doubt, write a short local learning and mark confidence honestly. You
can always decide later that it stays local. You cannot reconstruct a missing
incident trail from memory six months later.

## How do we draft a learning?

Ask your assistant to start from the shipped template
`agf/templates/learning-record.yaml` and prepare your adopter-owned learning
file at `agf/learnings/<your-codename>.yaml`. Review the result before it is
used as evidence.

Keep the first draft simple:

- what happened;
- why it mattered;
- which agents or humans were involved;
- what evidence proves it happened;
- what should change, if anything;
- how confident you are.

Do not invent globally meaningful sequential learning IDs. If your local
template has no ID field, leave it that way. If your project adds one later,
use `YYYYMMDDThhmmssZ-<4-char-random>`, or equivalent project-local tooling
that cannot collide. Keep existing IDs stable when updating old records; add a
new suffixed ID for a new or superseding learning.

Use the project codename at the boundary. Do not include customer names,
private paths, secrets, domain-revealing identifiers, or chat transcripts that
contain sensitive context.

For the model behind learnings, see
[Learning loop and evidence](../concepts/evidence-and-learnings.md).

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

- a first commissioning run revealed adoption friction or unusually smooth
  automation;
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
2. Validate it locally with the AGF tooling when available. Run
   `agov validate` for your repository and
   `agov validate learnings <path-to-learning.yaml>` for an upstream-submittable
   learning packet stored outside your canonical `agf/learnings/` register.
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

## Can an assistant work on behalf of a human?

Yes, if you make the delegation explicit and keep the AI identity visible.

Use a human-authored `kind: note` as the delegation grant. The grant names the
principal kind, identifier namespace, identifier, scope, and preferably an
expiry. The assistant then references that grant from its own coordination
records. It must not write a message that self-asserts "I am acting for
@someone" without a human-authored grant.

The handle is not pulled from git. Use the human-readable identifier from the
authority you name in `identifier_namespace`, for example a GitLab project or
organisation namespace. If individual attribution creates more privacy risk
than audit value, use a team alias, rotating duty alias, or batch grant instead.

Delegation does not transfer gate authority. A principal may authorize patch
preparation, evidence collection, or test runs; Gate 1, Gate 2, release
approval, and binding risk acceptance still require the governor-reviewer.

This convention is optional and docs-only in v2.x. Tooling may accept the
fields syntactically, but it does not enforce delegation semantics yet.

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

## What adoption pattern should we start with?

Start with the smallest pattern that still preserves human gate ownership.
These are examples, not separate AGF editions.

| Pattern | Start with | Keep explicit |
|---|---|---|
| Solo maintainer | One human governor, one assistant, one small governed change, plan and coordination validators | The maintainer may play governor, but the approval is still a deliberate recorded decision |
| Small team | One primary governor, one backup, normal MR review, shared coordination channel | Gate decisions and code-owner review are related but not identical |
| Security-conscious repository | Full two-gate path, protected default branch, required CI, secret scanning, visible-artifact review where relevant | AGF provides evidence; security controls still run in their own systems |
| Multi-assistant project | One role and branch per active session, claims before non-trivial work, frequent coordination status checks | Private chats are not shared state; commit only after branch verification |

If the pattern feels heavy, do not remove the governor. Narrow the pilot, use
fast track only where the criteria fit, and record friction as evidence before
loosening rules.

## What should our first week look like?

Keep it small.

Day 1: read [Should you adopt?](should-you-adopt.md), [Downstream impact](../adopt/downstream-impact.md),
and [Exit and decommission](../adopt/exit-and-decommission.md).

Day 2: ask your assistant to commission `agf/`, pick a codename, and customize
only the rules that plainly need local values. Review the diff instead of doing
the copying by hand.

Day 3: run [Your first governed change](first-governed-change.md) on a small,
real task.

Day 4: add the first CI validators for plans and coordination messages.

Day 5: review friction honestly. Keep what helped. Remove or defer what became
empty ceremony.

**First-week checklist:**

- [ ] Read decision, cost, and exit pages (Days 1 and 5 are the honest-cost moments)
- [ ] Install `agf/` and configure the assistant loader
- [ ] Verify your assistant actually loaded AGF: ask it to name the current framework version and an active rule
- [ ] Run one complete governed change — plan, implement, Gate 2, merge
- [ ] Add at minimum one CI validator job (coordination or plan schema)
- [ ] Prove the validator fires with a deliberately malformed test file
- [ ] Review friction after the first real task — not after setup, after actual work

## What should we avoid in the first week?

Avoid turning adoption into a platform project.

Do not start with ten agents, custom schemas, a full CI matrix, and upstream
contribution automation. Start with one governed change and one human governor.
Then add structure where the project actually feels pain.

AGF works best when it makes accountability lighter to practice, not heavier to
perform.
