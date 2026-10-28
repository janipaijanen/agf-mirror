# Text Moved Verbatim From AGENTS.md (Phase 1, issue #138)

Plan `20261022T205113Z-instruction-context-reduction`. Each block below is the
unchanged text of the named `AGENTS.md` passage. It keeps the force it had
there; `AGENTS.md` keeps a pointer. The normative owners are named in each
heading and win over this page on any difference.

## Agent activation history (was under "Active agents")

Authoritative status: `agf/agents/MANIFEST.yaml`.

`independent-implementation-reviewer` was **reactivated** by governor
decision `20260816T145707Z-governor-reviewer-3eqb` (Gate 2 for MR !1632 at
`903899f6`); the 2026-08-09 suspension is closed. See
`agf/agents/MANIFEST.yaml` for the full corrective-work trail.

`documentation-estate-steward` was activated 2026-09-10 by governor decision
(bridged `20260910T050007Z-planner-architect-jh3g`, ratified
`20260910T133145Z-governor-reviewer-q862`); see
`agf/agents/documentation-estate-steward.yaml`.

## Coordination-only publication (was under "Coordination channel")

Owner: `agf/rules/workflow/coordination-publication-rule.md`.

**Publication independence:** A coordination-only commit (title prefix
`coordination:`) is expected to publish promptly, independently of the Gate 2
approval of the implementation it describes — see
`agf/rules/workflow/coordination-publication-rule.md`. That fast path is
mechanically verified, not merely title-claimed: `agov coord
publication-check` inspects the full commit range against the target branch,
not only the tip diff, and CI runs it automatically via
`agov:governance:validate` whenever a `coordination:`-titled commit changes
`docs/coordination/*.md`. Batch same-cycle records when none independently
blocks another actor; publish a live obligation, security or gate finding,
escalation, or blocking review alone immediately. Branch it from the target
branch, not from an implementation branch's tip. **Never add `[ci skip]`/`[skip ci]` to a
coordination-only commit message** — that marker skips pipeline creation
entirely at the platform level, before any CI configuration runs; run `agov
coord publication-check --base-ref origin/main` locally before pushing
instead.

## Narrow research-maintenance exception (was under "Gate discipline")

Owner: `agf/rules/workflow/simplicity-rule.md`.

**Narrow research-maintenance exception:** an advisory, reversible change
limited in every commit and the final diff to the existing `docs/sources.md`
and/or `docs/radar.md` may skip the separate Gate 1 plan, contribution audit,
and pre-change coordination MR only when every eligibility, source-fidelity,
fresh target/open-MR collision-scan, and disqualifier condition in
`agf/rules/workflow/simplicity-rule.md` passes. Generic docs-only work does not
inherit this exception. The normal branch/MR, commit-message waiver, exact-head
pipeline, and binding governor-reviewer Gate 2 decision remain mandatory.
Uncertainty or any rule/authority, implementation, integration/dependency,
roadmap/resource, sensitive/legal/security/privacy, public AGF claim, governed
decision, or extra-path scope returns to full Gate 1. A coordination record is
not required merely to start or submit an eligible MR; separate obligations,
escalations, accepted risks, and decisions still use coordination.
