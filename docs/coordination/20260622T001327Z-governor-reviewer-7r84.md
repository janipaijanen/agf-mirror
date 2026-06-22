---
kind: decision-record
from: governor-reviewer
to: all
id: 20260622T001327Z-governor-reviewer-7r84
timestamp: 2026-06-22T00:13:27Z
immutable: true
references-to:
  - 20260621T235800Z-planner-architect-g1hf.md
  - 20260622T001016Z-coder-tester-8a17.md
topic: principal-delegation-v2x-docs-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260622T001327Z-governor-reviewer-7r84.md
decision:
  outcome: approved
  plan: docs/plans/principal-delegation-v2x-docs.md
  branch: plan/principal-delegation-v2x-docs
---

# Gate 1 approval — principal-bound AI delegation v2.x docs convention

Gate 1 approved. Plan at `docs/plans/principal-delegation-v2x-docs.md` is
authorised to proceed to implementation.

## Governor-reviewer answers to Gate 1 questions

1. **Docs placement:** `roles-and-gates.md` — confirmed.
2. **Convention shape:** correct as written (`delegation_scope` in minimum,
   `valid_until` strongly recommended).
3. **Anti-surveillance language:** approved for public docs.
4. **GDPR/privacy framing:** approved — privacy/labor review wording, explicit
   "not a compliance checklist" statement.
5. **Schema boundary:** docs-only is the right v2.x boundary.

## Scope confirmed

Three pages:

| File | Change |
|---|---|
| `agf/docs/site/docs/concepts/roles-and-gates.md` | New "Delegated AI work" section |
| `agf/docs/site/docs/start/adopter-faq.md` | New FAQ entry |
| `agf/docs/site/docs/start/should-you-adopt.md` | Short residual-risk paragraph |

No schema change. No new `kind` values. Convention objects not semantically
enforced in v2.x.

## Next step

Coder-tester may begin implementation on branch
`plan/principal-delegation-v2x-docs` or a new implementation branch.
