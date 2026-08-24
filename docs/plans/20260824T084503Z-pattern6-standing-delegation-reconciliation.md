---
id: pattern6-standing-delegation-reconciliation
status: pending-gate1
topic: pattern6-standing-delegation-reconciliation
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Requesting independent contribution-auditor review before Gate 1.
      I (planner-architect) author this plan; the originating carrier
      (issue #118) is a governor-authored corrective-work issue
      recording an already-binding disposition (`20260823T221311Z-
      governor-reviewer-nrpt.md`) — this plan's own design choices
      (exact reconciliation wording, negative example, verification
      scope) are new synthesis, not copied from either.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Pure documentation reconciliation in `agf/framework.md` (a
      normative-but-not-code file) plus a verification pass over
      already-shipped adopter/loader pages. No code, schema,
      credential, or security-relevant surface is touched.
---

# Plan: reconcile `agf/framework.md` Pattern 6 with the approved standing-delegation model

## The problem

Governor decision `20260823T221311Z-governor-reviewer-nrpt.md` approved
Gate 2 for MR !1878 (the sources/radar research-maintenance fast path,
now shipped in `agf/rules/workflow/simplicity-rule.md`'s "Sources/radar
research-maintenance fast path" section) while explicitly accepting two
bounded documentation risks and assigning their correction to this
issue:

1. **`CA-RMFP-C2`** — `agf/framework.md`'s Pattern 6 ("Fast Track") still
   says, unconditionally: "Only the Governor-Reviewer (or Human) can
   decide to use the fast track" and "The waiver decision must be
   recorded: what task, why it qualifies as low-risk, **who waived Gate
   1**." The shipped sources/radar fast path is a **different**
   mechanism: a standing, narrowly-bounded delegation, itself already
   approved once through ordinary Gate 1 (as a rule addition), which an
   implementer *applies* by satisfying every mechanical eligibility
   condition — not a fresh, individual governor decision naming a "who
   waived Gate 1" actor each time. Pattern 6's current text has no
   language describing this second mechanism at all, so the framework
   document is internally contradictory the moment a reader compares it
   against the rule that actually ships.
2. **`CA-RMFP-C-A1`** (advisory, accepted as minor documentation risk) —
   no concrete negative example separates "a radar entry's lifecycle
   was advanced via the fast path" from "this topic is now on the
   roadmap, assigned to someone, adopted, or authorized for
   implementation." The disqualifier prose in `simplicity-rule.md`
   already says the fast path "does not... promise implementation,
   assign work, select priority" — but no worked example shows what
   that means in practice, leaving room for a reader to conflate
   "radar entry updated" with "this is now being built."

The governor's decision recorded an explicit revisit condition: resolve
or explicitly redisposition issue #118 before any release containing
the sources/radar fast-path feature. This plan is that resolution.

## Verified: no other shipped surface needs a corresponding edit

Per the governor's own disposition text ("The canonical rule,
development-workflow pointer, loaders, and adopter guidance
**consistently state the approved standing-delegation model**"),
checked directly rather than assumed:

- `agf/rules/workflow/simplicity-rule.md`'s "Sources/radar
  research-maintenance fast path" section (the canonical rule) —
  correctly describes eligibility conditions and disqualifiers as a
  standing test, with no "governor decides case-by-case" framing.
- `.claude/rules/workflow/simplicity-rule.md` (foundry's local mirror)
  — verified byte-identical to the canonical file for this section.
- `agf/rules/workflow/development-workflow-rule.md`'s "Exact
  research-maintenance exception" paragraph — correctly points to
  `simplicity-rule.md` as canonical, does not restate or contradict it.
- `agf/docs/site/docs/operate/day-to-day.md` — "Fast track if it
  qualifies; plan + Gate 1 if not," describing the eligibility-test
  model correctly.
- `agf/docs/site/docs/operate/run-agf-in-real-situations.md` — its
  "You are maintaining `docs/sources.md` or `docs/radar.md`" row
  correctly says "Apply the exact research-maintenance eligibility
  test... If all pass, use the sources/radar fast path... otherwise
  stop and prepare Gate 1," and its "the task feels docs-only" row
  correctly warns not to generalize the exception.

**Only `agf/framework.md`'s Pattern 6 is stale.** This narrows the
plan's actual edit to one file, with the other loader/adopter surfaces
disposed as "already consistent, verified" rather than additionally
touched.

## Design

### 1. Add a "Standing, pre-approved delegations" subsection to Pattern 6 (fixes `CA-RMFP-C2`)

Insert after the existing "Required actions when fast-track is
granted" numbered list and before "Anti-pattern," in
`agf/framework.md`:

> **Standing, pre-approved delegations (as opposed to per-instance
> waivers).** The rules above describe the Governor-Reviewer deciding
> fast-track eligibility case by case, for one task at a time. A
> project MAY additionally define a **standing, narrowly-bounded
> delegation**: a mechanically-checkable eligibility test, itself
> approved once through ordinary Gate 1 as a rule addition, that lets
> an implementer skip the per-instance Gate 1 plan when — and only
> when — every condition of that pre-approved test holds. An
> implementer satisfying such a test is *applying* a rule the Governor
> already approved, not exercising fresh individual waiver authority;
> ambiguity about whether the test is satisfied, or any of the rule's
> own disqualifiers, returns the work to the ordinary ("who waived Gate
> 1") ceremony above. AGF-NG's own shipped example is the sources/radar
> research-maintenance fast path
> (`agf/rules/workflow/simplicity-rule.md`, "Sources/radar
> research-maintenance fast path" section): a bounded set of file
> paths, content categories, and evidence requirements, with an
> explicit disqualifier list, that a reversible `docs/sources.md` /
> `docs/radar.md` change may use instead of a fresh Gate 1 plan — Gate
> 2 (a binding governor-reviewer decision) still applies in full,
> exactly as it does for a per-instance waiver.
>
> A standing delegation does not relax Gate 2, does not authorize
> implementation of what the touched content merely discusses, and
> does not by itself change roadmap priority, staffing, or adoption
> status. **Negative example (fixes `CA-RMFP-C-A1`):** if the
> sources/radar fast path is used to advance a radar entry's lifecycle
> from "watch" to "candidate" with a refreshed `review-by` date, that
> action alone does not mean the topic is now roadmapped, assigned to
> anyone, adopted as a dependency, or authorized for implementation —
> those remain separate, later decisions through the project's ordinary
> planning process, exactly as the rule's own disqualifier list
> requires.

### 2. What this plan does NOT do

- Does not reopen, revoke, or reinterpret the binding Gate 1 or Gate 2
  decisions for MR !1878 (`am2p`-analogous governor authority is not
  exercised here; this plan only reconciles documentation with an
  already-decided model).
- Does not authorize any new implementation, adoption, or roadmap
  change by virtue of documenting the standing-delegation concept.
- Does not touch `simplicity-rule.md`, `development-workflow-rule.md`,
  or any adopter/loader page — verified above as already consistent;
  editing an already-correct file to "match" would risk introducing
  drift, not fix any.
- Does not generalize the standing-delegation concept into a new,
  broader governance mechanism — it documents the one that already
  shipped, at the scope it already has.

## Files expected to change

This plan's own diff (Gate 1 candidate): only this plan file.

On the implementation branch, after Gate 1 approval:

- `agf/framework.md` — the one new Pattern 6 subsection above, inserted
  verbatim (or lightly copy-edited for the surrounding document's
  voice, preserving all normative content) between "Required actions
  when fast-track is granted" and "Anti-pattern."

No other file changes — see "Verified: no other shipped surface needs a
corresponding edit" above.

## Test strategy

Documentation-only; no unit tests apply.

- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella): no new findings beyond pre-existing,
  unrelated warnings.
- Manual check: read the inserted subsection in context within
  `agf/framework.md`'s Pattern 6, confirming it does not contradict the
  existing per-instance-waiver rules above it, and that the negative
  example is concrete enough to resolve `CA-RMFP-C-A1`'s "one example
  would reduce operator ambiguity" request.
- Cross-check (Gate 2 acceptance evidence): re-verify, at the exact
  implementation head, that `simplicity-rule.md`,
  `development-workflow-rule.md`, and the two adopter Pages named above
  still say what this plan's "Verified" section quotes them as saying
  — confirming no drift occurred between Gate 1 and Gate 2.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `agf/framework.md` — not itself one of the rule's
enumerated trigger paths (`agf/rules/**/*.md`, `agf/agents/*.yaml`,
`tooling/rust/` or framework tooling, `agf/schemas/*.json`,
`agf/templates/`, `agf/dist/`,
`agf/docs/site/reference/environment-variables.md`,
`docs/coordination/agov-config.yaml`, `AGENTS.md` — checked directly
against the rule's current text, not assumed). Documenting the
classification here for transparency even though the rule's own screen
does not fire:

```
Impact:
  Foundry:          EDITORIAL. Reconciles framework prose with a
                     mechanism that already shipped and is already
                     binding via governor decision nrpt; no new
                     obligation, permission, or prohibition is created
                     — the sources/radar fast path already exists and
                     already behaves this way. This document brings the
                     description in line with the shipped behavior.
  Direct adopters:  EDITORIAL. An adopter who has not adopted the
                     sources/radar fast path (or an equivalent standing
                     delegation) sees no behavior change; an adopter
                     who has already adopted it now finds framework.md
                     no longer contradicts the rule they are actually
                     following.
  Downstream:       No change.
  Migration:        Not required (EDITORIAL, not BREAKING or ADDITIVE
                     in the behavioral sense).
  Evidence:          Governor decision `20260823T221311Z-governor-
                     reviewer-nrpt.md`; contribution-auditor findings
                     `CA-RMFP-C2` and `CA-RMFP-C-A1`
                     (`20260823T211206Z-contribution-auditor-g4rb.md`);
                     GitLab issue #118.
```

**Adopter docs parity:** `not applicable` — this plan edits
`agf/framework.md` itself, which *is* the adopter-facing explanation of
the framework's own governance patterns; there is no separate page to
keep in parity with it.

## Rollback

Pure prose addition to one file, no schema, code, or data implications.
A one-commit revert restores Pattern 6 to its pre-plan text; the
shipped sources/radar fast path itself is unaffected either way (this
plan documents it, it does not define or gate it).
