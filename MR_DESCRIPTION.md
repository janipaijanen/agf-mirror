# plan: threat model + governance primitives cluster (issues #42, #48, #49)

Branch: `plan/threat-model-governance-primitives`

## Summary

Three foundational/conceptual issues planned together so the primitives
(#48, #49) can cite specific entries in the threat model (#42) instead
of each justifying their own existence from scratch.

## What's in this branch

- **`threat-model-stpa-owasp.md`** (#42): declares AGF's threat model as
  **cooperative-but-fallible primary, incidental-adversarial-resistance
  secondary** — most gates are advisory (rely on agent cooperation), so
  AGF resists honest mistakes/drift/automation-bias but does not claim
  Byzantine-fault tolerance, non-repudiation, or collusion detection
  against a determined adversary with repo access. Derives an STPA
  control structure (implementer → governor-reviewer → human governor)
  with 9 unsafe control actions, each rated Addressed/Partially/Not-
  addressed against a named existing rule file, with OWASP LLM Top 10
  mappings (hedged, flagged for re-verification against the live OWASP
  page before publishing).
- **`governance-time-budget-primitives.md`** (#48): cooling-off,
  rate-limit, and token/action-budget throttles, explicitly citing
  UCA-3 (machine-speed gate saturation) from the threat model. Extends
  `deputy-governor-rule.md`'s existing `audit_trigger` (count-based,
  deputy-only) with a time-windowed, any-governor-identity sibling
  rather than inventing a parallel mechanism.
- **`input-provenance-manifest.md`** (#49): citing UCA-4 (silent
  instruction drift), reuses the existing coordination-message schema
  (one optional `provenance` object field — additive, no Rust code
  change needed since the validator is schema-driven) rather than a new
  manifest file type. Tracks CycloneDX's Agent BOM proposal at Level 1
  (Planned) only, no conformance claimed.

## Open question flagged for reconciliation (not silently resolved)

`input-provenance-manifest.md` explicitly flags a tension: this project
already chose SPDX over CycloneDX for tooling binary SBOMs
(`docs/coordination/00275-planner-architect.md`). Different artifact
(behavioral provenance vs. binary distribution), not a hard conflict,
but worth a one-line rationale for running two BOM-adjacent
vocabularies, or redirecting #49 at SPDX instead. See Finding 2 in
`docs/coordination/20260720T080808Z-governor-reviewer-mrp3.md`.

## Assumption flagged for reconciliation

The threat model guesses that the gate-enforcement cluster (#34/#47)
converts specific advisory gates to structural CI checks, since this
plan had no visibility into that cluster's actual text when written.

## Review checklist

- [x] All three plan files validate against `agf/schemas/gate1-plan.schema.json`
- [ ] Threat-model's cooperative-vs-adversarial framing agreed
- [ ] SPDX vs. CycloneDX question resolved (see open question above)
- [ ] No `Co-authored-by` trailers in commit
