# Proposal: Session Continuity and File-Backed Memory

## Status

Proposed.

## Context

Two practitioner references in `tmp/assess-8.txt` point at the same operating
problem from different angles:

- agent memory systems try to preserve useful facts across sessions through
  text blocks, vector/entity retrieval, temporal graphs, or extraction
  pipelines;
- context-management systems try to prevent long-running agents from losing
  important state during compaction, shutdown, or resume.

AGF-NG should not adopt an opaque memory subsystem as a core dependency. The
framework's strongest asset is already file-backed memory: coordination
messages, plan files, learning records, ADRs, and validator outputs. They are
reviewable, versioned, auditable, CI-checkable, and removable.

The gap is operational: agents need a disciplined closeout and reload loop so
that important state reaches those files before context is lost, and the next
session knows which files to read.

## Decision Direction

Use explicit file-backed session continuity, not hidden agent memory.

The core loop:

1. **Detect closeout risk.** A session is near compaction, shutdown, long pause,
   model switch, or handoff.
2. **Flush state to governed files.** The agent records pending work, branch,
   decisions, blockers, and next actions as coordination messages, plan updates,
   learning proposals, or completion notes.
3. **Make the flush visible.** If the message creates an obligation for another
   agent, it must be visible on that agent's polling branch or bridged by a
   visible note.
4. **Reload before acting.** The next agent/session reads the latest channel
   state, active handoffs, claimed artifacts, relevant plans, and learnings
   before committing new work.

Saving is only half the loop. Reloading is equally important.

## Why Not Graph Memory Now?

Graph or vector memory may become useful later for discovery over large
coordination histories, but it should not replace the audit trail.

Reasons to stay file-first:

- files are inspectable by humans and AI agents;
- git history preserves when state changed;
- validators can check structure deterministically;
- exit/decommission remains simple;
- branch visibility problems are explicit rather than hidden in a database;
- governance artifacts remain portable across assistant tools.

Future retrieval systems may index these files, but the files remain the
source of truth.

## Candidate Mix Commands

All tooling should be Elixir/Mix-first.

| Command | Purpose |
|---|---|
| `mix agov.session.closeout` | Generate a structured closeout note before compaction, shutdown, or handoff. |
| `mix agov.session.resume` | Print the minimum boot context for a role: pending handoffs, recent decisions, claims, dirty branch warnings. |
| `mix agov.session.guard` | Warn when session state is unsafe: dirty tree, hidden branch-local handoffs, unresolved claims, or missing completion. |
| `mix agov.handoff.snapshot` | Summarize a branch-local work item for another agent without requiring them to infer state from git log. |

These commands should compose existing readers where possible:

- `Agov.HandoffReader`
- `Agov.HandoffTracker`
- `Agov.ChannelReader`
- `Agov.ClaimTracker`
- future learning reader / validator modules

## Closeout Note Shape

A closeout note should answer:

- What branch/worktree was active?
- What was changed?
- What remains uncommitted or unmerged?
- Which coordination messages matter?
- Which plan or decision authorizes the work?
- What is blocked?
- What should the next agent read first?
- What must the next agent not touch?

Where possible, this should be an ordinary coordination message, not a special
sidecar file.

## Resume Output Shape

Resume output should be compact and actionable:

```text
Role: coder-tester
Branch: feat/example
Dirty tree: yes
Pending handoffs addressed to you: 2
Latest decisions: 00146, 00154
Open claims touching this branch: none
Must read:
  - docs/coordination/00154-coder-tester.md
  - docs/plans/example.md
Warnings:
  - coordination seq 153 exists only on plan/public-docs-customer-success-review
```

The goal is not to summarize the whole project. The goal is to prevent the
next session from starting with false context.

## Policy Hooks

This proposal reinforces several emerging learnings:

- L005: validated governance artifacts must be durable.
- L007: branch identity is volatile and must be checked at durable writes.
- L009: coordination messages must be visible to the branch/agent expected to
  consume them.

Potential rule:

> Before context compaction, long pause, shutdown, model switch, or cross-agent
> handoff, an agent with active work must either file a closeout coordination
> entry or explicitly state that no durable state needs preserving.

Potential complementary rule:

> Before acting after resume, an agent must read the current coordination
> status and any handoff/closeout entries addressed to its role.

## Scope Boundaries

This is not:

- a vector database;
- a knowledge graph;
- hidden long-term memory;
- a replacement for coordination messages;
- a model-training corpus format.

This is:

- a disciplined save/load loop over existing governed files;
- a small set of Mix tasks to make that loop easy;
- a bridge between human-readable audit trail and agent-readable session state.

## Proposed Gate 1 Slice

Create a Gate 1 plan for Slice S1:

1. Define closeout/resume message conventions.
2. Implement `mix agov.session.resume` read-only first.
3. Implement `mix agov.session.closeout --dry-run` to render a candidate note.
4. Add tests using fixture coordination channels with hidden branch handoffs,
   stale claims, and dirty branch warnings.
5. Document when agents must run closeout/resume.

Defer graph/vector indexing until there is enough coordination volume to
measure retrieval pain.
