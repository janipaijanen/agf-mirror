---
id: rule-projection-screening
status: pending-gate1
topic: rule-projection-screening
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: "The governor directed immediate planning so issue #72 implementation does not outrun the scope correction; contribution-auditor must review this cross-surface precedent before Gate 1 approval."
  - role: infosec-sentinel
    status: not-applicable
    reason: "This is documentation authority and projection hygiene with no credential, learning-record, network, or trust-boundary change."
  - role: fact-checker
    status: consulted
    reason: "Review 20260727T100546Z-fact-checker-2pqd verified the singular-authority split and existing Pages navigation surfaces that this plan generalizes."
---

# Plan: rule projection screening across Pages, runbooks, adopters, and AI loaders

## Decision requested

Require a projection screen whenever a Pages page or runbook introduces,
changes, or removes normative operating guidance. The screen prevents two
opposite failures: a rule that adopters or assistants cannot discover, and
duplicated rule text that drifts into competing authority.

Issue #73 tracks the general requirement. It is also a scope amendment to the
approved issue #72 implementation: coordination-publication guidance must be
screened and projected into the relevant AI landing surfaces before Gate 2.

After Gate 1 approval, implementation is release-blocking for the next
release. The release identifier is not inferred by this plan.

## Authority model

For every screened requirement, record exactly one normative owner:

- a named file under `agf/rules/` when the statement governs agent or workflow
  behavior;
- a schema or other existing governed contract when that is already the true
  owner; or
- `not normative` when the text is explanatory or situational guidance.

Pages explain the contract. Runbooks turn it into situation-oriented steps.
Adopter guidance explains installation and operation. AI landing/loaders make
the governing source discoverable to assistants. None of those projections
may silently redefine the owner.

## Mandatory projection screen

Every plan and Gate 2 review that adds or changes normative guidance in a
runbook or Pages page must include a matrix with these rows:

| Surface | Required disposition |
|---|---|
| Normative owner | `updated`, `existing pointer sufficient`, or `not applicable`, with exact path and reason |
| Adopter-facing explanation | same disposition, naming canonical explanatory page |
| Runbook/situation map | same disposition, using a concise step or pointer rather than copied rule text |
| AI landing/loaders | same disposition for `AGENTS.md`, `agf/templates/AGENTS.md`, `.claude/CLAUDE.md`, and any other supported loader actually shipped |
| Validation/drift evidence | `implemented`, `advisory only`, or `not mechanically decidable`, with reason and named authority |
| Distribution | confirm every referenced adopter/AI file ships in the relevant AGF or tooling artifact, or explain why it is project-local only |

`Not applicable` is a reasoned result, not an omitted row. A pointer is
sufficient when the target is loaded or discoverable in the relevant user or
assistant workflow. Full normative prose must not be copied merely to make the
matrix appear complete.

## Planned changes

### 1. Add a named projection rule

- Add `agf/rules/workflow/documentation-rule-projection-rule.md` defining the
  trigger, matrix, singular-authority requirement, and Gate evidence.
- Link it from the existing downstream-impact and documentation-impact
  guidance rather than restating either rule.
- Explicitly grant validation tooling blocking authority only for objective
  omissions: a triggered change with no matrix, missing required rows, a
  pointer to a nonexistent path, or a declared shipped surface absent from
  distribution input. Semantic adequacy remains human Gate review.

### 2. Make planning and review templates carry the screen

- Add the matrix/checklist to the canonical Gate 1 planning guidance and Gate
  2 review guidance.
- Update the contributor/operator documentation so authors know when the
  trigger fires and how to distinguish normative rules from explanation.
- Prefer one shared structured representation if automation is added; do not
  create several hand-maintained copies of the matrix.

### 3. Project issue #72 into AI landing surfaces

- Add concise pointers in `agf/templates/AGENTS.md` and supported shipped AI
  loader guidance so adopters' assistants discover the coordination-
  publication rule and operational checklist.
- Update project-root `AGENTS.md` only for foundry's own workflow and only with
  a pointer or compact mandate, not the normative rule body.
- Screen `.claude/CLAUDE.md` and other supported loaders; record
  `existing pointer sufficient` where they already load canonical AGF rules.
- Add tests or drift checks proving the adopter template and loader path still
  resolve to the governed rule/guidance.

### 4. Baseline-audit existing Pages and runbooks

- Inventory current runbooks and Pages statements that use normative language
  such as MUST, required, blocking, approve, or prohibited.
- Group statements by their actual normative owner rather than treating every
  occurrence as a new rule.
- Apply the projection matrix to each group. Fix bounded missing pointers and
  broken links within this implementation; route any semantic conflict or
  unplanned behavior change to a separate issue and release disposition.
- Publish the inventory as auditable evidence. Do not rewrite historical plans
  or coordination records.

### 5. Validate Pages and distribution

- Run a strict Pages build, navigation/search and internal-link checks, plus a
  duplicate-authority wording review.
- Verify relevant AI loaders/templates and canonical explanations are present
  in the adopter distribution paths that claim to provide them.
- Add positive and negative self-tests for the mechanically enforced subset.

## Acceptance criteria

1. A named rule requires the six-row screen for every triggered Pages/runbook
   normative change and distinguishes omission blocking from semantic review.
2. Gate 1 and Gate 2 guidance both require the screen at the appropriate
   evidence stage.
3. Issue #72's rule, adopter guidance, runbook, AI landing/loaders, validation,
   and distribution rows are all explicitly disposed before its Gate 2.
4. The existing Pages/runbook baseline inventory names each normative owner
   and contains no unexplained omitted surface.
5. Normative text has one owner; projections use explanations or links and do
   not create competing rules.
6. Missing matrices, rows, paths, or falsely declared distribution surfaces
   fail the authorized mechanical check, including negative self-tests.
7. Strict Pages build, navigation/search, internal links, loader/drift checks,
   distribution validation, and full relevant tests pass at exact Gate 2 head.
8. Contribution-auditor reviews the generalized precedent and fact-checker
   verifies public claims and surface inventory before Gate 2.

## Exclusions and sequencing

- No implementation begins before governor-reviewer Gate 1 approval.
- Issue #72 implementation may continue on its already-approved core scope but
  must not claim Gate 2 readiness until this amendment is resolved and its
  projection matrix is satisfied.
- This plan does not create a new assistant-specific copy of every rule,
  mandate edits to every loader when a canonical import already suffices, or
  convert explanatory prose into normative policy.
- It does not change schemas, role authority, versions, release branches,
  tags, or published releases.

