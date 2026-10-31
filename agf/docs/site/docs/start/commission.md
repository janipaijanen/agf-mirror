# Set up AGF <a id="commission-agf-and-agov"></a>

*Status: Released. Current framework version: see [Release information](../../reference/release-info.md). Audience: anyone
setting up a repository — no prior AGF install assumed.*

## Overview

This page gets AGF and its optional `agov` tooling running in your
repository, whether it is a brand-new project or an existing one. Your
coding assistant does the mechanical work; you review the diff and make the
decisions that stay yours: the layout, the governor, the codename, and any
risk-bearing rule choices. Step 6 below points you to your first governed
change once commissioning is done.

## Prerequisites

- A git repository for the project you want to govern.
- An AI coding assistant that loads natural-language instructions (see
  [assistant loader configuration](../adopt/assistant-loader-configuration.md)
  for tool-specific setup).
- A person accountable for approvals: you, in a solo project. AGF calls this
  person the human governor. Without one, stop here and re-read
  [should you adopt?](should-you-adopt.md).

## Step 1: Choose new or existing project

**What:** Decide which of the two commissioning paths applies to this
repository.

**Why:** A new project has nothing to conflict with. An existing project may
already use `docs/`, hold CI configuration, or contain a partial AGF install.
The step 3 prompt branches on this and never silently overwrites what is there.

**How:** No prior AGF install and no colliding `docs/`, CI, or assistant
loader content: this is a **new project**. Anything else — including a
repository with unrelated existing content, or one with a partial or failed
prior AGF attempt — is an **existing project**.

**Verify:** You can state which path applies before pasting the prompt in
step 3.

## Step 2: Get your pinned AGF and AGOV artifacts

**What:** Download a pinned AGF framework archive. Choose the optional AGOV
tooling archive if you want local summaries and validators. Verify each
artifact you choose before use.

**Why:** Pinning and verifying protects you from silently adopting an
unintended version or a corrupted download — the same discipline this
project's own CI and release process uses internally.

**How:**

**AGF: the framework files.** Download `agf-<version>.tar.gz` or
`agf-<version>.zip` from the
[GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).
Assistants can discover the same direct asset links through the unauthenticated
[Releases API](https://gitlab.com/api/v4/projects/83321848/releases).
The framework archive contains `agf/`; it does not contain the tooling binary.

Download the archive's individual `.sha256` checksum and
`-provenance.json` file, linked alongside it. Run `sha256sum -c` on the
checksum file with the archive in the same directory. Check that the provenance
names the selected version, archive and source revision; retain the result in
the commissioning report. A checksum detects changed bytes; it does not by
itself establish that the source is trusted.

Keep the verified archive in a project-local `tmp/` directory or extract it
there for your assistant. Use the single prompt in Step 3 to commission it.
A pinned source checkout's `agf/` directory is another supported source;
record its version and revision instead of claiming archive verification.

**AGOV: optional local Rust tooling.** Apply the same verification discipline
to the tooling archive if you choose to install it:

- **Supported platform today: `linux-x86_64` only.** `linux-aarch64` was
  published on releases through v2.5.0 but is not currently built for
  releases from `main` (CI runner capacity constraint; see
  [release artifacts](../../reference/release-artifacts.md) for detail).
  macOS and Windows tooling are unsupported. Use a Linux x86_64 host for
  the published `agov` binary.
- **Asset naming:** `agov-<version>-<platform>.tar.gz`, with a companion
  `agov-<version>-<platform>.tar.gz.sha256` and
  `agov-<version>-<platform>.tar.gz-provenance.json`, each linked
  individually on the Releases page. Discover the exact current asset URLs
  through the same [Releases API](https://gitlab.com/api/v4/projects/83321848/releases)
  used for the AGF archive above — do not hand-construct the URL.
- **Verify:** download the tarball and its `.sha256` file into the same
  directory, then run `sha256sum -c agov-<version>-<platform>.tar.gz.sha256`.
  Check the matching `-provenance.json` against the selected version,
  archive and source revision.
- **Install:** symlink the extracted `agov` binary into `~/.local/bin/agov`
  and confirm that directory is on your shell's `PATH`. In a full source
  checkout, use its existing `tooling/agov` wrapper instead of replacing it
  with a downloaded binary.

**Verify:** The selected artifacts match their checksums and provenance.
If you installed tooling, `agov version` prints the pinned version and build
provenance. Without tooling, follow the manual checks in Step 5; AGF files
and human gate decisions remain usable.

## Step 3: Paste one assistant commissioning prompt

**What:** Hand the mechanical setup to your assistant with one prompt that
covers AGF and your optional tooling choice, and branches for your project from
step 1.

**Why:** Let the AI act; make the human decide. The assistant moves files,
creates directories, installs rules, verifies AGOV, and prepares skeleton
evidence. You review the diff, choose the governor, accept the codename, and
own risk-bearing rule choices — none of that is delegable.

**How:** Paste this into your coding assistant:

```text
Commission AGF in this repository, with AGOV only if I select it below.

Use the pinned AGF distribution and, if selected, the pinned AGOV binary
I have made available in this repository (see the previous step), either as
release archives or extracted directories/binaries. Do not fetch or substitute
a different version. Prepare the repository for a first governed change, but do not
invent approvals or claim the framework is adopted until I review the diff.

This is a [NEW PROJECT / EXISTING PROJECT] commissioning — replace with the
answer from step 1.
Tooling choice: [INSTALL PINNED AGOV / FRAMEWORK FILES ONLY].

If NEW PROJECT:
- Create agf/ as the single framework directory.
- Create a small assistant loader file for this tool, if needed, that tells
  the assistant to read the rules from agf/rules/.
- Create agf/learnings/<project-codename>.yaml from
  agf/templates/learning-record.yaml.
- Create docs/plans/ and docs/coordination/.
- If AGOV is selected, install the pinned binary per the previous step and
  confirm `agov version` and `agov session resume` both succeed. Otherwise
  record that tooling is absent and prepare the manual review in Step 5.

If EXISTING PROJECT:
- Before writing anything, check for: a pre-existing docs/ directory used
  for something other than AGF, an existing assistant loader file, colliding
  CI paths, or a partial prior agf/ install.
- Never overwrite agf/agents/MANIFEST.yaml, agf/learnings/<codename>.yaml,
  docs/plans/, docs/coordination/, or any existing assistant loader file —
  these are adopter-owned. If any of these already exist, report them
  instead of resolving the collision yourself.
- For any upstream-owned file (agf/rules/**, agf/schemas/*, agf/templates/*,
  agf/agents/*.yaml other than MANIFEST.yaml, and similar) that already
  exists from a prior partial install: compare it against the pinned
  distribution's copy. If identical, refresh it silently. If different, do
  not overwrite it — report it as a collision requiring my decision.
- If AGOV is selected, install the pinned binary per the previous step and
  confirm `agov version` and `agov session resume` both succeed. Otherwise
  record that tooling is absent and prepare the manual review in Step 5.

Customize only the rule values that are obvious from this repository, such
as test commands, migration commands, locale rules, and CI paths. Leave
clear TODOs for anything that needs a human decision.

After editing, show me a report containing:
- pinned AGF version and source (release archive vs. checkout)
- pinned AGOV version, with checksum and provenance verification result,
  or an explicit note that optional tooling was not installed
- files created or changed
- confirmation the assistant can read agf/rules/ (name the rules it loaded)
- CUSTOMIZE markers that remain
- any collisions you found and left for me to decide, with file paths
- the `agov version` and `agov session resume` output if tooling is installed,
  or the manual checks performed and their results
- validation commands I should run
- any decisions you need from the governor before the first governed change
```

**Verify:** The assistant produced a report matching the checklist above,
not just a list of copied files.

## Step 4: Review the assistant's report

**What:** Read the diff and the report against the completion checklist
below before treating anything as commissioned.

**Why:** A repository is not commissioned merely because files were copied.
This is the one non-delegable review step in the middle of an otherwise
assistant-executed process.

**How:** Confirm the report includes every item:

- pinned AGF version and source;
- pinned AGOV version, checksum, and provenance verification result, or an
  explicit note that optional tooling was not installed;
- the changed-files list;
- confirmation the assistant loader is active (it names the rules it
  loaded — see the
  [loader verification prompt](../adopt/assistant-loader-configuration.md#verify-the-loader)
  for the stronger follow-up check);
- remaining `CUSTOMIZE` markers;
- any detected collisions and how they were left for you, not silently
  resolved;
- `agov version` and `agov session resume` output if tooling is installed,
  or the manual checks and results;
- a list of decisions still needed from you as governor.

**Verify:** Every item above is present in the report; anything missing
means the assistant should redo that part before you continue.

## Step 5: Confirm validation

**What:** Review validation evidence before continuing.

**Why:** The assistant's report is a claim; the validator output is
evidence.

**How:** With optional tooling installed, run:

```bash
agov version
agov session resume
agov validate
```

**Verify:** All three commands complete successfully. Read warnings and
advisories, record what they mean for this repository, and resolve blocking
findings under the applicable rule. Successful execution does not promise
empty output or supply a governor decision.

**Without tooling:** Review the prepared layout and loader, inspect plan and
coordination front matter against the shipped schemas, and record which checks
were manual and what remains unverified. Use the
[layout and validation reference](../adopt/install-and-layouts.md#step-6-wire-the-validators)
for details. Manual review does not waive applicable rules or gate decisions.

## Step 6: Continue to your first governed change

**What:** Move on to running one real change through both gates.

**Why:** Commissioning gets the framework running; it does not yet
demonstrate the governance loop working end to end.

**How:** Continue to
[your first governed change](first-governed-change.md).

**Verify:** Not applicable — this step is a link forward, not a check.

## Entry points summary

| Task | Location |
|---|---|
| Deep reference on layout, rule customization, agent definitions, and validators | [Install the `agf/` directory](../adopt/install-and-layouts.md) |
| Decide whether to adopt at all | [Should you adopt?](should-you-adopt.md) |
| Leave cleanly | [Exit and decommission](../adopt/exit-and-decommission.md) |

## See also

- [Install the `agf/` directory](../adopt/install-and-layouts.md) — the
  deep-reference page this one points into: layout theory, rule
  customization, agent-definition adoption, validator wiring, and evidence
  initialization.
- [Assistant loader configuration](../adopt/assistant-loader-configuration.md) —
  tool-specific loader setup and the stronger loader-verification prompt.
- [Roles and gates](../concepts/roles-and-gates.md) — the mental model
  behind the governor, Gate 1, and Gate 2 referenced throughout this page.
