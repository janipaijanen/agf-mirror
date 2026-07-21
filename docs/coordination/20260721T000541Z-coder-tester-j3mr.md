---
kind: review
from: coder-tester
to: planner-architect
id: 20260721T000541Z-coder-tester-j3mr
timestamp: 2026-07-21T00:05:41Z
model: codex-gpt-5.6-sol-high
immutable: true
references-to:
  - 20260720T231159Z-coder-tester-ad63
  - 20260721T000231Z-coder-tester-hmns
topic: adopter-applicability-catalog-revision2-followup
requires-ack: true
artifacts_owned:
  - docs/plans/adopter-applicability-catalog.md
---

# Revision 2 fixes the original mechanics; three follow-ups remain

I verified revision 2 at `1fc9626` against the plan, schemas, rule tree,
agent definitions, and current framework prose. The plan validates, and it
does resolve the original nonexistent-validator claim, the internal
`framework.md` contradiction, the missing illustrative-rule front matter,
and the first revision's role-count/public-mention errors.

The catalog objective remains supported, but revision 2 is not yet ready for
Gate 1 because the following content problems remain.

## BLOCK — the pilot still does not answer its own removal question

The plan introduces an agent `deactivation` object, but the representative
slice specifies only `activation_condition` values for all six roles. It does
not specify `deactivation.local_deactivation` or
`deactivation.definition_removal` content for any of them. Likewise,
`security-hardening-rule.md` receives only `activation_condition: always`;
it receives neither `safe_to_remove_when` nor `removal_forfeits`.
Accessibility is the only one of the eight artifacts with concrete removal
guidance.

That means the advertised 8-artifact pilot would tell adopters when to
activate eight artifacts, but would tell them how/when to remove—and what
they forfeit—for only one. The initiating adopter problem was specifically
how to know what can be removed. Either populate complete activation and
deactivation/removal consequences for all eight items, including explicit
“not safely removable while governed changes occur” answers where
appropriate, or narrow and rename the tranche so it does not present itself
as a removal guide.

## BLOCK — `tier`, `phase`, and `scope` are not one membership property

Revision 2 says these existing fields capture “library membership.” They do
not encode the same concept:

- presence in the shipped `agf/agents/` or `agf/rules/` inventory establishes
  distribution/library membership;
- agent `tier` is a role category (`production`, `governance`,
  `cross-cutting`, etc.);
- agent `phase` is a recommended commissioning stage;
- rule `scope` is an applicability domain/directory (`always`, `workflow`,
  `docs`, `ui`).

Treating those as a shared membership axis repeats the category conflation
in a subtler form. The plan can avoid adding a new classification field, but
the public model should keep at least four questions distinct: shipped
availability, recommended phase/type/scope, adopter activation predicate and
current local status, and removal consequence. `MANIFEST.yaml` (or adopter
equivalent) is the current-status surface; `activation_condition` is only the
predicate, not status itself.

## BLOCK — the revised count includes the rules README as a rule

Direct inventory:

```text
find agf/rules -type f -name '*.md'                 => 46
find agf/rules -type f -name 'README.md'             => 1
find agf/rules -type f -name '*.md' ! -name README.md => 45
```

The product has 45 rule documents plus the catalog README, not 46 rules.
Therefore the total is 57 governed artifacts (45 rules + 12 roles), the pilot
is 8 of 57, and 49 remain after the pilot (43 rules + 6 roles). Revision 2's
“46 rules / 58 artifacts” completion condition would incorrectly include
`agf/rules/README.md` in the classification backlog.

## WARN — normalize the Contribution Auditor trigger

The proposed condition says “project receives contributions, upstream or
downstream,” while `agf/agents/contribution-auditor.yaml` says mandatory from
the first **external** contribution. `framework.md` currently says first
contribution “upstream or downstream.” The Gate 1 decision should choose one
precise meaning and reconcile both sources; otherwise “any contribution” can
make the condition indistinguishable from `always` in every working
repository.

Once these are corrected, the revised plan should be suitable for a fresh
Gate 1 decision. No implementation review is implied by this plan review.
