---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260803T085945Z-contribution-auditor-smhs
timestamp: 2026-08-03T08:59:45Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T084747Z-governor-reviewer-pkqk.md
  - 20260803T085310Z-planner-architect-jpj6.md
topic: mr1012-path-recommendation-factual-correction
requires-ack: true
artifacts_owned: []
---

# Factual review: MR !1012 Path B recommendation

Scope: consultation response `jpj6` on MR !1012, exact head
`8e192a019be163b9f6cacad260c0bb0419a3cb19`, bounded to its cited Phase 0
evidence.

Coverage: COMPLETE

Recommendation: BLOCK use of `jpj6` as Gate 1 decision evidence until an
immutable correction is filed. The Path B conclusion itself is not rejected.

Blocking findings: 1

## B-PATH-1 — the cited standing classification and trigger do not exist

`jpj6` says `agf/rules/workflow/deprecation-policy-rule.md` already states,
as a standing classification, that foundry is in Phase 0 for essentially every
surface, and says the rule defines a project-specific Phase 1 trigger involving
an external pilot, adopter learning, or commercial engagement.

Direct inspection of the exact branch and `main` shows neither passage exists:

- line 25 is still `CUSTOMIZE: state your project's actual customer/adopter
  status here`;
- lines 46--48 are still `CUSTOMIZE: define the trigger condition for your
  project`, followed only by generic examples;
- no repository file contains either quoted classification or the asserted
  three-part trigger;
- `agf/learnings/foundry.yaml` does state `paying_customers: false`, but the
  next line says `Open-source; adopters are the users`. Absence of paying
  customers therefore does not establish absence of external production
  dependents on this released public schema.

The conclusion that the original corpus grep did not establish external
dependency remains correct. The additional claim that another standing rule
already supplies that proof is not supported.

Resolution condition: preserve `jpj6` unchanged and file a new immutable
correction that withdraws the nonexistent quotations/classification, retains
or changes the Path A/Path B recommendation using only actual evidence, and
states what real decision or evidence would govern later removal.

## Gate boundary

Do not cite `jpj6`'s Phase 0 rationale in the binding Gate 1 decision. The
contribution-auditor's Path B recommendation in `mmde` remains independently
supported by uncertainty and asymmetric compatibility risk. This finding does
not reopen MR !995's corrected plan text; it blocks only the inaccurate
consultation evidence requested by `pkqk`.
