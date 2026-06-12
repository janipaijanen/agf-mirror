# Downstream impact: what adoption changes in your project

*Status: Provisional (v2.0 documentation). Audience: evaluators and
administrators. Read this page **before** adopting. After it you can
itemise every artifact, obligation, and cost adoption introduces.*

Adoption is a deliberate trade: structure and auditability in exchange for
files in your repository, jobs in your CI, and discipline in your workflow.
This page lists the full price. The way back out is documented on the
[exit and decommission](exit-and-decommission.md) page.

## Files added to your repository

| Artifact | Location (Layout A) | Purpose |
|---|---|---|
| Agent definitions | `agents/*.yaml` | Role contracts: jurisdiction, authority, escalation |
| Governance rules | `.claude/rules/**/*.md` (or your tool's equivalent) | Constraints your AI assistant loads every session |
| Learning records | `learnings/<your-codename>.yaml` | Your project's incident and pattern evidence |
| Coordination channel | `docs/coordination/` | Append-only message files, one per decision/handoff |
| Governance docs | `GOVERNANCE.md`, ADRs | Who decides what; recorded decisions |
| Validators | `scripts/`, `src/` (Python tooling) | Schema checks runnable locally and in CI |

Layout B (subdirectory) confines everything except `.claude/rules/` to one
governance directory. Both layouts are described in
[install and layouts](install-and-layouts.md).

## CI jobs added

- **Validator jobs** — schema checks on agent definitions and learning
  records. Fast (seconds); run per merge request.
- **Optional gates** — pre-tag version checks, secrets detection,
  anonymization scanning, depending on which rules you adopt.

Long-running suites are *not* part of the framework; the CI-scoping rule in
the rule library exists to keep governance jobs from inflating your pipeline.

## Workflow obligations per change

| Change size | What the framework asks |
|---|---|
| Trivial (typo, comment) | Nothing beyond your normal commit |
| Small, low-risk | Fast-track waiver noted in the commit message |
| Non-trivial | Plan file → Gate 1 approval → implementation → Gate 2 review → merge via MR |
| High-risk (auth, audit, schema) | Stop-and-plan; explicit human approval; security tests |

The two-gate cycle is the core cost. For a solo project with an engaged
governor, a gate is minutes, not days — but it is never zero, and it is
per change.

## Ongoing obligations

- **Learning records** when incidents occur — structured YAML, validated.
- **Coordination entries** for actions other agents or humans must see.
- **Upgrade hygiene** — new framework releases replace upstream-owned files;
  your own files (codename learnings, customized rules) are preserved.
  The boundary between upstream-owned and adopter-owned is explicit in the
  distribution manifest.
- **Governor attention** — gates without a governor are theatre. Someone
  must own approvals, audits, and the decision to revoke any delegation.

## What adoption does *not* change

- Your build system, test framework, and deployment pipeline remain yours.
- Your data and code never leave your infrastructure; the anonymization
  rule governs the one outbound path (learnings shared upstream).
- No runtime, no daemon, no service dependency. Removing the framework is
  a file deletion plus a CI revert — see [exit](exit-and-decommission.md).

## Honest summary

In our assessment, the framework pays for itself where AI assistants
produce a significant share of changes and the project lives long enough
for reconstructible decisions to matter. Where those conditions do not
hold, the obligations above are overhead without return — the
[should you adopt?](../start/should-you-adopt.md) page is the honest
filter.
