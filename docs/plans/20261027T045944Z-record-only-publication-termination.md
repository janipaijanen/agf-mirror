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
    status: consulted
    reason: >-
      Independent review 20261027T050957Z-contribution-auditor-krqb found F1
      through F4. F1 is resolved by the explicit dependencies and sequencing
      disposition. F2 is resolved by distinguishing an unchanged pre-action
      decision whose authority is exhausted from a new retrospective authority
      claim. F3 is resolved by objective shared-publication, citation, and
      chronology evidence. F4 is resolved by binding eligibility to record
      kinds, requires-ack: false, and absence of obligation/lifecycle signals.
affected_paths:
  - agf/rules/workflow/coordination-publication-rule.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/templates/AGENTS.md
  - docs/reviews/rule-projection/20261027T045944Z-record-only-publication-termination.yaml
depends_on:
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260921T040743Z-assistant-rule-drift-resolution
  - 20261023T155559Z-agov-strict-args-ref-list
  - 20261024T070435Z-timer-reality-polling-alignment
  - 20261025T192020Z-supersedes-adopter-docs
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bound record-only publication so its audit chain terminates

## Intent and authority

Codify one narrow adopter-facing exception to the ordinary human merge-decision
path: after an already-authorized merge has completed, the author may merge a
coordination-only record whose sole remaining effect is publication of the
audit trail. This includes carrying to `main` an unchanged decision record that
was already published on an approved shared path before the action, supplied
the decision's authority is now exhausted by the completed action. It does not
include creating authority retrospectively.

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
decision without end. In the motivating cases, merge-decision records were
visible on shared branches before the authorized merges, but their later carry
to `main` happened after those actions completed. The desired terminal state is
one verified publication of exhausted authority or post-action evidence, not
absence of the audit record.

## Proposed change

Add a concise subsection to
`agf/rules/workflow/coordination-publication-rule.md`. It MUST define the
exception as a conjunction: every condition below is required.

1. The candidate is a coordination-only MR concerning one or more completed
   merges. Each merge had authority published on `main` or an approved shared
   path before it executed; local, untracked, and local-only records do not
   count.
2. Every added record has kind `note`, `completion`, or `decision-record` (so
   `claim` and `handoff` kinds are ineligible), sets `requires-ack: false`, and
   has none of the top-level `handoff`, `challenge`, `publication_hold`,
   `urgency`, or `supersedes` fields. It creates no new downstream obligation
   other than making the audit trail visible.
3. Prior authority is objective rather than self-attested:
   - an eligible carried `decision-record` has the same ID and unchanged
     content as its pre-action shared publication; both its record timestamp
     and that shared publication precede the action; and its authority is
     exhausted because the authorized action is complete; or
   - an eligible post-action `note` or `completion` cites the pre-action
     authorizing record in `references-to`; that authorizing record was already
     visible on `main` or an approved shared path before the action and both its
     timestamp and publication precede the action.
4. Merge evidence names the authorizing record, its approved shared location,
   the action/merge it authorized, and the relevant publication and action
   timestamps or commit identities, making the authority-before-action and
   completion chronology reviewable.
5. The full candidate passes `agov coord publication-check`, coordination
   validation, and every required CI job at the exact current head.
6. Immediately before merge, the current-target merge tree contains only the
   stated new coordination records, with no conflict, modification, deletion,
   or other carried content.
7. The author merges the exact reviewed SHA without squash and verifies the
   resulting merge parents and file-only diff against that SHA.

When all seven conditions hold, the author MAY perform that merge and MUST NOT
create a further decision record merely to authorize publication of the
exhausted decision or post-action evidence record. This is the explicit
termination condition. It does not alter the existing mechanical blocking
authority of `agov coord publication-check`; semantic eligibility remains a
human-reviewable claim, and uncertainty returns to the ordinary governed path.

The rule MUST state that the exception never covers:

- a newly authored decision first published after the action that attempts to
  create retrospective authority, or a decision needed before an unfinished
  action or before any irreversible action executes;
- creation or change of Gate 1 or Gate 2 authority (an unchanged pre-action
  decision may only be carried after its authorized action completed, when its
  authority is exhausted);
- implementation or mixed-scope content;
- a changed prior decision or a record that grants authority or creates a new
  downstream obligation;
- rule, schema, role, security, or delegation changes; or
- release scope, version choice, content freeze, tag, release, or deployment.

No schema, role, tooling, CI, release, or enforcement change is in scope. In
particular, this plan does not add a machine classifier for semantic
eligibility and does not generalize self-merge to other coordination-only MRs.

### Collision disposition

The forge-inclusive collision gate reports seven approved plan snapshots that
overlap the rule, explanation, or template paths named above. This plan
acknowledges them in `depends_on` and claims no parallel implementation
ownership. Any still-current work under those plans goes first. Before
implementation begins, the owner MUST fetch current refs, confirm which
counterparts remain live, rebase onto their merged results, and resolve their
semantics before editing an overlapping file. A stale or conflicting
counterpart is a stop for coordination, not permission to overwrite it. This
sequencing acknowledgment does not approve this plan or any predecessor.

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
| Before a merge, the Human decision record is published on an approved shared branch; after the authorized merge completes, the exact same record ID and content are carried to `main`; its timestamp and shared publication precede the merge, and all seven checks pass. | Eligible; the exhausted decision may be carried unchanged and the chain ends. | Publication completes the audit trail but creates no authority. |
| After an authorized merge completes, a `note` or `completion` with `requires-ack: false` cites the pre-action authorizing record, and the evidence proves the authority-before-action chronology. | Eligible; the author may merge the exact head and the chain ends. | It is post-action evidence tied to objective prior authority. |
| A decision record is first shared after the action and claims that it authorized the already-completed action. | Blocked. | This is retrospective authority creation, not carry of exhausted authority. |
| A carried decision's ID or content differs from its pre-action shared publication, or the publication/timestamp does not precede the action. | Blocked. | Prior authority and unchanged carry are not objectively established. |
| A note/completion does not cite the pre-action authorizing record, or merge evidence omits its shared location and chronology. | Blocked. | Authority is self-attested or unreconstructable. |
| A record has `requires-ack: true`, another kind, or a handoff, claim, challenge, hold, urgency, or supersession signal. | Blocked. | It may create an obligation or alter lifecycle state. |
| The same record asks for permission to merge an implementation that has not merged. | Blocked. | The authority is live and the action unfinished. |
| The record approves Gate 1/Gate 2, changes a decision, delegates authority, or creates work for another actor. | Blocked. | It grants authority or creates a downstream obligation. |
| The candidate includes code, a plan, a rule, a schema, role/security material, or any other non-coordination path. | Blocked. | It is implementation or mixed scope. |
| The candidate decides release scope/version/freeze or authorizes a tag, release, or deploy. | Blocked. | Release decisions and irreversible actions stay Human-controlled. |
| Validation, publication-check, an exact-head required job, current-target merge-tree inspection, or exact-SHA/no-squash verification is absent or fails. | Blocked. | Eligibility is conjunctive and fails closed. |

Gate 2 evidence MUST include:

- `agov coord publication-check` and coordination validation for any test or
  example records used by the implementing MR;
- strict rule-projection validation for the new manifest;
- `agov validate` and `git diff --check`;
- an exact-head review showing all seven eligible-case conditions and every
  exclusion are present in the canonical owner, with projections remaining
  subordinate, including fixtures or worked examples for unchanged exhausted
  decision carry, cited post-action evidence, retrospective authority, changed
  decision bytes/ID, missing chronology, and obligation signals; and
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
  Control: all conditions are conjunctive; pre-action shared publication and
  chronology are objective, and any new authority, changed decision, missing
  citation, or downstream-obligation signal makes the candidate ineligible.
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
