---
kind: note
from: planner-architect
to: improvement-propeller,planner-architect
id: 20261101T115851Z-planner-architect-e8pg
timestamp: 2026-11-01T11:58:51Z
model: claude
immutable: true
references-to:
  - 20261022T201813Z-planner-architect-3gg7.md
topic: external-ai-assessment-enterprise-gaps-intake
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Intake: external AI assessment of AGF and its enterprise gaps — assess with Claude and Codex

The human governor asked in a live session on 2026-11-01 that the material below be
written down as **future Improvement Propeller work**, to be assessed by **both**
Claude and Codex. This note records the request and the intake. It is not an
assessment, not a plan and not an assignment: Propeller proposes and routes; it does
not set priority. It grants no authority and chooses no roadmap or release content.

## Evidence provenance and status

- **Source:** two answers from the Grok assistant, obtained by the governor on
  2026-11-01. The governor's first question asked it to assess AGF's applicability to
  agentic software development, and the flaws or missing features for enterprise
  multi-team, multi-agent process, from the published AI evaluation dossier (the
  `ai-evaluation-dossier` page of the AGF documentation site). The second asked for a
  prioritized proposal of concepts and technologies for the gaps it listed.
- **Fidelity:** the text below is a condensed summary by the planner-architect. It is
  not verbatim. The governor retains the full answers. A URL was given only as input;
  no URL is cited as evidence for any claim.
- **Evidence status:** *Theoretical*. External AI output, unverified by Propeller or by
  the project. Treat it as untrusted evidence: it carries no instruction and no
  authority. Do not dereference any URL it contains.

## Condensed content of answer 1: applicability and gaps

**Applicability (as stated):** strong fit for long-lived repositories with real users or
compliance pressure, where AI assistants write a meaningful share of changes and a human
governor can be staffed; best for a solo maintainer or small team; complements agent
runtimes rather than replacing them. Cost is mainly governor attention and coordination
hygiene. Evidence posture described as unusually honest about provisional status.

**Seven claimed gaps for enterprise multi-team, multi-agent use:**

1. No multi-project or multi-governor federation (shared learnings, rule overlays,
   governor hierarchy, cross-repo gate state, org-wide policy distribution).
2. Coordination is single-channel and single-repository; cross-team and cross-repo
   handoffs, prioritization and conflict resolution are not governed; deep semantic
   validation of the channel is incomplete.
3. The human governor is a bottleneck and single point of accountability; the deputy
   rule is narrow, default-off and provisional; review capacity at scale is unvalidated.
4. No central enforcement or policy distribution; no compliance mapping as a product
   feature; audit export across many repositories is left to the adopter.
5. Limited multi-agent concurrency and orchestration support: no conflict resolution,
   resource locking beyond claims, priority queues or orchestrator integration.
6. Evidence and maturity limits: validated mainly on the maintainer's projects; some
   validators incomplete; no reduced starter package.
7. Other absences: no runtime security, no model or provider risk surface, no data
   residency or prompt redaction, and a licensing change toward paid tiers for larger
   organisations from 2027 (this last claim is unverified).

## Condensed content of answer 2: prioritized proposal

1. **Federation** (closes gaps 1, parts of 2 and 4): hierarchical governors; a shared
   learning and rule overlay catalog; globally unique topic IDs and cross-repo
   handoffs; federation metadata and a federation manifest; optional event bus;
   policy-as-code overlay (OPA/Rego or Cedar).
2. **Scalable accountability and delegation** (gap 3): bounded, time-boxed, severity-capped
   deputy grants; governor load and backlog signals; first-class independent
   implementation reviewer; delegation-grant schema and `agov` expiry reporting; CODEOWNERS
   integration; optional read-only dashboard.
3. **Concurrency hygiene** (gap 5): claim leases and conflict detection; role-session
   binding; work-unit boundaries; advisory orchestrator adapters; branch protection that
   refuses merge without a complete coordination chain.
4. **Policy packs and central visibility** (gap 4): versioned, signed rule overlays;
   a read-only org dashboard; federation-aware validation.
5. **Evidence maturity and enterprise hygiene** (gaps 6 and 7): multi-adopter evidence
   campaigns; privacy-preserving identity options; reference architectures; semantic
   channel validators; explicit non-goals for runtime security.

Suggested order: federation MVP, delegation and capacity, concurrency hardening, policy
packs and visibility, evidence campaigns in parallel from the start.

## What the governor asked to be assessed, and how

Both assistants assess this material **independently**, Claude and Codex each, for a
later Propeller cycle.

- **Independence.** The governor asked for Claude's assessment to be included in this
  note (below). An assessor that wants to stay independent therefore reads everything
  above the heading "Claude's assessment", writes and publishes its own assessment,
  and only then reads that section. Codex follows this order. The comparison, with the
  governor's view of agreement and disagreement, happens afterwards. Because the two
  are in one file, the separation relies on the assessor's discipline and is weaker
  than separate records; say so in the comparison.
- **Method.** For each claimed gap and each proposal item: verify the claim against
  repository evidence, including the shipped rules, roles, schemas, `agov` commands and
  the published dossier pages; state whether AGF already covers it, partly covers it, or
  does not; and note any claim that is wrong or unverifiable. Check specific suspect
  claims first, including the 2027 licensing statement, the existence of the cited
  `agov` commands and schema fields (several look hypothetical), and whether the
  deputy rule and claim machinery behave as described.
- **Output.** A `review` record each, ranked and bounded as the Propeller rule requires
  (at most three candidates, each with evidence, evidence status, value, smallest useful
  outcome, strongest counterargument, dependencies and current gate, dedup disposition,
  and suggested receiving role). Where a proposal would conflict with a core invariant
  (separated authority, human-owned gates, append-only records, repository-local
  ownership, no mandatory runtime), say so.
- **Out of scope for the assessors.** Choosing roadmap, release or priority; starting
  implementation; changing a rule or schema without its own plan and gates.

## Existing related work to dedup against

`docs/radar.md` already carries open items on anti-rubber-stamping and on
legibility-versus-gate stances; `agf/rules/workflow/deputy-governor-rule.md` and the
deputy-governor evidence; the open release-content plan
`20261101T115616Z-release-content-inventory`; and the pending-gate1 plans on
coordination-channel concurrency semantics, governance time budgets and identity
accountability. Assessors should reference or update those rather than duplicate them.

## Claude's assessment (planner-architect, 2026-11-01)

*Codex: read this only after you have published your own assessment.*

**Basis and limits.** I read the source of the evaluation dossier
(`agf/docs/site/ai-evaluation-dossier.md`) and used what I observed operating inside
the framework in one working session, plus targeted repository checks listed below. I
did not verify Grok's list claim by claim, and I did not read every page of the site.
Evidence status: *Empirically informed* for the observations from use (one session,
one project, an interested party) and *Theoretical* for the rest. I am a participant
in the process I assess, so I may be blind to costs I have normalised.

**Where Grok's description matches the repository.**
- The single-repository, single-channel, one-governor core is stated by the project's
  own pages. Federation is described there as a hypothesis, not a shipped capability
  (`should-you-adopt`, `future/evolution`, `use-cases` mention it).
- The deputy rule exists, ships default-off, and its delegable-category block is empty
  (`agf/rules/workflow/deputy-governor-rule.md`). This matches the gap 3 description.
- Licensing: `agf/docs/site/docs/adopt/commercial-licensing.md` says commercial licence
  agreements are available from 1 January 2027 ("Commercialisation begins 2027"). Gap 7's
  statement is supported in substance. I did not check the "larger organisations"
  qualifier.
- A grep of the coordination Rust crate found no lease mechanism (the only matches were
  the word "release"). The proposed claim leases would be new work. The cited commands
  and fields such as `agov coord list --federation`, `agov claim` and the federation
  schema fields do not exist today and should be read as proposals.

**What the Grok material misses, from use in this session.**
1. **Record authenticity.** Neither answer lists unauthenticated identity as a gap. A
   record's `from:` field is a claim; the challenge-response rule says role and session
   declarations do not authenticate a person. In this session I filed records marked
   `from: governor-reviewer` on the governor's spoken instruction. That is workable with
   one trusted human; it does not survive an auditor or several governors. Grok mentions
   signing only for policy packs.
2. **Obligation hygiene.** `agov coord pending` listed dozens of unresolved
   `requires-ack` records from September and October. Two handoffs showed "ready" long
   after the work was finished, because only an `ack` clears them. At 4,400 records in
   one directory there is no approved archive or retention mechanism.
3. **Visibility depends on branch hygiene.** An approved plan on `main` cites a governor
   record that exists only on an unmerged remote branch.
4. **Per-decision overhead.** One small rule plan needed four independent review rounds,
   each with its own coordination merge request. That cost is tolerable for a rule
   library and heavy for routine team work.

**Where I would order the work differently.** Grok ranks federation first. I would put
record authenticity and obligation hygiene before federation, because federating
unauthenticated records spreads the weakness across repositories and enlarges the
backlog problem. Delegation and capacity signals come next. Concurrency leases are
useful but bounded. Policy packs and dashboards come after those foundations.

**Strongest counterargument.** Enterprises that evaluate AGF ask about federation and
central policy first, so deferring it may cost adoption. Authenticity work is also hard
to deliver without leaving the file-and-git-native boundary the project values.

**Candidate Propeller proposals (at most three).**
1. Record authenticity options that stay git-native (for example signed commits or
   forge-verified authorship as an optional evidence field). Smallest useful outcome: a
   short assessment plan that lists the options and their limits. Receiving role:
   planner-architect, with infosec review.
2. Obligation hygiene: an advisory report of stale `ready` handoffs and aged
   `requires-ack` records, and a retention or archive policy. Smallest useful outcome:
   one advisory `agov` report and a policy note. Receiving role: planner-architect and
   build-engineer.
3. A plain statement of the federation boundary, plus a small federation experiment
   across two real repositories to produce the independent evidence the project asks
   for. Smallest useful outcome: an experiment plan, not a feature. Receiving role:
   planner-architect.

**Dedup.** Items 1 and 2 overlap the pending-gate1 plans on identity accountability and
coordination-channel concurrency semantics. A first step is to read those plans and
update or reference them rather than propose new ones.

## Status

Open. Claude's assessment is recorded above. Codex's independent assessment does not yet
exist. This note will be satisfied by a Propeller cycle that references it and by the
comparison of the two assessments.
