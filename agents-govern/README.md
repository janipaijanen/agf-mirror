<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# agents-govern

[![Cite this repository](https://img.shields.io/badge/cite-CITATION.cff-blue)](CITATION.cff)

A governance framework for multi-agent Artificial Intelligence (AI)
systems in software development.

When multiple AI agents work together on a codebase -- planning, coding,
testing, reviewing, deploying -- they need boundaries, quality gates, and
accountability. Without these, you get authority conflicts, capability
drift, accountability gaps, and knowledge decay. This framework defines
the structure to prevent those failures.

> 📖 **Prefer to browse?** The docs site at
> [**janipaijanen.gitlab.io/agent-governance**](https://janipaijanen.gitlab.io/agent-governance/)
> renders this content with search, navigation, and a browsable
> evidence corpus. Same source of truth, friendlier shape for
> newcomers and researchers. The README below stays repo-flavoured
> for adopters who already have the archive extracted.

### Where to look first

Three reader-intent paths plus a *try-it* command, sized for a
ten-minute first-contact budget:

- **What is this, exactly?** → [docs/position/overview.md](docs/position/overview.md)
- **Try it in 30 seconds.** Run
  `uv run --with agents-govern agov-validate-adoption` in any
  directory to see what the framework checks (governance-surface
  conformance summary, exits in under a second).
- **I want to adopt it.** → [ADOPTING.md](ADOPTING.md) — start at
  Step 0 (project assessment).
- **I want to read the framework theory.** → [framework.md](framework.md).
- **I'm an AI agent or AI architect reading framework artifacts.** →
  [framework.md "Appendix: AI Consumption of Framework Artifacts"](framework.md#appendix-ai-consumption-of-framework-artifacts-provisional)
  *(Provisional).*

*(Provisional 2026-05-04: routing block added after an external
Chief Technology Officer (CTO) bounced at the ten-minute filter;
corroborated by analytical follow-up. Second independent
empirical encounter would promote off Provisional. 2026-05-08:
extended with the Provisional AI-consumption route, sourced from the
`linked-data-probe` research thread.)*

### What this is, what this is not

**What this is:** Governance of *multi-agent collaboration in software
development workflows* — the boundaries, gates, and accountability needed
when AI agents (and humans) collaborate to plan, code, review, test, and
deploy software.

agents-govern describes the governance layer of an agentic operating
system: process definitions, permissions, communication contracts,
audit trails, gates, and conformance checks, while deliberately
delegating runtime execution to Integrated Development Environment
(IDE)-resident agents and other external runtimes.

**What this is not:**

- *Not* a general AI governance platform. The framework does not govern
  model behavior, prompt content, or AI products as artifacts.
- *Not* an ML governance framework. It does not handle experiment tracking,
  model cards, dataset lineage, evaluation sign-off, or drift monitoring.
  ML/research governance can be layered on top as an adopter profile (see
  [profiles/](profiles/)) but is not in the core.
- *Not* an agent runtime or orchestrator. It defines roles, gates, and
  artifacts; it does not execute agents.
- *Not* a runtime enforcement mechanism. The framework defines
  boundaries, authority, and gates; it does not mechanically enforce
  them. Tool permissions, branch protections, Continuous Integration
  (CI) checks, pre-commit hooks, and review-gate workflows must be
  supplied by your environment. Process discipline + environment
  enforcement = governance; process discipline alone is at-will.
- *Not* an agent tool-call authorisation framework. *"Governance"*
  here is over Software Development Lifecycle (SDLC) artefacts and
  gates — what an agent produces, whether reviews pass, whether
  evidence accumulates — not over which tools an agent may invoke
  at runtime. For runtime tool-call permissioning, see Model
  Context Protocol (MCP) authorisation specifications and similar
  runtime permission systems. *(Provisional 2026-05-04 addition:
  external Chief Technology Officer (CTO) reviewer first read
  "governance" as tool-call authentication; analytical second
  opinion corroborated. Second independent empirical encounter
  would promote off Provisional.)*

**Non-goal (hard boundary):** this framework does not enforce agent
runtime behavior. If you need containment, sandboxing, execution
blocking, or tool-call permissioning, use infrastructure-level
solutions.

If you want governance for Machine Learning (ML) research or ML product
development, this framework can be *part of* your stack (governing the
Software Development Lifecycle (SDLC) of the software
that hosts your models) but is not the whole stack.

**Cross-domain pilot adopters wanted (Provisional).** Several AI services
have suggested the *mechanisms* layer (gates, evidence discipline, learning
records) generalises beyond SDLC. The stated scope above is unchanged until
2+ cross-domain pilot adopters file learning records — see
[docs/position/beyond-sdlc.md](docs/position/beyond-sdlc.md) for the
hypothesis and pilot steps.

**Version:** see [VERSION](VERSION) | **Status:** Early production — single-developer evidence base (16 codenamed projects, 232 learning records). [Maturity self-assessment](docs/status/maturity.md) | [Independent adopters wanted](#independent-adopters-wanted).

```
                        ┌─────────────────────┐
                        │   agents-govern      │
                        │   release archive    │
                        └─────────┬───────────┘
                                  │ download + extract
                                  ▼
                        ┌─────────────────────┐
                        │   Your Project       │
                        │                      │
                        │  agents/*.yaml       │ ← copy & customize
                        │  .claude/rules/      │ ← copy & customize
                        │  .claude/skills/     │ ← copy & customize (optional)
                        │  GOVERNANCE.md       │ ← adapt to your team
                        └─────────┬───────────┘
                                  │ run governed pipeline
                                  ▼
               ┌──────────────────────────────────────┐
               │  Plan → Code → Review → Test → Deploy │
               │       (with gates and checklists)     │
               └─────────┬────────────────────────────┘
                         │ encounter gaps, validations,
                         │ tensions, adaptations
                         ▼
                ┌────────────────────┐
                │ Record a learning  │
                │ (codename.yaml)    │
                └────────┬───────────┘
                         │ submit upstream
                         │ (issue / MR / email)
                         ▼
                ┌────────────────────┐
                │ Sentinel reviews   │──── corroboration ────┐
                │ accumulated        │     from multiple      │
                │ learnings          │     projects?          │
                └────────┬───────────┘                       │
                         │ yes                               │ no
                         ▼                                   ▼
                ┌────────────────────┐          ┌────────────────┐
                │ Framework revision │          │ Watch — await  │
                │ (cites evidence)   │          │ second project │
                └────────┬───────────┘          └────────────────┘
                         │
                         ▼
                ┌────────────────────┐
                │ New release        │───── adopters upgrade ─────┐
                │ archive            │                            │
                └────────────────────┘                            │
                         ▲                                        │
                         └────────────────────────────────────────┘
```

---

> **Do not clone this repository into your project.** If you are reading
> this on GitLab/GitHub, you are looking at framework development artifacts
> (sentinel reviews, 232 learning records from 16 projects, CI pipelines)
> that do not belong in your codebase. Go to
> [Releases](https://gitlab.com/janipaijanen/agent-governance/-/releases)
> and download the archive. The archive contains only the files you need,
> with a MANIFEST.md that tells you exactly what to copy and what is
> reference-only.

---

## For adopters

### Getting started

1. Go to the [**latest release**](https://gitlab.com/janipaijanen/agent-governance/-/releases/permalink/latest)
   page and download the `.tar.gz` or `.zip` archive.
2. Extract into your project (e.g., `agents-govern/` subdirectory or
   alongside your existing files).
3. Follow the quick start below.

If you prefer browsing to scrolling, the same getting-started flow is
on the [docs site → Adopt section](https://janipaijanen.gitlab.io/agent-governance/adopt/).

The archive contains only adoptable files -- see `MANIFEST.md` inside
for the full contents and provenance. Nothing in the archive is
framework-internal.

> **AI agents and humans:** Before creating governance files or
> directories, read [ADOPTING.md](ADOPTING.md) — specifically the
> "Choose your layout" section at the top. The framework supports two
> layouts (A: framework-as-project, B: framework-as-subdirectory) and
> file paths differ between them. Guessing the wrong layout is a silent
> failure that can persist through upgrades.

### Already adopted? Upgrading to a newer release

If your project already has agents-govern installed, use the
gated [`/upgrade-agents-govern`](skills/upgrade/SKILL.md) skill
against the latest release archive. The skill diffs upstream vs
local across the entire archive and walks an extract → compare →
apply → verify workflow with explicit confirmation gates.

For deep version gaps (e.g., pre-v0.40 → current) the recommended
path is **fresh re-adoption** rather than incremental upgrade —
the customisation surface has shifted enough across major releases
that a clean re-adoption is faster than walking ~80 versions of
breaking changes. See the
[**Upgrade guide**](https://janipaijanen.gitlab.io/agent-governance/adopt/upgrade/)
on the docs site for the decision tree, gotchas, and
re-adoption procedure.

### What the release archive contains

| File / Directory | Purpose | Action |
|------------------|---------|--------|
| [agents/](agents/) | YAML agent definitions (Phase 1 + Phase 3 + Phase 4) | Copy to your project, customize |
| [rules-template/](rules-template/) | Governance rules (`.claude/rules/`) -- Test-Driven Development (TDD), code review, branch discipline, security hardening, and more | Copy to `.claude/rules/`, customize |
| [skills/](skills/) | AI assistant workflows (`.claude/skills/`) -- framework upgrade, learning scaffolding, infosec screening | Copy to `.claude/skills/`, customize (optional) |
| [project-specific-examples/](project-specific-examples/) | Stack-specific rules: Python + Flask, SaaS multi-tenant (RLS), Tailwind + DaisyUI | Copy if your stack matches |
| [learnings/_template.yaml](learnings/_template.yaml) | Learning record template | Copy to `learnings/<codename>.yaml` |
| [framework.md](framework.md) | Framework theory and reference | Read-only reference |
| [ADOPTING.md](ADOPTING.md) | Step-by-step adoption guide | Follow step by step |
| [GOVERNANCE.md](GOVERNANCE.md) | Governance template | Basis for project-specific governance |
| [LEARNINGS.md](LEARNINGS.md) | Learnings guide and privacy policy | Read-only reference |
| [LESSONS-LEARNED.md](LESSONS-LEARNED.md) | Narrative lessons from adopters | Read-only reference |

### Prerequisites

The framework's validation scripts are written in Python and managed by
[uv](https://docs.astral.sh/uv/). **You do not need to install Python
yourself** — uv downloads and manages Python automatically.

Install uv (one-time):

```bash
# Linux / macOS
curl -LsSf https://astral.sh/uv/install.sh | sh

# Windows (PowerShell)
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

After installing uv, all validation commands (`uv run agov-validate-learnings`,
`uv run agov-validate-infosec`, etc.) work immediately — uv handles Python
and dependencies on first run.

### Quick start

**1. Assess your project.** Read [ADOPTING.md](ADOPTING.md) Step 0. Fill
in the Personally Identifiable Information (PII), security, maturity, and environment checklists. This
determines which agents and cross-cutting concerns are urgent for you.

**2. Assign Phase 1 roles.** The framework starts with 3 combined agents
plus an optional Build Engineer:

| Agent | Does what |
|-------|-----------|
| **Planner-Architect** | Plans work, makes design decisions |
| **Coder-Tester** | Implements and verifies |
| **Governor-Reviewer** | Enforces boundaries, reviews, approves |
| **Build Engineer** | Owns CI pipeline, build automation *(activate when project has CI)* |

A common setup: one human plays Governor-Reviewer, one AI (Claude, etc.)
plays Planner-Architect + Coder-Tester + Build Engineer. See
[ADOPTING.md](ADOPTING.md) Step 1 for details.

**3. Copy agent definitions, rule templates, and skills.** From the
extracted archive:

```bash
# Agent definitions (YAML)
cp agents/*.yaml <your-project>/agents/

# Rule templates (Claude Code .claude/rules/ files)
cp -r rules-template/ <your-project>/.claude/rules/

# Skills (optional -- AI assistant workflows, see ADOPTING.md Step 1.4b)
cp -r skills/ <your-project>/.claude/skills/

# Stack-specific rules (if applicable)
cp project-specific-examples/python-flask/always/* <your-project>/.claude/rules/always/
cp project-specific-examples/python-flask/workflow/* <your-project>/.claude/rules/workflow/
# etc.
```

Search for `<!-- CUSTOMIZE -->` markers in the copied rules and skills,
and adapt to your stack. Create a `.claude/CLAUDE.md` index -- see
[project-specific-examples/python-flask/CLAUDE.md](project-specific-examples/python-flask/CLAUDE.md)
for a template.

**4. Define your pipeline and run your first task.** Map the 7-step
governance pipeline to your Git/CI workflow ([ADOPTING.md](ADOPTING.md)
Step 2), activate cross-cutting checklists (Step 3), and run a real
task through the full pipeline (Step 4).

**5. Record and submit learnings.** Copy `learnings/_template.yaml` to
`learnings/<codename>.yaml` in your project. Use a codename, not your
real project name. Record what works, what does not, and what is
missing. Then submit upstream -- see
[How to contribute learnings](#how-to-contribute-learnings) below.

---

## For framework contributors

This section is for people developing the framework itself. If you are
adopting the framework in your project, use the
[release archive](#getting-started) -- not a clone of this repository.

### Repository contents (not in the release archive)

These files support framework development and are excluded from the
distribution by design (via [`.distignore`](.distignore) and the
explicit allowlist in [`uv run agov-dist`](uv run agov-dist)):

| File / Directory | Purpose |
|------------------|---------|
| [learnings/](learnings/) | Learning records from 16 adopter projects (anonymized). Evidence that drives framework evolution. |
| [reviews/](reviews/) | Sentinel review documents -- analysis and decisions from periodic reviews. |
| [uv run agov-dist](uv run agov-dist) | Distribution archive builder. |
| [.gitlab-ci.yml](.gitlab-ci.yml) | CI pipeline for validation, build, and release. |
| [.distignore](.distignore) | Patterns for repo-only files excluded from the distribution. |

---

## Key concepts

> **Visual overview:** See the
> [Architecture Overview diagram](framework.md#architecture-overview) in
> framework.md for how the framework's components connect.

**The Verb Test.** No two agents share a primary action verb. The Planner
plans. The Architect designs. The Coder codes. The Tester verifies. If
two agents do the same thing, their boundaries are wrong.

**Artifact Ownership.** Every artifact (plan, spec, code, test, deployment)
has exactly one owner. Consumers read from owners. Owners propose,
decision-makers approve.

**Immutability Constraint.** Past decisions are immutable. They can be
superseded by new decisions but never edited in place. This preserves
the audit trail.

**Escalation over Improvisation.** When an agent encounters something
outside its jurisdiction, the correct action is to escalate -- not to
guess or expand scope.

**Growth by Pain.** Do not build 20 agents on day one. Start with 3-4.
Split agents when you feel the specific pain that the split addresses
(see [framework.md](framework.md) Section 7).

**Convergence with established frameworks.** Several boundary principles
emerged organically from field experience and were later found to
converge with concepts in established decision-making frameworks --
notably McKinsey's DARE (single decision point per gate), Bain's RAPID
(severity-based advisory escalation), and the
Performer-Accountable-Control-Suggest-Inform (PACSI) model (quality gates as a
first-class function distinct from execution). The convergence was
unintentional: these principles were not derived from those frameworks
but arrived at independently by solving real alignment problems in
multi-agent systems. See the
[full comparative assessment](reviews/2026-04-12_raci-alternatives-assessment.md).

---

## How to contribute learnings

The framework evolves from field evidence, not theory. If you adopt it,
your learnings make it better for everyone.

### 1. Record learnings in your project

Copy `learnings/_template.yaml` (included in the release archive) to
your project as `learnings/<codename>.yaml`. Use a **codename** -- not
your real project name (see the anonymization policy in the template).

Tag each learning with a category:

| Category | Meaning |
|----------|---------|
| `gap` | The framework missed something your project needed |
| `overkill` | The framework prescribed something that added overhead without value |
| `validation` | A framework principle correctly predicted or prevented a problem |
| `adaptation` | You adapted the framework in a way worth incorporating |
| `tension` | Two framework principles conflicted in practice |

Include context: what happened, which agents were involved, which
framework section is relevant, and a recommendation.

### 2. Submit learnings upstream

Choose the method that works for you:

| Method | Best for | How |
|--------|----------|-----|
| **GitLab issue** | Quick observations, single findings | [Open an issue](https://gitlab.com/janipaijanen/agent-governance/-/issues/new) with the learning details. Use the category labels above. |
| **Merge request** | Complete learning records (YAML files) | Fork the repository, add your `learnings/<codename>.yaml`, and submit a merge request. |
| **Email** | Private submissions, sensitive context | Send your learning YAML to the Sentinel (see LEARNINGS.md for contact). |

All submissions must use codenames -- the anonymization policy is
mandatory. See `learnings/_template.yaml` for what to mask and what to
keep. Submissions containing real project, product, or organisation
names will be returned for anonymization.

### 3. What happens next

The framework Sentinel reviews accumulated learnings periodically (after
every 3rd learning, or monthly). Learnings that meet the decision
criteria in [LEARNINGS.md](LEARNINGS.md) trigger framework changes.
Not every learning becomes a change -- corroboration from multiple
independent projects is required.

### What makes a good contribution

- A learning record with concrete evidence (dates, outcomes, context)
- A rule template that generalises across tech stacks
- A stack-specific example in `project-specific-examples/`
- A tension report between two principles, with your attempted resolution
- An overkill finding -- what you skipped and why it was the right call

### Proposing rules or agents for the framework

If you created a rule or agent definition that worked well in your
project, you can propose it for inclusion in the framework. The path
from project-specific rule to framework rule template:

**1. Create and validate locally.** Write the rule in your project's
`.claude/rules/` directory (or agent definition in `agents/`). Use it
for real work. Record a learning when it prevents a problem or improves
outcomes.

**2. Check the promotion criteria.** A rule is ready for the framework
when it meets these conditions:

| Criterion | Why |
|-----------|-----|
| **Validated in your project** | You used it and it helped -- not theoretical |
| **Stack-agnostic (or clearly stack-scoped)** | Generic rules go to `rules-template/`, stack-specific rules go to `project-specific-examples/` |
| **No real project names** | Anonymize before submitting (codenames only) |
| **Self-contained** | The rule makes sense without reading your project's code |
| **Has `<!-- CUSTOMIZE -->` markers** | Adopters know what to change for their stack |

**3. Submit.** Choose one:

- **GitLab issue:** Describe the rule, paste the content, and reference
  the learning record that validates it. Best for discussion before
  committing effort.
- **Merge request:** Fork the repository, add the rule file to the
  appropriate directory, and submit. Include the evidence (which project
  codename, what category of learning, what it prevented or enabled).

**4. What happens next.** The Sentinel reviews proposed rules against
the [decision criteria in LEARNINGS.md](LEARNINGS.md):

- Rules validated by **two or more independent projects** are promoted
  to `rules-template/` (generic) or `project-specific-examples/`
  (stack-specific).
- Rules validated by **one project** are noted and watched -- they may
  be promoted when a second project corroborates.
- Rules that are **domain-specific** (e.g., cybersecurity exercise
  scenarios, medical device compliance) stay in the originating project
  and are referenced in that project's learning record for others to
  find.

The same process applies to **agent definitions**. If your project
created a new agent (e.g., a documentation writer, a deployment
engineer, a compliance advisor), submit the YAML definition with the
learning that justified its creation. The framework ships Phase 1
agents (3 combined) and Phase 3 agents (documentation-writer,
deployment-engineer); additional agents can be proposed when evidence
supports them.

### What does not belong in the framework

- Domain-specific rules (Health Insurance Portability and Accountability Act (HIPAA), Payment Card Industry (PCI) Data Security Standard (DSS), General Data Protection Regulation (GDPR) specifics) -- these are
  project-level adaptations, not universal governance
- Tool-specific configurations -- these go in `project-specific-examples/`
- Theoretical additions without evidence ("we think there should be a
  Documentation Agent") -- where is the evidence that its absence caused
  a problem?

---

## Framework evolution

The framework is versioned (semver) and changes are evidence-driven.
Every revision cites the learning(s) that triggered it. Content is
classified by validation status:

- **Empirically validated** -- exercised by multiple independent projects
- **Empirically informed** -- derived from one project's experience
- **Theoretical** -- derived from principles, not yet tested in practice

Content that remains theoretical after 5+ project records is reviewed
for removal. See the [Appendix in framework.md](framework.md) and
[LEARNINGS.md](LEARNINGS.md) for the full evolution process.

---

## Current state

- 16 projects have adopted the framework and recorded ~232 learnings
- 5 Sentinel reviews completed, triggering framework versions 0.6.0
  through 0.17.0
- 38 generic rule templates, 7 stack-specific examples, 12 agent
  definitions, and 7 reusable skills available
- Zero overkill findings at 232 learnings (under investigation --
  single-developer bias cannot be ruled out until independent adopters
  contribute)

### Independent adopters wanted

All 16 projects to date involve the same developer who designed the
framework. The strongest claims (Phase 1 combined agents, boundary
principles, Verb Test) are consistent across projects but may reflect
designer bias rather than universal applicability.

The framework needs independent adopters — teams and individuals who
did not build it — to challenge, validate, or refute its assumptions.
Specific areas where independent evidence would be most valuable:

- **Overkill findings** -- has any part of the framework added overhead
  without value? (Zero overkill at 232 learnings is suspicious.)
- **Multi-person team dynamics** -- does the Governor-Reviewer role
  work when it is a different person from the Coder-Tester?
- **Phase 2+ splits** -- has anyone needed to split combined agents?
  What triggered it?
- **Non-Claude-Code tools** -- does the rule and skill content work
  with Cursor, Windsurf, Copilot, or other AI assistants?

If you adopt the framework, your learnings -- especially the
uncomfortable ones -- make it better for everyone. See
[How to contribute learnings](#how-to-contribute-learnings) above.

---

## License

This project uses a dual-license model:

| Component | License | Files |
|-----------|---------|-------|
| **Framework content** (agents, rules, framework.md, documentation) | [CC-BY-SA-4.0](https://creativecommons.org/licenses/by-sa/4.0/) | `SPDX-License-Identifier: CC-BY-SA-4.0` |
| **Validation tools** (Python code, tests) | [MIT](LICENSE-MIT) | `SPDX-License-Identifier: MIT` |

The framework content carries ShareAlike — adaptations must share alike.
The validation tools are Massachusetts Institute of Technology (MIT) licensed — embed them in any Continuous Integration (CI) pipeline without
ShareAlike obligations. See [LICENSE](LICENSE) and [LICENSE-MIT](LICENSE-MIT).

**Filled-in learning records** (`learnings/<codename>.yaml` instances
populated with project data) are the content of the adopter, not
adaptations of the framework template. CC-BY-SA-4.0 ShareAlike does
not automatically upstream or publish the data adopters pour into the
template. Adopters may keep their records private, share them with
selected parties, or submit them upstream -- the choice is theirs.

**Disclaimer:** This framework is provided as-is without warranty. All
guidance is advisory, not authoritative. See [DISCLAIMER.md](DISCLAIMER.md)
for the full disclaimer, limitation of liability, and your responsibilities.

---

## Versioning

The repo carries multiple semantically distinct version numbers. **The most common confusion is between the framework content version and the Python tooling package version — they are not the same thing.**

| Layer | Where | Example |
| --- | --- | --- |
| **Framework content** | [VERSION](VERSION) | `0.37.0` |
| **Python tooling package** | [pyproject.toml](pyproject.toml) | `0.26.0` |
| **Agent definition** | `agents/*.yaml` `version:` field | `0.3.2` |
| **Agent → framework conformance** | `agents/*.yaml` `framework_version:` field | `0.37.0` |

**When in doubt:** cite and record the framework content version ([VERSION](VERSION)). That is what defines "this framework" at any point in time, and what your `learnings/<codename>.yaml`'s `framework_version` field should reference. The Python tooling version moves independently and usually slower; agent definition versions track each agent's local revision history; agent-to-framework conformance tracks which framework version each definition was last validated against.

See [ADOPTING.md §1.2](ADOPTING.md#12-record-framework-version) for the record-keeping convention.

---

## Citation

The canonical citation lives in [`CITATION.cff`](CITATION.cff) at the
repo root (machine-readable, recognised by GitLab's "Cite this
repository" affordance and reference managers like Zotero / Mendeley).
The human-readable form with full American Psychological Association
(APA) + BibTeX renders on the docs site at
[Reference → Citation](https://janipaijanen.gitlab.io/agent-governance/reference/citation/).

Quick excerpt:

<!-- RELEASE: update Version X.Y below when bumping major.minor; agov-bump syncs this with CITATION.cff -->
> Päijänen, J. (2026). *agents-govern: Evidence-driven governance for multi-agent AI development* (Version 1.1) [Governance framework]. GitLab. https://gitlab.com/janipaijanen/agent-governance
