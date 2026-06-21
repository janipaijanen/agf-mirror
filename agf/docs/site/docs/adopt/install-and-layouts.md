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
  `~/.local/bin/agov` or run it directly. A framework-only release archive
  contains `agf/` without packaged tooling; use the optional Rust tooling
  archive for your platform when you want `agov` without a full checkout.
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

## Step 2: Commission the install with your assistant

Do not turn installation into a manual copying exercise unless you have to.
Ask your assistant to do the clerical work, then review the diff like any other
change.

Rule of thumb: let the AI act; make the human decide. The assistant should
move files, create directories, install rules, and prepare skeleton evidence.
The human reviews the diff, chooses the governor, accepts the codename, and
owns risk-bearing rule choices.

Paste this into your coding assistant:

```text
Commission AGF in this repository.

Use the AGF v2.0 distribution I have provided. Prepare the repository for a
first governed change, but do not invent approvals or claim the framework is
adopted until I review the diff.

Create or update:
- agf/ as the single framework directory
- a small assistant loader file for this tool, if needed, that tells the
  assistant to read the rules from agf/rules/
- agf/learnings/<project-codename>.yaml from agf/templates/learning-record.yaml
- docs/plans/
- docs/coordination/

Customize only the rule values that are obvious from this repository, such as
test commands, migration commands, locale rules, and CI paths. Leave clear
TODOs for anything that needs a human decision.

After editing, show me:
- files created or changed
- CUSTOMIZE markers that remain
- validation commands I should run
- any decisions you need from the governor before the first governed change
```

The assistant should prepare the files and tell you what it changed. You still
own the decision to adopt, the codename, the governor assignment, and any local
rule choices that affect risk.

Expected AI output:

- `agf/` present as the only framework-owned directory;
- `agf/learnings/<project-codename>.yaml` prepared from the learning template;
- `docs/plans/` and `docs/coordination/` created for project-owned evidence;
- a tiny tool-specific loader created only if your assistant needs one;
- no copied `agf/rules/` tree under `.claude/rules/`, `.cursor/rules/`, or an
  equivalent tool directory;
- a short report listing changed files, remaining `CUSTOMIZE` markers, and
  decisions still needed from the governor.

If you are working without an assistant, copy the distribution manually into
the visible `agf/` directory and then follow the same review checklist.

## Step 3: Review the framework layout

AGF v2.0 supports one layout: the assistant-prepared diff should place the
distribution in your repository as the visible `agf/` directory.

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
| Optional assistant loader | Tool-specific root file that points to `agf/rules/` |

Some assistants need a root-level configuration file before they will read
project instructions. That file is a local loader, not a copy of the
framework. It should point the assistant at `agf/rules/` and any local project
rules you choose to add. Do not duplicate the AGF rule tree into
`.claude/rules/` or an equivalent tool directory; that breaks the clean
carve-out promised by this layout.

Working plans and coordination entries are adopter-owned evidence, not
upstream-owned framework files. The current POC default keeps them in root
`docs/` because the shipped `agov` validators default to those paths. If your project
chooses another coordination path, configure and document it before agents
start writing files.

Mixed layouts are the documented failure mode: agents following path
instructions literally will create root-level `agents/`, `learnings/`, or
duplicate coordination directories unless the default boundary is stated where
humans and agents can read it.

## Step 4: Review rule loading and customization

1. Confirm the assistant will read `agf/rules/` from its canonical location.
   If your tool needs a root-level file, review that file as a pointer or
   loader, not as a copied rule tree.
2. Review every remaining `CUSTOMIZE` marker and adapt each one to your stack:
   locale set, migration commands, CI paths, audit functions, target consumers
   for generated files.
3. Delete rules that do not apply (a project with no UI does not need the
   accessibility rule). Deleting is better than ignoring — an inapplicable
   rule trains agents that rules are optional.

## Step 5: Adopt the agent definitions

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

## Step 6: Wire the validators

If you have the full checkout, run the shipped core validators with `agov`:

```bash
agov coord validate docs/coordination/*.md
agov plan validate docs/plans/*.md
```

A framework-only release archive has schemas and templates, but not the
current tooling executable. In that case, either use a full checkout for
validation or unpack the optional Rust `agov` tooling archive for your
platform.

When adding validator jobs to CI, verify each gate fires by feeding it a
deliberately bad input once on the merge request that introduces the job.

Validator output is advisory unless a named AGF rule, contract, or recorded
governor decision makes the check blocking. When a check is blocking, document
the rule that grants it blocking authority in the CI job or release procedure.

## Step 7: Initialise your evidence

- Confirm `agf/learnings/<your-codename>.yaml` exists and uses only the
  codename. The codename is the only project handle that crosses the codebase
  boundary — see the anonymization rule before sharing anything upstream.
- Record commissioning as your first learning. This is not busywork: the first
  adoption run tells you, and upstream, whether the assistant understood the
  commissioning prompt, kept AGF-owned files inside `agf/`, created only a
  small loader if needed, and left the human with clear decisions.
- Confirm the coordination channel path you will use and file your first entry:
  a note recording the adoption decision and the governor. The POC default is
  `docs/coordination/`.
- For the first governed change, ask the assistant to prepare starter files
  from `agf/templates/`; the walkthrough gives a pasteable prompt.

If commissioning exposed confusion, missing instructions, validation failures,
or surprisingly smooth automation, mark the learning as an upstream candidate.
Share only a sanitized summary or codename-safe learning record.

Use the fields in `agf/templates/learning-record.yaml`; do not invent fixed
sequential learning IDs. If your project later wants IDs, use
`YYYYMMDDThhmmssZ` with an optional `-<4-char>` suffix for same-second
disambiguation, or equivalent project-local tooling that cannot collide.

## Verify

- Your assistant lists the rules when asked what constraints apply.
- The validators pass on your fresh files.
- A trivial change still commits without ceremony (fast track works).
- You can remove the trial by following
  [exit and decommission](exit-and-decommission.md) without rewriting history.

## Next

Run one real change through the gates:
[your first governed change](../start/first-governed-change.md).
