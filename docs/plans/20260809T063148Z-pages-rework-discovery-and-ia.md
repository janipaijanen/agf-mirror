---
id: 20260809T063148Z-pages-rework-discovery-and-ia
status: pending-gate1
topic: pages-rework-discovery-and-ia
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      `20260808T171050Z-contribution-auditor-q1ge.md` independently
      assessed this exact scoping question (finding 5 — "finish Slice B
      independently; plan broader Pages work anew") and concurred with
      `kpdj`'s recommendation that this plan implements: scope the
      broader Pages rework as a separate Gate 1 plan, baselined on
      merged Slice B, rather than silently absorbing it. This plan does
      not introduce a new scope decision beyond what `q1ge` already
      reviewed; a fresh exact-head review of this plan file's specific
      wording remains open to any reviewer at Gate 1 per the normal
      advisory process, but the underlying scope-boundary question has
      independent two-model consensus already on record.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Public documentation content and site navigation structure only. No
      secrets, credentials, authorization, schema, or executable trust
      boundary touched by this plan.
---

# Plan: Get Started / Pages information-architecture rework — discovery and design phase

## Decision requested

Authorize a two-phase (discovery, then design) Gate 1 plan for the
deferred navigation/IA restructuring flagged in `20260806T112959Z-
governor-reviewer-k7mp.md` and `20260806T120408Z-governor-reviewer-dqg1.md`,
and named as the next program in `20260808T161743Z-planner-architect-
kpdj.md` finding 5 and `20260808T171050Z-contribution-auditor-q1ge.md`'s
independent concurrence.

This plan authorizes **discovery and design work only** — an audit of
where theory/taxonomy/rationale content currently lives across the public
Pages site relative to operational steps, and a written IA proposal for
where it should move. It does **not** authorize implementation. Any
content move, page split, or navigation change requires a separate,
subsequent Gate 1 decision scoped to the design phase's concrete output,
per `development-workflow-rule.md`'s plan-before-code discipline and
`downstream-impact-rule.md`'s classify-before-commit discipline.

## Baseline and preconditions (per `r6hp`, satisfied)

- Slice B (Get Started commissioning UX, Class A items 2-4) has a final,
  verified merged disposition: `20260808T182832Z-coder-tester-3jj2.md`
  confirms exact head `52151098c208578cf55799fc966f24654f94c479` merged
  to `main` under binding Gate 2 approval `20260808T182351Z-governor-
  reviewer-s3ms.md`.
- `web-ux-reviewer` is active (`agf/agents/web-ux-reviewer.yaml`,
  MR !1235).
- `independent-implementation-reviewer` has completed Gate 1
  (`20260808T193155Z-governor-reviewer-get2.md`) and is now implemented
  and merged (MR !1265) — available for exact-head review of this
  program's eventual implementation MRs, though not mandatory for every
  change per its own optional/named-per-candidate activation condition.
- The independent Pages-rework staffing consultations (`kpdj`, `q1ge`)
  are both on `main` and this plan preserves their conclusions rather
  than re-deciding them.

## Problem statement

Two governor review records, both predating Slice A/B's fixes, identified
that operational content (what a first-time visitor must *do*) and
theoretical/rationale content (gate taxonomy, responsibility diagrams,
governance philosophy, learning rationale) were interleaved on the Get
Started path, with theory sometimes preceding or crowding out the steps
themselves:

- `k7mp` (2026-08-06): the homepage hero foregrounded an eight-step
  governance tutorial ahead of prerequisites, and recommended moving
  "gate theory, responsibility taxonomy, diagrams, and learning rationale
  to follow-up material rather than the primary Get Started path."
- `dqg1` (2026-08-06): required that "theory, ownership tables, gate
  taxonomy, diagrams, and rationale must follow [the six-step commissioning
  path] or live behind reference links," and explicitly deferred the
  broader restructuring beyond the one page it was reviewing: "do not
  implement, change the CTA, or absorb this work into the deferred
  navigation slice before a separate Gate 1 approval."

Slice A and Slice B fixed the immediate, narrowest instances of this
problem: the homepage hero CTA now resolves to the commissioning page
(`commission.md`), and `commission.md`'s own Steps 1-6 are structurally
guaranteed (by `check_commissioning_first_screen.py`) to precede any
pre-step theory heading. Both fixes are mechanically enforced and narrow
by design (`gs-b-1-r3`'s governor disposition, `20260808T164952Z-
governor-reviewer-kqmj.md`, explicitly narrowed the invariant to headings
on that one page rather than attempting page-wide paragraph-level theory
detection).

What remains unaddressed, and is this plan's actual scope: whether the
same operational-before-theory problem recurs on **other pages** in the
Get Started path and its immediate neighbors, and whether the site's
**navigation structure** (not just individual page ordering) surfaces
theory-heavy pages to a first-time visitor before they need them.

## Scope

### In scope for this plan (discovery + design)

1. **Content/IA audit** of the following pages, each classified by
   operational-vs-theory content ratio and placement, using the same
   distinction `k7mp`/`dqg1` already established for `commission.md`:
   - `agf/docs/site/docs/start/should-you-adopt.md`
   - `agf/docs/site/docs/start/commission.md` (already fixed by Slice B;
     included as the audit's calibration baseline, not for re-work)
   - `agf/docs/site/docs/start/first-governed-change.md`
   - `agf/docs/site/docs/start/framework-map.md`
   - `agf/docs/site/docs/start/adopter-faq.md`
   - `agf/docs/site/docs/concepts/roles-and-gates.md`
   - `agf/docs/site/docs/concepts/coordination-protocol.md`
   - the top-level site navigation (`agf/docs/build/mkdocs.yml`'s `nav:`
     structure) and homepage (`agf/docs/site/docs/index.md` or
     equivalent), specifically what a first-time visitor's click path
     looks like before reaching `commission.md`.
2. **Written IA design proposal**: for each page found to mix operational
   and theory content ahead of the operational path, a concrete proposed
   fix (move to a new/existing reference page, move to page-end, or leave
   as-is with a stated reason). For navigation, a concrete proposed
   click-path from homepage to first governed change with a rationale.
3. **Visual/UX and accessibility evidence for the *current* state**: a
   `web-ux-reviewer` pass over the pages named above, in situ, as they
   render today — establishing a baseline before any design work is
   accepted, per `web-ux-reviewer.yaml`'s independence-from-implementer
   design principle applied at planning time, not only at Gate 2.
4. **Externally verifiable claims audit**: any public-facing copy on the
   named pages that makes a factual claim (version numbers, release
   counts, third-party comparisons, capability claims) is inventoried and
   handed to `fact-checker` for verification, per `verifiable-claims-
   rule.md` and `implementation-honesty-rule.md`.

### Explicitly out of scope for this plan

- **Implementation of any content move, page split, or navigation
  change.** This plan's acceptance criteria (below) are discovery/design
  deliverables only; a design proposal is not authorization to build it.
- **Re-litigating Slice A/B's already-approved and merged ordering** on
  `commission.md` or the homepage hero CTA. Those are settled under their
  own Gate 2 decisions and stay as calibration baseline only.
- **Visual/brand redesign** (palette, typography, layout system) beyond
  what the IA move itself requires. If the audit surfaces a case for
  broader visual redesign, that is a separate future plan, not folded in
  here.
- **CI, build, or publication pipeline changes.** `build-engineer` is
  named as consulted-if-needed (see Staffing) precisely because this
  plan's own scope should not require one — a page move or nav-entry
  change does not touch `.gitlab-ci.yml` or `mkdocs.yml`'s plugin
  configuration. If discovery finds otherwise, that finding routes back
  to the governor before design proceeds, not silently absorbed.
- **The `independent-implementation-reviewer` role's own further
  refinement.** It is complete and merged (MR !1265); this plan may use
  it for eventual implementation MRs but does not modify it.

## Staffing (preserving `kpdj` and `q1ge`'s independent conclusions)

| Role | Function in this program |
|---|---|
| `planner-architect` | Authors this plan and, later, the design-phase output and any implementation Gate 1 plan(s) it produces |
| `coder-tester` | Performs the content/IA audit and drafts the design proposal under this plan; implements only after a subsequent Gate 1 approval |
| `web-ux-reviewer` | Reviews the named pages' current rendered state (baseline) during discovery, and reviews any eventual implementation's rendered result before its Gate 2, per its existing independence-from-implementer requirement |
| `fact-checker` | Verifies externally-checkable claims found on the named pages during the audit |
| `build-engineer` | Consulted only if discovery finds a genuine CI/build/publication-pipeline touchpoint; not activated by default for this plan |
| `contribution-auditor` | Advisory review of this plan (requested) and, per its existing charter, of any subsequent implementation plan this program produces |
| `governor-reviewer` | Gate 1 approval of this plan; a separate binding Gate 1 approval for any design-phase output that authorizes implementation; Gate 2 approval of implementation MRs |

No new role is introduced by this plan. `independent-implementation-
reviewer` may be named for a specific future implementation MR if the
governor judges the change non-trivial enough to warrant it, consistent
with that role's own named-per-candidate activation condition — this
plan does not pre-commit to using it.

## Process

1. **Discovery** (this plan's primary deliverable): content/IA audit of
   the pages listed in Scope, `web-ux-reviewer` baseline pass, and
   `fact-checker` claims inventory. Published as a coordination record
   or a discovery appendix to this plan file.
2. **Design proposal**: a written IA proposal responding to discovery's
   findings, listing concrete per-page dispositions and a proposed
   navigation click-path. Routed through the same advisory consultation
   this plan itself received (`contribution-auditor`; `infosec-sentinel`
   remains not-applicable unless discovery finds otherwise).
3. **Governor decision on the design proposal**: approve as proposed,
   approve with changes, defer specific pages, or reject. This is a
   separate binding decision from this plan's own Gate 1 approval.
4. **Implementation Gate 1 plan(s)**, scoped from the approved design —
   likely sliced per page or small page group, following this project's
   established branch-sizing discipline (`branch-and-worktree-rule.md`'s
   "one yes/no question" test), not one large omnibus implementation.
5. **Implementation and Gate 2**, per-slice, with `web-ux-reviewer`
   rendered review mandatory before each slice's Gate 2 (per
   `visible-artifact-verification-rule.md`'s visual acceptance gate) and
   `independent-implementation-reviewer` named where the governor judges
   it warranted.

## Acceptance criteria (for *this* plan's Gate 1 — discovery/design phase only)

1. The content/IA audit covers every page named in Scope and classifies
   each as: operational-content-first (no change needed), theory-before-
   operational (candidate for the design proposal), or not-applicable
   (page is inherently reference/theory content, e.g. `roles-and-gates.md`,
   and is correctly reached only after the operational path, not instead
   of it).
2. The navigation audit produces a concrete description of the current
   homepage-to-first-governed-change click path and identifies any point
   where a first-time visitor is offered a theory-heavy page before an
   operational one.
3. `web-ux-reviewer` publishes a baseline rendered-state review of the
   named pages (both themes), independent of whoever performs the
   content/IA audit.
4. `fact-checker` publishes a claims-verification pass over any
   externally-checkable statement found on the named pages during audit.
5. The design proposal, once written, names a concrete disposition for
   every page and navigation point the audit flagged — no page is left
   "TBD" without an explicit reason.
6. No implementation occurs under this plan's authority. The design
   proposal is submitted for a separate binding governor decision before
   any implementation Gate 1 plan is authored.
7. `contribution-auditor`'s advisory review of this plan (requested
   alongside publication) is resolved — CLEAR or governor-accepted
   disposition of any BLOCK finding — before this plan's own Gate 1
   decision.

## Non-goals

- No content move, page split, navigation change, or visual/brand
  redesign implementation under this plan's authority.
- No re-opening of Slice A/B's merged, Gate-2-approved decisions.
- No CI, build, schema, release, version, or deployment change.
- No expansion of any role's charter or activation of a role not already
  active or Gate-1-approved.
- No claim that this discovery/design phase alone improves the site —
  the improvement, if any, is realized only at implementation, which
  this plan does not authorize.

## Risks and mitigations

- **Scope creep into visual redesign.** Mitigated by the explicit
  out-of-scope statement above and by naming `web-ux-reviewer`'s role as
  baseline-and-later-verification, not design authority — IA and visual
  design are different decisions, and this plan keeps the audit's
  IA-only frame explicit.
- **Discovery findings pre-empting a future governor decision.** The
  design proposal step exists specifically so discovery's *findings* do
  not silently become implementation *authorization* — a separate
  binding decision is required between them, per the Process section.
- **Ceremony inflation for what could be a small change.** If discovery
  finds that, in fact, only one or two pages need a small fix, the design
  proposal and subsequent implementation plan should be scoped that
  small — this plan does not commit the program to a large omnibus
  effort if the evidence doesn't support one.

## Validation

- `agov plan validate docs/plans/20260809T063148Z-pages-rework-discovery-and-ia.md`
- `agov coord validate` for this plan's advisory-consultation request and
  the governor's eventual Gate 1 decision record
- `git diff --check`
- No `mkdocs build` or code validation is required for this planning-only
  change; it applies once discovery/design work begins producing content.

## Rollback

This plan authorizes no implementation, so there is nothing to roll back
at this stage. If the discovery/design phase produces findings the
governor rejects, the design proposal is simply not approved — no
reversal of already-merged work is implicated, since none occurs under
this plan.
