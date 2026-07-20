---
id: cra-entity-structure-decision
status: pending-gate1
topic: cra-open-source-steward-applicability
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      no code, no security boundary, no secrets — a documented legal/
      organisational position statement only.
  - role: contribution-auditor
    status: consulted
    reason: >-
      this is a compliance-facing claim about AGF's regulatory status;
      contribution-auditor should confirm the hedging language and
      recheck-before-citing caveats are adequate before the position
      is published anywhere external-facing.
---

# Plan/decision note: CRA entity-structure applicability (#44)

**Date:** 2026-07-20T07:50:58Z
**Status:** PENDING GATE 1 APPROVAL (lightweight — see process-weight
note below)
**Cluster:** compliance-export / buyer-side AGF / CRA entity-structure
decision (this file covers #44 only; #40/#41 are planned separately
in `docs/plans/compliance-export-buyer-side-agf.md`)

---

## Process-weight note (per `simplicity-rule.md` task-size calibration)

This is a **decision-and-document issue, not a build project.** Per
the task-size table in `.claude/rules/workflow/simplicity-rule.md`
("Typo, comment, doc update → Edit directly, commit"; "New field on
existing model → Plan note + ... + commit"), this issue is closer to
the lighter end of that spectrum: it touches zero code, zero modules
in the implementation sense, and produces one documented position with
a named trigger condition. A case could be made that no Gate 1 plan
file is strictly required at all — a commit-message-level "fast-track
waiver" recording the decision would arguably suffice.

This plan file is written anyway, at **reduced weight** (no affected-
files table beyond the one doc page, no test strategy, no rollback
section beyond "revert the doc commit"), for two reasons specific to
this issue rather than as a template default:

1. The decision is regulatory-facing and will likely be read by
   external parties (procurement counterparties, auditors) evaluating
   AGF's compliance posture — the same class of reader
   `always/verifiable-claims-rule.md` is written for. A written,
   reviewable artifact is cheap insurance against a future "when did
   we decide this and on what basis" question.
2. It sits in the same GitLab triage batch as #40/#41 and benefits
   from being reconciled at the same Gate 1 pass, even though its own
   internal complexity does not warrant the same weight.

**Recommendation to the human reconciling all seven clusters:** treat
this file as approvable on a much faster path than #40/#41 — it is a
documentation-only change with no build phase, no migration, and no
test strategy to review.

---

## The decision

**Current answer: the CRA Art. 24 "open-source steward" regime does
not currently apply to AGF.**

Reasoning: CRA Art. 3(14) requires a **legal person** to qualify as a
"steward" under the open-source steward definition. AGF is currently
maintained by a natural person (a solo maintainer), not through any
legal entity. A natural person cannot be a "steward" under this
definition — the obligation category (cybersecurity policy +
vulnerability-reporting duties; explicitly no-fines regime per the
Art. 24 framing) has no current AGF subject to attach to.

**This position is qualified, not asserted as settled law:**
per `docs/sources.md`'s own note on this source, the Art. 3(14)
"legal person" reading is "partially unverified — based on draft
Commission guidance as of 2026-03-03." Two things follow from that
qualifier, both stated here rather than left implicit:

1. Draft guidance can change before the CRA's reporting date. This
   decision must be re-checked against final (non-draft) Commission
   guidance before the 2026-09-11 reporting-start date, not treated as
   permanently settled today.
2. Nothing in this decision should be read as a firm legal opinion —
   it is AGF's documented working position, citable as such, not a
   substitute for counsel if AGF is ever formally incorporated.

## Trigger condition (supersedes this decision)

**If AGF is ever operated through a legal entity** (a foundation, a
company, a formal steward organisation — any legal person), this
decision is superseded automatically and the following becomes due:

- A CRA Art. 24 cybersecurity policy must be **published before
  2026-09-11 or the date of entity formation, whichever is later**.
- Vulnerability-reporting process obligations under the same Art. 24
  regime become live for that entity.

Relevant dates as surveyed in `docs/sources.md` (2026-07), **recheck
before citing** per the verifiable-claims-rule — regulatory dates move:

| Milestone | Date (as surveyed) |
|---|---|
| CRA Art. 24 reporting obligations begin (for in-scope entities) | 2026-09-11 |
| CRA full applicability | 2027-12-11 |

## Explicitly out of scope

- Forming any legal entity for AGF. This decision does not recommend,
  imply, or schedule entity formation — it only names what happens
  *if* that occurs.
- Drafting the Art. 24 cybersecurity policy itself. That drafting work
  is only triggered if the condition above fires; drafting it now,
  speculatively, would be exactly the kind of premature complexity
  `simplicity-rule.md` warns against ("configuration options that no
  one will ever change... future-proofing for requirements that do not
  exist").

## Where this gets recorded

- This plan file is the Gate 1 artifact for the decision itself.
- On approval, the decision's substance (current answer + trigger
  condition, in a form suitable for an external reader) should be
  added to wherever AGF documents its own regulatory posture for
  adopters/auditors — likely alongside or cross-linked from
  `agf/docs/adopt/compliance-profile.md`'s existing "framework's own
  tier" self-declaration pattern, since both are instances of "AGF
  states its own compliance-relevant status explicitly." Exact target
  page is a Gate 2 decision, not fixed here, to avoid pre-empting
  whichever cluster owns the public-docs site structure.
- `docs/radar.md` already carries a corresponding radar entry ("CRA
  steward structure... review-by: 2026-09") — once this decision is
  approved, that radar entry should be updated from "decision pending"
  to "decided, see docs/plans/cra-entity-structure-decision.md," or
  promoted off the radar entirely, per the radar's own promote/keep/
  drop discipline. This plan does not edit `docs/radar.md` itself
  (out of scope for this cluster; flagged for whoever owns radar
  upkeep).

## Assumption about another cluster's ownership

This note assumes no other cluster in this triage batch is separately
deciding AGF's legal-entity status for unrelated reasons (e.g. a
funding or foundation-formation proposal). If such a proposal exists
elsewhere, this decision's "does not currently apply" answer becomes
the input to that proposal's trigger condition, not a conflicting
decision — the two are compatible by construction (this note only
fires the *obligation*, it never argues for or against forming an
entity).
