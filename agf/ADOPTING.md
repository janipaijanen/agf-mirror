<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Adopting the Governance Framework

> **RC readiness label — `adapted-from-v1x`, partially verified for AGF v2.**
> This long-form operator guide contains material inherited from agents-govern
> v1.x and older tooling eras. The current adopter-facing path is the public
> documentation site under `agf/docs/site/`. Treat any passage describing
> not-yet-shipped commands as historical or provisional unless a current
> `agov` command and public docs page confirm it.

Concrete steps for applying this framework to an existing software project.
Follow in order. Each step has a defined output — do not skip to the next
step until the output exists.

For the theory behind each step, see framework.md. This document is the
assistant/operator practice guide: AI assistants may execute the file setup and
produce reviewable diffs, while humans decide whether to adopt, which risks to
accept, and which governance roles have authority. Human-facing flow guidance
lives in `docs/site/docs/adopt/install-and-layouts.md`.

> **Disclaimer:** All guidance in this document is advisory, not
> authoritative. Procedures (especially anonymization and git history
> rewriting) may damage your codebase if applied without proper
> pre-flight checks and backups. You are responsible for evaluating
> whether each step is appropriate for your project. See
> [disclaimer](docs/site/reference/disclaimer.md) for the full terms.
> Run AI assistants and AGF tooling only on systems, repositories, and
> data where you have the necessary authorization.

---

## Confirm the layout (mandatory — do this before any step below)

AGF v2.0 supports one adoption layout: the framework lives inside your
existing project as the visible `agf/` directory. Framework-owned governance
artifacts stay under `agf/`; assistant tool configuration such as
`CLAUDE.md`, `AGENTS.md`, or another tool-specific loader may stay at the
project root because the assistant reads it there. That loader must point to
`agf/rules/`; it must not duplicate the AGF rule tree.

Treat `agf/` as the adopter-facing product and `tooling/` as implementation
support. Tooling can validate, package, summarize, and report on governance
artifacts, but it does not become the source of governance authority. Authority
lives in the framework rules, schemas, role decisions, and the coordination
record. A tool check is blocking only when a named rule or recorded governor
decision grants that check blocking authority; otherwise it is advisory
evidence.

| Artifact | Path |
|----------|------|
| Learning records | `agf/learnings/<codename>.yaml` |
| Agent definitions | `agf/agents/*.yaml` |
| ADRs and governance docs | `agf/docs/adr/` |
| Framework rules | `agf/rules/` |
| Working plans | `docs/plans/` |
| Coordination channel | `docs/coordination/` |
| Optional assistant loader | Tool-specific root file that points to `agf/rules/` |

Working plans and coordination entries are adopter-owned evidence. The POC
default keeps them in root `docs/` because the current `agov` validators use
those paths. Framework-owned files stay under `agf/`.

> **Why this matters — silent failure mode.** AI agents and humans following
> this guide literally create files at the paths written here. If the guide
> alternates between `agf/docs/coordination/` and `docs/coordination/`, agents
> will split the audit trail. AGF v2.0 avoids that by stating one default path
> before work begins.

---

## Quickstart (15-minute path)

If you want to try the framework before committing to full adoption,
this minimal path gets you running. It assumes the release archive has been
extracted as `agf/` in your existing project.

0. **Make `agov` available** if you want to run the current core validators
   locally. For a release installation, download the `agov` archive for your
   platform from the current
   [GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases),
   and verify its adjacent `.sha256` and `-provenance.json` files. Assistants
   that cannot read the client-rendered Releases page can discover the same
   assets through the unauthenticated
   [GitLab Releases API](https://gitlab.com/api/v4/projects/83321848/releases).
   See the canonical
   [installation guide](docs/site/docs/adopt/install-and-layouts.md) for the
   supported artifact-first procedure. In a full checkout, you may instead
   symlink the repository-local executable:
   ```bash
   mkdir -p ~/.local/bin
   ln -sfn "$PWD/tooling/agov" ~/.local/bin/agov
   ```

1. **Download** the [latest release archive](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/permalink/latest)
   and extract it into your project as `agf/`.
   Patch releases are validated from clean `.tar.gz` and `.zip` extractions;
   every Python regression test included in the framework archive must run
   without relying on files that exist only in the upstream repository.

2. **Ask the assistant to wire rule loading** without copying the rules out of
   `agf/`. If your tool needs a root file, create a small local loader such as
   `CLAUDE.md`, `AGENTS.md`, or the tool's equivalent that tells the assistant
   to read `agf/rules/` before acting. Keep the framework-owned rule files in
   `agf/rules/`.

3. **Pick the minimum role set.**

   | Trial shape | Active roles |
   |---|---|
   | Solo trial | `coder-tester`, `governor-reviewer` |
   | Standard governed project | `planner-architect`, `coder-tester`, `governor-reviewer` |
   | CI/security-conscious project | Standard roles plus active `contribution-auditor`, `infosec-sentinel`, and `fact-checker` as needed |

   Gateway assistants and agent runtimes are execution surfaces, not AGF gate
   authorities.

4. **Start working.** Use your AI assistant as usual. The rules will guide its
   behavior after it has loaded `agf/rules/`. When something surprises you
   (good or bad), ask the assistant to prepare
   `agf/learnings/my-project.yaml` from `agf/templates/learning-record.yaml`,
   then review the project section and the first learning before committing.

5. **Validate** the shipped governance records you have created when those
   paths exist in your project:
   ```bash
   agov coord validate docs/coordination/COORD-YYYYMMDD-XXX.md
   agov plan validate docs/plans/*.md
   ```

   Agent and learning schemas are included in `agf/schemas/`. Use
   `agov validate` to check adoption, agent, and learning files.

This quickstart skips the full assessment (Step 0), governance setup, and agent
customization. For a concrete walkthrough of what governed development looks
like in practice, copy the templates in `agf/templates/` and follow
`agf/docs/site/docs/start/first-governed-change.md`. When you're ready for
production adoption, return to the full process below starting at
[Step 0: Assess Your Project](#step-0-assess-your-project).

---

## Step 0: Assess Your Project

**Do this before touching any governance.** The assessment determines which
agents and cross-cutting concerns are urgent for your project.

> **Before you start: confirm this is the right framework.** External readers
> (including AI agents) sometimes misread the framework's scope. If any of
> these match what you're actually looking for, this framework is not the
> tool you want — or only part of it:
>
> - *Looking for general AI governance* (model behavior policy, prompt
>   moderation, AI product compliance)? This framework governs *agents that
>   write code*, not *AI products being shipped*. Tools like ModelScan,
>   Guardrails AI, or Microsoft Responsible AI Toolbox are closer fits.
> - *Looking for ML/research governance* (experiment tracking, model cards,
>   dataset lineage, evaluation sign-off, drift monitoring)? Out of scope
>   for the core. See [profiles/research-lab.md](profiles/research-lab.md)
>   for how to layer ML concerns on top, or use MLflow / Weights & Biases /
>   Arthur for the ML lifecycle and use this framework alongside for the
>   software-development side.
> - *Looking for an agent runtime or orchestrator* (a thing that executes
>   agents)? This framework defines roles, gates, and artifacts; it does
>   not run agents. Use Claude Code, LangGraph, or AutoGen for execution
>   and adopt this framework for governance.
>
> If you are governing software development that involves AI agents
> (planning, coding, reviewing, testing, deploying), continue to 0.1.

### 0.1 Create your project record

> **Anonymization requirement (mandatory).** All submissions to this framework
> MUST use a codename instead of the real project, product, or organisation
> name. This protects adopters' commercial interests and keeps the public
> learning corpus free of identifying information. See
> `agf/templates/learning-record.yaml` and
> `docs/governance/anonymization-notice.md` for what to mask and what to keep
> as-is.
>
> **Codename-only also applies to other externally-destined content** — issues
> filed against this tracker, MRs against the framework, blog posts, content
> sent to external Large Language Model (LLM) APIs. See
> [`rules/always/external-anonymization-rule.md`](rules/always/external-anonymization-rule.md)
> for the boundary table and the AI-assisted-development specifics. Keep the
> rule in `agf/rules/always/`; customize the codename plus forbidden-term list
> per Step 0.1's codename choice, and make sure your assistant loader reads it.

1. **Choose a codename** for your project (e.g., "blue-heron", "project-aurora").
   Use it consistently in the filename and all fields.
2. **Record the mapping** between codename and real name in your own private
   records (see `agf/templates/private-codename-mapping.example` for the
   format). Do NOT commit the mapping to this repository.
3. Ask the assistant to prepare `agf/learnings/<codename>.yaml` from
   `agf/templates/learning-record.yaml`.

> **Artifact naming.** Use domain-first names for all adopter-created files:
> `phase0-foundation.md`, `crm-gate1-plan.md`, `blue-heron.yaml`. Reserve
> the `agf-*` and `agov-*` prefixes for files that are part of the framework
> tree itself. Adopter plans, coordination notes, and learning records named
> with an `agf-` prefix create confusion about ownership and may be overwritten
> by future framework upgrades.

Fill in every field in the `project`, `business_domain`, `regulatory_context`,
`team`, `lifecycle_state`, `customer_state`, `revenue_context`, and `codebase`
sections. Do not skip fields — write "unknown" if you genuinely do not know.
Do not include real project, product, organisation, or customer names in any
free-text field.

### 0.2 Run the Personally Identifiable Information (PII) assessment checklist

Go through your codebase and infrastructure. For each item, record
where PII exists (file, database, service) or confirm it does not.

- [ ] User accounts (names, emails, usernames, passwords/hashes)
- [ ] Billing data (credit cards, bank accounts, billing addresses)
- [ ] Contact information (phone numbers, physical addresses)
- [ ] Government identifiers (Social Security Number (SSN), national ID, tax ID, passport)
- [ ] Health or biometric data
- [ ] Location data (Global Positioning System (GPS), IP addresses used for geolocation)
- [ ] Behavioural data (usage logs with user identifiers, analytics with user IDs)
- [ ] User-generated content that may contain PII (messages, comments, uploads)
- [ ] Third-party data received via APIs or integrations
- [ ] Logs that contain PII (access logs with user IDs, error logs with email addresses)
- [ ] Backups and archives that contain any of the above
- [ ] Environment variables or config files that contain credentials to PII stores

**If any box is checked:** Privacy Guardian and Data Guardian are urgent from
day one. Activate them as checklists in Step 3.

### 0.3 Run the security posture checklist

- [ ] Dependencies: do you have a lock file? When was it last audited for CVEs?
- [ ] Secrets: are all secrets in environment variables or a secrets manager (not in code)?
- [ ] Access control: who has production access? Is it documented?
- [ ] Supply chain: are your build artifacts signed? Is your CI/CD pipeline access-controlled?
- [ ] Infrastructure: are containers/VMs hardened? Are unused ports closed?
- [ ] Authentication: is user auth using established libraries (not hand-rolled)?
- [ ] HyperText Transfer Protocol Secure (HTTPS): is Transport Layer Security (TLS) enforced everywhere, including internal services?
- [ ] Logging: do access logs exist? Are they tamper-evident?

**Record gaps.** These become the Security Guardian's initial focus areas.

> **Verify against code, not just documentation.** For each gap you record,
> confirm it by checking the actual code, CI config, or infrastructure — not
> just design documents, threat analyses, or agent summaries. Assessments
> based on secondary sources consistently overclaim gaps (see stone-arch
> learning: 3 of 5 security gaps were false positives because the assessment
> relied on a threat analysis rather than inspecting CI config and migration
> files).

### 0.4 Assess codebase maturity

- [ ] Test coverage: what percentage? (estimate is fine)
- [ ] CI/CD: does a pipeline exist? Does it run tests automatically?
- [ ] Documentation: does a README exist? Application Programming Interface (API) docs? Architecture docs?
- [ ] Code review: is there a review process? Is it enforced or optional?
- [ ] Branching strategy: are there protected branches? Is main/master deployable?

### 0.5 Map your environments

List every environment that exists or needs to exist:

| Environment | Exists? | Has real PII? | Policy level needed |
|-------------|---------|---------------|-------------------|
| Development | ? | ? | relaxed |
| Testing/CI | ? | ? | moderate |
| Demo/Staging | ? | ? | strict |
| Production | ? | ? | maximum |

(See framework.md Section 8 for what each policy level means.)

### Step 0 output

A filled-in `agf/learnings/<codename>.yaml` with all context sections complete,
plus the four checklists above with boxes checked. You now know:
- Whether PII is present (determines Privacy/Data Guardian urgency)
- Your security gaps (determines Security Guardian focus)
- Your codebase maturity (determines what pipeline gates are realistic)
- Your environment state (determines policy gradient priorities)

---

## Step 1: Map Phase 1 Roles to Your Team

Phase 1 uses three combined agents. Assign each to a person or AI.

### 1.1 Assign roles

Fill in this table:

| Agent | Played by | Notes |
|-------|-----------|-------|
| **Planner-Architect** | __________ | Plans work, makes design decisions |
| **Coder-Tester** | __________ | Implements and verifies |
| **Governor-Reviewer** | __________ | Enforces boundaries, reviews, approves |

**Guidelines:**
- One person can play multiple roles (see GOVERNANCE.md for our example:
  Jani plays Human + Governor-Reviewer, Claude plays Planner-Architect +
  Coder-Tester).
- The Governor-Reviewer should ideally be a different person than the Coder-Tester.
  If not possible, accept the conflict temporarily — it is Phase 1, and the split
  signal will tell you when to separate.
- If using AI agents: assign the LLM (Claude, Generative Pre-trained Transformer (GPT), etc.) to Planner-Architect
  and/or Coder-Tester. A human should always play or oversee the Governor-Reviewer.
- **Advisory agents** are optional domain specialists (e.g., UX Architect,
  Domain Specialist) that participate during planning but have no pipeline
  authority — they advise, they do not gate. If your project has
  domain-specific design constraints, consider defining advisory agents that
  the Planner-Architect can consult at Gate 1. These do not need Yet Another Markup Language (YAML)
  definitions or framework tracking; they are project-specific and informal.

### 1.2 Record framework version

Before copying anything, record which version of the framework you are adopting.
This is critical for auditability — future learnings and incidents must be
traceable to the framework version that was in effect.

> **Which version?** The `framework_version` to record is the **framework content version** from the [VERSION](VERSION) file (or the header of `framework.md`).

In your project's `agf/learnings/<codename>.yaml`, fill in the `adoption`
section:

```yaml
adoption:
  framework_id: "agents-govern"
  framework_version: "2.4.6"        # check agf/VERSION for current version
  framework_commit: "<git-hash>"    # run: git rev-parse HEAD in the framework repo
  agent_definitions:
    - id: "agents-govern:planner-architect:0.4.0"
      local_path: "agf/agents/planner-architect.yaml"
      customized: false
    - id: "agents-govern:coder-tester:0.3.2"
      local_path: "agf/agents/coder-tester.yaml"
      customized: false
    - id: "agents-govern:governor-reviewer:0.4.0"
      local_path: "agf/agents/governor-reviewer.yaml"
      customized: false
```

### 1.3 Copy and adapt agent definitions

Use the Phase 1 YAML files from the adopted `agf/agents/` directory. They are
framework-owned reference definitions; customize copies only when your project
has a documented reason.

```
ls agf/agents/
```

Then edit each file:
- Update `identity.purpose` to reflect your project's domain
- Add `identity.design_principles` with 1-3 principles that guide how this
  agent approaches work in your domain (e.g., "Reproducible builds over fast
  builds", "Precision over recall for blocking findings"). These shape
  decision-making beyond what jurisdiction and forbidden_actions cover.
- Update `jurisdiction.owns/reads/cannot_touch` to match your project's artifacts
- Update `communication` to match your team's actual channels
- Keep `boundaries.forbidden_actions` unchanged — these are framework-level rules
- **If you customize a definition:** set `customized: true` in your adoption
  record and note what changed. The `id` stays the same (it tracks the
  upstream origin), but the customization is documented for auditability.

**Create an agent manifest when your project needs one.** If you track role
activation locally, keep the manifest under your adopted framework directory
unless your project has documented a different role-authority file:

```yaml
# agf/agents/MANIFEST.yaml
agents:
  planner-architect:
    status: active
    played_by: "claude"
  coder-tester:
    status: active
    played_by: "claude"
  governor-reviewer:
    status: active
    played_by: "human (you)"
  build-engineer:
    status: active              # activate if project has CI
    played_by: "claude"
  localizer:
    status: skipped
    reason: "english only"      # suspended/skipped require a reason
```

The manifest declares which agents are commissioned. Use `agov validate` to
check consistency: every active manifest entry should have a corresponding YAML
role definition, and every role definition you rely on should have an explicit
status.

**Copyright and licensing for custom agents.**
When you create new agent definitions for your project:

- **Derived from framework agents** (copied and modified from `agf/agents/*.yaml`):
  your modifications are yours, but the result is a derivative work under
  CC-BY-SA-4.0. If you share it (e.g., submit upstream), it carries
  ShareAlike. Add your copyright line alongside the original:
  ```yaml
  # SPDX-License-Identifier: CC-BY-SA-4.0
  # Copyright (c) 2026 Jani Päijänen (original)
  # Copyright (c) 2026 Your Name (modifications)
  ```
- **Written from scratch** using only the YAML template structure: your
  copyright, your choice of license. The template structure (field names,
  section layout) is not copyrightable — only creative content is. However,
  if you want to contribute the agent upstream, CC-BY-SA-4.0 is required.
When in doubt: keep your copyright header, add CC-BY-SA-4.0, and
contribute upstream. The framework benefits from every new agent
definition that others can adopt.

**Verify communication symmetry after adding or customizing agents.**
When you add a project-specific or domain-specific agent, or modify an
existing agent's `sends_to`/`receives_from`, run the agent validator after
the AGF v2 Rust command ships. Planned command shape:

```bash
# Check structure, communication symmetry, ownership overlaps, phase distribution
agov validate agents ../../agf/agents/*.yaml

# Generate a Mermaid communication graph (paste into any Mermaid renderer)
agov validate agents ../../agf/agents/*.yaml --graph
```

The validator checks that every `sends_to` has a matching `receives_from`
on the other side and vice versa. Asymmetries accumulate silently — an
agent that sends messages to a peer that doesn't acknowledge receiving
them is a governance gap. Fix all asymmetries before committing.

> **Tip — keep the framework carve-out clean.** For complex projects with many
> domain-specific rules, keep AGF's upstream-owned rules in `agf/rules/` and
> put only local project guidance in your tool's native rule location. If you
> need a root `CLAUDE.md`, `AGENTS.md`, `.cursor/rules/`, or similar file, make
> it a local loader that points the assistant at `agf/rules/`.

### 1.4 Load and customize rules

The framework provides ready-to-adopt rule files in
[`agf/rules/`](rules/). These implement governance principles as
assistant-readable Markdown files, and they stay there as the canonical
framework copy.

**To adopt:**
1. Ask the assistant to confirm it can read `agf/rules/` from the repository.
2. If your tool needs a root-level loader, copy the appropriate starter
   template from `agf/templates/` and fill in the `<!-- CUSTOMIZE -->` markers:
   - **Codex / GitHub Copilot:** copy `agf/templates/AGENTS.md` to your project
     root as `AGENTS.md`.
   - **Claude Code:** copy `agf/project-specific-examples/python-flask/CLAUDE.md`
     (or equivalent) to `.claude/CLAUDE.md` and adapt.
   - **Other tools:** ask the assistant to create an equivalent loader that points
     at `agf/rules/`.
   Do not copy the AGF rule tree into the tool directory — keep rules under `agf/`.
3. Search `agf/rules/` for `<!-- CUSTOMIZE -->` comments and adapt only the
   values that belong to your project: test commands, migration commands,
   locale sets, CI paths, and generated-file consumers.
4. If your stack matches a provided example (e.g., Python + Flask), ask the
   assistant to prepare a local rule or loader section derived from
   [`project-specific-examples/`](project-specific-examples/). Keep clear
   provenance: upstream framework files stay under `agf/`; project-specific
   additions are local project rules.
5. Ask the assistant to summarize what it produced: loader files, customized
   AGF rules, remaining `CUSTOMIZE` markers, and decisions still needed from
   the governor.

**Rule categories:**

| Directory | Scope | Example rules |
|-----------|-------|---------------|
| `always/` | Core constraints on every change | uncertainty-and-scope, high-risk-operations, security-hardening |
| `workflow/` | Process: commits, testing, branching, releases | Test-Driven Development (TDD), debugging, code review checklist, branch discipline |
| `ui/` | Templates, Cascading Style Sheets (CSS), accessibility | Web Content Accessibility Guidelines (WCAG) AA compliance |
| `docs/` | Documentation governance | audience targeting, communication tone |

Each rule file includes a header comment citing the framework principle
it implements — e.g., `# Framework principle: Immutability Constraint`.

### 1.4b Configure skill access (optional)

> **Not using Claude Code?** Skip this step entirely. The governance
> framework works without skills — they are tool-specific automation,
> not governance requirements. The principles, agents, and rules work
> regardless of which AI assistant (or no assistant) you use.

The framework provides reusable AI assistant workflows in
[`skills/`](skills/). Keep the canonical workflow definitions under
`agf/skills/`. The workflows they encode can be adapted to AI coding
assistants that support skill-like prompts, but this is optional.

> **Two-layer design.** The framework's conceptual layer (agent taxonomy,
> jurisdiction model, governance principles in framework.md) is
> technology-agnostic. Skills are part of the implementation layer —
> concrete automation for a specific tool. Adopters not using AI coding
> assistants can skip this step entirely.

**To adopt:**
1. Ask the assistant to confirm it can read `agf/skills/` directly.
2. If your tool needs slash-command wrappers, ask the assistant to create
   small local wrappers that point to the canonical `agf/skills/*/SKILL.md`
   files. Do not copy the upstream skill definitions into the tool directory
   as a second source of truth.
3. **Other AI assistants:** the wrapper path differs per tool. Consult your
   tool's docs; the principle is the same — local wrapper, canonical source
   under `agf/skills/`.
4. Search for `<!-- CUSTOMIZE -->` comments and adapt paths to your layout.
5. When creating project-specific skills, set `origin: project` in the
   YAML frontmatter — do not modify `origin: upstream` skills in place.

**The `origin:` convention.** Each skill declares its provenance in
frontmatter: `origin: upstream` (from the distribution) or
`origin: project` (created by the adopter). The upgrade skill uses this
to distinguish files it should compare against upstream from files it
must leave untouched. See [`skills/README.md`](skills/README.md) for
details.

| Skill | What it automates |
|-------|-------------------|
| `upgrade/` | Gated framework upgrade: extract → compare → apply → track → verify → summarise |
| `learning/` | Learning record scaffolding: find next ID, format YAML, insert, validate |
| `infosec-screen/` | Comprehensive leak screening: sentinel + pre-commit + Software Package Data Exchange (SPDX) + patterns |
| `health/` | Periodic governance health check: all validators in one command |
| `release/` | Gated release workflow: bump → changelog → branch → commit → MR → tag |
| `contribution-audit/` | Adversarial review of contributions: six audit dimensions |
| `capability-review/` | Agent evolution analysis: phase splits, maturity, new agent candidates |

### 1.4a Audit codebase against adopted rules

After wiring rule loading (Step 1.4), audit your existing codebase against
each adopted rule before running your first pipeline task. For each canonical
rule file in `agf/rules/`, assess whether the codebase is:

- **CONFORMANT** — codebase already follows the rule
- **PARTIAL** — codebase follows in some places but not others
- **GAP** — codebase does not follow the rule at all

Record the results. Fix all GAPs before your first pipeline run (Step 4).
Track PARTIALs as backlog items with target dates.

> **Why this matters.** Without this audit, conformance gaps compound silently.
> Stone-arch discovered 8 PARTIAL and 1 GAP issues only after an explicit
> post-adoption audit — including a pre-commit hook bug that had silently
> failed on all 11 governance commits. The Step 0 assessment covers security
> and PII but not rule conformance; this step fills that gap.

### 1.5 If using AI agents: create system prompts

Translate each YAML definition into a system prompt using the three-section
format from framework.md Section 6:
1. Identity and Jurisdiction ("You are the Coder-Tester agent. Your purpose is...")
2. Boundaries ("FORBIDDEN ACTIONS — you must NEVER...")
3. Process ("WORKFLOW: 1. Receive a task... 2. Verify it has approval...")

Restrict each agent's tool access to only what its `capabilities.allowed` lists.

### Step 1 output

- Role assignment table filled in
- Framework and agent versions recorded in `adoption` section of learnings YAML
- Three YAML definitions adapted for your project (customizations documented)
- Assistant can read `agf/rules/`; any root loader points to that canonical
  rule directory (Step 1.4)
- Assistant can read `agf/skills/`; any tool-specific skill wrappers point to
  the canonical skill definitions (Step 1.4b, if using AI assistants)
- Codebase audited against adopted rules; GAPs fixed, PARTIALs tracked (Step 1.4a)
- System prompts created (if using AI agents)

---

## Step 2: Define Your Pipeline

Map the governance pipeline to your existing Git/CI workflow.

### 2.1 Map the pipeline to your workflow

The governance pipeline has 7 steps. Map each to something concrete in your
workflow:

| Pipeline Step | Maps To (example) | Your Mapping |
|---------------|-------------------|-------------|
| 1. Requirement | GitHub Issue / Jira ticket / Slack message | __________ |
| 2. Plan | Branch + plan doc or PR description | __________ |
| 3. Review gate | PR review by Governor-Reviewer | __________ |
| 4. Implement | Commits on feature branch | __________ |
| 5. Verify | CI pipeline runs tests + checklists | __________ |
| 6. Review gate | PR approval before merge | __________ |
| 7. Commit/merge | Merge to main/protected branch | __________ |

### 2.2 Define gate rules

For each gate, decide what is required:

**Gate 1 (plan approval):**
- [ ] Plan exists (written description of what and why)
- [ ] Scope is bounded (not open-ended)
- [ ] Governor-Reviewer has approved

**Gate 2 (merge approval):**
- [ ] All tests pass
- [ ] Cross-cutting checklists pass (see Step 3)
- [ ] Is this the simplest solution that meets the requirements and allows roadmap items development?
- [ ] Governor-Reviewer has approved the implementation
- [ ] No unresolved escalations

**Fast track (low-risk tasks):**
The Governor-Reviewer can waive Gate 1 for small, well-understood, low-risk
tasks — collapsing the pipeline to a single gate (Gate 2 only). Rules:
- Only the Governor-Reviewer decides; agents cannot self-authorize
- Record the waiver: what task, why low-risk, who waived
- Gate 2 still applies in full (tests, checklists, approval)
- If the task turns out bigger than expected: stop and escalate, don't continue without a plan
- Not for tasks involving PII, security boundaries, architecture, or data model changes

See framework.md Section 4, Pattern 6 for full details.

### 2.3 Write your project's GOVERNANCE.md

Copy this repo's `GOVERNANCE.md` as a starting point. Adapt:
- Role assignments (from Step 1)
- Pipeline mapping (from Step 2.1)
- Gate rules (from Step 2.2)
- Artifact ownership (who owns what in your project)

### 2.4 Set up Architecture Decision Records

Create a `docs/adr/` directory in your project. Copy the Architecture Decision Record (ADR) template:

```bash
mkdir -p docs/adr
cp agf/docs/adr/_template.md docs/adr/
cp agf/docs/adr/README.md docs/adr/
```

Record significant architecture decisions as they arise — technology
choices, data model designs, integration approaches, scope boundaries.
See `docs/adr/README.md` for the required format and lifecycle.

Name ADR files `YYYYMMDD-short-slug.md`, matching the framework's own ADR
convention. Sequential numbers (`0001-*.md`) create avoidable merge conflicts
in branch-based and multi-agent workflows.

You do not need to backfill past decisions immediately. Start recording
from now; backfill selectively when a past decision becomes relevant
(e.g., someone asks "why did we choose X?").

### Step 2 output

- Pipeline mapping table filled in
- Gate rules defined
- Project GOVERNANCE.md written and committed
- `docs/adr/` directory created with template

---

## Step 3: Activate Cross-Cutting Checklists

In Phase 1, cross-cutting agents do not exist as standalone agents. Instead,
their concerns are embedded as checklists that the Governor-Reviewer applies
at each gate. This is the accelerated path — you get the protection without
the agent overhead.

Cross-cutting checklists operate at **two cadences** (see framework.md
Section 4, "Two Cadences of Cross-Cutting Review"):
- **Per-change items** — checked at every Gate 2 (PR/commit review)
- **Per-milestone items** — checked at defined milestones (before release,
  before demo, end of sprint). These catch structural issues that no
  individual change introduced.

### 3.1 Privacy and data checklist (if PII present)

The Governor-Reviewer checks these at every review gate:

**At plan review (Gate 1):**
- [ ] If the task involves PII: is there a data flow description?
- [ ] Is consent handling specified for any new data collection?
- [ ] Are data retention requirements stated?
- [ ] If data crosses borders: are transfer safeguards identified?

**At code review (Gate 2) — per-change:**
- [ ] No PII logged in plain text (check log statements)
- [ ] No PII in URLs or query parameters
- [ ] PII access is authenticated and authorized
- [ ] Data at rest is encrypted (or encryption is documented as a known gap)
- [ ] Deletion/erasure is cascade-complete (no orphaned PII)
- [ ] No new PII storage without corresponding retention policy
- [ ] Input validation on all PII fields
- [ ] Database migrations preserve existing data integrity
- [ ] Concurrent write operations are safe (no duplicate records from stale state)

**Per-milestone:**
- [ ] Data Subject Access Request (DSAR) endpoint exists: can export all data for a single user (not just org/entity-scoped)
- [ ] Deletion verification: can confirm all PII for a user is actually removed
- [ ] Retention policies are enforced, not just documented
- [ ] Data flow diagram is up to date with actual PII paths

### 3.2 Security checklist (all projects)

**At plan review (Gate 1):**
- [ ] If the task changes auth/authz: is the change documented?
- [ ] If new dependencies: are they audited for known CVEs?
- [ ] If new API endpoints: are they authenticated?

**At code review (Gate 2) — per-change:**
- [ ] No hardcoded secrets, credentials, or API keys
- [ ] No Structured Query Language (SQL) injection vectors (parameterized queries)
- [ ] No Cross-Site Scripting (XSS) vectors (output encoding)
- [ ] No command injection vectors (input sanitization)
- [ ] Dependencies pinned to specific versions (not floating)
- [ ] No unnecessary permissions requested or granted
- [ ] Error messages do not leak internal details to users

**Per-milestone:**
- [ ] Full dependency Common Vulnerabilities and Exposures (CVE) audit (not just new dependencies)
- [ ] All SPDX licence headers present on source files
- [ ] Root LICENSE file exists
- [ ] Supply chain integrity: build artifacts signed (if distributing)
- [ ] Access control documentation is current
- [ ] No unauthenticated admin/management endpoints — or each has formal
      risk acceptance signed by a human with date and rationale recorded
- [ ] All security gaps identified in Step 0.3 are either resolved or have
      formal risk acceptance (see Step 5.3 Lifecycle Gates)

### 3.3 Accessibility checklist (if user-facing)

**At code review (Gate 2) — per-change:**
- [ ] Interactive elements have accessible names (aria-label or visible text)
- [ ] Colour is not the only means of conveying information
- [ ] Colour contrast meets WCAG AA (4.5:1 for text)
- [ ] Keyboard navigation works for all interactive elements
- [ ] Form fields have associated labels
- [ ] Images have alt text (or are marked decorative)
- [ ] All form inputs use the shared input component (no raw HyperText Markup Language (HTML) bypassing a11y attributes)

**Per-milestone:**
- [ ] Scaffold/generator boilerplate removed from production UI
- [ ] App-level navigation exists and works (not just in-page back-links)
- [ ] Navigation is consistent across all pages (tabs, menus appear everywhere they should)
- [ ] No internal IDs (UUIDs, database keys) exposed as user-editable fields
- [ ] No placeholder/TODO/stub text visible in user-facing strings
- [ ] Full keyboard-only walkthrough of primary user journeys

### 3.4 Localization checklist (if multi-locale planned)

**At code review (Gate 2) — per-change:**
- [ ] No user-facing strings hardcoded in source files
- [ ] Date, time, and number formatting uses locale-aware functions
- [ ] UI layout accommodates text expansion (translations are often 30% longer)
- [ ] No assumptions about text direction (Left-to-Right (LTR) / Right-to-Left (RTL))

**Per-milestone:**
- [ ] All user-facing strings are in the translation system (not just new ones)
- [ ] Locale test matrix covers all supported locales
- [ ] RTL layout tested (if applicable)

### 3.5 Data integrity checklist (if persistent data)

**At code review (Gate 2) — per-change:**
- [ ] Schema changes match the corresponding code model (field names, types, nullability)
- [ ] Migrations have rollback safety
- [ ] No mutable operations where immutability policy requires append-only

**Per-milestone:**
- [ ] Schema parity check: code models match actual database schema. **Automate this in CI** — manual review consistently misses field name, type, and nullability mismatches. Use your Object-Relational Mapping (ORM) introspection capabilities (e.g. Ecto: compare schema module fields against `mix ecto.dump`; Alembic: `alembic check` or migration graph validation)
- [ ] Concurrent write safety: no stale-state mutations that produce duplicate records
- [ ] Data lineage is documented for all critical data paths
- [ ] Backup and recovery procedures are tested

### 3.6 Put checklists in your review process

Choose an activation mode for each cross-cutting concern (see framework.md
"Cross-Cutting Activation Modes" for the full spectrum):

**`checklist-in-governor` (default):** The Governor-Reviewer manually applies
the relevant checklists at Gate 2. Simple to set up but relies on discipline.

**`gate-triggered-review` (recommended for projects with CI or AI tooling):**
Attach the checklist to a gate event so it fires automatically. Options:

```bash
# Option A: CI job triggered on MR (GitLab example)
# Add a job that runs the security checklist against changed files
security-review:
  stage: review
  script:
    - "# planned AGF v2 command: agov validate infosec --include-md"
  rules:
    - if: $CI_MERGE_REQUEST_IID

# Option B: AI agent prompted at MR creation
# Include the cross-cutting checklist in the AI reviewer's system prompt
# so it evaluates every MR against the relevant items automatically

# Option C: Pre-merge hook
# Run checklist validation as a pre-merge check that blocks merge on findings
```

**Scope triggers by change type** to avoid noise:
- Security checklist → code changes, dependency changes
- Accessibility checklist → UI/frontend changes
- Data integrity checklist → schema/migration changes
- Privacy checklist → changes touching PII fields or user data

**Per-milestone items:** schedule at defined milestones. Add to your calendar
or sprint process. The Governor-Reviewer (or Human directly) walks through
the per-milestone items before each release, demo, or sprint boundary.
Per-milestone items are not gate-triggered — they require human judgment
about the system as a whole.

**Migration from existing modes:**
- If you're using `manual-review` and checklists are being skipped → upgrade
  to `gate-triggered-review`
- If you're using `checklist-in-governor` and the governor is an AI →
  consider `gate-triggered-review` to make the trigger explicit
- Record the mode change in your learning record's `activated_as` field

### Step 3 output

- Relevant checklists selected based on Step 0 assessment
- Activation mode chosen for each cross-cutting concern
- Per-change items embedded in PR template, CI job, or AI reviewer prompt
- Per-milestone items scheduled at defined cadence
- Governor-Reviewer knows the difference: per-change at every gate,
  per-milestone at milestones only

---

## Step 4: Run Your First Task Through the Pipeline

Pick a small, real task — not a test exercise. Something that would normally
take a day or less: a bug fix, a small feature, a configuration change.

### 4.1 Walk through the pipeline

Follow each step explicitly, even if it feels ceremonial:

1. **Requirement:** Write it down. Create an issue or document.
2. **Plan:** The Planner-Architect writes a brief plan: what will change, why, which files.
3. **Gate 1:** The Governor-Reviewer reviews the plan. Approve, reject, or ask questions.
4. **Implement:** The Coder-Tester implements the plan. No more, no less.
5. **Verify:** The Coder-Tester runs tests and applies the cross-cutting checklists from Step 3.
6. **Gate 2:** The Governor-Reviewer reviews the implementation against the plan and checklists.
7. **Commit:** Merge to main. Done.

### 4.2 Record what happened

After the task, answer these questions honestly:

- What felt useful? (Keep doing it.)
- What felt like overhead? (Might be overkill — record it.)
- What was missing? (Might be a gap — record it.)
- Did anyone step outside their role? (Capability drift signal.)
- Did the checklists catch anything? (Validation signal.)
- Did anything get escalated? (Escalation path is working.)

### 4.3 Record your first learning

Add a dated entry to the `learnings` section of your project's YAML file.
Tag it with the appropriate category (gap, overkill, validation, adaptation, tension).

### Step 4 output

- One real task completed through the full pipeline
- Honest retrospective documented
- First learning recorded

---

## Step 5: Set Up Your Environments

Apply the policy gradient from framework.md Section 8 to your actual
infrastructure.

### 5.1 Enforce data rules per environment

| Environment | PII Allowed? | Action |
|-------------|-------------|--------|
| Development | No | Use synthetic/fake data generators. Strip PII from any seed data. |
| Testing/CI | No | Test fixtures use synthetic data. CI never connects to production databases. |
| Demo/Staging | No (anonymized OK) | Anonymize or synthesize. Never demo with real customer data. |
| Production | Yes (with full controls) | Full privacy, security, and data integrity policies enforced. |

### 5.2 Define deployment gates

Even without a standalone Deployer agent, define what must be true before
code moves between environments:

**Dev → Testing/CI:**
- [ ] All unit tests pass
- [ ] Linter/formatter passes
- [ ] No hardcoded secrets

**Testing/CI → Demo/Staging:**
- [ ] All integration tests pass
- [ ] Cross-cutting checklists pass
- [ ] Governor-Reviewer has approved

**Demo/Staging → Production:**
- [ ] All e2e tests pass (if they exist)
- [ ] Security checklist passes
- [ ] Privacy checklist passes (if PII project)
- [ ] No known critical vulnerabilities in dependencies
- [ ] Rollback plan exists

### 5.3 Lifecycle gates

Deployment gates (5.2) control code moving between environments.
Lifecycle gates control the **project itself** moving between stages:
early-development → beta → production. These are different — a project
can deploy to a staging environment while still in early-development,
but it should not accept paying customers until lifecycle gates pass.

Lifecycle gates are enforced by the Governor-Reviewer (or Human directly).
Before the project's `lifecycle_state` in its learnings YAML can change,
the following must be verified:

**Early-development → Beta (first external users):**
- [ ] All `critical` security gaps from Step 0.3 are **resolved** (not just documented)
- [ ] All `significant` security gaps from Step 0.3 are either **resolved** or
      have **formal risk acceptance**: a dated record, signed by a human, with
      rationale for why the risk is acceptable at this stage
- [ ] No unauthenticated admin/management endpoints without formal risk acceptance
- [ ] If PII present: privacy per-milestone checklist passes
- [ ] At least one full per-milestone cross-cutting review completed

**Beta → Production (paying customers):**
- [ ] All `critical` and `significant` security gaps are **resolved** (risk
      acceptance is no longer sufficient — fix or remove)
- [ ] All per-milestone checklists pass (security, privacy, accessibility,
      data integrity, localization — whichever are active)
- [ ] SLOs defined and monitoring in place (see SaaS addendum)
- [ ] Incident response roles assigned (see SaaS addendum)
- [ ] Environment gradient enforced: production at maximum policy level

**What formal risk acceptance looks like:**
A risk acceptance is not "we know about it." It is a record with:
- Date
- The specific risk being accepted
- Who accepted it (name, not role — a human, not an agent)
- Why the risk is acceptable at this stage
- Conditions under which it must be resolved (e.g., "before first paying customer")
- Review date (when to re-evaluate)

Store risk acceptances in the project's learnings YAML or in a dedicated
`risks/` directory. The Governor-Reviewer checks them at each lifecycle gate.

**The key principle:** Documentation is not enforcement. An ADR that says
"we chose not to add auth" is a design decision. A risk acceptance that says
"admin endpoints are unauthenticated, accepted by Jani on 2026-03-22, must
be resolved before beta" is enforcement. The difference is accountability
and a deadline.

### 5.4 Document environment configuration

Add the `environments` section to your project's learnings YAML file.

### 5.5 CI artifact hygiene

CI job artifacts accumulate silently in GitLab project storage unless you
apply explicit expiry policies. Left unmanaged, repeatable build outputs
(preview archives, docs builds, review-only jobs) can grow to many gigabytes
while the actual repository stays small.

A rule of thumb:

| Artifact purpose | Expiry |
|---|---|
| Preview/review — available until MR is decided | 1 day |
| Tag/release build — needed until release job uploads to registry | 1 week |
| Durable release binary — uploaded to package registry | Never (uploaded, artifact path then expired) |
| Test reports, coverage, linting output | 1 day or less |

**To avoid unbounded growth:**
- Declare `expire_in:` on every `artifacts:` block. Do not leave it unset.
- Upload durable release binaries to the package registry before the CI
  artifact expires. Once the package registry copy exists, the CI artifact
  path is redundant.
- Periodically check GitLab project storage statistics
  (`Settings → Usage Quotas`) and delete stale preview/docs-build job
  artifacts rather than deleting packages or releases.
- When CI artifact storage dominates the project size, audit expiry values
  before concluding that the project itself has grown.

### Step 5 output

- Data rules enforced per environment (especially: no real PII outside production)
- Deployment gates defined
- Lifecycle gates defined with explicit risk acceptance requirements
- Environment configuration documented

---

## Step 6: Start Recording Learnings

### 6.1 Set your review cadence

Choose one:
- [ ] After every 3rd learning record, review accumulated learnings
- [ ] Monthly, review all learnings from that month
- [ ] Before each sprint/cycle, review learnings from the previous one

Put it on a calendar. It will not happen if it is not scheduled.

### 6.2 Feed learnings back to the framework

If you adopted the framework from the agents-govern repository:
- Submit significant learnings (gaps, tensions, adaptations) as issues or PRs
  to the framework repo
- This helps the framework evolve based on real evidence (see docs/governance/learning-system.md)

### 6.3 Learning record sync policy

Your project's local `agf/learnings/<codename>.yaml` is the **source of truth**
for your project's evidence. It is adopter-owned and must be preserved across
every framework upgrade.

**During a framework tarball sync or upgrade:** upstream-owned files in `agf/`
may be added, changed, or deleted — that is expected and correct. Your files
under `agf/learnings/` are evidence, not framework content. Never let a sync
script, `cp -r`, or automated upgrade step overwrite or delete them. If you
are unsure whether a file is adopter-owned or upstream-owned, check `agf/MANIFEST.md`.

The upstream copy in the agents-govern repo is a submitted snapshot.

When upgrading the framework:
- **Do not overwrite** your local learning YAML with the upstream copy
- Submit new learnings via the quick path (GitLab issue) — the Sentinel
  integrates them into the upstream copy
- If you need to verify upstream integration, compare your local file
  against the upstream version after the Sentinel processes your submission

### 6.4 Upgrade cadence

You do not need to upgrade every time the framework releases a new
version. Pin to a minor version and upgrade when:
- A new release contains a feature or fix your project needs
- Your learning record references a framework version newer than yours
- A security-relevant change is announced

For fast-moving upstream periods (multiple releases per day), wait for
the dust to settle — upgrade to the latest patch once the release
cadence stabilises.

**Audit upstream changes on upgrade.** When upgrading to a new framework
version, run your Contribution Auditor against the diff between your
current version and the new one. Upstream is not exempt from auditing —
the same red flags apply: governance relaxation without evidence, boundary
erosion, absence-bias reasoning. This is not distrust of upstream; it is
the same standard applied uniformly to all guidance entering your project.

**Adopter documentation parity on upgrade.** Upstream release preparation runs
`agov validate adopter-docs` so framework/tooling changes that affect adopters
must ship with downstream-facing docs/templates or a governor-visible waiver.
When reviewing a new release, check the changelog for `Adopter docs: updated`,
`Adopter docs: not applicable`, or `Adopter docs: waived <coordination-id>` on
adopter-relevant entries, then review the named docs before applying the
upgrade.

**Configuration drift review on upgrade.** Each upgrade is also an
opportunity to catch drift between your customized configuration and the
new framework defaults. After applying the upgrade, check these four areas:

- **Agent definition divergence.** Compare your `agf/agents/*.yaml` files
  against the new `agf/templates/` to find fields the upstream added that
  your customization doesn't carry yet. New fields are usually optional, but
  missing a new `advisory_phases:` entry or `escalates_to:` pointer means
  your agents won't use the new guidance.
- **Rule customization points.** Search for `<!-- CUSTOMIZE -->` comments in
  upgraded rule files. If a rule's customize section changed, your local
  customization may no longer cover the intended cases.
- **Skill source paths.** If the upgrade moves a skill or renames a command,
  any assistant loader (`CLAUDE.md`, `AGENTS.md`) that hard-codes the old
  path will silently fall back to built-in behavior.
- **Schema version.** If `agf/schemas/` changed, run `agov validate` to
  confirm your coordination and learning files still pass.

### Step 6 output

- Review cadence chosen and scheduled
- Feedback channel established
- Learning record sync direction understood (local is source of truth)

---

## Step 7: Grow When It Hurts

Do not split agents or add new ones on a schedule. Split when you feel
the specific pain that the split addresses.

### Split signals to watch for

| Signal | What It Means | Action |
|--------|--------------|--------|
| Planner-Architect produces plans that conflict with its own designs | Planning and architecture need separate jurisdictions | Split into Planner + Architect (Phase 2) |
| Coder-Tester misses bugs in its own code | Testing your own work is failing | Split into Coder + Tester (Phase 2) |
| Governor-Reviewer is bottlenecked | Review workload slows governance | Split into Governor + Reviewer (Phase 3) |
| Cross-cutting checklists are routinely skipped or rushed | Checklists need enforcement, not goodwill | Extract into standalone cross-cutting agents (Phase 3-4) |
| Bugs escape one test level that should have caught them | Single Tester cannot cover unit + integration + e2e + system | Split testing hierarchy (Phase 5) |
| CI/build changes conflict with code changes or waste resources | Build pipeline needs its own jurisdiction | Activate Build Engineer (Phase 1) or merge with Deployer (Phase 3) |
| The project has its first contribution to the framework or an adopter project | Contributions need adversarial/soundness screening before entering the corpus | Activate Contribution Auditor (Phase 1 — library-core from Phase 1, but not always active; see `framework.md` §7) |
| The project has its first learning record submission | Learning records need information-leak screening before publication | Activate InfoSec Sentinel (Phase 1 — library-core from Phase 1, but not always active; see `framework.md` §7) |
| Multiple learnings point to the same agent capability gap | Agent definitions need systematic evolution | Activate Capability Developer (Phase 4) |

### How to split

1. Take the combined agent's YAML definition
2. Create two new definitions, each with half the responsibilities
3. Update `role_mapping` notes (they exist for this purpose)
4. Update communication rules (new agents need explicit channels)
5. Update your GOVERNANCE.md
6. Record the split as a learning

### When to activate the Capability Developer (Phase 4)

The `capability-developer.yaml` agent is a meta-tier agent that analyzes
your accumulated learnings and proposes changes to agent definitions. It
is **reactive only** — it does not scan proactively. Activate it when:

1. **You have 10+ learnings** across your project, with recurring themes
   (e.g., the same gap category appears 3+ times).
2. **Escalations repeat** — the same type of escalation happens 5+ times,
   suggesting a boundary is drawn too tightly or a capability is missing.
3. **Phase split signals fire** — an agent's defined split condition
   (in its YAML `phase_notes`) is observed in practice.

**How to apply it:**

1. Review `agf/agents/capability-developer.yaml` and activate it in your
   local role manifest when the trigger conditions apply.
2. Feed it your learnings, escalation logs, and agent definitions.
3. Ask it to analyze patterns and produce proposals.
4. All proposals go through the Governor-Reviewer gate and require human
   approval before agent definitions are changed.

**What it does NOT do:**

- It does not modify agent definitions (it proposes changes).
- It does not act on single-project, single-instance evidence.
- It does not activate without a trigger (no periodic scanning).
- It does not approve its own proposals.

**Example application:** After accumulating learnings, run the capability
developer against your evidence. If it finds corroborated patterns (e.g.,
"CI/build files ungoverned" reported multiple times), it will produce a
proposal with evidence citations, affected agents, and a recommended
change. You review the proposal, approve or reject it, and apply it
through your governance pipeline.

### Run the Capability Developer periodically

Even if evidence thresholds haven't been formally met, schedule a
periodic capability-development review — like a health check for your
agent definitions. Suggested cadence:

| Project activity | Cadence |
|-----------------|---------|
| Active development (weekly commits) | Every 5-10 learnings or monthly |
| Maintenance mode | Quarterly or after any incident |
| Post-milestone / post-launch | Once, immediately after |

**What to ask it:**

- "Analyze my learnings and agent definitions. Are there recurring gaps?"
- "Check communication symmetry and ownership overlaps."
- "Are any phase split signals being met?"
- "Which agents are still at v0.1.0 — are they exercised or dead weight?"

**For AI agents:** When playing the capability-developer role, read all
files in `agf/learnings/`, `agf/agents/`, and any review documents. Cross-reference
learning categories (gap, overkill, tension) against agent names in
`agents_involved`. Report patterns that appear 3+ times. Produce proposals
in the format shown in the capability-developer YAML definition.

**For humans:** Once the agent validator ships, run it and review the phase
distribution and communication graph. Until then, compare `agf/agents/*.yaml`
against your governance file manually. Read your last 5-10 learnings and ask:
"Is the same pain showing up repeatedly? Does an agent definition need to
change?"

Planned command shape: `agov validate agents ../../agf/agents/*.yaml
--graph > docs/agent-communication.md`.

The capability-developer is not a replacement for human judgment — it is a
systematic way to surface the evidence that human judgment acts on.

### Step 7 output

- Split signals documented in your GOVERNANCE.md
- Splits happen when pain is felt, not on a schedule
- Capability Developer activated when learning patterns warrant it
- Periodic capability-development reviews scheduled

---

## SaaS-Specific Addendum

If you are moving toward publishing as SaaS, these concerns apply on top
of the base framework.

### Multi-tenancy

- [ ] Data isolation: can Tenant A's data ever leak to Tenant B?
- [ ] Test for it: write explicit cross-tenant data access tests
- [ ] Query patterns: do all data queries filter by tenant ID?
- [ ] Deployment: is each tenant isolated at the infrastructure level, or logical?
- [ ] Monitoring: can you detect cross-tenant data access attempts?

### Licensing and IP

- [ ] Code-available does not mean code-exposed: verify that SaaS deployment
      does not expose proprietary source code to end users
- [ ] Dependency licences: audit all dependencies for licence compatibility
      with your SaaS model (General Public License (GPL) dependencies in a proprietary SaaS can be
      a legal problem)
- [ ] Open-source contributions: if you use open-source, are you compliant
      with contribution requirements?

### Customer data boundaries

- [ ] Data Subject Access Requests (DSARs): can you export all of a
      customer's data on request?
- [ ] Right to erasure: can you delete all of a customer's data on request,
      including backups?
- [ ] Data residency: do you know where customer data is physically stored?
      Can customers choose their region?
- [ ] Data processing agreement: do you have a Data Processing Agreement (DPA) template for enterprise
      customers?

### SLOs (define before launch)

- [ ] Availability target: what uptime do you promise? (e.g., 99.9%)
- [ ] Latency target: what response time? (e.g., p95 < 500ms)
- [ ] Data durability: what is your Recovery Point Objective (RPO)?
- [ ] Recovery time: what is your Recovery Time Objective (RTO)?
- [ ] Define these BEFORE launch. Defining SLOs after customers are angry
      is too late.

### Incident response

Map incident response to the framework:

| Incident phase | Who acts | Framework role |
|---------------|----------|---------------|
| Detection | Observer agent (or monitoring system) | Observer |
| Triage | Human on-call | Human |
| Decision | Team lead | Governor |
| Fix | Developer | Coder |
| Verification | QA / automated tests | Tester |
| Deployment | Release engineer / CI | Deployer |
| Post-mortem | Team | Sentinel (system-level review) |
| Learning | Everyone | Record in learnings YAML |

---

## For Independent Adopters

If you are adopting agents-govern without having been involved in its
design, your experience is especially valuable. All current evidence
comes from the framework's designer — which means the framework may
have blind spots that only an outsider can see.

### What we need from you

- **Honest learnings**, especially:
  - **Overkill findings** — parts of the framework that added overhead
    without value. Zero overkill at 194 learnings is suspicious.
  - **Onboarding friction** — where the docs confused you, where you
    got stuck, what assumptions weren't obvious.
  - **Boundary disagreements** — places where the Verb Test or
    jurisdiction rules felt wrong for your context.
- **Your context matters:** team size, domain, tech stack, and AI
  tooling all affect how the framework fits.

### Minimum viable adoption

You do not need all agent definitions. Start with a few:

- **Solo developer with AI:** Use `coder-tester.yaml` (AI) +
  Governor-Reviewer (you). This is the most common configuration.
  Add `build-engineer.yaml` when the project has a CI pipeline.
- **Academic / documentation:** Use `academic-peer-reviewer.yaml` +
  Governor-Reviewer (you).
- **Add more agents only when you feel pain** — the "grow by pain"
  principle applies to adoption too.

### How to contribute learnings

1. Fork the repository (or submit via email if you prefer).
2. Copy `agf/templates/learning-record.yaml` to
   `agf/learnings/<your-codename>.yaml`.
3. Fill in the project context and add your learnings.
4. Validate the learning against `agf/schemas/learning.schema.json` and run
   `agov validate` to check infosec and anonymization concerns.
5. Submit a merge request. The framework maintainer will review.

Your project's real name is never used — only the codename. See
[anonymization-notice](docs/governance/anonymization-notice.md) for the
privacy policy.

### What "independent" means

You did not design the framework and are not the same developer who
created it. You bring a different perspective, different assumptions,
and different project constraints. That is exactly what makes your
feedback valuable.

---

## Quick Reference: What to Do When

| Situation | Action |
|-----------|--------|
| Starting fresh | Begin at Step 0 |
| Already have CI/CD and code review | Skim Step 0, start at Step 1 |
| PII in the codebase | Step 0.2 is urgent. Step 3.1 is mandatory. |
| No tests at all | Step 0.4 is your priority. Add testing before governance. |
| Solo founder with AI agents | One human plays Governor-Reviewer. AI plays both other roles. |
| Team of 5+ | Assign Governor-Reviewer to a dedicated person from day one. |
| Pre-launch SaaS | Complete all steps + SaaS addendum before launch. |
| Post-launch SaaS | Start at Step 0, apply retroactively, prioritize gaps from assessment. |
| Academic / documentation project | Pipeline maps naturally (plan→draft→review→verify→commit). Skip most cross-cutting agents. Verifiable-claims and implementation-honesty rules are high-value. See academic agents in `agf/agents/`. |
| Mixed artifact types (docs + code) | One set of agent definitions, one GOVERNANCE.md, but split pipelines and checklists per artifact type. |
| Framework files outside `agf/` | Move framework-owned files under `agf/`; v2.0 supports the single visible `agf/` directory. |
