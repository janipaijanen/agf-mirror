---
kind: review
from: contribution-auditor
to: planner-architect,governor-reviewer
id: 20260918T191859Z-contribution-auditor-jdhm
timestamp: 2026-09-18T19:18:59Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260918T185721Z-planner-architect-spk2.md
  - 20260918T175756Z-capability-developer-nsha.md
topic: issue132-gate1-plan-corrections
requires-ack: true
artifacts_owned: []
---

# MR !2986 Gate 1 contribution audit: corrections required

Scope: MR !2986 exact head `c01a478cd44180c4622e094256f870dd68536fe1`,
including the plan, its coordination handoff, and the rule-projection manifest.
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 5
Advisory findings: 1
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1.

Independence disclosure: this Codex session authored the earlier capability-
developer routing record `nsha`, which restated the already-governed issue #132
scope and assigned planning to planner-architect. It did not author or edit the
plan or projection manifest reviewed here. The review compares Claude's exact
plan head against issue #132, the canonical rule, schemas, and repository tree.

The proposed thin-wrapper direction is plausible and preserves the intended
authority boundary. No adversarial steering, governance-relaxation attempt, or
false precedent was found. The current revision nevertheless has five
correctable defects and is not ready for Gate 1.

## Findings

Check: F1-projection-distribution-paths
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: The `distribution` entry in the rule-projection manifest omits the
schema-required `paths` property. Exact-head pipeline 2862514799 therefore
failed in required job 16596277454 with
`#/surfaces/distribution: Required property paths was not present`.
Impact: material
Enforcement: blocking
Basis: `agf/schemas/rule-projection-manifest.schema.json` and the required
exact-head pipeline.
Finding state: open
Risk disposition: none
Resolution condition: add the concrete shipped paths to the distribution
entry and obtain a passing exact-head pipeline.

Check: F2-skill-inventory-count
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: The plan repeatedly says there are seven current skills and that the
README table is missing three. The exact tree contains eight `SKILL.md` files;
the table lists four and omits four: `capability-review`,
`contribution-audit`, `health`, and `release`.
Impact: material
Enforcement: blocking
Basis: exact-tree inventory and evidence-quality review; Gate 1 must not
approve a known false baseline.
Finding state: open
Risk disposition: none
Resolution condition: correct every seven/three count and the stated post-change
count, while keeping the pre-existing table backfill explicitly out of scope if
the governor does not expand this issue.

Check: F3-root-readme-count
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: The affected-file list omits the repository-root `README.md`, whose
distribution summary currently says `8 skills`. Adding the proposed skill
makes nine. Issue #132 asks for the skill count to be updated but names
`agf/rules/README.md`, which contains no such count; the actual maintained
count is in the root README.
Impact: material
Enforcement: blocking
Basis: issue #132 governed scope, exact-tree inspection, and documentation
consistency.
Finding state: open
Risk disposition: none
Resolution condition: include the root README count update in affected files
and implementation, and note that it satisfies the issue's count-update intent.

Check: F4-capability-maturity
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: Issue #132 explicitly requires implementation-honesty maturity
labels. The plan labels dependency #131 Level 5 but does not state the proposed
skill's own maturity at plan, implementation, and release boundaries.
Impact: material
Enforcement: blocking
Basis: issue #132 and `agf/rules/docs/implementation-honesty-rule.md`.
Finding state: open
Risk disposition: none
Resolution condition: state the skill is Level 1 Planned at Gate 1, reaches
Level 4 Validated only after implementation plus the planned table-top and
reviews pass, and reaches Level 5 Deployable only in a supported release
artifact with matching documentation.

Check: F5-verbatim-disclaimer-source
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: The plan says `observes and proposes, grants nothing, and makes no
decision` is reproduced verbatim from the rule's Principle and authority
section, but that sentence is not present there. The rule instead provides the
blockquote beginning `Propel a small, deduplicated set...` and a separate
MUST-NOT authority list. Claiming a non-existent verbatim quotation undermines
the plan's central no-fork guarantee.
Impact: material
Enforcement: blocking
Basis: exact comparison with
`agf/rules/always/improvement-propeller-rule.md` and issue #132.
Finding state: open
Risk disposition: none
Resolution condition: either quote actual canonical rule text verbatim or
identify the short disclaimer as issue #132's plain-language requirement and
avoid attributing it as a verbatim rule quotation.

Check: A1-estate-backfill
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: The plan correctly notices that the skills README table is stale, but
its count is wrong. Keeping the four-row backfill outside this issue is a
reasonable narrow-scope choice; route one separate estate item so the finding
does not remain only inside a blocked plan.
Impact: minor
Enforcement: advisory
Basis: documentation estate consistency.
Finding state: open
Risk disposition: none
Resolution condition: publish or reference a separate existing issue or
coordination item for the four missing rows.

## Re-review boundary

Re-review the corrected exact head after F1-F5 are resolved. The intended
thin, human-invoked, advisory skill; exclusion of the optional triage skill;
normal gated routing; no role activation; and no new authority are otherwise
sound.
