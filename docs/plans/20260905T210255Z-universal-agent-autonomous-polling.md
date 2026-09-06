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

## Claude design challenge disposition (2026-09-05)

Claude planner review `20260905T210846Z-planner-architect-gp1b` returned
REVISE with four blocking and four advisory findings. This revision accepts
B1-B4 and A2-A4. A1 is narrowed without exempting the governor: every active
development role still observes current shared state, including a governor
about to make a binding decision, while work selection and execution remain
role-specific. This preserves the governor's explicit “every agent must poll”
instruction without turning the governor into an implementation dispatcher.

The governor then added a second binding design requirement: when concurrent
Claude/Codex sessions' current roles do not fit the work requested by a durable
record, the affected sessions MUST reach durable consensus on a temporary role
allocation before mutation. Section 4 below specifies that handshake without
allowing peer consensus to manufacture new authority.

## Outcome

Make complete polling and autonomous continuation a normative obligation for
**every active AGF development agent/session**, not an optional response to the
word “poll,” a foundry-only habit, or a duty limited to the agent that opened a
particular MR.

Every active agent observes changes across the complete shared work surface,
with periodic full reconciliation. Execution remains
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

The rule applies to every **active** human or AI development role/session in an
AGF-governed project, including an active governor-reviewer before a binding
decision. “Active” means the session is executing, has resumed, or has a real
configured wake-up. It does not claim that an exited process or an assistant
surface without a scheduler continues running. Observation is universal;
selection and execution remain role-specific, so a governor observing current
state is not obliged or authorized to claim implementation work.

Every active agent performs a complete poll:

- before selecting work after cold start;
- whenever it resumes after inactivity or compaction;
- at each real configured wake while a monitoring window is armed; and
- before claiming an item when its last complete poll is stale relative to a
  newly observed coordination or forge event.

Scheduled polling is explicitly **non-reentrant**. A wake that becomes due
while the same agent/session is actively executing an authorized work item
MUST NOT interrupt that work or start a concurrent poll cycle. The runtime
coalesces one or more such wakes into a single pending poll. Immediately after
the active item reaches its next safe completion, handoff, gate, or evidenced
blocker boundary, the agent runs one complete poll before selecting or resuming
further work. “Deferred while busy” is not “skipped,” and accumulated timer
ticks never create parallel poll loops.

### 2. Complete observation surface with bounded cost

Each cycle fetches/prunes shared refs, then performs a cursor-based delta scan
across the **whole** remote-aware surface: coordination, all forge merge
requests/change requests and issues regardless of author or apparent
relevance, notes and discussions, assignments, labels, draft/approval/merge
state, source and target SHAs, pipeline/job changes, and every
branch/tag/release/runner or other exact external state the agent started or
awaits. “Delta” narrows transferred records by a durable last-seen timestamp,
ID, or SHA; it does not narrow which classes, authors, projects, or roles are
observed. APIs without reliable changed-since semantics must be paginated in
full.

A full paginated reconciliation of every currently open MR/issue and each
monitored exact head/pipeline/job is mandatory on cold start, post-compaction
resume, cursor loss or corruption, evidence of missed events, and at least once
per UTC day while the session remains active. This bounds routine work at
O(changed items) while making cursor blindness temporary and detectable. A
project may configure a shorter deep-sweep interval; it may not omit the daily
maximum without a governed local override that states the accepted risk.

Durable UTC/ID/SHA cursors are required so “nothing new” is evidence-backed.
The cursor lives in a verified git-ignored runtime path. Before compaction or
handoff, an agent records the cursor through the shipped `compact-agf` skill
when available, or an equivalent durable session-handoff note plus ignored
cursor file; the next session validates or conservatively discards it.

All agents observe everything; they do not all claim everything.

### 3. Collision-safe selection and action

After observation, each agent classifies visible items by priority, authority,
current gate, dependencies, and ownership. It checks for existing claims and
overlapping plans/MRs immediately before claiming or writing. Before the first
mutating action on an item, it MUST publish a `kind: claim` coordination record
that names the exact issue/MR/plan/artifact, branch or lane, and intended file
ownership. If another unresolved claim covers the same item or overlapping
artifacts, it MUST abort mutation and coordinate ownership. An existing
explicit handoff or claim to that exact lane may satisfy this requirement when
it is already visible and current; a chat statement or local-only file may not.
It then advances
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

### 4. Temporary role-allocation consensus

Before acting on a durable request, each session compares the record's
recipient and requested work with its currently declared role and the role
authority in `agf/agents/MANIFEST.yaml`. This section triggers when concurrent
sessions share an ambiguous role, the requested work fits a different already-
active role better, or their current allocation would create overlapping or
ownerless execution.

The affected sessions MUST agree on a temporary role allocation before either
performs the first mutating action. The handshake is:

1. one session publishes a `kind: note` or `kind: claim` referencing the
   triggering record and proposing the temporary role, exact task/item,
   artifacts or lane, authority basis, start condition, end condition, and
   role to resume afterward;
2. every affected concurrent session publishes an explicit `kind: ack`
   referencing that proposal and either accepts the same allocation or records
   a conflicting proposal;
3. the allocation becomes operative only when all affected sessions' visible
   acknowledgements agree and no unresolved overlapping claim exists; and
4. the temporary assignee publishes completion, handoff, or withdrawal when
   the end condition occurs, after which each session resumes its prior role.

“Consensus” means unanimous agreement among the affected execution sessions,
not silent non-response and not a same-session self-ack. If they disagree or an
affected session cannot be identified, mutation pauses and the conflict is
routed to the governor-reviewer with both proposals and the exact artifact
overlap. Read-only polling and evidence collection continue.

The handshake may allocate work only among roles already active in the
manifest and only within those roles' existing authority. It cannot activate a
suspended/skipped role, change manifest status, create a role, grant a gate or
merge/release power, or override the original record. Any such authority
change requires a governor decision. A direct governor assignment may settle
the allocation, but the affected sessions still acknowledge it so the
execution lane is unambiguous.

### 5. Monitoring is work

An agent that starts or depends on a pipeline, review, MR, release, or external
operation remains responsible for following the exact state to a reviewable or
terminal outcome. A pending/running healthy state is not a reason to ask the
human what to do next. Between complete broad polls, the agent may perform
narrow checks of already-monitored exact identifiers to reduce cost, but the
next scheduled delta cycle still covers every shared-surface class and the next
daily reconciliation remains mandatory.

### 6. Spare-capacity loop and visible decomposition

At every safe wait boundary — including a healthy pipeline, external review,
or another agent's claimed execution — the agent asks:

> What is the highest-priority authorized small item I can advance now without
> delaying, invalidating, or colliding with the work already in flight?

The question is a continuing loop, not a one-time cold-start prompt. The agent
must inspect the ranked queue and either claim one bounded item or record the
specific gate, authority boundary, collision, or absence of feasible work that
prevents it. Waiting on the primary item alone is not a stop condition.

The spare item must be independently reviewable, fit the role and current gate,
avoid the active branch/head and its acceptance pipeline, and not create a new
dependency for the primary item. Release-critical defect removal and already
approved roadmap work outrank unrelated research or polish. By default a
session carries at most one primary execution lane and one spare lane; it
finishes, hands off, or withdraws one before claiming another. This prevents
“stay busy” from becoming unbounded work-in-progress or pipeline spam.

When the highest-value authorized item is too large for the available window,
decomposition is itself the small work item. Before creating anything, scan the
issue board, open MRs, plans, and coordination for an existing decomposition.
If none exists, create or update the parent issue and a small set of linked,
ordered child issues/tasks on the issue board, and publish a coordination note
that references the parent and children. Each child states a concrete outcome,
bounded artifact scope, dependencies/current gate, acceptance evidence, and
whether it is unclaimed or already owned. Decomposition grants no implementation
or gate authority and must not fragment one atomic safety/release change into
invalid intermediate states.

On the next feasible poll cycle, the agent re-reads the remote issue/coordination
state. It may claim one child only if the child is still open, authorized,
unclaimed, and no other AI or human has responded with ownership or overlapping
work since the decomposition was filed. If another actor has responded or
worked it, the agent observes that state, avoids duplication, and selects the
next eligible item. A stale local decomposition is never execution authority.

### 7. All-agent capability-review trigger

Every active AI assistant tracks the learning-record cursor from the most
recent completed capability-review outcome and counts valid learning entries
added after it. At **10 new learnings**, the next complete poll or safe
spare-capacity cycle MUST propose running the capability-developer review. If
no review has been launched by **15 new learnings**, the assistant MUST publish
a visible handoff to the governor-reviewer naming the count, baseline review,
recurring themes observed, and proposed reviewer/role assignment.

The threshold is 10–15 **new** records since the last completed review, not the
repository's lifetime total. Human request, a release that changes agents, or a
material recurring escalation may trigger a review earlier. Before filing, the
assistant scans coordination, issues, plans, MRs, and active claims for an
existing capability-review proposal or run. One visible current proposal
satisfies every assistant's duty; other assistants acknowledge/observe it
instead of producing duplicates.

Crossing the threshold grants proposal authority only. It does not activate a
suspended role, assign a reviewer, approve a capability proposal, or authorize
implementation. Those remain manifest/governor decisions. A completed review
records its terminal learning cursor/count so the next 10–15 window is
deterministic.

The trigger must be startup-visible. Root `AGENTS.md`, the distributable
`agf/templates/AGENTS.md`, and every shipped specialist definition under
`agf/agents/*.yaml` (excluding manifests) receive the same compact obligation
and a pointer to this canonical rule. Specialist files do not duplicate the
algorithm. Claude continues to inherit root `AGENTS.md` through its one-line
loader; supported loaders must prove that path during assistant-loader
validation.

### 8. Exhaustive stop boundary

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

Before pausing mutation on any item, the agent MUST name which boundary above
applies and cite the missing decision, authority, conflict, or blocker evidence.
If it cannot name and evidence one, it MUST continue the next authorized action.
The pause applies to that item; unless a binding instruction stops the whole
session, the agent returns to the ranked queue and advances another authorized
item.

These are “genuine gates or equivalent authority boundaries.” Ordinary
uncertainty, waiting CI, an empty chat inbox, task difficulty, a desire for
confirmation, or the absence of another prompt are not stop conditions.
Polling never grants approval, merge, release, destructive-action, secrets, or
scope-expansion authority.

A concrete blocker requires one bounded diagnostic pass: reproduce or inspect
the current evidence, identify the failing boundary, and check at least one safe
in-scope alternative (or the task plan's stricter diagnostic budget). Repeating
the same failed action without changed evidence does not count. After that
bound, record exact evidence and an owner, pause the item, and continue the
queue; do not thrash indefinitely.

### 9. Empty authorized queue

If the complete poll proves there is no claimable authorized work, the agent
does not manufacture implementation authority. It records or proposes a small,
sourced, ranked candidate set through the existing improvement-proposal and
Gate 1 paths, without creating an acknowledgement obligation unless one is
actually intended.

This subsumes the useful behavior proposed by pending MR !1817
(`unassigned-work-discovery-rule`) under one lifecycle owner. Gate 1 approval of
this plan should authorize closing !1817 as superseded; it is not closed before
that decision. Its evidence and safeguards remain cited as design input.

### 10. Scheduling honesty and quiet notifications

A real assistant/orchestrator wake mechanism may implement the cadence. Shell
`sleep`, cron, detached processes, or timers the assistant cannot observe do
not count. A surface without a wake mechanism polls on activation/resume and
states that limitation.

The default armed window is `15mins/180min`: run a delta cycle every 15 minutes
and suspend the window after 180 continuous minutes without relevant activity,
resetting that idle window whenever relevant coordination or forge activity
appears. A project may configure a different governed cadence and idle window.
New activity or an explicit resume rearms the window. This is not an indefinite
background-work mandate.

Cadence measures when a poll becomes due, not permission to preempt active
work. A due tick during active execution sets one pending-poll latch; later
ticks coalesce into it. The pending poll runs immediately at the next safe work
boundary and then cadence continues from that completed cycle. The 180-minute
idle calculation uses relevant observed activity, not the number of coalesced
ticks.

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
- every shipped specialist role definition under `agf/agents/*.yaml`
  (excluding `MANIFEST.yaml` and `MANIFEST.example.yaml`) — add the compact
  startup pointer for the 10–15-learning capability-review trigger; no role
  receives new gate, activation, assignment, or implementation authority.
- `agf/docs/site/docs/operate/poll-and-coordinate.md` — change applicability
  from “relevant” work to every active agent/all open items; add collision-safe
  execution, temporary role-allocation consensus, and exhaustive stop
  semantics.
- `agf/docs/site/docs/operate/runbook.md` — replace the prompt-triggered “Asked
  to poll” entry with an always-on active-session situation and the recurring
  spare-capacity question at safe wait boundaries.
- `agf/skills/poll-and-coordinate/SKILL.md` — align invocation behavior with
  the canonical rule; make clear that a named primary watcher does not narrow
  other agents' scans; and require remote revalidation before claiming a
  decomposed child item.
- `agf/rules/workflow/branch-and-worktree-rule.md` — reconcile primary watcher
  ownership with universal observation.
- `AGENTS.md` — project-local projection and pointer to the canonical shipped
  rule; preserve foundry's stricter GitLab loop and add the Claude/Codex
  temporary-role handshake.
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

The role-file/startup-trigger addition changes required assistant behavior and
is therefore BREAKING under `downstream-impact-rule.md`, even though the new
polling mechanism itself is additive. Migration: adopters record the latest
completed capability-review learning cursor/count; when none exists, use the
current valid count as baseline and schedule one initial human-requested review
instead of emitting retroactive duplicate proposals. Local specialist files
receive the canonical pointer; suspended roles remain suspended until an
explicit manifest/governor decision.

## Dependency and collision disposition

- **MR !2369:** merged as `9ed176de7a7f2c6d379dc327f8c7708acfb8bbc8`.
  This branch incorporated that merge before adding its structured projection
  evidence. The later implementation changes the starter guide from fresh
  `main`; it does not edit or commandeer !2369's historical branch.
- **MR !1817:** overlaps the empty-queue fallback. This plan proposes one
  broader canonical owner and preserves !1817's sourced-candidate/no-invented-
  authority safeguards. !1817 remains untouched until a binding Gate 1
  decision authorizes supersession.
- **MR !2027 / issues #121-#123:** session addressing, supersession fields,
  and collision discovery are complementary future mechanisms. This plan uses
  today's role addressing plus explicit session prose and the mandatory
  pre-mutation published claim and temporary-role consensus disciplines; it
  does not assume those features exist. When #121 lands, the claim and
  acknowledgement lane identities migrate to that mechanism.
- **v2.9 release chain:** remains the product-release priority. This governance
  plan may be reviewed while release pipelines/reviews run, but implementation
  must not take ownership of or delay the release branches.
- **`20260905T202547Z-planner-architect-mmm3`:** its post-v2.9 sequencing is
  superseded for this universal-polling item only by the governor's later
  explicit high-priority instruction. It continues to govern the remaining
  coordination-protocol/status-machine work. This plan does not silently treat
  the earlier instruction as absent.

## Projection screen

| Surface | Gate 1 disposition |
|---|---|
| Normative owner | **Planned:** new `agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`; no existing task-only rule covers pre-selection, waiting, and empty-queue states for every role. |
| Adopter-facing explanation | **Updated:** `agf/docs/site/docs/operate/poll-and-coordinate.md` explains the rule and links to it. |
| Runbook/situation map | **Updated:** `agf/docs/site/docs/operate/runbook.md`, the skill, and the starter/cold-start guides map cold start, scheduled wake, monitoring, and gate-stop situations. |
| AI landing/loaders | **Updated:** adopter `agf/templates/AGENTS.md`, foundry `AGENTS.md`, and the Claude projection point to the one owner. Other shipped example loaders retain existing generic rule-directory imports and are dispositioned in the manifest. |
| Validation/drift evidence | **Advisory only:** projection validation and assistant-drift validation check paths/surfaces; table-top cases test meaning. No validator claims to prove that an inactive external assistant actually wakes or acts. |
| Distribution | **Updated/verified:** canonical rule, template, skill, and Pages files must be present in the appropriate AGF distribution manifests; foundry-only projections are marked local. |

The structured projection manifest is filed at
`docs/reviews/rule-projection/20260905T210255Z-universal-agent-autonomous-polling.yaml`
after incorporating !2369. Gate 1 evidence must include the direct
`--allow-planned` projection check; Gate 2 transitions the new owner from
`planned` to `existing` and runs the strict check.

## Test and acceptance strategy

Before Gate 1:

1. Claude planner returns a durable design challenge referencing `pm6m`; all
   blocking findings are resolved or explicitly escalated.
2. Fresh contribution-auditor and infosec-sentinel reviews cover the exact
   plan head; `advisory_consultation` is updated honestly.
3. MR !2369 is merged and incorporated before the structured projection
   manifest is validated.
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
     release/build owner publishes the exact-item claim and mutates, while the
     others observe the claim and avoid duplicate work;
   - an agent waiting on CI monitors it and continues at the next authorized
     state without asking “proceed?”, while using a safe wait boundary to ask
     and answer the recurring spare-capacity question;
   - a primary pipeline remains in flight while the agent claims one unrelated,
     bounded, authorized spare item on a separate lane; it neither amends the
     pipeline's exact head nor starts duplicative acceptance CI;
   - a large top-ranked item cannot fit the current window, so the agent first
     collision-scans and then creates a linked parent/child issue-board
     decomposition plus coordination note; each child has scope, dependency,
     gate, acceptance, and ownership fields;
   - on the next feasible cycle another AI has responded to or claimed one
     child; the agent does not execute it and selects the next still-open,
     authorized, unclaimed child instead;
   - proposed decomposition would split an atomic release/safety correction
     into invalid intermediate states; the split is rejected and the atomic
     unit is preserved;
   - a session already has a primary and spare lane; it does not claim a third
     item merely to appear active;
   - the tenth new learning since the last completed capability review causes
     one assistant to publish a review proposal; other assistants find it and
     do not duplicate it;
   - no review has launched by the fifteenth new learning, so the next active
     assistant publishes the mandatory governor handoff with the baseline,
     count, recurring themes, and proposed assignment;
   - an assistant attempts to treat the threshold as permission to activate a
     suspended capability-developer or implement its proposal; the action is
     rejected because the trigger grants proposal authority only;
   - a completed capability review stores its terminal learning cursor/count,
     and the next threshold calculation begins after that baseline rather than
     using the lifetime total;
   - an implementation reaches Gate 2 and the agent stops mutation for the
     governor decision while continuing read-only monitoring;
   - a role discovers an issue outside its authority and routes it without
     implementing it;
   - an empty authorized queue yields sourced candidates, not invented work;
   - a surface without a scheduler states the limitation and polls on resume;
   - a routine cycle uses whole-surface deltas while a cold start and daily deep
     sweep reconcile all currently open items and monitored exact heads;
   - a proposed pause without a named/evidenced boundary is rejected and the
     agent continues the next authorized action;
   - after one bounded diagnostic/alternative pass, a blocked item is recorded
     and the agent advances another item rather than thrashing or stopping;
   - 180 idle minutes suspend the default window and a new relevant event
     rearms it;
   - a 15-minute tick arrives during active implementation; no overlapping
     poll starts, repeated ticks coalesce, and exactly one complete poll runs
     immediately when the implementation reaches its next safe boundary;
   - unchanged healthy state produces no notification spam; and
   - a claimed item remains visible to every agent but is not executed twice;
   - Claude and Codex both start as planner-architect for a coder-tester
     implementation request; they publish one proposed temporary allocation,
     both acknowledge it, only the agreed coder-tester lane mutates, and both
     resume their prior roles at the recorded end condition;
   - two sessions disagree on the temporary allocation; neither mutates, both
     keep polling read-only, and the exact conflict is routed to the governor;
   - two peer sessions agree to use a suspended or unauthorized role; the
     handshake is rejected because consensus cannot create authority; and
   - a governor directly assigns an already-active temporary role; affected
     sessions acknowledge the binding allocation rather than negotiating a
     conflicting one.
4. Gate 2 evidence names the exact SHA and canonical pipeline.

## Risks and controls

- **Universal observation becomes universal execution.** Control: four-part
  authority/plan/gate/ownership eligibility test and immediate collision scan.
- **Peer consensus manufactures authority.** Control: temporary allocation is
  limited to already-active roles and existing authority; manifest/status or
  gate-power changes still require the governor.
- **Temporary roles become permanent or ambiguous.** Control: every proposal
  names exact scope, start, end, and resume role; completion/handoff/withdrawal
  closes the allocation.
- **Authority inflation from “only stop at gates.”** Control: define equivalent
  authority boundaries and concrete blockers exhaustively; polling grants no
  new power.
- **Duplicate pipelines, comments, and claims.** Control: exact cursors,
  idempotency checks, one execution owner, and quiet unchanged-state policy.
- **Impossible scheduling claim.** Control: active-session definition and real
  wake-mechanism requirement.
- **Expensive full scans.** Control: whole-surface delta cycles, full
  reconciliation at cold start/resume/cursor failure and at least daily, true
  pagination where delta semantics are unavailable.
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
