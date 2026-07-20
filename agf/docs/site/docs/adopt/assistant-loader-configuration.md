<!-- audience: A (ai-operator); answers: "which docs are meant for AI assistants to execute while humans supervise" -->
# Assistant loader configuration

*Status: Released (v2.1 documentation). Audience: AI assistants preparing AGF
rule loading, with developers and administrators supervising the setup. After
this page you can make the active assistant see AGF, verify that it did, and
avoid copying the framework rules into tool-specific directories.*

## Why this matters

AGF only works when the assistant doing the work can read the framework files.
The repository can contain perfect rules, plans, schemas, and role definitions;
an isolated web chat or misconfigured IDE assistant will still behave as if AGF
does not exist.

Configure rule loading before the first governed change. Treat this as an
adoption gate: if the active assistant cannot name the AGF files it loaded,
stop and fix the loader before relying on any governance behavior.

## Loader contract

Use a small tool-specific file that points the assistant at AGF. Do not copy
the AGF rule tree into `.claude/rules/`, `.cursor/rules/`, `.github/`, or any
other tool directory.

The loader should say three things:

- AGF lives in `agf/`.
- The assistant must read `agf/rules/`, `agf/agents/`, and the relevant
  adopter-owned paths (`docs/plans/`, `docs/coordination/`) before governed
  work.
- The assistant must not invent approvals; Gate 1 and Gate 2 decisions belong
  to the governor-reviewer.

Keep the loader short. The canonical rules stay in `agf/` so upgrades replace
one framework directory instead of chasing copied fragments across tool
configuration paths.

## Common loader patterns

| Assistant surface | Loader pattern | Verification |
|---|---|---|
| Claude Code | Use project `CLAUDE.md` or `.claude/CLAUDE.md`. If you already use `AGENTS.md`, a `CLAUDE.md` file can import it with `@AGENTS.md`; Claude Code also supports `@path/to/import` for other files. | Run `/memory` and confirm the expected `CLAUDE.md` file is listed, then ask the assistant to name the AGF rules it will follow. |
| Cursor | Use Project Rules or `AGENTS.md`, depending on your project convention. The loader should point to `agf/` rather than duplicate `agf/rules/`. | Ask Cursor which project rules or `AGENTS.md` instructions it loaded and which AGF paths it inspected. |
| VS Code Copilot / GitHub Copilot | Use repository custom instructions such as `.github/copilot-instructions.md`; newer Copilot surfaces also support repository `AGENTS.md` files and path-specific instructions. | In VS Code, check the response references for the custom-instructions file; in GitHub, confirm the repository context includes the instruction file. |
| Codex CLI and other `AGENTS.md` readers | Put a concise loader in the repository root `AGENTS.md` or adapt the existing one. | Ask the assistant to restate the worktree boundary, gate discipline, and AGF-owned paths before it edits. |
| Isolated web chat | No automatic local loader. Attach or paste the relevant AGF files for that session, or do not treat the session as governed work. | Ask which files it has actually read; if the answer is only conversation text, it has not loaded the repository rules. |

Tool vendors change their instruction-loading mechanisms. Prefer the vendor's
current documentation for exact file names and settings, then keep the AGF
loader contract above intact.

Current vendor references:

- [Claude Code memory and `CLAUDE.md`](https://code.claude.com/docs/en/memory)
- [Cursor rules](https://cursor.com/docs/rules)
- [GitHub Copilot repository custom instructions](https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/add-custom-instructions/add-repository-instructions)
- [VS Code Copilot custom instructions](https://code.visualstudio.com/docs/agent-customization/custom-instructions)

## Minimal loader text

Adapt this to the file your assistant reads:

```markdown
# Project AI instructions

This repository uses agents-govern (AGF).

Before governed work, read:

- `agf/agents/` for role authority;
- `agf/rules/` for project governance rules;
- `agf/schemas/` for validated artifact shapes;
- `docs/plans/` for Gate 1 plans;
- `docs/coordination/` for handoffs, approvals, reviews, and decisions.

Do not copy AGF rules into this tool directory. Treat this file as a loader
that points to the canonical framework files under `agf/`.

Do not invent approvals. Gate 1 and Gate 2 decisions belong to the human
governor-reviewer and must be recorded in `docs/coordination/`.

Before editing, state which AGF files you read and which gate applies.
```

## Local cold-start overlay

Some repositories need local reminders that should not become governed project
instructions: workstation paths, current operator preferences, assistant
surface quirks, or temporary notes about how this machine is configured. Use an
optional repository-root `ai-cold-start.md` file for that local state.

Read order:

1. Read the governed loader, such as `AGENTS.md` or `CLAUDE.md`.
2. If `ai-cold-start.md` exists at the repository root, read it as a local
   overlay.
3. Run the normal cold-start command, such as `agov session resume`, or run it
   alongside the overlay read if your assistant surface makes that easier.

The overlay is subordinate to governed sources. It may add local paths,
machine quirks, current operator reminders, or assistant preferences. It must
not override gate discipline, coordination rules, role authority, product
scope, or any governor-reviewer decision. Anything another agent or human must
see belongs in the shared coordination channel, not in a local overlay.

The file should be gitignored by default so fresh clones do not contain local
state. Projects may track a template or short pointer if that helps adopters,
but the actual `ai-cold-start.md` file should remain optional and local unless
the project deliberately chooses otherwise.

Minimal example:

```markdown
# Local AI cold start

- Use `/home/example/src/project` as the canonical local checkout.
- Prefer the local release build at `tooling/rust/target/release/agov`.
- When polling is requested, use the documented `15mins/180min` cadence.

This file is local. It does not override AGF gates, coordination records,
role authority, or governor-reviewer decisions.
```

## Optional polling timer guidance

Some teams ask assistants to poll for coordination or merge-request activity
while a human is away. If you use timer shorthand in your loader or local
instructions, define it explicitly. For example, `15mins/180min` should mean:

- scan every 15 minutes for new coordination messages and merge-request
  activity;
- stop after 180 minutes with no new incoming activity;
- reset the 180-minute idle window whenever new relevant activity appears.

For AI assistants, this should be an internal reminder or wake-up in the active
assistant surface. Do not tell the assistant to start a background shell timer,
cron job, detached `sleep`, or other external process unless you actually want
an external monitor; the assistant may not notice it. If the assistant cannot
schedule internal wake-ups, it should say so and scan when it is active or
resumed.

If you are upgrading from an older copied layout, remove AGF-provided copies
from `.claude/rules/`, `.claude/skills/`, or similar tool directories. Keep a
small loader instead. Intentional project-specific assistant instructions can
stay local, but keep them outside copied AGF rule or skill trees so future
upgrades do not mistake them for upstream framework content.

For Claude Code, a project `CLAUDE.md` can import a shared `AGENTS.md`:

```markdown
@AGENTS.md

## Claude Code

Use the AGF loader instructions above before governed work.
```

For Copilot repository custom instructions, put the minimal loader text in
`.github/copilot-instructions.md` or point that file to the canonical AGF
paths. Do not paste the full rule library into the Copilot file.

## Verify the loader

Before the first governed change, ask:

```text
Which AGF files did you load for this repository?
List the role authority file, the rule directory, the coordination path, and
the plan path. Then name the gate that applies before implementation starts.
If you have not read those files, stop and say what loader is missing.
```

Good answer:

- names `agf/agents/`, `agf/rules/`, `docs/plans/`, and
  `docs/coordination/`;
- says Gate 1 comes before implementation and Gate 2 before merge;
- says the governor-reviewer approves gates;
- admits uncertainty when it has not read a file.

Bad answer:

- gives generic review advice without naming AGF paths;
- claims the assistant can approve its own work;
- says the rules are loaded because they exist in the repository, without
  showing that this assistant can see them.

## Pair with one structural guardrail

A loader is still instructions in model context. It improves behavior; it does
not mechanically stop a bad merge. Before relying on AGF for real work, add the
[minimum structural guardrail](../advanced/contracts-and-validators.md#minimum-structural-guardrail):
run the `agov` validators in merge-request CI and require the job before merge.

Schemas catch malformed governance records. They do not prove that a human
really approved a decision or that the implementation is correct. That final
judgment remains with the governor-reviewer.

## Next

- Install the framework layout: [install the `agf/` directory](install-and-layouts.md)
- Wire validation: [contracts and validators](../advanced/contracts-and-validators.md)
- Try one change: [your first governed change](../start/first-governed-change.md)
