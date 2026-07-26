---
id: public-feature-summary
status: pending-gate1
topic: public-feature-summary
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: public documentation content and placement only; no credentials, access control, schema, runtime, or security-boundary change
  - role: contribution-auditor
    status: consulted
    reason: this plan sets canonical public-facing claims about the framework's own governance mechanics; contribution-auditor review of claim accuracy, message-integrity (single canonical source), and scope-versus-evidence before Gate 2 is warranted per the existing coordination trail (aqjp, g4gs)
---

# Plan: public feature-summary content and placement

**Date:** 2026-07-26T07:12:00Z
**Status:** PENDING GATE 1 APPROVAL

## Trigger and problem

`20260725T214402Z-planner-architect-3krp` proposed a tight public
feature-summary bullet list for the README and public docs landing page,
covering gates, coordination, learning, and improvement, and asked the human
governor to decide reviewer staffing. The governor designated existing roles
(`20260725T215315Z-governor-reviewer-h4hm`): planner-architect for the
Pedagogist perspective, fact-checker for the Marketing perspective.

Both first passes were produced
(`20260726T064928Z-planner-architect-r5mm`, `20260725T215458Z-fact-checker-hqq2`).
fact-checker's critical re-review of the Pedagogist pass
(`20260726T065401Z-fact-checker-aqjp`) found the independence protocol had not
been observed as published (finding P1) plus four advisory content findings
(P2-P5). The governor waived the strict independence condition
(`20260726T070448Z-governor-reviewer-trra`), retaining the Pedagogist pass as
non-blind advisory input. A first synthesis attempt
(`20260726T070541Z-planner-architect-mgsm`) was published as a coordination
note rather than a schema-conforming plan and was BLOCKed by fact-checker's
Gate 1 advisory review (`20260726T070842Z-fact-checker-g4gs`) on three
grounds: missing plan artifact (S1, this file resolves it), a self-contradictory
placement contract (S2), and a bullet six that duplicated bullet one's
human-authority point instead of restoring the omitted removability/adoption
value (S3).

This plan supersedes `mgsm` as the Gate 1 artifact and resolves S1-S3.

## Independence status disclosure

This plan is built from one **non-blind** Pedagogist pass (`r5mm` —
independence condition waived by governor decision `trra`, not a genuinely
blind pass) and one **blind** Marketing pass (`hqq2`). Any published artifact
derived from this plan, and any future audit of it, must state this plainly
rather than describe both inputs as blind-independent.

## Content: six bullets, outcome before mechanism

Both passes converge on this set (per `aqjp`'s content-synthesis guidance and
`hqq2`'s six-message hierarchy):

1. **Human plan and merge decisions** — planning and merge decisions stay
   with a named human governor (gate numbers follow parenthetically, not
   leading — see Terminology below).
2. **Named roles, explicit authority** — proposing, implementing, reviewing,
   and approving are separate, named responsibilities.
3. **Durable, reconstructible records** — plans, handoffs, reviews, and
   decisions form a repository-native trail tied to exact revisions.
4. **Evidence and maturity labels** — capability claims are labeled by
   maturity level to limit overclaiming.
5. **Reviewed learning, separately governed improvement** — operational
   evidence can become a reviewed learning record; a learning may motivate an
   improvement proposal, but neither step changes rules, roles, or scope
   automatically. Both stages and the fresh-gate boundary between them stay
   explicit in the sentence — not compressed into one vague feedback loop
   (resolves `aqjp` P3).
6. **Repository-native ownership, bounded exit** (resolves S3 — rewritten,
   not the `mgsm` wording). The framework ships as files, rules, and
   conventions with no runtime and no service that holds an adopter's data;
   adoption is reversible by three documented exit modes — pause, downgrade,
   remove (canonical source:
   `agf/docs/site/docs/adopt/exit-and-decommission.md`). Optional validation
   tooling supports consistency but is not required to adopt or to leave.
   No numeric exit-time or "no-cost removal" claim is made — the canonical
   exit page itself makes no such claim, and this bullet must not exceed its
   evidence.

### P2 disposition — working-memory claim removed

`r5mm`'s uncited "4-6 item working-memory ceiling" claim is dropped per
`verifiable-claims-rule`. The six-item cap stands as pedagogical judgment
only (`aqjp`: "the five/six-item recommendation can stand as pedagogical
judgment" without the unsourced empirical framing).

## Placement: one deterministic contract (resolves S2)

`mgsm`'s placement section offered two different outcomes (byte-identical
dual placement vs. a shrunk README fallback) without choosing between them.
This plan selects **one** contract, per `g4gs`'s recommendation:

- The six-bullet block has exactly **one canonical source**.
- It is rendered or included **byte-identically** on exactly two public
  surfaces: the root README (after the opening problem statement, before
  cost/adoption detail) and the docs landing page
  (`agf/docs/site/docs/index.md`, before "How to read this site").
- If the build toolchain supports safe single-source inclusion (e.g. a
  shared partial/include), both placements render from it.
- If safe inclusion is not available at implementation time, the block is
  duplicated verbatim in both locations, and a deterministic equality check
  (CI step or pre-commit hook diffing the two blocks) is added in the same
  change that introduces the duplication — never a silently shortened
  README variant.
- All other public pages get a one-line pointer/link only, no repetition.
  Release and reference pages are excluded by default (`hqq2`).

## Terminology

"Gate 1/2," "exact-revision handoffs," and "improvement-proposal loop"
require a plain-language gloss before or immediately after first use in the
top-level list. Gate numbers follow the plain-language explanation
parenthetically, never lead the sentence.

## Acceptance criteria (deterministic, resolves `aqjp` P5)

- The six-bullet block's canonical wording is byte-identical between README
  and docs landing page (or generated from one source) — checked by diff or
  a CI grep/hash comparison, not visual inspection.
- No third public page contains the full six-bullet block — checked by
  grepping for the block's opening phrase across `docs/` and `README.md`.
- Every capability phrase in the block maps to a released artifact at the
  selected implementation baseline.
- Every benefit claim carries an evidence-status/maturity-level check
  consistent with `agf/rules/docs/implementation-honesty-rule.md`.
- Every gate mention has a plain-language gloss in the same sentence or the
  immediately following one — mechanically checkable by a reviewer per
  bullet.
- Bullet six's exit/removal claim does not exceed what
  `exit-and-decommission.md` itself states (no added numeric or no-cost
  claim).
- "Above the fold" placement and general readability remain **human-review
  prompts** at Gate 2 visual acceptance
  (`agf/rules/workflow/visible-artifact-verification-rule.md`), not
  automated acceptance criteria — `r5mm`'s non-mechanical wording on these
  two points is not carried forward as if it were machine-checkable.

## Claim constraints (from `hqq2`, unchanged)

No "continuous/automatic self-improvement" language; no implying CI/`agov`
approves a gate; no "complete audit trail" or "guaranteed compliance"
phrasing; distinguish shipped mechanism from organizational-benefit
maturity; no counts in the top block; keep `agov` visibly optional; preserve
the "does not run the assistant" boundary.

## Scope boundary

This plan authorizes content-and-placement decisions only. No copy drafting
or page edits begin until Gate 1 clears. `README.md`, `agf/docs/site/`, and
`agf/agents/MANIFEST.yaml` remain untouched by this plan itself.

## Next owner

Per `g4gs`'s "release advancement" instruction: after Gate 1 clears, route
the exact plan SHA to contribution-auditor (per this plan's advisory
consultation) and infosec-sentinel (already not-applicable, no further
action needed) before implementation begins.
