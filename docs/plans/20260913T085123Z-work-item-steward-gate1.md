---
id: 20260913T085123Z-work-item-steward-gate1
status: pending-gate1
topic: work-item-steward-role-and-existing-trigger-integration
candidate_release: v2.11
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Self-assessed at drafting time (real infosec-sentinel pass still
      requested before Gate 2): the role reads GitLab MR/issue bodies,
      comments, labels, and states via existing `glab`/API credentials
      already used by build-engineer and planner-architect sessions — no
      new credential, token, or network-access surface. Its only write
      surface is repairing unambiguous Markdown/structure defects in
      MR/issue *descriptions* it does not own the substance of; it never
      writes to `docs/coordination/`, never touches immutable records,
      and per jkhg's own boundary section never rewrites another
      person's comments or historical decisions. `agov validate infosec`
      run at drafting time: OK, no findings.
  - role: contribution-auditor
    status: waived
    reason: >-
      Not yet requested — this plan is being drafted under explicit
      time pressure ("provide the plan asap") from the governor's own
      "provide the plan asap" instruction. A contribution-auditor pass
      is welcome before Gate 2, particularly on the repair-authority
      boundary (Markdown/structure fixes vs. substantive content, which
      remains the change owner's responsibility per jkhg's own
      boundaries section) and on whether this role's checks meaningfully
      differ from documentation-estate-steward's existing scope.
affected_paths:
  - agf/agents/work-item-steward.yaml
  - agf/agents/MANIFEST.yaml
  - AGENTS.md
  - .gitlab/merge_request_templates/*
  - .gitlab/issue_templates/*
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: work-item-steward — MR/issue readiness and Markdown quality

**Gate 1: PENDING.** Drafted from Codex's proposal
(`20260913T082918Z-coder-tester-jkhg`) plus the governor's accepted
trigger-integration constraint
(`20260913T083739Z-governor-reviewer-s4km`, transcribed by Codex: "good
idea to attach it to the existing trigger mechanism"). This plan does
not activate the role; it is the Gate 1 submission `jkhg` itself
requested ("prepare its Gate 1 plan for governor review").

## Problem and evidence

At the 2026-09-13 audit (cited in `jkhg`), 56 of 60 open MR descriptions
and all 15 recently merged MR descriptions lacked at least one of the
six sections `agf/rules/workflow/merge-request-description-rule.md`
requires. This is a real, measured gap in whether work items carry
enough information for a reviewer or a cold-started agent to act on
them without re-deriving context — the same failure mode this session
already hit once today with the unmerged-coordination-MR-backlog
finding (`sh2b`/`b4nk`): work exists, but the record of it is
incomplete or unreachable where a reviewer actually looks.

## Scope (Level 1 Planned — this Gate 1 authorizes definition + narrow pilot only)

1. **New agent definition**: `agf/agents/work-item-steward.yaml`,
   modeled on the existing `documentation-estate-steward.yaml` shape
   (role description, evidence surfaces, check list, repair authority,
   boundaries). Status `proposed` in `agf/agents/MANIFEST.yaml` — this
   Gate 1 does not itself activate the role; activation is a separate
   governor decision naming the player, per this project's existing
   pattern for every other role (documentation-estate-steward's own
   activation was a distinct step after its definition existed).

2. **Checks** (per `jkhg`, unchanged from the proposal):
   - **MRs**: the six required sections from
     `merge-request-description-rule.md` — problem/outcome, scope and
     changed artifacts, source branch, relevant governance, head-bound
     verification, review findings, residual risks and next
     action/owner. (Note: `jkhg` lists these as six items but the
     enumeration above has seven — this plan flags the discrepancy for
     governor/contribution-auditor resolution rather than silently
     picking one count; the actual six-section list must be read
     verbatim from `merge-request-description-rule.md` at
     implementation time, not re-derived from this summary.)
   - **Issues**: purpose, acceptance criteria, owner or explicit
     unassigned status, dependencies, next action, and a disposition
     supported by closure evidence. `jkhg` is explicit that the MR
     six-section rule does not apply to issues — a distinct check, not
     a copy-paste of the MR one.
   - **Markdown quality**: rendered output (not raw-text heuristics) —
     literal escape artifacts, malformed nesting, unreadable walls of
     text, leftover template placeholders.
   - **Currency**: distinguish historical evidence from current
     head/state; flag contradictory done/pending claims, stale
     blockers, unsupported completion claims.
   - **Completed-item sampling**: a bounded recent sample plus reported
     older defects — not a full historical sweep in one pass.

3. **Trigger integration (s4km's binding constraint)**: the steward
   sweep attaches to this project's **existing**
   `agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`
   poll cycle — the same 10-15 minute internal-attention loop
   build-engineer and planner-architect already run. **No new
   timer, cron, scheduled pipeline, or standalone service is created
   merely to instantiate this role.** Concretely:
   - One named sweep owner per active session (not a separate always-on
     process) runs the steward checks as one more step inside its
     existing poll cycle, alongside the MR/issue/coordination scans
     `AGENTS.md`'s polling section already requires.
   - A persisted cursor (`updated_at` + head SHA + body hash per
     work item) lets the sweep send only actionable deltas on
     subsequent cycles, not a full re-scan every time.
   - Author/submission-time and completion/handoff-time checks
     complement the periodic scan (check a work item when it's opened
     or handed off, not only on the next poll tick) — this is
     additional trigger *points* on the same mechanism, not an
     additional mechanism.
   - The plan must name the actual wake path this pilot will use
     (which active session's poll cycle) and its behavior when no
     session is actively polling (inactivity is honestly reported as
     "steward checks did not run this interval," never silently
     assumed clean) — matching this project's own
     improvement-propeller-rule.md precedent for distinguishing a real
     wake mechanism from a role definition that merely exists on
     paper.

4. **Repository templates**: `.gitlab/merge_request_templates/` and
   `.gitlab/issue_templates/` populated with the actual required
   sections (MR six-section list; issue purpose/acceptance-
   criteria/owner/dependencies/next-action/disposition list), committed
   to Git per s4km's request ("proper GitLab templates stored in
   Git... a check that GitLab makes them available"). A template
   existing in the repo but not selectable in GitLab's new-MR/issue UI
   (wrong path, wrong branch) is a plan-time verification item, not an
   assumption.

## Repair authority (bounded, per `jkhg`)

- MAY fix unambiguous Markdown/structural defects in a work item's own
  description (escaped newlines, broken heading levels, malformed
  tables/lists), preserving a before/after audit trail and verifying
  the repaired rendering.
- MUST NOT invent verification, acceptance, approval, or closure
  evidence that doesn't already exist.
- MUST NOT rewrite immutable coordination records, other people's
  comments, or historical decisions.
- Missing *facts* (not formatting) route to the change's existing
  owner or the governor — this role never manufactures a disposition.
- No new automated merge veto or CI gate is introduced by this role;
  its checks are advisory findings, consistent with
  `always/tool-authority-boundary-rule.md` ("a tool or CI check may
  only block work when a named rule in `agf/rules/` explicitly grants
  that check blocking authority").

## Explicitly out of scope for this Gate 1

- Activating the role for any adopter (this project included) beyond
  the pilot below.
- Any new CI job, scheduled pipeline, or webhook.
- Rewriting or reinterpreting existing, closed decisions.
- Overlapping with `documentation-estate-steward` (adopter-facing
  documentation estate) or `contribution-auditor` (plan/proposal
  review) — this role's surface is GitLab work-item presentation and
  currency specifically, not documentation content or plan
  soundness. If the pilot finds this distinction doesn't hold up in
  practice, that is a pilot finding, not something this plan resolves
  in advance.

## Pilot and acceptance (per `jkhg`)

Pilot scope: active RUW (v2.11) work items, plus ten recently merged
MRs and ten recently closed/open issues (a fixed, named sample at pilot
start, not "whatever looks convenient at review time"). Measure and
report, per item: missing-information findings, rendering defects,
recurrence after a prior fix, false positives, repair completion, and
approximate token/tool-call cost. Every pilot finding gets an explicit
disposition — "the role would have caught/fixed this" is not itself
evidence; the pilot must show it actually did, on this project's real
current backlog. No token-savings claim is made in advance of the
pilot; this follows `docs/implementation-honesty-rule.md`'s maturity
discipline — the role stays Level 1 Planned until the pilot produces
Level 3+ evidence.

## Migration/impact classification

Per `always/downstream-impact-rule.md`: this touches `AGENTS.md` (the
active-agents table) and `agf/agents/MANIFEST.yaml`, both trigger
paths. Classification: **ADDITIVE** — a new optional role definition
and template files; no existing agent's contract, gate, or authority
changes. No migration note required.

## Rollback

If the pilot finds the role's checks don't meaningfully differ from
existing reviewer/steward coverage, or the repair-authority boundary
proves unworkable in practice, the role definition stays at Level 1
Planned / `proposed` in MANIFEST with no activation — no code or
process to unwind, since nothing beyond the pilot's own bounded sample
runs before a governor decision.

## Test strategy

- `agov validate learnings`-equivalent schema check for the new agent
  YAML against whatever shape `documentation-estate-steward.yaml`
  already establishes as precedent.
- Negative test for the Markdown-repair check: feed it a deliberately
  malformed MR description (literal `\n` escapes, broken table) and
  confirm it flags the defect and produces a correct rendered repair
  proposal without altering substantive content.
- Positive test: a well-formed MR description produces zero findings.
