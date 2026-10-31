# Install the `agf/` directory

*Status: Released. Current framework version: see [Release information](../../reference/release-info.md). Audience: administrators and
developers who want the full layout, rule-customization, agent-definition,
validator, and evidence reference. First-time commissioning starts at
[commission AGF and AGOV](../start/commission.md) instead — it links back
into this page for everything below. After this page you have chosen the
supported layout, reviewed rule customization, adopted agent definitions,
wired validators, and initialized your evidence.*

## Prerequisites

- A git repository for the project you want to govern.
- An AI coding assistant that loads natural-language instructions
  (the rule content is tool-agnostic; the author-operated evidence covers the
  Claude and Codex integrations in Visual Studio Code, while independent
  corroboration for other assistants is still wanted). Before the first
  governed change, configure the assistant to see the AGF files and verify
  that it can name the rules it loaded; see
  [assistant loader configuration](assistant-loader-configuration.md).
- Optional local `agov` tooling for summaries and validators. Follow
  [Commission Step 2](../start/commission.md#step-2-get-your-pinned-agf-and-agov-artifacts)
  to choose and verify the separate framework and tooling artifacts. In a full
  checkout, use its existing `tooling/agov` wrapper directly or symlink that
  wrapper into `~/.local/bin/agov`; do not overwrite it with a release binary.
- A human willing to act as governor — without one, stop here and re-read
  [should you adopt?](../start/should-you-adopt.md).

For the ordered setup path and the single assistant prompt, use
[Commission AGF](../start/commission.md). Return here for layout choices,
customization, validators, evidence, and collision troubleshooting.

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

The commissioning prompt lives on one page:
[commission AGF and AGOV](../start/commission.md). That page covers both the
AGF layout and the optional AGOV binary, and branches correctly for a new or
existing project. Use it, then return here for Step 3 onward.

If you are working without an assistant, copy the distribution manually into
the visible `agf/` directory and then follow the Step 3 review checklist
below.

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

The shipped archive contains `agf/agents/MANIFEST.example.yaml`, a neutral
template — not an adopter-ready roster. Copy it to `agf/agents/MANIFEST.yaml`
and edit each role's `status` and `played_by` before wiring manifest-driven
tooling (`agov` only ever reads `agf/agents/MANIFEST.yaml`, or the path set
by `AGOV_MANIFEST_PATH`; it never falls back to the example file). See
`ADOPTING.md` §1.3 for the full procedure.

Minimum commissioning matrix:

| Trial shape | Active roles | Advisory/checklist coverage |
|---|---|---|
| Solo trial | `planner-architect`, `coder-tester`, `governor-reviewer` | Human governor checks plans and merges; use fast track only when criteria fit |
| Standard governed project | `planner-architect`, `coder-tester`, `governor-reviewer` | `contribution-auditor` and `infosec-sentinel` are consulted for non-trivial plans |
| Project with a CI pipeline | Standard roles plus active `build-engineer` | CI configuration and build automation |
| Project that submits learning records | Standard roles plus active `infosec-sentinel`, which screens learning records for credential and cross-record leaks. Required once the project submits learning records | Learning-record security screening |
| Project that takes contributions | Standard roles plus active `contribution-auditor`, which reviews contribution integrity. Required from the first contribution to the framework or to the project | Contribution integrity review |
| Project that publishes factual claims | Standard roles plus active `fact-checker`, which checks factual claims. Recommended from the first release, as needed | Evidence for published factual claims |
| Independent-review project | Standard roles, plus any rows above, plus active `independent-implementation-reviewer`, named per non-trivial Gate 2 candidate | A session/model distinct from the implementer must review the exact head before Gate 2 on named changes |

In a solo trial one assistant may perform both the `planner-architect` and
`coder-tester` responsibilities. Planning and Gate 1 plan authorship still
happen, and the human governor decides Gate 1.

Carried over from the former CI/security-conscious row: CI runs validators;
security-sensitive changes require explicit evidence.

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
# Choose an exact published version from the GitLab Releases page.
AGF_VERSION="<version-from-release-page>"
podman pull "registry.gitlab.com/janipaijanen/agents-govern-framework/agov:${AGF_VERSION}"
podman run --rm \
  -v "$PWD:/repo" \
  -w /repo \
  "registry.gitlab.com/janipaijanen/agents-govern-framework/agov:${AGF_VERSION}" \
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
- Set `adoption.checkout_role: live` on this freshly-created record. This
  marks it as your checkout's own, currently-maintained adoption state —
  the only kind of record `agov validate learnings` checks against
  `agf/VERSION` for version drift, and the mechanism that lets the check
  tell your live record apart from any imported reference copy of another
  adopter's (or historical) submission. Exactly one `checkout_role: live`
  record is expected per checkout; if `agf/learnings/` has zero, every
  `agov validate` run shows an advisory reminder (non-blocking) until you
  set the field; if it ever finds two or more, that is a blocking
  ambiguity you resolve by choosing one. During an upgrade, the framework's
  own upgrade skill briefly marks the live record with
  `adoption.upgrade_in_progress: true` and a timestamp while it applies
  changes — this suppresses the drift check for that one record for up to
  24 hours, clearing automatically once the upgrade's own version update
  lands.
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
