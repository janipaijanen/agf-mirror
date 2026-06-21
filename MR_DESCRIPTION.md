# docs: adoption flow + Rust tooling page

## Summary

- Homepage restructured: "How to adopt" section (three numbered steps) appears
  immediately after the status block, replacing the philosophy-first layout.
  New "Token savings with the Rust tooling" block with command maturity table
  (Level 5 / Level 1).
- New page `agf/docs/site/docs/adopt/tooling-and-tokens.md`: agov CLI setup,
  Phase 1 and Phase 2 Rust commands with honest maturity levels, three setup
  options (wrapper / release archive / CI), token savings model, telemetry POC.
- `tooling-conformance.md` updated: Phase 1 parity status reflected for
  coord validate, coord brief, plan validate, validate, handoff next. Phase 2
  parity status for coord status (shipped). Session resume and coord stale
  marked "Phase 2 in progress".
- `mkdocs.yml` and `docs/index.md` updated to include the new page in nav.
- Maturity references updated from "Phase 2A" to `rust-agov-complete.md` plan
  after the Phase 2A plan was superseded.
- `coord status` maturity updated to Level 5 after `feat/rust-agov-complete`
  merged (coord status is now in production in Rust).

## Rationale

The old homepage led with philosophy before offering any action path. An
evaluator reading for the first time had to navigate several paragraphs before
reaching the decision guide or install steps. The restructured homepage leads
with action, with costs and evidence after.

The tooling page covers the token savings angle with honest maturity levels per
the implementation-honesty-rule: no overclaiming on commands still in progress.
The Level 5/Level 1 split is updated as Rust commands ship.

## Collaboration

A coordination note was filed to coder-tester requesting review of tone/flow,
a before/after token example, and potential use-cases.md additions. No response
received yet — MR is ready to merge as-is and improves with Codex's additions.

## Checklist

- [x] Docs-only change, no schema or tooling changes
- [x] All new links exist in the nav and resolve within the site
- [x] Maturity levels accurate per implementation-honesty-rule
- [x] No vendor names alongside weakness language (marketing-tone-rule)
- [x] Documentation audience: developers and system administrators
- [x] Coordination message filed to coder-tester
- [x] coord status maturity updated to Level 5 after Phase 2 merge
