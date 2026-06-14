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
- The `agov` command available if you want to run the current core validators
  locally and in CI. In a full checkout, symlink `tooling/agov` into
  `~/.local/bin/agov` or run it directly. A release archive containing only
  `agf/` does not yet include packaged tooling.
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

`agf/` is the adopter-facing framework product. It contains the normative
assets your project adopts: roles, rules, schemas, templates, and docs.
`tooling/` is implementation support for validation, build, release, and
conformance checks. Tooling reports evidence against the framework; it is not
itself the source of governance authority.

| Artifact | Location |
|---|---|
| Framework directory | `agf/` |
| Agent definitions | `agf/agents/*.yaml` |
| Framework rules | `agf/rules/**/*.md` |
| Schemas | `agf/schemas/*` |
| Learning records | `agf/learnings/<your-codename>.yaml` |
| Working plans | `docs/plans/` |
| Coordination channel | `docs/coordination/` |
| Assistant rule install target | `.claude/rules/` or your tool's equivalent |

The assistant rule install target stays at the project root because tools read
their own configuration there. It is the exception, not a second framework
layout.

Working plans and coordination entries are adopter-owned evidence, not
upstream-owned framework files. The current POC default keeps them in root
`docs/` because the shipped `agov` validators default to those paths. If your project
chooses another coordination path, configure and document it before agents
start writing files.

Mixed layouts are the documented failure mode: agents following path
instructions literally will create root-level `agents/`, `learnings/`, or
duplicate coordination directories unless the default boundary is stated where
humans and agents can read it.

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
[roles and gates](../concepts/roles-and-gates.md) page explains the mental
model, and the shipped raw reference at `agf/docs/reference/quick-reference.md`
lists which phase each role belongs to.

The manifest file currently present in `agf/agents/MANIFEST.yaml` is the
AGF-NG/foundry dogfooding manifest. Treat it as an example of role-status
tracking, not as an adopter-ready roster. Before wiring manifest-driven
tooling, create or document your own local role-status authority and point
tooling at it explicitly.

Minimum commissioning matrix:

| Trial shape | Active roles | Advisory/checklist coverage |
|---|---|---|
| Solo trial | `coder-tester`, `governor-reviewer` | Human governor checks plans and merges; use fast track only when criteria fit |
| Standard governed project | `planner-architect`, `coder-tester`, `governor-reviewer` | `contribution-auditor` and `infosec-sentinel` are consulted for non-trivial plans |
| CI/security-conscious project | Standard roles plus active `contribution-auditor`, `infosec-sentinel`, and `fact-checker` as needed | CI runs validators; security-sensitive changes require explicit evidence |

Gateway assistants, chat bridges, and agent runtimes are execution surfaces.
They do not become AGF approval authorities unless a human governor explicitly
changes the role manifest and records that decision.

## Step 5: Wire the validators

If you have the full checkout, run the shipped core validators with `agov`:

```bash
agov coord validate docs/coordination/*.md
agov plan validate docs/plans/*.md
```

A release archive containing only `agf/` has schemas and templates, but not
the current tooling executable. In that case, either use a full checkout for validation
or wait for a packaged `agov` tooling distribution.

When adding validator jobs to CI, verify each gate fires by feeding it a
deliberately bad input once on the merge request that introduces the job.

Validator output is advisory unless a named AGF rule, contract, or recorded
governor decision makes the check blocking. When a check is blocking, document
the rule that grants it blocking authority in the CI job or release procedure.

## Step 6: Initialise your evidence

- Create `agf/learnings/<your-codename>.yaml` with your project codename.
  The codename is the only project handle that crosses the codebase
  boundary — see the anonymization rule before sharing anything upstream.
- Create the coordination channel path you will use and file your first entry:
  a note recording the adoption decision and the governor. The POC default is
  `docs/coordination/`.
- Copy starter templates from `agf/templates/` when taking the first governed
  change through both gates.

## Verify

- Your assistant lists the rules when asked what constraints apply.
- The validators pass on your fresh files.
- A trivial change still commits without ceremony (fast track works).
- You can remove the trial by following
  [exit and decommission](exit-and-decommission.md) without rewriting history.

## Next

Run one real change through the gates:
[your first governed change](../start/first-governed-change.md).
