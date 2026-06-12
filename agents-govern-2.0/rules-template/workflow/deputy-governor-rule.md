<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Deputy Governor: Time-Bounded Delegated Approval
#
# Framework principle: Human review escalation, boundary enforcement
# See: framework.md — Pattern 6 (Fast Track) and Principle 5
# (Escalation over Improvisation)

> **Provisional status.** This rule is shipped on single-project
> evidence (framework self-use, 2026-04-18) plus theoretical grounding
> in existing consent-agenda and acting-authority governance patterns.
> It has not been validated by an external adopter. The structural
> requirements and red list are load-bearing; the specific delegable
> categories intentionally ship empty — **delegation is off by default**
> and each adopter must opt in explicitly per category with their own
> risk-profile reasoning. Adopters hitting this rule: please file a
> learning record so we can tighten the pattern or retire it.

**Scope:** Approval authority at Gate 2 (and elsewhere where the
Governor-Reviewer signs off) may be delegated to an agent acting in a
*Deputy Governor* role under the constraints in this rule. The Human
Governor retains ultimate authority and may revoke any delegation, or
pull any deputized decision into full review, at any time without
justification.

## Why this exists

Framework governance prizes human judgment at gates where decisions
are hard to reverse or have wide blast radius. But it also slows routine
work — typo fixes, pinned-dependency bumps, documentation formatting —
to the same cadence as architectural decisions. A deputy mechanism lets
the Governor free attention for the decisions that matter, while keeping
the failure-mode surface narrow through a red list, an explicit artifact,
an expiry, and an audit.

## The rule

### 1. Default state: no delegation

Agents have no approval authority unless an explicit Delegation Artifact
is active. Absent a Delegation Artifact, all Gate 2 approvals go to the
Human Governor exactly as before. **Delegation is opt-in, not a
framework default.**

### 2. Red list (never delegable, regardless of artifact)

These categories MUST NOT be delegated under any Delegation Artifact.
A Deputy Governor that encounters any of them MUST refuse the
delegation and escalate to the Human Governor:

- **Irreversible or destructive operations** covered by
  `destructive-op-preflight-rule.md` (git history rewrites, force push,
  filesystem destruction, destructive DB operations).
- **Security-boundary changes** — authentication, authorization,
  secrets handling, permission models, cryptographic policy,
  tenant isolation.
- **Architecture or data-model changes** — new services, schema
  changes, new external dependencies, new integration points.
- **Critical-severity items** — anything labelled severity=critical in
  the review output. Criticality overrides delegation.
- **Delegation scope itself** — changes to this rule, changes to the
  red list, widening the delegable categories, or extending an active
  Delegation Artifact. Meta-governance is never delegable.
- **Rule and agent definitions** — modifications to
  `rules-template/`, `agents/*.yaml`, `framework.md`, or the adopter's
  `.claude/rules/` directory.
- **Compliance, licensing, or legal artifacts** — SPDX headers,
  LICENSE, consent artifacts, risk-acceptance records.

<!-- CUSTOMIZE: add project-specific red-list entries. Items on the red
     list are NEVER delegable. Use this for categories your Step 0
     assessment identified as high-risk for your domain (e.g.,
     migrations touching PII columns, changes to billing logic,
     modifications to user-facing financial totals, curriculum
     content in learning-systems projects). -->

### 3. Delegation Artifact — what an active delegation looks like

A delegation is valid only if a written Delegation Artifact exists with
ALL of the following fields:

- **Delegator** (Human Governor name and date).
- **Deputy** (agent identifier, not a role class — e.g.,
  `coder-tester@2026-04-18T15:00Z`).
- **Scope** (enumerated delegable categories; none if the
  CUSTOMIZE block below is empty).
- **Severity ceiling** (the highest severity the Deputy may auto-approve;
  never critical; WARN-only is the recommended starting point).
- **Expiry** (absolute timestamp; relative windows — "4 hours" — must
  be resolved to absolute timestamps when written).
- **Audit trigger** (after how many deputized approvals a mandatory
  full audit happens; recommended: 5 or 10).
- **Revocation conditions** — automatic revocation on: first pull
  (Human Governor pulls any deputized item into review), first
  challenge (any agent flags a deputized approval as wrong),
  expiry, or Human Governor explicit revocation.

Store active Delegation Artifacts in a location your project agrees
on. A project-local `governance/delegations/` directory matching the
adopter's Layout (A or B — see `ADOPTING.md` §Choose your layout) is
the recommended convention. Expired or revoked artifacts stay in
history, not deleted.

### 4. Deputy obligations

A Deputy Governor MUST:

1. **Check the red list first.** If any red-list item is present in
   the change under review, refuse and escalate.
2. **Check the Delegation Artifact.** If it is expired, revoked, or
   scope-mismatched, refuse and escalate.
3. **Respect the severity ceiling.** Any finding above the ceiling —
   including findings surfaced only during review — refuses and
   escalates. The Deputy does not "round down" severity.
4. **Log every deputized decision** as a separate artifact referencing
   the active Delegation Artifact by identity. Logs MUST be greppable
   and MUST include: the change identifier, the Deputy's reasoning
   summary, and the severity of every finding surfaced.
5. **Stop at the audit trigger.** After N deputized approvals (per
   the Artifact's audit_trigger), refuse all further deputized
   approvals until the Human Governor completes a full audit and
   issues a fresh Delegation Artifact (or declines to).

### 5. Consent-agenda pattern (recommended entry-level shape)

The smallest blast-radius shape of delegation. At each Gate 2:

- The Deputy produces a *consent agenda* listing all items that
  qualify for auto-approval under the active Delegation Artifact.
- The agenda is posted for a brief review window (a few minutes for
  interactive sessions; end-of-day for batch workflows).
- Any agent — Human Governor, other Deputy, any reviewer — may **pull**
  an item from the agenda into full Governor review with no
  justification required.
- Unpulled items at window close are auto-approved and logged. Pulled
  items revert to ordinary Gate 2 review.
- First pull in any window automatically revokes the Delegation
  Artifact that authorized the agenda. A fresh Artifact must be issued
  for the next session.

This pattern has zero lock-in: every deputized decision is cheap to
reverse at review time, and a single pull returns the session to full
Governor control.

### 6. Audit requirements

The periodic audit is mandatory, not decorative. At each
audit_trigger boundary, the Human Governor MUST:

1. Read every deputized approval log in the triggering window.
2. Spot-check the underlying changes for any deputy decision they
   cannot reconstruct from the log alone.
3. Record outcomes: decisions that survived audit, decisions revised,
   decisions reversed.
4. If any decision is reversed or the Governor finds drift, revoke
   all active Delegation Artifacts and block issuance of new ones
   for at least one review cycle.
5. If the audit finds no issues, the next Delegation Artifact may be
   issued with the same or stricter parameters — never looser without
   a second clean audit.

A reversal or revocation at audit counts as a learning event and
MUST produce a learning-record entry under the adopter's
`learnings/<codename>.yaml`.

## CUSTOMIZE: delegable categories (opt-in per project)

Start with this block **empty**. Fill it in only after:

1. Your Step 0 assessment (ADOPTING.md §Step 0) is complete and
   documents your project's risk profile.
2. Your first full Gate 2 audit cycle has been performed manually
   (no delegation) and produced a learning record identifying which
   categories are *routinely* low-risk in your project.
3. The Human Governor explicitly signs off in writing on each category
   entered here.

<!-- CUSTOMIZE: enumerate delegable categories for your project. Each
     entry MUST include: the category name, examples of in-scope
     changes, examples of near-boundary changes that are NOT in scope,
     and the severity ceiling for that category. Keep categories
     narrow — broader categories accumulate drift faster.

     Template:

     - category: "<name>"
       in_scope_examples:
         - "<concrete example>"
       out_of_scope_examples:
         - "<concrete example that almost qualifies>"
       severity_ceiling: "<WARN | INFO>"   # never "critical"
       notes: "<why this category is routinely low-risk for this project>"

     Leave this block empty if you have not yet earned the evidence to
     populate it. Empty is the correct state until you have it.
-->

## Related rules

- `workflow/destructive-op-preflight-rule.md` — items covered there
  are automatically on the red list; delegation cannot weaken
  preflight.
- `always/high-risk-operations-rule.md` — code areas covered there
  are automatically on the red list.
- `workflow/systematic-code-review-rule.md` — the Deputy's review
  process MUST follow the same steps; delegation changes who approves,
  not how review is performed.
- `always/uncertainty-and-scope-rule.md` — delegation does not relax
  the scoping discipline; a Deputy that cannot bound the change
  refuses and escalates.
