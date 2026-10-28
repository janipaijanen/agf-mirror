---
id: 20260920T113336Z-governance-mode-classification
status: approved
gate1_approved_by: 20260920T115342Z-governor-reviewer-3es7
gate1_approved_at: 2026-09-20T11:53:42Z
topic: governance-mode-classification-light-normal-strict
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Rule-content-only change touching no learning records, credentials,
      or cross-record leak surface. `agov validate infosec` scans learning
      records; this plan has no file under agf/learnings/ in its diff, so
      that mechanical check has nothing to run against.
  - role: contribution-auditor
    status: waived
    reason: >-
      Same-actor limitation disclosed, not a fully independent audit: the
      planner-architect statement and the contribution-auditor-lens
      statement below were both produced by this same session/model
      (claude-sonnet-5). Codex (the usual different-model contribution-
      auditor player) is dormant (token-exhausted, expected back
      ~2026-09-23). The governor was told this explicitly and directed
      "proceed as proposed" anyway in the live session
      (2026-09-20T~11:45Z), which this plan treats as the governor's
      direct-assignment acceptance of self-analysis in place of an
      independent audit -- the same resolution path used previously for a
      genuine same-actor conflict (see coordination record `rn3m`,
      2026-09-12, CP-B-1). A durable independent contribution-auditor
      re-review by a different model remains available and welcome once
      Codex is back; this waiver does not foreclose one.
affected_paths:
  - agf/rules/workflow/simplicity-rule.md
  - .claude/rules/workflow/simplicity-rule.md
  - agf/rules/always/downstream-impact-rule.md
  - agf/rules/always/high-risk-operations-rule.md
  - agf/rules/always/gate-self-test-rule.md
  - docs/reviews/rule-projection/20260920T113336Z-governance-mode-classification.yaml
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governance mode classification: LIGHT / NORMAL / STRICT

## Intent

The governor observed (2026-09-20, live session) that a single process
weight for every task doesn't scale toward "machine speed": "fix two
paragraphs" and "produce the contractual ERP requirements baseline" cannot
honestly go through the same gate ceremony, but today the choice of how
much process a task gets is implicit and scattered across several rules
that a session has to independently know and cross-reference:

- `simplicity-rule.md`'s task-size table and fast-track waiver already
  define a *lighter* path (skip the plan file, commit-message
  justification) for small, low-risk work — this is today's unnamed
  LIGHT tier.
- The full Gate 1 → Gate 2 flow in `development-workflow-rule.md` is
  today's unnamed NORMAL tier (the default).
- `high-risk-operations-rule.md` (named code areas: audit log pipeline,
  access control, auth flow, RLS/tenant isolation, migrations,
  time-lock logic — STOP and plan first, full security suite after) and
  `gate-self-test-rule.md` ("gate mechanics are never eligible for
  fast-track regardless of diff size") are today's unnamed STRICT tier.

None of these three tiers currently have a name, a single place that
states "these are the three tiers and here is how you pick one," or a
heuristic a session can run *before* starting work to self-classify. A
session currently has to already know to check `simplicity-rule.md`'s
disqualifier list AND separately check whether the change touches a
`high-risk-operations-rule.md` area AND separately check
`gate-self-test-rule.md`'s gate-mechanics carve-out AND separately check
`downstream-impact-rule.md`'s BREAKING classification — four rules, no
single decision point. This plan names the three tiers explicitly and
gives one ordered heuristic for picking among them, absorbed into
`simplicity-rule.md` (the rule that already owns task-size calibration),
per the governor's directive to formalize LIGHT/NORMAL/STRICT and the
governor's explicit choice among the design options this plan's Gate 1
consultation round surfaced:

- Mode selection: **heuristic-first**, driven by change characteristics
  (cosmetic / local-impact vs. scope-changing / contract-changing),
  "towards machine speed" — not a manual per-task human tag as the
  primary mechanism.
- Mode scope: **consolidation, not expansion** — LIGHT = today's
  fast-track waiver formalized; STRICT = today's high-risk-operations
  list formalized; NORMAL = today's unchanged default. No new
  obligations invented in this pass.
- Rule structure: **absorbed into `simplicity-rule.md`**, replacing its
  existing task-size table and fast-track section, rather than shipping
  as a new standalone rule.

## What this plan is, and is not

**Is:** a rename-and-consolidate pass. Every substantive obligation named
below already exists in the codebase today, scattered across
`simplicity-rule.md`, `high-risk-operations-rule.md`, and
`gate-self-test-rule.md`. This plan gives the three existing tiers names,
puts the selection heuristic in one place, and cross-references (never
duplicates) each tier's substantive content at its existing owning rule —
following `documentation-rule-projection-rule.md`'s single-owner
discipline so LIGHT/STRICT criteria don't drift into two competing
copies.

**Is not:** a new obligation, a new gate, a new validator, a security
change, or an expansion of what LIGHT is permitted to skip or what STRICT
requires beyond today. Per `downstream-impact-rule.md`'s Parnas boundary
("a change is editorial if and only if the set of actions the rule
requires, forbids, or permits is unchanged"), the LIGHT/NORMAL eligibility
criteria are **unchanged** from today's fast-track disqualifier list, and
the STRICT trigger list is **unchanged** from today's
`high-risk-operations-rule.md` area list plus `gate-self-test-rule.md`'s
gate-mechanics carve-out plus `downstream-impact-rule.md`'s BREAKING
classification. Classification: **PREVENTIVE** (makes an already-implicit
tier structure explicit; does not restrict previously-permitted behavior)
for the LIGHT and NORMAL definitions, **ADDITIVE** for the STRICT-tier
name and the selection heuristic itself (new decision-point content, no
prior contract removed).

**Also is not, and explicitly deferred:** mechanical/CI enforcement of a
declared or heuristically-selected mode (e.g. a validator that checks a
plan's declared mode against its actual diff shape). `gate-self-test-rule`
requires a negative test for any *gate* — this plan introduces no gate,
only rule *content*; a future plan that adds machine enforcement (e.g.
`agov plan mode-check`) would itself be gate mechanics and would need its
own Gate 1, its own negative self-test, and is out of scope here. Flagging
as the natural next step, not silently rolled in.

## Observation for the governor (not a finding, not fixed here)

`high-risk-operations-rule.md`'s current content (audit log pipeline,
access control, authentication flow, RLS/tenant isolation, migration
scripts, time-lock logic) reads as generic adopter-application boilerplate
— it describes a SaaS-style app's high-risk code areas, not this project's
own actual domain (a governance framework whose "high-risk areas" are
things like `agf/rules/**`, `agf/schemas/*.json`, coordination lifecycle
semantics, release/tag mechanics, and gate validators — i.e. much of
`downstream-impact-rule.md`'s own trigger-path list). It appears
uncustomized for AGF-NG itself. The STRICT tier this plan defines
therefore cross-references `high-risk-operations-rule.md`'s list
as-written (per the governor's "formalize as-is" instruction) plus
`downstream-impact-rule.md`'s trigger paths, which together give a
STRICT trigger set that *is* meaningful for this project even though
`high-risk-operations-rule.md` itself is not yet customized. Recommend a
separate follow-up plan to rewrite `high-risk-operations-rule.md` for
AGF-NG's actual domain; not attempted here to keep this plan's diff
reviewable and single-purpose.

## Proposed content (draft — subject to Gate 1 review)

Replace `simplicity-rule.md`'s "Calibrating workflow overhead to task
size" and "Fast-track waiver" sections, and its "Sources/radar research-
maintenance fast path" section's framing (content unchanged, re-parented
under LIGHT), with:

### Governance mode: LIGHT / NORMAL / STRICT

Every task is classified into exactly one mode before work starts. The
classification is a heuristic self-check the acting session runs, not a
mandatory human pre-declaration — matching the governor's "heuristics
first, towards machine speed" instruction — but any human or coordination
record may explicitly declare a mode, and an explicit declaration
overrides the heuristic result when they disagree (a human declaring
STRICT on a change the heuristic would call LIGHT always wins; the reverse
requires the disqualifier list below to genuinely not apply).

**Step 1 — check STRICT disqualifiers first (if any apply, mode is
STRICT, full stop):**
- Touches any `high-risk-operations-rule.md` named area (audit log
  pipeline, access control/authorization, authentication flow, row-level
  security/tenant isolation, migration scripts touching security policy
  or constraints, time-lock/state-lock logic).
- Touches any `downstream-impact-rule.md` trigger path
  (`.claude/rules/**`, `agf/rules/**`, `agf/agents/*.yaml`,
  `agents-govern/agents/*.yaml`, `agents-govern/src/agents_govern/`,
  `agf/schemas/*.json`, `docs/coordination/agov-config.yaml`,
  `AGENTS.md`, `tooling/**`) **and** is classified BREAKING under that
  rule's Parnas test.
- Is gate mechanics per `gate-self-test-rule.md` (introduces or modifies
  a git hook, validator, CI check, pre-commit script, or MR-blocking
  rule) — always STRICT regardless of diff size, per that rule and per
  `simplicity-rule.md`'s existing "gate mechanics are ineligible
  regardless of diff size" statement.
- PII, security boundary, architecture, or data-model change (today's
  existing fast-track disqualifier list, unchanged).
- Public AGF product/marketing/competitive/performance/maturity/
  compliance claim, or a governed-decision rewrite (today's existing
  sources/radar fast-path disqualifier list, generalized to all work —
  unchanged in substance).
- **Not included in this plan's STRICT list** (deferred, per the Gate 1
  resolution below): contractual, compliance, legal, or externally-binding
  deliverables — the governor's own "contractual ERP requirements
  baseline" example. Routed to a fast-follow plan instead of this one; see
  "Open question for Gate 1 — RESOLVED" below.

**Step 2 — if none of the above apply, check LIGHT eligibility (today's
fast-track criteria, unchanged):** cosmetic/local-impact task — typo,
comment, ordinary doc update, single-file bug fix with obvious cause, a
narrow reversible research-maintenance edit to `docs/sources.md`/
`docs/radar.md` meeting that section's existing eligibility list. If
eligible: skip the separate Gate 1 plan file; record the fast-track
waiver in the commit message (task + why low-risk, unchanged from today).
If the task grows mid-flight into a STRICT or NORMAL shape, stop and
escalate — do not keep the LIGHT process past the point it fits.

**Step 3 — otherwise, NORMAL (today's default, unchanged):** full Gate 1
plan → governor-reviewer approval → implementation → Gate 2 review →
merge, exactly as `development-workflow-rule.md` and
`systematic-code-review-rule.md` already require.

### Open question for Gate 1 — RESOLVED

The STRICT disqualifier list's contractual/compliance/legal/externally-
binding-deliverable trigger was the one genuinely new category in this
plan — not literally present in `high-risk-operations-rule.md`'s current
enumerated areas, though it matches the governor's own "contractual ERP
requirements baseline" worked example. Two options were put to the
governor at Gate 1: (1) include it now, accepting one ADDITIVE bullet
alongside the otherwise-PREVENTIVE consolidation; or (2) defer it to a
fast-follow plan that first customizes `high-risk-operations-rule.md` for
AGF-NG's actual domain, keeping this diff byte-for-byte a rename-and-
consolidate pass.

**Resolved 2026-09-20 (live session): option 2 (defer).** The governor
directed "proceed as proposed," adopting this plan's author recommendation
and the converged multi-role synthesis below. This plan's STRICT list is
therefore exactly: `high-risk-operations-rule.md`'s current areas, plus
`downstream-impact-rule.md`'s BREAKING trigger, plus
`gate-self-test-rule.md`'s gate-mechanics carve-out — nothing more. The
contractual/compliance/legal deliverable trigger is explicitly **not**
part of this plan's scope; it is a named fast-follow (see "Observation for
the governor" above), not silently dropped. See bridge/decision record
referenced in this plan's `gate1_approved_by` field for the binding
transcription of this instruction.

#### Role consultation summary (multi-role synthesis, single session)

Before the governor's "proceed as proposed" instruction, this session
was asked what its own agentic roles would propose on this exact
question, and answered with the following per-role positions (transcribed
here verbatim, per the governor's explicit "include the statements above"
instruction). **Disclosure:** all four are this same session/model
(claude-sonnet-5) reasoning from each role's mandate, not four
independent reviewers — see the `advisory_consultation` waiver above for
why that is being accepted rather than treated as blocking.

> **planner-architect** (already on record in the plan): defer. Keep
> MR !3074 a single-purpose consolidation diff — renaming existing tiers
> vs. inventing a new obligation are different review questions, and
> bundling them fails `branch-and-worktree-rule.md`'s own sizing test
> ("can a reviewer answer one yes/no question?").
>
> **contribution-auditor**: also defer, on a sharper ground — this
> repo's own pattern (`environment-interrogation-rule.md`,
> `destructive-op-preflight-rule.md`, `boundary-shape-testing-rule.md`,
> `deputy-governor-rule.md` — all shipped as "Provisional status" on
> single-adopter/single-incident evidence, explicitly requiring
> corroboration before promotion) is exactly what a contractual-
> deliverable STRICT trigger would need. Right now the entire evidence
> base for that trigger is one conversational example. Folding it into
> STRICT as an unqualified MUST skips the evidence discipline the rest
> of the codebase applies to itself. If it ships at all in this pass, it
> should ship "Provisional," not co-equal with the existing
> high-risk-operations list.
>
> **build-engineer**: a different objection — as scoped, this plan
> doesn't actually buy machine speed yet. It's pure rule-text; no
> pipeline runs faster, no CI job gets skipped, because mechanical
> enforcement is explicitly deferred. The real "machine speed" payoff is
> a follow-up that wires mode into CI path-scoping
> (`ci-trigger-scoping-rule.md`'s pattern — LIGHT-tier changes skip the
> heavy jobs the way governance-only MRs already do). Prioritize that
> follow-up plan over debating one more STRICT bullet now.
>
> **coder-tester**: the disqualifier list currently mixes two kinds of
> criteria — objective/path-based (mechanically checkable, e.g. "touches
> `agf/rules/**`") and subjective ("is this a contractual/compliance
> deliverable?", which needs human judgment). Anything meant to
> eventually gain a negative self-test (per `gate-self-test-rule.md`)
> needs to be in the mechanical half. Keep the subjective trigger out of
> this pass, or at least flag it as advisory-only, not something a
> future validator could check.
>
> **Convergent recommendation across all four**: defer the
> contractual/compliance trigger to its own follow-up plan (paired with
> the already-flagged `high-risk-operations-rule.md` customization),
> keep this MR as pure consolidation, and prioritize a second follow-up
> that wires mode into CI scoping — that's where "machine speed" actually
> gets delivered, not in the rule text alone.

The governor's "proceed as proposed" instruction adopts this converged
recommendation. Two follow-ups are named, not scheduled by this plan:
(a) customize `high-risk-operations-rule.md` for AGF-NG's domain and
reconsider the contractual/compliance STRICT trigger there, with proper
evidence framing (Provisional-status header, per this codebase's own
pattern); (b) wire LIGHT/STRICT mode into CI path-scoping per
`ci-trigger-scoping-rule.md`, which is where the actual runtime "machine
speed" gain would come from.

## Affected files

- `agf/rules/workflow/simplicity-rule.md` (canonical owner; content
  change described above)
- `.claude/rules/workflow/simplicity-rule.md` (this project's Claude-facing
  loader; per `governance-file-layout-rule.md` and this rule's own header
  it is a projection of the canonical file — confirm at implementation
  time whether it needs a matching edit or already just points at the
  canonical file)
- `docs/reviews/rule-projection/20260920T113336Z-governance-mode-classification.yaml`
  (six-row projection screen per `documentation-rule-projection-rule.md`,
  since this changes normative guidance; see below)

No schema, validator, CI, or agent-definition file changes in this plan's
scope (confirmed no gate mechanics — this is rule content only).

## Rule-projection screen (`documentation-rule-projection-rule.md`)

To be completed in full as
`docs/reviews/rule-projection/20260920T113336Z-governance-mode-classification.yaml`
before Gate 2; six-row summary for Gate 1 visibility:

| Surface | Disposition (draft) |
|---|---|
| Normative owner | `agf/rules/workflow/simplicity-rule.md` (existing owner, content updated) |
| Adopter-facing explanation | Not applicable — no Pages/adopter-facing page currently explains task-size calibration separately from the rule itself |
| Runbook/situation map | `docs/reference/agf-development-starter.md` (non-normative checklist) — check whether it references the old table language and needs a pointer update |
| AI landing/loaders | `AGENTS.md`'s "Gate discipline" section references `simplicity-rule.md`'s fast-track waiver by name already; confirm the reference still reads correctly after the rewrite, update if the section name changes |
| Validation/drift evidence | Advisory only — no mechanical validator for mode selection in this plan's scope (see "explicitly deferred" above) |
| Distribution | `agf/rules/` ships in the adopter distribution archive already; no new file added outside that tree |

## Test strategy

No code/tooling under test — this is rule-content-only. Verification is:
- `agov plan validate` on this plan file.
- Manual read-through confirming every STRICT/LIGHT criterion cited here
  matches the current text of `high-risk-operations-rule.md`,
  `gate-self-test-rule.md`, and `downstream-impact-rule.md` at
  implementation time (those rules may have moved since this plan was
  drafted — re-check, don't assume).
- `documentation-rule-projection-rule.md`'s own mechanical checks
  (`agov plan projection-check`) once the projection YAML exists.

## Risks

- **Drift risk**: cross-referencing four rules instead of duplicating
  their content is the whole point (single-owner discipline), but it
  means a future edit to `high-risk-operations-rule.md`'s area list or
  `downstream-impact-rule.md`'s trigger paths silently changes what
  STRICT covers without touching `simplicity-rule.md` itself. Acceptable
  — that is exactly the single-owner behavior
  `documentation-rule-projection-rule.md` asks for — but worth naming so
  a future reviewer isn't surprised.
- **Adoption risk**: renaming an existing informal structure changes
  vocabulary contributors and sessions use; low risk since the
  underlying rules are unchanged, but coordination records and prior
  learnings that say "fast-track" should still resolve correctly (the
  term is kept as a synonym for LIGHT within `simplicity-rule.md`, not
  replaced outright, to avoid breaking historical references).
- **Scope-creep risk**: the "Open question for Gate 1" above is exactly
  the kind of small addition that could quietly expand STRICT beyond
  consolidation; deliberately surfaced for an explicit governor choice
  rather than resolved unilaterally.

## Rollback

Pure documentation/rule-content change on a feature branch behind Gate 1
and Gate 2; revert-by-new-commit if the governor finds the consolidated
text worse than the status quo. No runtime, schema, or data effect to
unwind.
