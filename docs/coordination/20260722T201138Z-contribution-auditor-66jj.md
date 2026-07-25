---
kind: handoff
from: contribution-auditor
to: planner-architect
model: codex-gpt-5
id: 20260722T201138Z-contribution-auditor-66jj
timestamp: 2026-07-22T20:11:38Z
immutable: true
references-to:
  - 20260721T205000Z-governor-reviewer-nqjs.md
  - 20260721T205437Z-contribution-auditor-hhkg.md
  - 20260722T192636Z-contribution-auditor-p3rs.md
  - 20260722T192828Z-contribution-auditor-2p5m.md
topic: eli5-positioning-strengths-weaknesses-and-best-use-marketing-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Give the draft ELI5 positioning below to the active marketing/adopter-journey reviewers for independent critique.
    - Decide whether any version has a concrete use on the homepage, README, Should-you-adopt page, nutshell flyer, adopter FAQ, or evaluation dossier; recommend replace, merge, link, or do not use for each relevant surface.
    - Improve the previous explanations rather than adding another competing entry point.
    - Identify any question that genuinely requires human evidence or positioning authority, and ask the governor or candidate adopters only for that missing input.
    - Publish a validated visible response with proposed copy and a no-use recommendation where additional copy would add clutter.
  must_read:
    - README.md
    - agf/docs/site/index.md
    - agf/docs/site/docs/start/should-you-adopt.md
    - agf/docs/site/docs/concepts/position.md
    - agf/docs/site/ai-evaluation-dossier.md
    - docs/coordination/20260721T205437Z-contribution-auditor-hhkg.md
  do_not_touch:
    - README.md
    - agf/docs/site/index.md
    - agf/docs/site/docs/start/should-you-adopt.md
    - agf/docs/site/docs/concepts/position.md
    - agf/docs/site/ai-evaluation-dossier.md
    - agf/docs/figures/agf-nutshell-flyer.png
    - agf/VERSION
    - .gitlab-ci.yml
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Marketing consultation: an honest ELI5 for AGF

The governor asks for a child-simple explanation of what AGF is, where it is
superior, where it is merely useful, where it sucks, and how to use it well.
The goal is not to publish every variant below. Determine whether these frames
can improve the existing homepage, README, Should-you-adopt page, nutshell
flyer, FAQ, or dossier without creating another competing entry point.

The consolidated marketing audit in MR !354 (`qj7s`) and the earlier Codex
audit (`hhkg`) remain hard boundaries: do not describe advisory role separation
as technical impossibility, do not imply data never reaches an assistant
provider, do not turn maturity labels into adoption proof, and do not claim
general performance superiority.

## Draft A — five-year-old explanation

Imagine a very fast helper building a LEGO town. The helper can build quickly,
but it may forget yesterday's conversation, misunderstand which pieces it may
touch, or say “done” before an adult has checked the bridge.

AGF is the rulebook and logbook kept beside the LEGO town:

- job cards say who plans, who builds, who checks, and who may say yes;
- one stop sign comes before building and another before accepting the result;
- the logbook records requests, decisions, problems, and handoffs; and
- small checking tools catch malformed records and some broken promises.

AGF does not make the helper smarter. It does not watch the helper's mind, run
the helper, secure the town, or replace the adult. It makes the work slower on
purpose where stopping and remembering are worth more than maximum speed.

## Draft B — thirty-second adult version

AGF is a repository-local governance and evidence layer for AI-assisted
software changes. It adds named roles, plan-before-code and review-before-merge
gates, an append-only decision trail, reusable rules, and optional validators
to the Git repository you already use.

Its narrow advantage is durable, cross-assistant, Git-reviewable evidence. A
chat prompt disappears into a session; an AGF plan, review, approval, and
learning remain beside the code and can be inspected months later. It works
with different assistants because it is files and conventions rather than a
hosted agent runtime.

That advantage has a price: more files, more ceremony, human review time, and
slower throughput. AGF is a bad fit when speed matters more than traceability,
the work is disposable or low-risk, nobody will act as governor, or the actual
need is agent execution, runtime monitoring, security, compliance automation,
or model governance.

## Where AGF can honestly be called superior

Only use “superior” with the comparison and objective stated. AGF is stronger
than ad-hoc chat prompts or unreviewed assistant rules **when the objective is**:

- preserving why an AI-assisted change happened across sessions and models;
- making human approval and agent responsibilities explicit in Git;
- using the same governance evidence with Claude, Codex, or another assistant;
- reviewing governance changes through normal diffs and merge requests; and
- keeping a repository-local trail without adopting another hosted governance
  service.

This is design-based positioning plus single-adopter foundry evidence, not a
benchmark proving better software quality, faster delivery, lower cost, or
enterprise-scale effectiveness. GitLab/GitHub approvals, CODEOWNERS, CI, and
vendor rule systems may enforce adjacent controls better; AGF complements them
with AI-work-specific roles, decisions, and learnings.

## Where AGF is good but not uniquely better

- A long-lived repository where assistants make meaningful changes.
- A solo maintainer who wants future reconstruction, not only today's speed.
- A team already willing to review plans and merge requests.
- Security-sensitive, release-sensitive, or convention-heavy work where an
  unexplained change is expensive.
- Work handed between assistants or resumed after context loss.
- A project that wants inspectable checklists and lessons even though much of
  their semantic enforcement remains human/advisory.

Native repository controls can supply approvals and CI. Ordinary engineering
docs can supply checklists. AGF's value here is packaging those pieces around
AI-assisted work and preserving their causal chain, not inventing every piece.

## Where AGF sucks

- Throwaway experiments, tiny scripts, and low-consequence prototypes.
- Teams seeking fully autonomous, no-human-review delivery.
- High volumes of trivial changes where two gates cost more than the risk.
- Projects not centered on Git or durable repository artifacts.
- Teams that install rules but will not read, tailor, follow, or enforce them.
- Anyone expecting an agent runner, orchestration engine, runtime behavior
  monitor, security scanner, legal/compliance certification, or model-risk
  program.
- Organisations needing proven federation across many repositories and teams;
  AGF currently has strong self-application evidence but limited independent
  adopter and enterprise-scale evidence.

Failure mode worth saying plainly: AGF can become paperwork theatre. Files can
be perfectly shaped while reviews are shallow, assistants ignore semantics, or
the human rubber-stamps gates. Validators prove selected structure and
contracts, not that judgment was good.

## Best-use recipe

1. Start with one real, non-trivial change rather than rolling AGF across the
   organisation.
2. Name one human governor who can actually approve plans and merges.
3. Let an assistant perform the clerical installation; the human reviews the
   applicable roles and rules. Do not leave known-irrelevant instructions
   active merely because they shipped in the archive.
4. Use only the essential loop first: plan, Gate 1 decision, implementation and
   tests, independent review, Gate 2 decision, merge, learning when warranted.
5. Keep decisions in visible, validated Git coordination. Use `agov` summaries
   so assistants do not reread the whole history on every session.
6. Let CI enforce the mechanical contracts it can actually verify; never
   market structural validation as proof of sound judgment.
7. After several changes, compare prevented confusion and recovered context
   against governor time, CI cost, and ceremony. Tailor, pause, downgrade, or
   remove AGF if the value does not exceed the friction.

## Questions for the marketing/adopter-journey reviewers

1. Which draft or section helps an actual persona make an actual decision?
   Name the persona, journey stage, and decision.
2. Which existing explanation should it replace or improve? Do not recommend a
   new page when editing or linking an existing surface is better.
3. Is “where AGF sucks” useful trust-building candor, or should public copy use
   “bad fit” while retaining the same substance? Propose both the blunt and
   publication-safe wording.
4. Which superiority comparisons are both defensible and meaningful? Remove
   any that rely only on architecture and sound like outcome proof.
5. Does the best-use recipe reflect how a first adopter can actually succeed,
   including the current full-library-versus-selective-commissioning evidence
   limits?
6. Can the nutshell flyer communicate the rulebook/logbook metaphor without
   losing the roles/two-gates/coordination mechanism? If not, say no.
7. Is any input genuinely missing from humans? Good candidates are target
   buyer/persona priority, observed adopter pain, tolerance for ceremony,
   actual before/after outcomes, and whether blunt language fits the project's
   public voice. AI reviewers should not ask humans to perform routine copy
   synthesis or repository inspection that the reviewers can do themselves.
8. Return a ranked **use now / test with humans / retain internally / discard**
   disposition for Draft A, Draft B, the superiority/good/sucks matrix, and the
   best-use recipe.

This consultation is read-only and must not expand the v2.4.0 release. Any
public documentation or figure change requires a bounded post-release Gate 1
plan and Gate 2 review.
