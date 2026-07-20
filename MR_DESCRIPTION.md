# plan: Gate 1 plan for issue #39 (removability principle, exit guarantee, CI drill)

Branch: `plan/removability-exit-guarantee`

## Summary

Elevates AGF's low-lock-in architecture from an implicit property to a
stated design principle, a downstream-facing exit guarantee, and a
CI-tested removal drill. This plan is unusually foundational — every
other cluster planned this session was told to treat its constraint as
standing while working.

## What's in this branch

`docs/plans/issue-39-removability-exit-guarantee.md` covers three layers:

1. **Design principle** — new canonical page `agf/docs/site/docs/concepts/removability.md`, with a pointer added to `agf/framework.md`'s Boundary Principles list.
2. **Exit guarantee** — new README section, positioned relative to the (separately in-flight) `docs/append-only-audit-trail-policy` branch's ledger section — links out rather than restating, per message-integrity.
3. **Removal drill** — a toolchain-independent fixture under `tooling/removal-drill/` (deliberately not a Rust/Elixir project, so the drill can't false-positive via ambient tooling on PATH) + a path-scoped, tag-pipeline-only CI job.

## Reconciliation done before planning

Confirmed `docs/append-only-audit-trail-policy` (unmerged) covers a
different claim (history-is-never-rewritten / GDPR tension), not
removability/exit — no overlap. Found and flagged (not fixed) a
pre-existing duplicate: `agf/docs/site/docs/adopt/exit-and-decommission.md`
("Released v2.1") vs. the older `agf/docs/adopt/decommission.md`.

## Honesty discipline applied

Explicitly stated as Theoretical/Level 1 until the removal drill ships
and passes — the plan forbids claiming validated/deployable status
prematurely, per the implementation-honesty rule.

## Cross-cluster flag

The plan makes the #35 git-native-identity constraint concrete, but
also flags honestly that the drill only checks file *survival*
(checksum-identity), not whether an identity field inside those files
stays *semantically resolvable* if #35 ships something requiring a live
external lookup. The human reconciler should check #35's actual design
against the git-native constraint directly.

## Review checklist

- [x] Plan validates against `agf/schemas/gate1-plan.schema.json`
- [ ] Pre-existing exit/decommission page duplication (flagged, not
      fixed) — confirm whether to address in this MR or a follow-up
- [ ] Removal-drill fixture design reviewed (dependency-free by
      construction)
- [ ] No `Co-authored-by` trailers in commit
