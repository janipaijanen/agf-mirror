---
id: 20260905T210255Z-universal-agent-autonomous-polling
status: pending-gate1
topic: universal-agent-autonomous-polling
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      No independent infosec review exists yet. Although this is an
      instruction-only change with no new credential, dependency, network
      access, or data path, it broadens the default frequency and scope of
      forge inspection and autonomous actions. A fresh review must confirm
      that polling grants no new read/write authority and preserves secrets,
      destructive-action, and security-ambiguity stops before Gate 1.
  - role: contribution-auditor
    status: waived
    reason: >-
      No independent contribution audit exists yet. Claude planner design
      challenge was requested in
      20260905T205921Z-planner-architect-pm6m, but that collaboration is not a
      substitute for contribution-auditor review. A fresh audit must screen
      authority inflation, duplicate execution, notification spam, hidden
      collision with MR !1817, and false claims of autonomous scheduling
      before Gate 1.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Universal agent polling and autonomous continuation

## Outcome

Make complete polling and autonomous continuation a normative obligation for
**every active AGF development agent/session**, not an optional response to the
word “poll,” a foundry-only habit, or a duty limited to the agent that opened a
particular MR.

Every active agent observes the complete shared work surface. Execution remains
collision-safe and role-scoped: an agent acts only inside its existing role,
approved plan or handoff, current gate, and explicit ownership. It does not ask
the governor to redispatch visible authorized work or to say “proceed.” It
pauses only for a genuine binding gate, an equivalent missing authority, or a
concrete external blocker after safe diagnostics and alternatives are
exhausted.

This is a Level 2 (Surface-defined) plan until approved and implemented. The
behavioral claim is Theoretical plus empirically informed by foundry incidents;
the implementation must not describe it as mechanically enforced.

## Problem

Foundry's root `AGENTS.md` and internal cold-start page already define a strong
poll/action loop. The shipped AGF contract is weaker and fragmented:

- `poll-and-coordinate.md` addresses assistants already “responsible for
  advancing already-authorized work” and asks for every **relevant** MR and
  issue, leaving each agent free to define away unseen work;
- the shipped `poll-and-coordinate` skill runs when invoked, but no canonical
  rule requires every active development agent to invoke or reproduce the
  lifecycle on cold start, configured wake, and resume;
- the adopter `AGENTS.md` template points to the runbook but does not say that
  every active agent polls or autonomously continues work;
- the runbook maps “Asked to poll or coordinate,” which makes polling appear
  prompt-triggered;
- branch-watch guidance names a primary watcher but does not clearly say that
  passive or role-specific agents retain their own complete observation duty;
  and
- MR !2369's useful starter guide is explicitly non-normative and cannot close
  the rule gap by itself.

The result is predictable: one agent watches its own work, another waits for a
chat prompt, and the human becomes a dispatcher for obligations already visible
in GitLab and coordination.

## Canonical rule design

Create
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md` as the
single normative owner. A new `always/` rule is justified instead of extending
`development-workflow-rule.md`: polling applies before a task is selected, when
the assignment queue is empty, during CI/review waits, and to advisory/reviewer
roles that may not be implementing a non-trivial task.

The rule will define these invariants.

### 1. Applicability

The rule applies to every **active** human or AI development agent/session in an
AGF-governed project. “Active” means the session is executing, has resumed, or
has a real configured wake-up. It does not claim that an exited process or an
assistant surface without a scheduler continues running.

Every active agent performs a complete poll:

- before selecting work after cold start;
- whenever it resumes after inactivity or compaction;
- at each real configured wake while a monitoring window is armed; and
- before claiming an item when its last complete poll is stale relative to a
  newly observed coordination or forge event.

### 2. Complete observation surface

Each cycle fetches/prunes shared refs, then inspects remote-aware coordination,
**all open** forge merge requests/change requests and issues regardless of
author or apparent relevance, new notes and discussions, assignments, labels,
draft/approval/merge state, exact source and target SHAs, exact-head pipelines
and jobs, and every branch/tag/release/runner or other exact external state the
agent started or awaits. True pagination and durable UTC/ID/SHA cursors are
required so “nothing new” is evidence-backed.

All agents observe everything; they do not all claim everything.

### 3. Collision-safe selection and action

After observation, each agent classifies visible items by priority, authority,
current gate, dependencies, and ownership. It checks for existing claims and
overlapping plans/MRs immediately before claiming or writing. It then advances
the highest-priority item that is:

1. inside the active role's authority;
2. authorized by an approved plan, handoff, standing rule, or direct binding
   instruction;
3. inside the current gate; and
4. unclaimed, explicitly delegated to it, or safe/idempotent to share.

Normal in-scope continuation includes claiming or acknowledging work,
diagnosing failures, running read-only checks and tests, addressing review
findings on an owned branch, batching/pushing owned corrections, publishing
required coordination, maintaining owned MR metadata, and advancing the next
already-authorized handoff. The agent does not ask the governor to repeat the
instruction, redispatch the item, or say “proceed.”

When more than one agent monitors the same branch or pipeline, one may own the
scheduled watcher/notification responsibility, but that designation is not an
exemption from any active agent's complete poll. Non-owners stay collision-safe
by observing and deferring claimed execution, not by narrowing their scan.

### 4. Monitoring is work

An agent that starts or depends on a pipeline, review, MR, release, or external
operation remains responsible for following the exact state to a reviewable or
terminal outcome. A pending/running healthy state is not a reason to ask the
human what to do next. Between complete broad polls, the agent may perform
narrow checks of already-monitored exact identifiers to reduce cost, but the
next scheduled complete cycle still scans every shared surface.

### 5. Exhaustive stop boundary

The rule uses “stop” to mean stop autonomous mutation and request or await the
named authority. The allowed stops are:

- Gate 1 approval;
- Gate 2 approval or merge;
- tag, release, or deployment authorization;
- scope expansion or materially conflicting instructions;
- destructive or irreversible action requiring authority;
- new role/permission authority;
- secrets, security, privacy, or legal ambiguity governed outside the current
  authority; or
- a concrete external blocker after safe in-scope diagnostics and reasonable
  alternatives are exhausted, with evidence and an owner recorded.

These are “genuine gates or equivalent authority boundaries.” Ordinary
uncertainty, waiting CI, an empty chat inbox, task difficulty, a desire for
confirmation, or the absence of another prompt are not stop conditions.
Polling never grants approval, merge, release, destructive-action, secrets, or
scope-expansion authority.

### 6. Empty authorized queue

If the complete poll proves there is no claimable authorized work, the agent
does not manufacture implementation authority. It records or proposes a small,
sourced, ranked candidate set through the existing improvement-proposal and
Gate 1 paths, without creating an acknowledgement obligation unless one is
actually intended.

This subsumes the useful behavior proposed by pending MR !1817
(`unassigned-work-discovery-rule`) under one lifecycle owner. Gate 1 approval of
this plan should authorize closing !1817 as superseded; it is not closed before
that decision. Its evidence and safeguards remain cited as design input.

### 7. Scheduling honesty and quiet notifications

A real assistant/orchestrator wake mechanism may implement the cadence. Shell
`sleep`, cron, detached processes, or timers the assistant cannot observe do
not count. A surface without a wake mechanism polls on activation/resume and
states that limitation.

Unchanged, healthy, pending, or otherwise non-actionable state stays quiet
unless periodic reports were explicitly requested. Agents notify on meaningful
completion, failure, material state change, or required human action. Universal
polling must not become universal notification spam.

## Files expected to change after Gate 1

- `agf/rules/always/universal-polling-and-autonomous-continuation-rule.md` —
  new canonical owner.
- `.claude/rules/always/universal-polling-and-autonomous-continuation-rule.md`
  — foundry Claude-loader projection naming the canonical owner.
- `agf/rules/README.md` — add the rule to the directory and principle maps.
- `agf/templates/AGENTS.md` — make the universal duty discoverable at cold
  start without copying the full rule.
- `agf/docs/site/docs/operate/poll-and-coordinate.md` — change applicability
  from “relevant” work to every active agent/all open items; add collision-safe
  execution and exhaustive stop semantics.
- `agf/docs/site/docs/operate/runbook.md` — replace the prompt-triggered “Asked
  to poll” entry with an always-on active-session situation.
- `agf/skills/poll-and-coordinate/SKILL.md` — align invocation behavior with
  the canonical rule and make clear that a named primary watcher does not
  narrow other agents' scans.
- `agf/rules/workflow/branch-and-worktree-rule.md` — reconcile primary watcher
  ownership with universal observation.
- `AGENTS.md` — project-local projection and pointer to the canonical shipped
  rule; preserve foundry's stricter complete GitLab loop.
- `docs/reference/agent-cold-start-habits.md` — align terminal and collision
  examples.
- `docs/reference/agf-development-starter.md` — strengthen the non-normative
  guide after MR !2369 merges.
- `docs/reviews/rule-projection/20260905T210255Z-universal-agent-autonomous-polling.yaml`
  — six-surface evidence.

No schema, Rust code, `agov-mcp` surface, CI gate, release metadata, version,
tag, or migration is changed. Any mechanical enforcement proposal is a new
gate/mechanism and returns to Gate 1 with explicit blocking authority and
negative tests.

## Dependency and collision disposition

- **MR !2369:** owns the starter guide and is already Gate-2-ready. This plan
  depends on !2369 merging first, then changes that file from fresh `main`.
  It must not edit or commandeer !2369's branch. If !2369 is rejected, this
  plan is revised before Gate 1 to remove or replace that projection.
- **MR !1817:** overlaps the empty-queue fallback. This plan proposes one
  broader canonical owner and preserves !1817's sourced-candidate/no-invented-
  authority safeguards. !1817 remains untouched until a binding Gate 1
  decision authorizes supersession.
- **MR !2027 / issues #121-#123:** session addressing, supersession fields,
  and collision discovery are complementary future mechanisms. This plan uses
  today's role addressing plus explicit session prose and manual collision
  checks; it does not assume those features exist.
- **v2.9 release chain:** remains the product-release priority. This governance
  plan may be reviewed while release pipelines/reviews run, but implementation
  must not take ownership of or delay the release branches.

## Projection screen

| Surface | Gate 1 disposition |
|---|---|
| Normative owner | **Planned:** new `agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`; no existing task-only rule covers pre-selection, waiting, and empty-queue states for every role. |
| Adopter-facing explanation | **Updated:** `agf/docs/site/docs/operate/poll-and-coordinate.md` explains the rule and links to it. |
| Runbook/situation map | **Updated:** `agf/docs/site/docs/operate/runbook.md`, the skill, and the starter/cold-start guides map cold start, scheduled wake, monitoring, and gate-stop situations. |
| AI landing/loaders | **Updated:** adopter `agf/templates/AGENTS.md`, foundry `AGENTS.md`, and the Claude projection point to the one owner. Other shipped example loaders retain existing generic rule-directory imports and are dispositioned in the manifest. |
| Validation/drift evidence | **Advisory only:** projection validation and assistant-drift validation check paths/surfaces; table-top cases test meaning. No validator claims to prove that an inactive external assistant actually wakes or acts. |
| Distribution | **Updated/verified:** canonical rule, template, skill, and Pages files must be present in the appropriate AGF distribution manifests; foundry-only projections are marked local. |

The structured projection manifest is intentionally not filed in this first
draft because its required starter-guide path does not exist on `main` until
!2369 merges. Before Gate 1, rebase onto that merge, add the manifest with all
six rows, and run the direct check without hiding a missing path. Gate 1 must
not be requested while this dependency or manifest remains unresolved.

## Test and acceptance strategy

Before Gate 1:

1. Claude planner returns a durable design challenge referencing `pm6m`; all
   blocking findings are resolved or explicitly escalated.
2. Fresh contribution-auditor and infosec-sentinel reviews cover the exact
   plan head; `advisory_consultation` is updated honestly.
3. MR !2369 is merged (or the affected-file/projection design is revised).
4. The structured six-surface manifest exists and `agov plan projection-check
   --allow-planned --base-ref origin/main <manifest>` passes.
5. `agov plan validate <plan>` and `git diff --check` pass.

After implementation:

1. Transition the manifest owner stage from `planned` to `existing`, then run
   strict `agov plan projection-check --base-ref origin/main <manifest>`.
2. `agov validate assistant-loader`, `agov validate adopter-docs`, the shipped
   skill validator, and `git diff --check` pass at the exact head.
3. Exact-head review checks at least these table-top cases:
   - every agent sees an unowned failing release pipeline; only the authorized
     release/build owner claims the fix, while the others avoid duplicate work;
   - an agent waiting on CI monitors it and continues at the next authorized
     state without asking “proceed?”;
   - an implementation reaches Gate 2 and the agent stops mutation for the
     governor decision while continuing read-only monitoring;
   - a role discovers an issue outside its authority and routes it without
     implementing it;
   - an empty authorized queue yields sourced candidates, not invented work;
   - a surface without a scheduler states the limitation and polls on resume;
   - unchanged healthy state produces no notification spam; and
   - a claimed item remains visible to every agent but is not executed twice.
4. Gate 2 evidence names the exact SHA and canonical pipeline.

## Risks and controls

- **Universal observation becomes universal execution.** Control: four-part
  authority/plan/gate/ownership eligibility test and immediate collision scan.
- **Authority inflation from “only stop at gates.”** Control: define equivalent
  authority boundaries and concrete blockers exhaustively; polling grants no
  new power.
- **Duplicate pipelines, comments, and claims.** Control: exact cursors,
  idempotency checks, one execution owner, and quiet unchanged-state policy.
- **Impossible scheduling claim.** Control: active-session definition and real
  wake-mechanism requirement.
- **Expensive full scans.** Control: compact cursors and narrow interim checks
  are allowed, but not as replacements for complete scheduled cycles.
- **Competing normative owners.** Control: one new `always/` rule, explicit
  supersession proposal for !1817, and projections that point back to the
  owner.
- **Release displacement.** Control: plan/review can proceed concurrently with
  waiting CI, but implementation does not seize release-owned branches or
  bypass their gates.

## Rollback

Revert the canonical rule and its projections together, restore the previous
runbook/skill wording, and remove the projection manifest. Do not reopen or
merge !1817 automatically; a separate governor decision reassesses the fallback
after rollback. No data, schema, or release artifact migration is required.
