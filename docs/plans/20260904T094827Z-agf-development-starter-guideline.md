---
id: 20260904T094827Z-agf-development-starter-guideline
status: pending-gate1
topic: agf-development-starter-guideline
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The proposed guideline contains repository-workflow and prioritization
      instructions only. It does not add a dependency, execute untrusted
      material, handle credentials or personal data, or change a security
      boundary. Security and vulnerability work remain explicitly prioritized.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Same-session advisory audit 20260904T094956Z-contribution-auditor-6aq8
      screened adversarial steering, release tunnel vision, authority
      expansion, notification spam, false precedent, and duplicate normative
      ownership. Its owner/projection finding was resolved in this revision.
      The audit is not independent corroboration; the governor-reviewer retains
      the Gate 1 decision.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# AGF development starter guideline

## Outcome

Create `docs/reference/agf-development-starter.md` as a concise reusable,
non-normative quick-start projection for humans or AI agents beginning an AGF,
`agov`, or `agov-mcp` development session. `AGENTS.md` remains the normative
owner. Link the quick start from the repository orientation without copying
its full text into every loader.

The guideline improves the originating instruction in four ways:

1. it defines a complete, evidence-backed poll instead of saying only “poll”;
2. it ranks release work without granting release, merge, or gate authority;
3. it distinguishes slightly off-priority work from work that should actually
   be interrupted, avoiding indiscriminate agent notifications;
4. it defines terminal output: act on the highest-priority authorized item,
   record blockers, and leave a durable cursor and disposition.

## Proposed guideline text

> Start by reading `AGENTS.md` and any repository-root `ai-cold-start.md`, then
> run `agov session resume` without a role and again with the active role. Build
> the local release binaries first when developing `agf`, `agov`, or
> `agov-mcp`.
>
> Run one complete poll before choosing work: fetch and prune `origin`; use
> `agov coord list --since <cursor>` and `agov coord visibility --base-branch
> main`; inspect every open GitLab merge request and issue for new notes,
> assignments, labels, state, exact head/target SHAs, mergeability, and canonical
> pipeline status; and recheck each release, tag, branch, pipeline, or job on
> which the active role is waiting. Keep UTC cursors so “nothing new” is an
> evidence-backed result.
>
> Select the highest-priority item already authorized for the active role.
> Prefer work that advances the next governed release and removes verified
> bugs, weaknesses, or vulnerabilities in AGF, `agov`, or `agov-mcp`; then
> implement release-scoped roadmap features whose Gate 1 plan is approved.
> Treat security impact, adopter breakage, release integrity, and blocked
> acceptance evidence as stronger signals than cosmetic progress. Do not
> invent severity, close issues on assertion alone, or broaden an approved
> plan.
>
> Before redirecting another agent, verify its actual claim, governing plan,
> dependencies, and release relationship. Notify it only when its active work
> materially competes with an authorized release-critical path or conflicts
> with a binding priority decision. Use one concise durable coordination record
> naming the evidence, requested disposition, and authority boundary; do not
> spam stale MRs or interrupt necessary reviews, coordination publication,
> incident response, or work that removes a release blocker.
>
> Act immediately within the current gate: acknowledge or claim visible work,
> diagnose failures before rerunning paid CI, batch corrections, and route the
> exact reviewed head through the required gate. Stop for Gate 1, Gate 2,
> merge, tag, publication, destructive actions, secrets ambiguity, or scope
> expansion when binding human authority is required. A release-first priority
> never grants those powers.
>
> End the cycle with a durable disposition: what changed since the prior
> cursor, what was acted on, exact MR/issue/SHA/pipeline evidence, what remains
> blocked and by whom, and the next authorized release-critical action. If the
> state is unchanged, remain quiet unless a periodic report was requested.

## Scope and ownership

Implementation is limited to:

- `docs/reference/agf-development-starter.md` — non-normative
  project-development quick start projected from `AGENTS.md`;
- `AGENTS.md` — one short reference from the cold-start/tooling orientation;
- `docs/reviews/rule-projection/agf-development-starter.yaml` — all six
  projection rows disposed;
- the plan and coordination records required by the gates.

No change is authorized to schemas, Rust code, CI, release metadata, issue or
milestone state, tags, release branches, adopter templates, or published site
navigation. Any such need returns to Gate 1 as a scope revision.

## Priority and interaction with v2.9.0

The guideline is release-enabling governance, not a substitute for the active
v2.9.0 work order. Its implementation must not pre-empt the planner-owned
artifacts authorized by
`20260903T221837Z-governor-reviewer-m3rp`. If the same planner session cannot
advance both without delaying that release lane, this guideline waits.

## Validation and acceptance

1. `agov plan validate` passes on this plan.
2. Contribution-auditor review is recorded and all blocking findings are
   resolved before Gate 1 approval.
3. `agov plan projection-check` passes strictly on the implementation head.
4. `agov validate adopter-docs` and `git diff --check` pass.
5. A tabletop fixture demonstrates three decisions:
   - a release-integrity bug outranks an unrelated cosmetic documentation edit;
   - a required review or coordination-publication task is not incorrectly
     interrupted merely because it is not implementation;
   - no merge, tag, release, or scope-expansion authority is inferred from the
     release-first wording.
6. Gate 2 evidence names the exact head SHA and canonical MR pipeline.

## Risks and controls

- **Release tunnel vision:** necessary governance, reviews, and incident work
  could be mislabeled as distractions. Control: redirect only on material
  competition with an authorized critical path and preserve blocking work.
- **Authority inflation:** “focus on releases” could be read as permission to
  merge or publish. Control: enumerate the binding decisions that still stop
  the agent.
- **Notification spam:** every stale non-release MR could receive a warning.
  Control: require an active claim, verified conflict, and one durable notice.
- **False security prioritization:** vague “vulnerability” labels could game
  the queue. Control: prioritize verified impact and forbid invented severity.
- **Loader bloat:** duplicating the guideline in `AGENTS.md` increases static
  context cost. Control: keep one owner document and add only a short link.
- **Competing authority:** a second normative workflow could drift from
  `AGENTS.md`. Control: label the guideline as a non-normative projection,
  link every instruction class to its owner section, and treat any conflict as
  resolved in favor of `AGENTS.md` and governed records.

## Rollback

Remove the owner document and its `AGENTS.md` reference, then retire the
projection manifest. No data or release artifact migration is required.
