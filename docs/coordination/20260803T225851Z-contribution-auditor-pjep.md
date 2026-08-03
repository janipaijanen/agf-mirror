---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260803T225851Z-contribution-auditor-pjep
timestamp: 2026-08-03T22:58:51Z
model: codex-gpt-5.4
immutable: true
references-to:
  - 20260803T225314Z-planner-architect-b27h
  - 20260803T200227Z-governor-reviewer-5m41
  - 20260803T204257Z-planner-architect-sjqm
  - 20260803T225012Z-contribution-auditor-5k14
topic: audience-rule-and-rc1-hold-second-opinion
requires-ack: true
artifacts_owned: []
---

# Second opinion: correct the audience rule; maintain the rc.1 hold for a bounded rc.2 decision

Scope: independent inspection of the active and distributed
`documentation-audience-rule.md`, the shipped Pages audience declarations,
the downstream-impact and rule-projection rules, the frozen rc.1 evaluation,
and the current state of MRs !1086, !1087, and !1090.
Coverage: COMPLETE for the two questions in `b27h`.
Authority: advisory recommendation only; the governor decides the rule change
and release sequence.

## 1. The audience mismatch is real

The active `.claude/rules/docs/documentation-audience-rule.md` and distributed
`agf/rules/docs/documentation-audience-rule.md` have the same substantive
contract. They describe a generic business application: public Pages target
directors, managers, employees, and system administrators and should focus on
data entry, configuration, backup, schema migrations, audit logging, and data
quality.

The actual AGF Pages site does something materially different and internally
coherent. Its index declares four page kinds — Human decision, AI action,
Shared concept, and Reference. Individual pages address evaluators, adopters,
human governors, developers, operators, and AI assistants; they explain role
authority, repository artifacts, schemas, validators, CLI/MCP operation, and
release practice. AGF does not provide a business application's data-entry or
backup capability. The rule is therefore not a useful high-level description
of the present site and can direct an agent toward unsupported maturity
claims.

Recommendation: replace the generic rule with an AGF-specific audience
contract, rather than split the site into a nominal business track and a
technical track. Keep one learning path, but require each page to state:

- primary and secondary audience;
- the reader's task and prerequisite knowledge;
- whether execution belongs to a human, an AI assistant, or both; and
- the human verification or troubleshooting path for an AI-action page.

The site-wide primary audiences should be technical adopters/operators and
human governors evaluating or supervising AI-assisted development. Engineering
leaders and risk, security, compliance, and assurance stakeholders are valid
secondary audiences on decision pages. Reference pages may remain precise and
assume the relevant Concepts prerequisite. The README remains a repository
entry point for developers, evaluators, and assistants and should route each
reader to the appropriate Pages task.

This is **BREAKING** under the active downstream-impact rule, not EDITORIAL:
the set of documentation actions the rule requires changes, and the rule is a
trigger-path artifact. A migration note must exist before proposing the
change. Amend the active project rule and the distributed adopter-facing rule
in the same governed change; changing only one preserves the contradiction.
Because the Pages audience explanation and runbook projections are affected,
the plan should also dispose the six-row rule-projection screen and identify
the one normative owner rather than copy the new contract into several pages.

## 2. Maintain the rc.1 hold now; make the next decision specifically about rc.2

The new evidence does not justify lifting the observation hold straight into
stable v2.5.0. It does, however, supply genuine candidate-release findings,
not merely a request to wait longer:

- `sjqm` found the published rc.1 artifacts, identity chain, overlays, CLI/MCP
  behavior, and positive/negative validator controls broadly sound.
- The frozen artifact also exposes real structural assurance gaps around gate
  approver fields, actor authentication, and immutable provenance. These are
  limitations, not newly introduced regressions, but the observation boundary
  expressly includes independent-review feedback and documentation
  misunderstandings.
- The audience rule is itself shipped in the rc.1 framework archive and can
  instruct adopters to make claims about a product shape AGF does not have.
  That is a candidate documentation/rule defect in the immutable artifact.
- The learning corrections are not yet “landed”: !1086 and !1087 remain open
  substantive MRs. !1090 is also not awaiting a governor Gate 1 decision in
  its present form; exact-head review `5k14` BLOCKS it on seven specification,
  evidence, migration, scope, and test findings, including the unresolved
  advisory-first versus block-now choice.

Recommendation: keep `5m41` binding as written while those dispositions are
made. Do not start stable or version work yet. The hold already permits
evidence collection and separately governed corrective planning, so no textual
“narrowing” is needed.

The next governor decision should be bounded and binary:

1. If the shipped audience-rule mismatch and/or structural gate-claim gap must
   be corrected before stable, authorize an rc.2 plan containing only the
   accepted corrective slices after their own Gate 1 plans are reviewable.
2. If either item is explicitly accepted for post-2.5.0 work, record that risk
   and its tracking target; stable planning may then begin after !1086/!1087
   are dispositioned and a final rc.1 acceptance check finds no artifact,
   packaging, or publication defect.

My recommendation is Path 1 for the audience rule because it is a shipped
normative instruction capable of producing false public claims. The authority
hardening belongs in rc.2 only if the governor accepts a corrected, bounded
Gate 1 design; `5k14` shows that the current !1090 design is not ready to make
that commitment. This creates a concrete exit from observation without using
elapsed time as a proxy for evidence.
