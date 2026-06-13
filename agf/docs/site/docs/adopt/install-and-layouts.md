# Install the `agf/` directory

*Status: Provisional (v2.0 documentation). Audience: administrators and
developers installing the framework. After this page you have the files in
place and understand the single supported v2.0 layout.*

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
`agf/` directory from a checkout. Releases are tagged and immutable; pin the
version you adopt. The distribution manifest (`agf/docs/governance/manifest.md`
in v2.0) lists every file with its purpose and its ownership:
**upstream-owned** files are replaced on upgrade; **adopter-owned** files
(your codename's learnings, your customized rules) are never touched by an
upgrade.

## Step 2: Put AGF in one directory

AGF v2.0 supports one layout: copy the distribution into your repository as
the visible `agf/` directory.

| Artifact | Location |
|---|---|
| Framework directory | `agf/` |
| Agent definitions | `agf/agents/*.yaml` |
| Framework rules | `agf/rules/**/*.md` |
| Schemas | `agf/schemas/*` |
| Learning records | `agf/learnings/<your-codename>.yaml` |
| Coordination channel | `agf/docs/coordination/` or the project path you explicitly configure |
| Assistant rule install target | `.claude/rules/` or your tool's equivalent |

The assistant rule install target stays at the project root because tools read
their own configuration there. It is the exception, not a second framework
layout.

Mixed layouts are the documented failure mode: agents following path
instructions literally will create root-level `agents/`, `learnings/`, or
`docs/coordination/` directories unless the single `agf/` boundary is stated
where humans and agents can read it.

## Step 3: Copy and customize the rules

1. Copy `agf/rules/` into your assistant's rules directory
   (Claude Code: `.claude/rules/`).
2. Search the copied rules for `CUSTOMIZE` markers and adapt each one to
   your stack: locale set, migration commands, CI paths, audit functions,
   target consumers for generated files.
3. Delete rules that do not apply (a project with no UI does not need the
   accessibility rule). Deleting is better than ignoring — an inapplicable
   rule trains agents that rules are optional.

## Step 4: Adopt the agent definitions

Use `agf/agents/*.yaml`. The role definitions ship with explicit
jurisdiction and authority boundaries; start with the core pipeline roles
(planner-architect, coder-tester, governor-reviewer) and add the extended
roles only when the work warrants them — the
[quick reference](https://gitlab.com/janipaijanen/agent-governance)
(shipped at `agf/docs/reference/quick-reference.md`) lists which phase each
role belongs to.

## Step 5: Wire the validators

Add the validator jobs to CI so schema drift is caught per merge request.
Validators run in seconds; per the gate-self-test discipline, verify each
gate fires by feeding it a deliberately bad input once, on the MR that
introduces it.

## Step 6: Initialise your evidence

- Create `agf/learnings/<your-codename>.yaml` with your project codename.
  The codename is the only project handle that crosses the codebase
  boundary — see the anonymization rule before sharing anything upstream.
- Create the coordination channel path you will use and file your first entry:
  a note recording the adoption decision and the governor. Keep it under the
  `agf/` boundary unless you have an explicit project reason to configure a
  different path.

## Verify

- Your assistant lists the rules when asked what constraints apply.
- The validators pass on your fresh files.
- A trivial change still commits without ceremony (fast track works).

## Next

Run one real change through the gates:
[your first governed change](../start/first-governed-change.md).
