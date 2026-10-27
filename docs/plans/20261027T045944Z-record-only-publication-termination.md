---
id: 20261027T045944Z-record-only-publication-termination
status: pending-gate1
topic: bounded-record-only-publication-termination
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The proposed change is prose-only and does not touch credentials,
      learning records, security policy, schemas, tooling, or release
      mechanics. Security-related records are explicitly ineligible for the
      exception and remain on the ordinary governed path.
  - role: contribution-auditor
    status: pending
    reason: >-
      This narrowly changes merge authority for one post-action audit-record
      case. Independent review must confirm that the eligibility conjunction,
      exclusions, and termination example cannot authorize an unfinished
      action or expand Gate 1, Gate 2, release, role, or security authority.
affected_paths:
  - agf/rules/workflow/coordination-publication-rule.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/templates/AGENTS.md
  - docs/reviews/rule-projection/20261027T045944Z-record-only-publication-termination.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bound record-only publication so its audit chain terminates

## Intent and authority

Codify one narrow adopter-facing exception to the ordinary human merge-decision
path: after an already-authorized merge has completed, the author may merge the
coordination-only record that documents that completed merge without requiring
another decision record whose only purpose would be to authorize publication of
the first record.

The Human Governor approved this foundry behavior in the bridge and binding
decision records
`20261027T045323Z-improvement-propeller-mg2a` and
`20261027T045324Z-governor-reviewer-kmrg`, published for shared review in
[MR !3269](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3269).
That decision applies immediately to foundry and expressly requires normal
planning, independent consultation, exact-head review, and Human authority
before adopter-facing rule text changes. This plan requests that next decision;
it does not implement, approve, or merge the rule change.

The motivation is a concrete recursion: an authorized implementation merge
requires a coordination record; publishing that record requires another merge
decision; publishing the new decision record would require another merge
decision without end. The desired terminal state is one verified, post-action
audit-record merge, not absence of the audit record.

## Proposed change

Add a concise subsection to
`agf/rules/workflow/coordination-publication-rule.md`. It MUST define the
exception as a conjunction: every condition below is required.

1. The candidate is a coordination-only MR whose records document one or more
   already-completed merges that each had valid authority before execution.
2. Every added record is post-action audit evidence only. It grants no new
   authority, changes no prior decision, and creates no downstream obligation
   other than making the audit trail visible.
3. The full candidate passes `agov coord publication-check`, coordination
   validation, and every required CI job at the exact current head.
4. Immediately before merge, the current-target merge tree contains only the
   stated new coordination records, with no conflict, modification, deletion,
   or other carried content.
5. The author merges the exact reviewed SHA without squash and verifies the
   resulting merge parents and file-only diff against that SHA.

When all five conditions hold, the author MAY perform that merge and MUST NOT
create a further decision record merely to authorize publication of the
post-action record. This is the explicit termination condition. It does not
alter the existing mechanical blocking authority of `agov coord
publication-check`; semantic eligibility remains a human-reviewable claim, and
uncertainty returns to the ordinary governed path.

The rule MUST state that the exception never covers:

- a pre-action decision or any unfinished or irreversible action;
- Gate 1 or Gate 2 approval;
- implementation or mixed-scope content;
- a changed prior decision or a record that grants authority or creates a new
  downstream obligation;
- rule, schema, role, security, or delegation changes; or
- release scope, version choice, content freeze, tag, release, or deployment.

No schema, role, tooling, CI, release, or enforcement change is in scope. In
particular, this plan does not add a machine classifier for semantic
eligibility and does not generalize self-merge to other coordination-only MRs.

## Projection screen and affected files

The implementation is limited to these four files:

| Surface | Disposition |
|---|---|
| Normative owner | Update `agf/rules/workflow/coordination-publication-rule.md` with the bounded permission, conjunction, exclusions, and examples. |
| Adopter-facing explanation | Add a short explanation and link to `agf/docs/site/docs/concepts/coordination-protocol.md`; do not duplicate the full rule. |
| Runbook/situation map | Existing pointer in `agf/docs/site/docs/operate/poll-and-coordinate.md` is sufficient; record that disposition without editing the page. |
| AI landing/loaders | Add a concise operational summary and canonical-rule pointer to `agf/templates/AGENTS.md`. Foundry's `.claude` rule path is a symlink to the owner; root `AGENTS.md` is project-local and already contains stricter Human authority, so neither needs an edit. |
| Validation/drift evidence | Human-reviewable semantics; existing publication-check, validation, CI, and Git verification remain the objective evidence. No new mechanical authority. |
| Distribution | The owner, explanation, and template already ship through existing `agf/` distribution coverage. |

Record all six dispositions in
`docs/reviews/rule-projection/20261027T045944Z-record-only-publication-termination.yaml`
and run strict projection validation at Gate 2.

## Regression examples and acceptance evidence

The rule and explanation MUST include enough examples for a reviewer to derive
these results without inventing policy:

| Scenario | Expected result | Reason |
|---|---|---|
| An authorized implementation MR is merged; its author then publishes a pure-addition coordination MR recording only that completed merge; all five checks pass. | Eligible; the author may merge the exact head, and the chain ends without another decision record. | This is the sole recursion-stop case. |
| The same record asks for permission to merge an implementation that has not merged. | Blocked. | It is pre-action authority, not post-action evidence. |
| The record approves Gate 1/Gate 2, changes a decision, delegates authority, or creates work for another actor. | Blocked. | It grants authority or creates a downstream obligation. |
| The candidate includes code, a plan, a rule, a schema, role/security material, or any other non-coordination path. | Blocked. | It is implementation or mixed scope. |
| The candidate decides release scope/version/freeze or authorizes a tag, release, or deploy. | Blocked. | Release decisions and irreversible actions stay Human-controlled. |
| Validation, publication-check, an exact-head required job, current-target merge-tree inspection, or exact-SHA/no-squash verification is absent or fails. | Blocked. | Eligibility is conjunctive and fails closed. |

Gate 2 evidence MUST include:

- `agov coord publication-check` and coordination validation for any test or
  example records used by the implementing MR;
- strict rule-projection validation for the new manifest;
- `agov validate` and `git diff --check`;
- an exact-head review showing all five eligible-case conditions and every
  exclusion are present in the canonical owner, with projections remaining
  subordinate; and
- a current-target diff/merge-tree inspection proving the implementation MR
  itself contains only its approved four-file scope. The implementation MR is
  a rule change and therefore is not eligible to self-merge under the exception
  it documents.

## Downstream impact

Classification: **additive but authority-sensitive**. Adopters gain a narrow
post-action publication permission; existing records and workflows remain
valid, and adopters may continue using an independent Human merge decision.
There is no data or schema migration. The evidence is **Empirically informed**
by foundry's observed recursive approval chain; this plan does not claim broad
cross-project validation.

## Risks and controls

- **A record smuggles prospective authority into a post-action narrative.**
  Control: all conditions are conjunctive; any new authority, changed decision,
  or downstream obligation makes the candidate ineligible.
- **A clean source head becomes unsafe after target movement.** Control:
  re-fetch and inspect the current-target merge tree immediately before the
  exact-SHA merge; stale pipeline evidence alone is insufficient.
- **The exception expands through examples or projections.** Control: the
  canonical owner carries the complete boundary; projections summarize and
  link. Independent contribution audit and Human Gate 1 review are required.
- **The implementation uses its own new exception.** Control: rule changes are
  explicitly excluded, so the implementation follows the ordinary Gate 2 and
  Human merge path.

## Rollback

Before merge, close the implementation MR. After merge, revert the rule,
explanation, template, and projection manifest together through the ordinary
governed workflow; do not rewrite coordination history.
