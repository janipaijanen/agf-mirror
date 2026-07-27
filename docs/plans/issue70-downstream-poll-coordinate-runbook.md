---
id: issue70-downstream-operations-runbook
status: approved
topic: issue70-downstream-operations-runbook
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-27T08:06:46Z"
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Review 20260727T080326Z-contribution-auditor-mtm2 returned ADAPT/BLOCK on the narrow plan; this revision incorporates its continuity, single-source, portability, role-authority, and Pages requirements."
  - role: fact-checker
    status: consulted
    reason: "Review 20260727T080327Z-fact-checker-dj7s checked current command capabilities and graceful/abrupt recovery claims; this revision incorporates its scenario and publication findings."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The plan documents existing recovery and read-only forensic checks; it adds no credential or network mechanism. The runbook nevertheless routes suspected credential exposure after a workstation incident to infosec-sentinel."
---

# Plan: downstream operations and continuity runbook for v2.4.5

## Trigger and revision

Governor-reviewer direction makes issue #70 critical downstream-experience
content for v2.4.5. The originating note is
`20260727T073930Z-governor-reviewer-4hrb` from MR !689.

This revision expands the initial poll-only plan after governor feedback and
role consultation. The deliverable is an overarching operations runbook. Poll
and coordinate is one linked procedure inside that journey; session continuity
is another, with separate paths for graceful checkpointing and abrupt loss.

## Current-state finding

Strong material exists, but in the wrong authority and delivery surfaces:

- `.claude/commands/compact-agf.md` has the fullest graceful closeout recipe,
  but it is foundry-local Claude configuration and is not shipped downstream;
- `docs/reference/session-resume.md` and
  `docs/reference/agent-cold-start-habits.md` explain resume and polling well,
  but root `docs/reference/` is not the adopter Pages tree;
- public `day-to-day`, real-situation, assistant-loader, tooling, coordination,
  review/release, install/upgrade, and exit pages contain partial procedures;
  and
- GitLab Pages has no single decision-oriented operations entry point or
  explicit abrupt-loss recovery procedure.

The existing `/compact-agf` adapter validates a local coordination file but
does not itself commit, push, or publish a bridge. Under the visibility rule,
that is only a draft for same-checkout recovery. The public contract must make
publication part of graceful closeout whenever another machine, person, or
agent may need the state.

## Information architecture and single source of truth

### Canonical source

`agf/docs/site/docs/operate/runbook.md` is the public, canonical,
decision-oriented operations map. Source Markdown under `agf/docs/site/**` is
authoritative; GitLab Pages is its rendered projection, not another copy.

The runbook links detailed canonical procedures rather than repeating them:

1. start or resume a session;
2. checkpoint, pause, compact, update the IDE, or reboot;
3. recover after abrupt context, process, IDE, network, or system loss;
4. poll and coordinate;
5. execute a governed change;
6. handle reviews, failed checks, and blocked gates;
7. commission or upgrade AGF and tooling;
8. release and verify artifacts; and
9. exit or decommission AGF.

Add focused public pages only where a unique procedure is missing,
provisionally:

- `agf/docs/site/docs/operate/session-continuity.md`; and
- `agf/docs/site/docs/operate/poll-and-coordinate.md`.

Existing detailed guides remain canonical for their domains. Root
`docs/reference/*`, `.claude/commands/compact-agf.md`, and assistant loaders
become concise internal/adaptor pointers plus genuinely surface-specific facts;
they must not carry competing full normative checklists.

### Authority map

The runbook describes responsibilities by capability and links to the adopter's
local role authority; it does not copy foundry's mutable role-status table:

| Capability | Typical role responsibility | Reserved decision |
|---|---|---|
| Structure work and recovery boundaries | planner-architect | none merely from planning |
| Reconstruct implementation state, test, and prepare handoff | coder-tester | no Gate 2 or merge |
| Check consolidation, precedent, and contribution provenance | contribution-auditor | advisory recommendation |
| Verify command/capability and documentation claims | fact-checker | advisory recommendation |
| Investigate possible credential exposure after workstation incidents | infosec-sentinel | blocks only where a named rule grants it |
| Approve Gate 1/Gate 2, merge, release, and resolve ambiguous authority | governor-reviewer | binding human decision |

Claude, Codex, IDE, or model names are provenance/adapters, never role
authority.

## Required procedures

### 1. Graceful interruption and checkpoint

Cover deliberate token-context cleanup, model/tool switch, `/compact`, IDE
update/reload, planned reboot/shutdown, long pause, and cross-agent handoff.

The tool-neutral closeout contract is:

1. stop new edits and identify active role, branch, worktree, and task;
2. capture UTC time, exact commit, dirty/untracked paths, completed changes,
   validation results, blockers/decisions, next authorized action, must-read
   references, and anything the next session must not assume;
3. write a `note` for same-operator continuity or a `handoff` when another role
   has a concrete obligation;
4. validate the coordination record;
5. commit and push it, or publish an approved visible bridge naming the branch
   or MR, whenever another machine/person/agent may resume;
6. report remaining dirty, uncommitted, local-only, or unpushed state; and
7. only then compact, update, restart, reboot, or hand off.

`/compact-agf` may be documented as a Claude adapter for this contract. It is
not an `agov` command, is not guaranteed downstream, and must be aligned so it
does not call local-only state durably published.

### 2. Abrupt-loss recovery without a checkpoint

Cover automatic compaction, IDE/process crash, killed terminal, power loss,
unexpected reboot, network loss, model/session disappearance, and resumption
on another machine.

Recovery begins read-only and must not fabricate lost state:

1. locate the intended checkout and all relevant worktrees;
2. inspect `git status`, branch, log, worktree list, and reflog where available;
3. preserve unknown user/agent changes—do not reset, overwrite, or stash them as
   a supposed shared handoff;
4. fetch/prune shared refs when transport is available and compare exact local
   and remote heads;
5. run unscoped then role-specific `agov session resume`;
6. inspect coordination status/stale/visibility/graph and pending handoffs;
7. inspect open changes/MRs, exact-head pipelines/jobs, discussions/comments,
   issues, and issue comments through the active forge adapter;
8. classify what is recoverable locally, visible remotely, or irretrievably
   unknown;
9. publish a recovery note naming evidence, gaps, ownership ambiguity, and the
   next authorized action; and
10. resume only already-authorized work, escalating unclear approval,
    ownership, security, or destructive recovery decisions.

Include explicit branches for:

- uncommitted work in a persistent checkout;
- a local commit not pushed;
- a pushed branch without a handoff;
- lost/unsaved IDE buffers;
- missing base branch or temporarily unavailable network; and
- a fresh clone on another machine, where local commits, stashes, worktrees,
  `tmp/`, overlays, terminal history, and unsaved buffers do not exist.

`agov session resume` is read-only. It does not fetch Git, discover every
remote branch/MR/comment, interpret dirty changes, approve work, or recover
reasoning that was never externalized.

### 3. Poll and coordinate

Define one complete cycle:

1. establish last-seen IDs, timestamps, SHAs, and monitored objects;
2. fetch/prune shared refs;
3. inspect coordination and branch visibility using supported Rust `agov`;
4. inspect changes/MRs, exact-head CI, discussions and comments;
5. inspect issues and issue comments;
6. classify by authority and urgency;
7. act within the current gate, split independent work with non-overlapping
   ownership, communicate progress, and publish canonical handoffs;
8. monitor running work through a reviewable or blocked terminal point; and
9. rearm only through a real assistant/orchestrator wake-up, or say polling
   resumes when the session is active.

Distinguish **poll** (observe), **coordinate** (act/assign/record/make visible),
and **approve/merge/release** (reserved authority never created by polling).

Current Rust `agov coord list --since` exists in the v2.4.4 source/release line
and may be documented after exact implementation-head and release-artifact
verification. `coord watch` remains future issue #36 work and must not be
claimed as shipped.

### 4. Forge and assistant adapters remain removable

Core procedures use generic surfaces: shared refs, change requests,
discussions/comments, tickets/issues, and CI jobs. Put GitLab UI/API/CLI
commands in a labeled adapter section. Put Claude `/compact-agf` and other
assistant affordances in labeled adapter sections.

Removing the GitLab or Claude adapter must leave the core checkpoint, recovery,
polling, authority, and visibility contracts coherent.

## GitLab Pages and distribution impact

- Add the runbook and focused pages to `agf/docs/build/mkdocs.yml` under
  **Operate** and make the runbook the section entry point.
- Link it from the public documentation index, day-to-day operations,
  real-situation guide, assistant-loader guidance, and relevant landing route.
- Update `agf/docs/site/content-plan.md` inventory/audience/objective as needed.
- Add the pages intentionally to the AI evaluation dossier `PAGE_ORDER` so
  generated review material identifies its source pages and does not become
  normative.
- Use existing Pages/site jobs from the canonical Markdown. Do not create
  separate Pages prose, generated normative copies, or role-status copies.
- Add release-note placement for v2.4.5 without claiming autonomous assistant
  scheduling or recovery of unsaved state.

## Expected implementation surfaces

- `agf/docs/site/docs/operate/runbook.md`
- `agf/docs/site/docs/operate/session-continuity.md`
- `agf/docs/site/docs/operate/poll-and-coordinate.md`
- `agf/docs/build/mkdocs.yml`
- `agf/docs/site/docs/index.md` and relevant existing guide links
- `agf/docs/site/content-plan.md`
- `agf/docs/build/hooks/ai_evaluation_dossier.py`
- `.claude/commands/compact-agf.md`, `docs/reference/session-resume.md`,
  `docs/reference/agent-cold-start-habits.md`, and `agf/templates/AGENTS.md`
  only to align adapter/pointer semantics and remove divergent checklists
- documentation command, link, nav, and dossier tests/fixtures as required
- v2.4.5 release notes/changelog placement

New CLI commands, schemas, background timers, daemons, webhooks, IDE plugins,
or hosted orchestration are outside scope.

## Acceptance criteria

1. A downstream operator can choose the correct existing guide from one public
   operations map without knowing foundry's internal file layout.
2. A fresh agent successfully resumes a planned checkpoint using only the
   published record, repository/forge state, and runbook.
3. An abrupt-loss tabletop reconstructs exact branch/head, dirty-state
   uncertainty, pending obligations, and next authorized action without chat
   memory and without inventing completion or approval.
4. Separate tabletop fixtures cover no checkpoint, dirty tree, local-only
   commit, remote branch without handoff, missing base/network, and fresh clone.
5. Graceful closeout is not called complete for cross-machine/person recovery
   until the record is remotely visible or visibly bridged.
6. Pages states `/compact-agf` is an optional Claude adapter and that local-only
   output is draft state; the adapter is corrected or explicitly warns before
   declaring compact readiness.
7. Every documented command is exercised against the supported Rust CLI and,
   where availability is claimed, the v2.4.4 or v2.4.5 release artifact.
8. The runbook never implies `session resume` polls MRs/comments/issues, saves
   dirty work, or grants authority.
9. Contribution, fact, security, and binding governor responsibilities are
   linked to local role authority rather than duplicated as mutable status.
10. GitLab- and Claude-specific adapter sections can be removed while the core
    runbook remains coherent.
11. A content audit finds no divergent normative operations checklist across
    Pages, templates, internal references, and assistant adapters.
12. MkDocs strict build, nav/orphan/internal/external link checks, command
    example checks, dossier inclusion/order, and a rendered navigation review
    pass at the exact Gate 2 head.
13. v2.4.5 release evidence confirms the Pages source is included downstream
    and the release note links the canonical runbook.

## Coordination and sequencing

- Claude playing planner-architect is requested to independently review the
  revised information architecture through handoff
  `20260727T080327Z-planner-architect-q3jq`; model identity is provenance, not
  authority.
- Human Gate 1 was approved on 2026-07-27. The requested independent Claude
  response remains welcome advisory input and must be considered if it arrives
  before Gate 2, but it is not a post-approval implementation blocker.
- Coder-tester implements and tests; contribution-auditor checks consolidation
  and single-source discipline; fact-checker verifies capability/command/Pages
  claims; governor-reviewer alone decides Gate 2 and merge.
- Implementation must merge before v2.4.5 content lock.
- This plan does not authorize version bumps, tags, release creation, or issue
  #36 tooling implementation.
