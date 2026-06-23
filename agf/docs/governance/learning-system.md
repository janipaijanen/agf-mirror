<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Framework Learning System

> **For adopters:** This document explains how the framework's own
> learning system works. It is **reference-only** — read it to understand
> the feedback loop, but you do not need to copy or implement it.
> Your project's learning record is `learnings/<your-codename>.yaml`
> (copied from `learnings/_template.yaml` during adoption).

> 📖 **Looking for the actual learnings?** The docs site renders the
> evidence corpus as browsable per-project pages at
> [janipaijanen.gitlab.io/agent-governance/evidence/projects/](https://janipaijanen.gitlab.io/agent-governance/evidence/projects/).
> This file describes the *system*; the site shows the *content*.

How the governance framework evolves from field evidence.

---

## The Problem

The framework was designed from principles and informed by the Mega-OS
experience (starting-point.md). But principles alone produce theory.
The framework must improve based on what actually happens when real
projects adopt it.

Without a structured feedback loop:
- Learnings stay trapped in individual projects
- The framework grows from theory ("we should add X") instead of evidence ("project Y showed X was missing")
- There is no way to tell if the framework is getting better or just bigger

---

## The Feedback Loop

```
   ┌──────────────┐
   │  Framework   │──── adopted by ────→ ┌──────────────┐
   │  (this repo) │                      │  Project A   │
   └──────┬───────┘                      └──────┬───────┘
          │                                     │
          │                              encounters problems
          │                              or successes
          │                                     │
          │                                     ↓
          │                              ┌──────────────┐
          │                              │  Records a   │
          │                              │  learning    │
          │◄──── learning record ────────│  (YAML file) │
          │                              └──────────────┘
          │
          ↓
   ┌──────────────┐
   │  Review      │  Human reviews learning,
   │  (Sentinel)  │  decides if framework
   │              │  should change
   └──────┬───────┘
          │
          ↓
   ┌──────────────┐
   │  Framework   │  Updated framework
   │  revision    │  cites the learning
   └──────────────┘  that triggered it
```

### Step by Step

1. **A project adopts the framework.** Ask the assistant to prepare
   `learnings/<codename>.yaml` from `learnings/_template.yaml`, then review
   the project context section. Use a codename, not the real project name
   (see Submission Privacy Policy below).

2. **The project encounters something noteworthy.** A governance gap, an
   agent that was overkill, a principle that worked exactly as predicted,
   a tension between two framework rules, or an adaptation that improved
   outcomes.

3. **Record a learning.** Add a dated entry to the `learnings` section of
   the project's YAML file. Tag it with a category and severity.

4. **InfoSec Sentinel screens the submission.** The CI pipeline runs
   `agov validate infosec` on changed files. It checks for leaked
   credentials, sensitive internal details, cross-field re-identification
   risk, cumulative information leakage across entries, and cross-record
   linkage. If findings are flagged, the MR is blocked until the human
   submitter adds `sentinel_ack` entries acknowledging each finding (see
   the template for the format). The sentinel may propose redactions;
   the human decides whether to accept, modify, or dismiss them.

5. **The framework Sentinel reviews.** The Human (currently Jani) periodically
   reviews accumulated learnings. Cadence: after every 3rd learning record,
   or monthly, whichever comes first.

6. **Decide whether to act.** Not every learning triggers a framework change.
   The decision criteria are below.

7. **If acting: revise the framework.** The revision follows the normal
   governance pipeline (plan → approve → implement → verify → commit).
   The commit message cites the learning(s) that triggered it.

---

## What to Capture

Each project record (`learnings/<project-name>.yaml`) captures three layers:

### Layer 1: Project Context (set once)

The static facts about the project that make learnings interpretable:

| Field | Why It Matters |
|-------|---------------|
| `business_domain` | Agent configurations that work in fintech may fail in e-commerce |
| `regulatory_context` | Compliance pressures determine which cross-cutting agents are urgent |
| `team_size` | Determines which growth phase is realistic |
| `lifecycle_state` | Greenfield vs. legacy migration stresses different parts of the framework |
| `customer_state` | Pre-customer vs. paying customers changes the risk profile |
| `revenue_context` | Did governance have measurable business impact? |
| `codebase_maturity` | Test coverage, CI/CD, documentation — the starting conditions |
| `pii_present` | Determines whether Privacy/Data Guardian are day-one concerns |

### Layer 2: Agent Configuration (evolves)

How the project actually uses the framework — which agents are active,
which are combined, which are skipped. Tracked as dated snapshots so you
can see how the configuration evolved and why.

### Layer 3: Learnings (accumulated)

Individual observations, tagged and structured. Each learning captures:
- **What happened** (description)
- **Which agents were involved** (traceability)
- **Which framework section is relevant** (where to look for improvement)
- **Business impact** (directional, not precise)
- **Recommendation** (what should change, if anything)

---

## Learning Categories

| Category | Meaning | Framework Response |
|----------|---------|-------------------|
| `gap` | The framework missed something. A real project exposed a missing agent, principle, or rule. | High priority. Likely triggers a framework addition. |
| `overkill` | The framework prescribed something that added overhead without value in this context. | Important. May trigger a framework simplification or a "when to skip" note. |
| `validation` | The framework correctly predicted a problem or prevented one. | Record for confidence. No change needed, but strengthens the case for the principle. |
| `adaptation` | The project adapted the framework in a way worth incorporating. | Evaluate. If the adaptation works across domains, incorporate it. |
| `tension` | Two framework principles conflicted in practice. | Critical. These are the most valuable learnings. May trigger new arbitration rules. |

### Observed distribution

The current distribution across 161 categorized entries (as of 2026-04-27, 14 codename files) is:

| Category | Count | Share |
| --- | --- | --- |
| `gap` | 69 | 43% |
| `validation` | 55 | 34% |
| `adaptation` | 30 | 19% |
| `tension` | 8 | 5% |
| `overkill` | 0 | 0% |

The zero-`overkill` count has been raised twice: by the framework's own Sentinel reviews 2 and 4 (referenced in the pine-ridge and silver-index entries dated 2026-03-30, both `tension`-categorized) and by the Windsurf agent in the 2026-04-27 brief-aware external engagement.

**The framework's answer:** the framework distinguishes *principles* (behavioral — Verb Test, Escalation, Artifact Ownership, etc.) from *ceremony* (procedural artifacts — YAML definitions, GOVERNANCE.md, rules files). Behavioral principles are inherently non-overkill because they shape how agents reason, not artifacts an adopter pays to maintain. Procedural ceremony has a skip mechanism (Phase 1 combined agents, optional artifacts, "grow by pain" discipline) which catches overkill cases at the adoption boundary rather than producing learning entries about them.

**Caveat:** at least one historical entry (quiet-river, 2026-03-22, `adaptation`) describes the multi-agent structure as "overkill at this scale" but is not categorized as `overkill` because the adopter's resolution path (skipping ceremony, keeping principles) was the principal subject of the entry. When a future learning is *both* "X was overkill" *and* "we adapted by skipping X," prefer `tension` and explain in the description.

For the full investigation, see [reviews/2026-04-27_zero-overkill-investigation.md](reviews/2026-04-27_zero-overkill-investigation.md).

---

## Business Impact Types

Captured as directional signals, not precise numbers. The goal is to
understand whether governance has a positive, negative, or neutral
relationship with business outcomes.

| Type | Meaning | Example |
|------|---------|---------|
| `prevented_loss` | Governance prevented a costly incident | Data Guardian caught a migration that would have corrupted customer records |
| `enabled_revenue` | Governance enabled something revenue-positive | Security audit enabled SOC2 certification, unlocking enterprise customers |
| `cost_overhead` | Governance added cost without visible return | Reviewer gate added 2 days to every release cycle with no quality improvement |
| `neutral` | No measurable business impact either way | Agent split improved clarity but had no effect on outcomes |

---

## Severity Levels

Each learning is tagged with a severity that indicates its importance for
framework evolution. Use these definitions consistently — do not invent
intermediate levels (e.g., "moderate").

| Severity | Meaning | Framework response |
|----------|---------|-------------------|
| `critical` | Governance failure that caused or nearly caused real harm (data loss, security breach, broken deployment, legal exposure). | Act immediately. Single-project evidence may be sufficient. |
| `significant` | Governance gap or tension that materially affected outcomes — wasted effort, missed requirements, blocked progress. | Act when corroborated by a second project. |
| `minor` | Small friction, cosmetic gap, or low-impact observation. Useful for pattern detection but not urgent alone. | Collect. Act when a pattern emerges across 3+ projects. |
| `informational` | Context-setting record with no direct framework implication. First-use validations, configuration choices, process observations. | Record only. No action expected. |

---

## Decision Criteria: When to Change the Framework

Not every learning triggers a change. The criteria:

### Change the framework when:

- **Multiple independent projects report the same gap.** One project's gap
  might be context-specific. Three projects' gap is a framework deficiency.

- **A tension has no arbitration path.** If two principles conflict and the
  framework provides no guidance on resolution, that is a structural gap.

- **An adaptation improves outcomes across domains.** If a fintech project
  and a healthcare project independently make the same adaptation, it should
  be in the framework.

- **An agent or rule is consistently skipped.** If multiple projects skip
  the same agent with the same reason, the framework should either explain
  when to skip it or remove it.

### Do NOT change the framework when:

- **One project had a bad experience.** It might be context-specific. Record
  it but wait for corroboration.

- **The learning is domain-specific.** "In healthcare, HIPAA requires X" is
  not a framework change — it is a domain-specific adaptation. Record it for
  other healthcare projects, but do not add HIPAA to the framework itself.

- **The proposed change adds complexity without evidence of value.** The
  framework's biggest risk is bloat. Every addition must justify its
  governance cost.

---

## The Framework's Sentinel

The framework itself needs a meta-observer. This is a human role (currently
Jani) that periodically asks:

1. **Is the framework getting better or just bigger?** Count agents, principles,
   and rules. If they only grow, something is wrong. Good frameworks also
   simplify, merge, and remove.

2. **Are learnings converging or diverging?** If different domains report the
   same patterns, the framework is capturing something real. If every domain
   needs different adaptations, the framework may be too prescriptive.

3. **Which parts have never been validated?** Any agent, principle, or rule
   that no project has ever exercised is theoretical. Mark it as such. If it
   remains theoretical after 5+ project records, consider removing it.

4. **Which parts have been validated by multiple projects?** These are the
   framework's strongest claims. They should be marked as empirically
   validated (future: add a validation status to framework sections).

**Cadence:** Review after every 3rd learning record, or monthly, whichever
comes first. This is not a formal agent — it is a calendar reminder and a
checklist.

**Standing items (check at every sentinel review):**
- Self-conformance: does the framework repo follow every ADOPTING.md step it
  prescribes to adopters? (`agf/` exists, assistants can read `agf/rules/`,
  tests exist, GOVERNANCE.md exists, pre-commit hooks active)
- Communication symmetry: run `agov validate agents` — zero asymmetries
  expected
- README stats: `agov-validate-readme-stats` is deferred to the E4 Elixir
  migration slice; until then, verify README stats manually or with the v1.x
  Python-era tool if maintaining that line.

**Authority and succession.** The Sentinel's decisions (change vs. no-change)
are final but must be documented with rationale. If the Sentinel role
transfers to another person, the new Sentinel inherits the accumulated
learning corpus and decision history — not the previous Sentinel's
preferences. If multiple people share the role, each decision records who
decided and why. Minority opinions are preserved as comments on the
relevant learning record.

---

## Submission Privacy Policy

**All learning records MUST be anonymized.** This is a mandatory requirement,
not a suggestion.

> **Broader scope: external-anonymization rule.** The codename-only discipline
> applies not just to learning records but to *every* artifact crossing the
> codebase boundary — issues filed on external trackers, MRs against upstream
> frameworks, public blog posts, content sent to external LLM APIs, etc. The
> framework rule [`rules/always/external-anonymization-rule.md`](rules/always/external-anonymization-rule.md)
> codifies this. The Submission Privacy Policy below remains the canonical
> reference for **learning record submissions**; the external-anonymization
> rule is the canonical reference for **all other externally-destined content**.

### What must be masked

| Category | Examples | Replacement |
|----------|----------|-------------|
| Project / product names | "Acme Billing", "HealthTrack v2" | Use a codename (e.g., "blue-heron") |
| Organisation / company names | "Acme Corp", "HealthTech Inc" | Omit or use "the organisation" |
| Internal tool or service names | "acme-auth-service", "ht-pipeline" | Use generic descriptions ("auth service", "data pipeline") |
| Customer or partner names | "BigCorp", "Partner X" | "a customer", "an integration partner" |
| URLs, hostnames, infrastructure IDs | "acme.internal", "prod-db-3" | Omit or genericize ("internal hostname", "production database") |

### What to keep (valuable for cross-project analysis)

- Business domain (fintech, healthcare, e-commerce, etc.)
- Technology stack (Python, Flask, React, etc.)
- Regulatory frameworks (GDPR, SOC2, HIPAA, etc.)
- Team size ranges and role composition
- Architecture patterns (monolith, microservices, etc.)
- Quantitative data (test coverage %, incident counts, etc.)

### Why this matters

Learning records are shared across all adopters and may be referenced in
public ADRs, sentinel reviews, and framework changelog entries. Real names
would expose adopters' technology choices, compliance posture, and internal
architecture to competitors and the public.

### Enforcement

- The Sentinel will reject submissions that contain identifiable project,
  product, or organisation names.
- Existing submissions that predate this policy will be migrated to codenames
  (see `docs/ANONYMIZATION-NOTICE.md`).

---

## File Structure

```
learnings/
  _template.yaml              # source template for each new project file
  .adopter-mapping.example    # format for the private codename → real-name mapping
  blue-heron.yaml             # example: project codename "blue-heron"
  project-aurora.yaml         # example: project codename "project-aurora"
  ...
```

Each file is self-contained. The project owns its file and updates it as
learnings accumulate. The framework Sentinel reads across all files to
identify patterns.
