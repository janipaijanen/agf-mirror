# Install the `agf/` directory

*Status: Released (v2.1 documentation). Audience: administrators and
developers commissioning the framework with an AI assistant. After this page
you have chosen the supported layout, asked the assistant to prepare the files,
and reviewed the evidence.*

## Prerequisites

- A git repository for the project you want to govern.
- An AI coding assistant that loads natural-language instructions
  (the reference implementation targets Claude Code; the rule content is
  tool-agnostic and has been tested with Cursor, Codex CLI, Copilot, and
  Windsurf). Before the first governed change, configure the assistant to
  see the AGF files and verify that it can name the rules it loaded; see
  [assistant loader configuration](assistant-loader-configuration.md).
- The `agov` command available if you want to run the current core validators
  locally and in CI. In a full checkout, symlink `tooling/agov` into
  `~/.local/bin/agov` or run it directly. A framework-only release archive
  contains `agf/` without packaged tooling; use the optional Rust tooling
  archive from the
  [project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases)
  for your platform when you want `agov` without a full checkout. Every
  archive's individual `.sha256` checksum and `-provenance.json` file are
  linked directly on the Releases page alongside it — verify with
  `sha256sum -c` after downloading both into the same directory.
- A human willing to act as governor — without one, stop here and re-read
  [should you adopt?](../start/should-you-adopt.md).

!!! tip "A quick way to hand the archive to your assistant"
    Download `agf-<version>.tar.gz` or `agf-<version>.zip` from the
    [Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases)
    into a `tmp/` directory inside your project, then prompt your assistant:
    "There is a `.tar.gz` file in `tmp/`. Adopt it into the project."
    The assistant will unpack and wire everything; you review the diff.
    You can also extract the archive yourself first and point the assistant
    at the directory — either way works. — *Jani*

## First decide the repository layout

The human layout decision is small but binding: AGF v2.0 uses one supported
layout, with the framework installed as the visible `agf/` directory in the
repository root. Make that decision before asking an assistant to move files.

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
`docs/` because the shipped `agov` validators default to those paths. If your
project chooses another coordination path, configure and document it before
agents start writing files.

Mixed layouts are the documented failure mode: agents following path
instructions literally will create root-level `agents/`, `learnings/`, or
duplicate coordination directories unless the default boundary is stated where
humans and agents can read it.

!!! note "Human decision, assistant execution"
    The human chooses the supported layout, governor, codename, and any
    risk-bearing local rule values. The assistant copies files, creates
    directories, prepares loaders and evidence, runs validators, and reports
    the diff for review.

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

The prompt below tells the assistant to implement the layout you already chose
above. It should not invent an alternative directory structure or treat its own
output as approval.

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

## Step 3: Review the prepared layout

The canonical distinction between the complete release library and a project's
selected operating subset is [What ships and what you activate](../start/should-you-adopt.md#what-ships-and-what-you-activate).
The review steps below perform that selection; they do not install a separate
starter package.

Review the assistant's diff against the layout decision above:

- `agf/` is present as the single framework directory.
- Assistant loaders point to `agf/rules/` instead of copying that tree.
- Working plans and coordination entries are in the documented project-owned
  path.
- Any non-default coordination path is documented before agents begin writing
  records.
- No root-level `agents/`, `learnings/`, or duplicate coordination directory
  appeared unless you deliberately documented a local override.

## Step 4: Review rule loading and customization

1. Confirm the assistant will read `agf/rules/` from its canonical location.
   If your tool needs a root-level file, review that file as a pointer or
   loader, not as a copied rule tree.
2. Review every remaining `CUSTOMIZE` marker and adapt each one to your stack:
   locale set, migration commands, CI paths, audit functions, target consumers
   for generated files.
3. Delete rules that do not apply (a project with no UI does not need the
   accessibility rule). Deleting is better than ignoring — an inapplicable
   rule trains agents that rules are optional. See the
   [adopter applicability catalog](applicability-catalog.md) for a
   per-rule activation condition and removal consequence, for the first
   slice of rules it classifies.

For tool-specific loader patterns and a verification prompt, use
[assistant loader configuration](assistant-loader-configuration.md). A loader
that exists but is not read by the active assistant is no protection.

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

Assistant tools and automation runtimes are execution surfaces. They do not
become AGF approval authorities unless a human governor explicitly changes the
role manifest and records that decision.

## Step 6: Wire the validators

If you choose to use the optional `agov` CLI from the full checkout or tooling
archive, run the shipped core validators:

```bash
agov validate
agov coord validate docs/coordination/*.md
agov plan validate docs/plans/*.md
```

A framework-only release archive has schemas and templates, but not the
current tooling executable. In that case, either use a full checkout for
validation or unpack the optional Rust `agov` tooling archive for your platform
from the
[project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).
Container-based CI can instead use the official `agov` OCI image from this
project's GitLab container registry:

```bash
podman pull registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.3.1
podman run --rm \
  -v "$PWD:/repo" \
  -w /repo \
  registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.3.1 \
  validate
```

Pin an exact version tag or digest. Do not use floating tags for CI. Published
image tags are immutable: if an image needs a fix, upstream publishes a new
version instead of replacing the old tag.
If you operate without `agov`, perform the equivalent manual review: confirm
the framework layout, inspect plan and coordination front matter against the
schemas, and record which checks were manual in the upgrade or adoption MR.

When upgrading an older copied-assistant layout, check for stale AGF copies in
tool-specific directories such as `.claude/rules/` and `.claude/skills/`.
If a file is an AGF-provided rule or skill copied for convenience, replace the
copy with a small loader reference to `agf/`. If it is an intentional
project-specific override, move it outside the copied AGF rule or skill tree so
future upgrades do not confuse local instructions with upstream framework
content.

When adding validator jobs to CI, verify each gate fires by feeding it a
deliberately bad input once on the merge request that introduces the job.

Validator output is advisory unless a named AGF rule, contract, or recorded
governor decision makes the check blocking. When a check is blocking, document
the rule that grants it blocking authority in the CI job or release procedure.

For the first real governed change, install at least the
[minimum structural guardrail](../advanced/contracts-and-validators.md#minimum-structural-guardrail):
validate changed coordination and plan files in the merge request, require the
job before merge, and self-test the job once with a deliberately malformed
governance file. Without that bridge to branch protection, the gates still
bind the process, but malformed governance files may not mechanically stop a
merge.

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
`YYYYMMDDThhmmssZ-<4-char-random>`, or equivalent project-local tooling that
cannot collide. When updating an existing learning, keep its existing ID
stable; when replacing it, add a new suffixed ID and point the old entry
forward.

## Verify

- Your assistant lists the rules when asked what constraints apply.
- The validators pass on your fresh files.
- A trivial change still commits without ceremony (fast track works).
- You can remove the trial by following
  [exit and decommission](exit-and-decommission.md) without rewriting history.

## Next

Run one real change through the gates:
[your first governed change](../start/first-governed-change.md).
