---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260723T001947Z-contribution-auditor-hhmm
timestamp: 2026-07-23T00:19:47Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T001812Z-contribution-auditor-27kr.md
  - 20260723T001504Z-coder-tester-rrrk.md
topic: codex-operating-guidance-proposal
requires-ack: true
artifacts_owned: []
---

# Supplement: proposed instructions for running Codex efficiently

The governor requested a concrete Codex counterpart after review `27kr`.
This is planning input for MR !436's required Gate 1 path, not authority to add
an ungated page. It is based on the current official Codex manual's Best
practices, Speed, and Subagents sections plus this repository's observed
incidents.

## Proposed Codex guidance

### 1. Start with the right surface and scope

- Use Codex CLI for terminal-first local repository work, the IDE extension
  for editor-attached work, and the Codex app for interactive planning and
  review. State the task as **goal, context, constraints, and done-when**.
- For difficult or ambiguous work, use Plan mode (`/plan` where supported) or
  explicitly ask Codex to plan and challenge assumptions before editing.
- Start a new thread for genuinely unrelated work instead of carrying a large
  task history forward. Treat an AGF class/slice or completed Gate 2 cycle as a
  natural thread boundary; do not invent Claude `/compact` or `/clear`
  equivalents that Codex does not document for the active surface.

### 2. Put durable behavior in the correct place

- Put repository layout, commands, constraints, and verification expectations
  in `AGENTS.md`; closer nested files govern their subtree.
- Put trusted-repository Codex settings in `.codex/config.toml` and personal
  defaults in the user-level Codex configuration. Keep one-off task constraints
  in the prompt rather than permanently expanding `AGENTS.md`.
- When the same mistake recurs, perform a retrospective and strengthen the
  procedure or `AGENTS.md`; do not rely on a memory-only reminder.

### 3. Control cost before tools run

- Keep Fast mode off unless latency is worth its higher credit consumption;
  inspect it with `/fast status` and disable it with `/fast off` where those CLI
  commands are available.
- Batch independent read-only checks and summarize large outputs. Use `agov`
  summaries before manually reading full coordination histories, but inspect
  raw evidence when a binding decision depends on details the summary omits.
- Cap speculative debugging before starting: state the number of approaches or
  the time/tool-call budget, then disclose the unresolved limitation and stop
  when the cap is reached.
- Poll only on request, on a meaningful expected event, or at a declared
  interval. Do not claim that a shell process will wake an inactive assistant.

### 4. Use subagents selectively

- Delegate only independent work that can run in parallel and return a compact
  result; subagents consume additional tokens and are not a free context reset.
- Keep requirements, authority decisions, and final synthesis in the main
  thread. Do not delegate a sequential read-edit-verify chain merely to appear
  parallel.
- In this repository, use subagents only when the user or applicable
  `AGENTS.md`/skill instructions explicitly request them.

### 5. Preserve safety and review boundaries

- Start with default sandbox and approval settings; loosen them only for a
  trusted repository and a demonstrated need.
- Before every remote push, classify the diff and state the expected pipeline
  count. Coordination-only commits use the exact `coordination:` subject plus
  `[ci skip]`; implementation acceptance commits must not skip CI.
- Ask Codex to run the relevant tests, inspect the final diff, and verify the
  requested behavior. Use `/review` where supported for uncommitted changes, a
  commit, or base-branch review, while remembering that AGF Gate 2 approval
  remains a governor decision.

### 6. AGF development runbook for Codex

Use this sequence for work in the AGF-NG repository:

1. **Cold start from the repository root.** Read `AGENTS.md`, then
   `ai-cold-start.md` when present. Set the repo-local tooling path and the
   Codex-specific token-ROI log. Run `agov session resume --base-branch main`
   first, then repeat with the governed role actually being played.
2. **Confirm the live obligation before reading broadly.** Use `agov handoff
   next <role>`, `agov coord status --recent 30 --roles`, and `agov coord
   visibility --base-branch main`. Read the named `must_read` files and recent
   notes addressed to the role; do not treat a stale local branch or an old
   handoff as current authority.
3. **Respect gates before implementation.** For non-trivial work, verify that
   the relevant plan exists, validates, and has a governor decision approving
   Gate 1. A user request can identify desired work, but implementation still
   needs the durable governed authority required by this repository.
4. **Isolate work correctly.** Branch from current `origin/main` and create
   linked worktrees only under `.worktrees/<purpose>`. Before editing, check
   ownership and existing worktrees so two agents do not modify the same
   artifact or duplicate an active slice.
5. **Use `agov` before raw-channel exploration.** Prefer role resume, handoff,
   status, visibility, and validators for deterministic summaries. Open the
   raw coordination record when reviewing its reasoning, resolving ambiguity,
   or producing a binding Gate 1/Gate 2 assessment.
6. **Work locally to a coherent checkpoint.** Batch the complete approved MR
   slice, run focused tests during development, then run the required full
   validation once per meaningful candidate rather than after every trivial
   edit. For visual artifacts, batch browser renders after the logical edit set
   and inspect every required viewport/theme named by the plan.
7. **Publish with a pipeline prediction.** Immediately before each push, state
   the diff class, exact expected pipeline records, and whether paid jobs are
   intended. For a new implementation branch, create the skipped seed and MR
   boundary before the first real acceptance push. For coordination-only work,
   use `coordination: ... [ci skip]` and verify the resulting records are
   skipped.
8. **Make reviews exact-SHA and complete.** Query every pipeline for the
   implementation SHA, not only the MR widget. Record IDs, sources, statuses,
   retries, and duplicate graphs. A scoped CLEAR does not clear unrelated open
   BLOCK findings, and neither CLEAR nor a green pipeline is governor approval.
9. **Publish coordination before bridge notes.** Generate IDs with `agov coord
   brief`, validate the immutable record, commit and push it to a visible
   branch/MR, then add a short GitLab note naming the record and commit. Do not
   create the downstream obligation in chat or an MR note first.
10. **Poll as an action loop.** Scan merge requests, MR notes, issues and issue
    comments, plus published coordination/visibility. Resolve or advance the
    highest-priority authorized obligation; do not merely list it. If the next
    action belongs to the governor or another role, state the exact blocker and
    monitor only when active or resumed.
11. **Close the slice cleanly.** After merge or handoff, verify the published
    state, release owned artifacts with the appropriate completion record, and
    start unrelated work in a fresh thread. Delete obsolete local branches only
    after confirming they contain no unique work and no unpublished evidence.

## Recommended documentation shape

Do not add a second long page that duplicates the Claude retrospective.
Instead, after Gate 1 approval:

1. retain a short Claude-specific subsection for `/compact`, `/clear`, its
   usage panel, and Claude scheduling mechanisms;
2. add the concise Codex subsection above for Plan mode, `AGENTS.md`, Codex
   configuration, `/fast`, `/review`, and bounded subagent use;
3. keep shared principles once: clean task boundaries, bounded debugging,
   batched verification, evidence-aware polling, and procedural controls;
4. label product commands as surface-dependent and link to the current official
   Codex Best practices and Speed documentation rather than freezing volatile
   product details as AGF requirements.

Official source basis:

- <https://learn.chatgpt.com/guides/best-practices.md>
- <https://learn.chatgpt.com/docs/agent-configuration/speed.md>
- <https://learn.chatgpt.com/docs/agent-configuration/subagents.md>
