---
id: issue70-downstream-poll-coordinate-runbook
status: pending-gate1
topic: issue70-downstream-poll-coordinate-runbook
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: "This is a maintainer-directed documentation consolidation with no contribution precedent or third-party attribution change."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The runbook documents existing read-only polling and governed handoffs; it adds no credential, network, or cross-record data flow."
---

# Plan: downstream poll-and-coordinate runbook for v2.4.5

## Trigger and release placement

Governor-reviewer direction makes issue #70 critical downstream-experience
content for v2.4.5. The originating note is
`20260727T073930Z-governor-reviewer-4hrb` from MR !689.

The internal `docs/reference/agent-cold-start-habits.md` already contains a
substantial poll contract, including intake scope, timer semantics,
act-without-reprompt boundaries, and honest assistant wake-up limitations.
The defect is not absence of all content: it is that this contract is outside
the adopter-facing documentation set, dispersed across adjacent pages, and not
presented as a literal supervising-session procedure.

The note suggested `agov coord list --since`, but that command is planned in
issue #36 and is not part of the current supported Rust CLI. v2.4.5 documentation
must not present it as available. This plan uses current commands and leaves a
clean insertion point for issue #36 after that feature ships.

## Scope

### 1. Publish one canonical adopter-facing runbook

Add an adopter-facing operations page, provisionally
`agf/docs/site/docs/operate/poll-and-coordinate.md`, and include it in the
MkDocs navigation.

The page must define one complete cycle:

1. establish the baseline and last-seen state;
2. fetch/prune shared Git refs before comparing them;
3. inspect coordination and branch visibility with supported Rust `agov`
   commands;
4. inspect open MRs, exact-head pipelines/jobs, MR discussions and comments;
5. inspect open issues, issue state changes, and issue comments;
6. classify each new item by authority and urgency;
7. act within the current gate, split independent work when useful, publish
   progress and canonical handoffs, and make obligations visible;
8. record blockers or exact review heads; and
9. rearm the next cycle only through a real assistant/orchestrator wake-up
   mechanism, or state plainly that polling resumes when the session is active.

The runbook must distinguish:

- **poll:** observe and compare shared state;
- **coordinate:** claim or assign authorized work, communicate progress,
  publish/validate handoffs, and monitor work to a terminal review point; and
- **approve/merge/release:** separate governor authorities never created by a
  poll request.

### 2. Provide current, executable examples

- Use only commands available in the supported Rust `agov` surface at the
  implementation baseline (`session resume`, `coord status`, `coord stale`,
  `coord visibility`, `coord graph-index`, and `handoff next` as applicable).
- Show GitLab/API or UI checks as platform examples without making GitLab a
  framework requirement.
- Do not document Elixir/Mix fallback or a not-yet-released `coord list
  --since` command.
- Mention issue #36 only as a future incremental-reader enhancement; the
  operational loop works without it.

Every literal repository command must be exercised against the implementation
head or covered by an existing command-example validation mechanism. Platform
examples that require credentials must be clearly marked and must not embed
tokens, private endpoints, or maintainer-specific paths.

### 3. Connect AI-facing and human-facing entry points

- Add a concise link and summary in the adopter assistant-loader guidance so
  an AI assistant receives the same canonical loop contract.
- Link from the relevant day-to-day operations page for a human supervising an
  agent session.
- Reconcile overlapping polling prose in coordination-protocol and assistant-
  loader pages by linking to the runbook; retain only context-specific details
  locally.
- Keep `docs/reference/agent-cold-start-habits.md` as project-development
  guidance, but point it to the adopter contract or explicitly state any
  foundry-only additions. Do not maintain two divergent normative checklists.

### 4. Make the downstream behavior unambiguous

The runbook must explicitly cover these anti-patterns:

- checking only the local coordination directory without fetching or scanning
  branch/MR visibility;
- polling MRs without their comments/discussions, or issues without comments;
- reporting an actionable obligation while leaving authorized work untouched;
- treating an open/local-only coordination file as delivered;
- claiming an internal timer exists when the assistant cannot self-wake;
- delegating overlapping work without file/authority boundaries;
- merging past Gate 2 or treating discovery as approval; and
- retrying paid CI merely to obtain green status without diagnosing failure.

## Expected implementation surfaces

- `agf/docs/site/docs/operate/poll-and-coordinate.md` (new canonical page)
- `agf/docs/build/mkdocs.yml` (navigation)
- `agf/docs/site/docs/operate/day-to-day.md`
- `agf/docs/site/docs/adopt/assistant-loader-configuration.md`
- `agf/docs/site/docs/concepts/coordination-protocol.md` as needed to remove
  duplication and link the canonical procedure
- `docs/reference/agent-cold-start-habits.md` for the foundry/adopter boundary
- documentation command/link validation fixtures if required

Exact prose files may narrow during implementation. Adding new CLI commands,
schema behavior, timer infrastructure, or remote orchestration is out of scope
and requires its own approved plan.

## Acceptance criteria

1. A downstream reader can execute one complete poll-and-coordinate cycle from
   a literal checklist without relying on session-specific tribal knowledge.
2. Coordination, MRs, MR comments/discussions, issues, and issue comments are
   all named intake surfaces.
3. The page states when to act autonomously and when human Gate 1, Gate 2,
   merge, release, security, or scope authority is required.
4. The page explains safe work splitting, progress communication, exact-head
   handoffs, visibility, and monitoring through review readiness.
5. Every documented `agov` invocation exists in the Rust CLI at the exact
   implementation head and passes its example validation or a recorded manual
   execution check.
6. No page claims `coord list --since` or `coord watch` is currently shipped.
7. MkDocs strict build, internal-link checks, governance validation, and the
   relevant GitLab pipeline pass at the Gate 2 head.
8. v2.4.5 release notes identify the runbook as downstream operational guidance
   without claiming the framework can autonomously schedule assistant wake-ups.

## Sequencing and exclusions

- Implementation begins only after human governor-reviewer Gate 1 approval.
- The documentation implementation must merge before the v2.4.5 content lock.
- This plan does not implement issues #32, #33, or #36.
- It does not authorize version bumps, tags, release creation, background
  timers, webhooks, daemons, or hosted orchestration.

