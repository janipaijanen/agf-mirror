# plan: Gate 1 plans for compliance-export/buyer-side AGF (#40/#41) and CRA entity-structure decision (#44)

Branch: `plan/compliance-export-buyer-side`

## Summary

Three issues, two of them a genuine pair (#40 produces evidence, #41
consumes it — planned against one shared schema) and one a lightweight
decision (#44).

## What's in this branch

- **`compliance-export-buyer-side-agf.md`** (#40 + #41): a joint
  evidence-pack schema (`agf_evidence_pack_version`,
  `layers_claimed`/`layers_not_available`/`layers_out_of_scope`) that
  integrates with the repo's **existing** three-tier compliance
  vocabulary (`agf/docs/adopt/compliance-profile.md`) rather than
  inventing a parallel one — each section is tagged by which
  verifiability layer it draws from, with Layer 3 (binding) explicitly
  marked "not available, planned/deferred" rather than silently
  omitted, and Layer 4 (runtime) never populated. The pack never claims
  more than the adopter's declared `compliance_target` tier. Maturity
  classified per the five-level model: mapping/schema work is Level 2;
  the export/verify commands themselves are Level 1/Planned.
  Regulatory dates (MCC-AI, AI Act Art. 26, CRA) cited from
  `docs/sources.md` with explicit recheck-before-citing flags.
- **`cra-entity-structure-decision.md`** (#44): deliberately lightweight
  — stated explicitly as closer to a fast-track decision than a full
  Gate 1 project. Decision: CRA Art. 24 steward regime does not
  currently apply (AGF is a natural-person maintainer, not a legal
  person per Art. 3(14)), with the source survey's "partially
  unverified, draft guidance" caveat preserved, plus a named trigger
  condition (entity formation → policy due by 2026-09-11 or formation
  date, whichever is later).

## Assumption flagged for reconciliation

CLI subcommand namespace (`agov compliance ...`) and reuse of `agov
coord validate` internals may need reconciliation with the coordination
CLI cluster (#33/#32/#36/#37) — no collision found (different
namespace), but stated explicitly for the human reconciler to confirm.

## Review checklist

- [x] Both plan files validate against `agf/schemas/gate1-plan.schema.json`
- [ ] Evidence-pack schema reviewed against existing compliance-profile.md
      vocabulary for consistency
- [ ] #44's fast-track-weight recommendation reviewed (agree it doesn't
      need a full plan cycle?)
- [ ] No `Co-authored-by` trailers in commit
