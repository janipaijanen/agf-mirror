# Install and layouts

*Status: Provisional (v2.0 documentation). Audience: administrators and
developers installing the framework. After this page you have the files in
place and know which layout you chose and why.*

## Prerequisites

- A git repository for the project you want to govern.
- An AI coding assistant that loads natural-language instructions
  (the reference implementation targets Claude Code; the rule content is
  tool-agnostic and has been tested with Cursor, Codex CLI, Copilot, and
  Windsurf).
- Python 3.x available if you want to run the validators locally and in CI.
- A human willing to act as governor — without one, stop here and re-read
  [should you adopt?](../start/should-you-adopt.md).

## Step 1: Get the distribution

Download the release archive from the project repository, or copy the
distribution directory from a checkout. Releases are tagged and immutable;
pin the version you adopt. The distribution manifest (`docs/governance/manifest.md`
in v2.0) lists every file with its purpose and its ownership —
**upstream-owned** files are replaced on upgrade; **adopter-owned** files
(your codename's learnings, your customized rules) are never touched by an
upgrade.

## Step 2: Choose a layout

| | Layout A — root | Layout B — subdirectory |
|---|---|---|
| Governance files live | At the repository root (`agents/`, `learnings/`, `GOVERNANCE.md`) | Under one directory (e.g. `governance/`) |
| Best for | Projects adopting governance as a first-class concern | Projects keeping governance visually separate from product code |
| Constant | `.claude/rules/` (or your tool's equivalent) stays at the root in both layouts — it is tool configuration, not a governance artifact | same |

Record the choice in your `GOVERNANCE.md`. Mixed layouts are the
documented failure mode: agents following path instructions literally will
re-create root directories in a Layout B project unless the layout is
stated where they can read it.

## Step 3: Copy and customize the rules

1. Copy `rules-template/` into your assistant's rules directory
   (Claude Code: `.claude/rules/`).
2. Search the copied rules for `CUSTOMIZE` markers and adapt each one to
   your stack: locale set, migration commands, CI paths, audit functions,
   target consumers for generated files.
3. Delete rules that do not apply (a project with no UI does not need the
   accessibility rule). Deleting is better than ignoring — an inapplicable
   rule trains agents that rules are optional.

## Step 4: Adopt the agent definitions

Copy `agents/*.yaml`. The twelve definitions ship with explicit
jurisdiction and authority boundaries; start with the core pipeline roles
(planner-architect, coder-tester, governor-reviewer) and add the extended
roles only when the work warrants them — the
[quick reference](https://gitlab.com/janipaijanen/agent-governance)
(shipped at `docs/reference/quick-reference.md`) lists which phase each
role belongs to.

## Step 5: Wire the validators

Add the validator jobs to CI so schema drift is caught per merge request.
Validators run in seconds; per the gate-self-test discipline, verify each
gate fires by feeding it a deliberately bad input once, on the MR that
introduces it.

## Step 6: Initialise your evidence

- Create `learnings/<your-codename>.yaml` with your project codename.
  The codename is the only project handle that crosses the codebase
  boundary — see the anonymization rule before sharing anything upstream.
- Create `docs/coordination/` and file your first entry: a note recording
  the adoption decision, the layout chosen, and the governor.

## Verify

- Your assistant lists the rules when asked what constraints apply.
- The validators pass on your fresh files.
- A trivial change still commits without ceremony (fast track works).

## Next

Run one real change through the gates:
[your first governed change](../start/first-governed-change.md).
