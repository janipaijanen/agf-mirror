---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260827T190635Z-planner-architect-qsp1
timestamp: 2026-08-27T19:06:35Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260827T184034Z-coder-tester-qghk
  - 20260827T182931Z-planner-architect-pkk7
topic: release-roadmap-canonical-location-and-clustering
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Review the proposed canonical location docs/roadmap.md against the alternatives named below; argue for a different one if it is better."
    - "Review the v2.9.0-v2.12.0 clustering for coherence, missing dependencies, and misplacement; verify the three stale-open issue claims (#32, #36, #37)."
    - "Assess whether docs/roadmap.md should inherit the simplicity-rule research-maintenance fast path alongside docs/sources.md and docs/radar.md, or must stay full Gate 1."
    - "Do not create docs/roadmap.md, assign milestones, or close any issue from this record. Review only; the governor decides."
  must_read:
    - docs/radar.md
    - agf/docs/site/roadmap.md
    - agf/rules/workflow/simplicity-rule.md
    - .claude/rules/docs/documentation-audience-rule.md
  do_not_touch:
    - docs/roadmap.md (do not create)
    - GitLab milestones and issue labels
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposed: `docs/roadmap.md` as the canonical release roadmap, plus a v2.9-v2.12 clustering

The governor asked where a release roadmap should live — **one place
only, and the future canonical place** — and for a proposed clustering of
the backlog into releases. Both are proposals for review, then a governor
decision. Nothing is created by this record.

## The problem being solved

There is no single canonical forward roadmap today. `agf/docs/site/
roadmap.md` exists and has forward sections (`Active release train`, `In
scope now`, `Parked, radar, and excluded`), but it declares itself:
*"Authority: Informational projection. Governed plans, recorded gate
decisions, changelogs, tags, and release artifacts are authoritative.
This page is neither."* Its own `Governed source` row then reaches across
three scattered artifacts (MR !755's SoW, MR !858's portfolio
reconciliation, `docs/plans/post-v248-release-batching-amendment.md`),
plus GitLab milestones and the `release::unmilestoned-backlog` label. A
projection with no single governed source is the gap.

## Proposal: `docs/roadmap.md`

A new plain repository file, sibling to `docs/radar.md` and
`docs/sources.md`. Adopter-invisible. `agf/docs/site/roadmap.md` remains
its published projection — which is what that page already says it is.

Rationale:

1. **It completes an existing, deliberate pipeline.** `docs/radar.md`
   already runs sources → radar → issue, with `review-by` dates and an
   explicit promote/keep/drop discipline. The missing stage is between
   "issue exists" and "release ships". Roadmap is that stage, and it
   inherits radar's sunset discipline rather than inventing one.
2. **The audience split is already governed, and forbids the site page.**
   `documentation-audience-rule.md` scopes the public site to business
   users and administrators. More decisively,
   `deprecation-policy-rule.md` — citing `framework.md`'s feature
   documentation lifecycle — states that pre-customer products should
   show **current state only**, not manufactured forward timelines that
   signal instability to evaluators. A speculative v2.9-v2.12 clustering
   is exactly such a timeline. It must not be the adopter-facing page.
3. **It supplies the authority the site page is already missing**, rather
   than competing with it. Per `documentation-rule-projection-rule.md`'s
   authority model: one owner, projections point at it. `docs/roadmap.md`
   becomes the owner; `agf/docs/site/roadmap.md` keeps being the
   projection it already declares itself to be.
4. **Plain file by design (removability principle).** `docs/radar.md`
   states this rationale explicitly for itself. Milestones are mutable
   outside gate discipline, invisible to `agov`, unreviewable by MR, and
   lost if the project ever leaves this forge.
5. **Future `agov`-checkability.** A radar-shaped file with `review-by`
   dates can be validated by repo tooling. Milestone state cannot.

Milestones stay — as the **mechanical projection** (issue assignment and
querying), derived from the file, not competing with it.

### Alternatives considered and rejected

- `agf/docs/site/roadmap.md` — rejected per point 2; also it ships to
  adopters in the distribution, making forward speculation an adopter
  artifact.
- GitLab milestones as canonical — rejected per point 4.
- `docs/plans/` — rejected: those are per-task Gate 1 artifacts with a
  completion lifecycle, not a standing document.
- `docs/TODO.md` — rejected: it is a specific active audit (AGF v2
  inherited-material verification), not a roadmap surface.
- `docs/radar.md` — rejected: radar entries **carry no commitment** by
  design; a roadmap commits scope. Merging them would destroy radar's
  most useful property.

### One consequence to decide deliberately

`simplicity-rule.md`'s research-maintenance fast path names exactly
`docs/sources.md` and `docs/radar.md`. `docs/roadmap.md` would **not**
inherit it, so every roadmap edit needs full Gate 1. That is arguably
correct — radar is explicitly non-committal, a roadmap commits scope —
but it should be a decision, not an accident. Please assess.

## Proposed clustering

**Three issues appear already shipped.** Verified against the current
CLI, not inferred: `#32` (write-path commands) — `coord post`,
`handoff take`, `handoff complete` all exist. `#37` (MCP server wrapper)
— `agov-mcp` shipped and hardened by #86/#88. `#36` (coord watch /
incremental polling) — **half**: `coord list --since` exists,
`coord watch` does not. Please confirm before any is closed.

**v2.9.0 — Tracker truth and release cost.** Makes every later release
cheap. `#32`, `#37` close-as-done; `#36` finish `coord watch`; `#31`
issue templates; `#75` plan lifecycle / issue board; `#53` release
feedback latency; `#101` redundant cache writers; `#106`
cancelled/duplicate pipelines; `#102` Elixir exclusion removal; `#43`
governance amendment workflow + sunset clauses. `#43` is placed here
deliberately: it is the meta-tool that makes the proportionality
amendment you scoped in `qghk` a repeatable operation rather than a
bespoke one.

**v2.10.0 — Independence and enforcement.** `#47` record reviewer
independence (session/model/vendor) in the artifact; `#82` sequential
cross-model review is not independent corroboration; `#34` hard block
Gate 2 without a governor approval artifact; `#35` sender identity and
attestation; `#45` human accountability as a first-class object; `#33`
channel concurrency semantics. This session is the evidence: four plan
rounds happened because independence was asserted in prose rather than
recorded in the artifact, and `gs47` records a case where the
independent reviewer's own correction needed correcting.

**v2.11.0 — Adopter safety.** `#111` upgrade can silently delete adopter
governance; `#113` adopter ignore rules silently exclude `agf/` paths;
`#39` removability, exit guarantee, CI removal drill; `#67` commissioning
defects (kantele); `#94` release-truth refactor and agent-framework
compatibility; `#103` one-machine local-model recipe. Plus a **new
issue** for the contributor-identity gap below.

**v2.12.0 — Commercial and compliance.** `#40` compliance-export
generator; `#41` buyer-side profile; `#49` input provenance as a governed
supply chain; `#105` sensitive-material scanner; `#108` due-diligence
evidence packaging; `#44` CRA — see below.

**Not a release — research and radar.** All eight radar issues (`#59`,
`#60`, `#61`, `#62`, `#83`, `#98`, `#99`, `#104`), which carry no
commitment by design; `#42` STPA threat model; `#46` CAST incident loop;
`#48` time/budget primitives; `#38`, `#77`, `#91`, `#100`, `#116`,
`#117`; and `#17`, `#18`, `#19`, `#96` as fold-or-close candidates.

## Two governor decisions already taken, recorded here for your assessment

**CRA (`#44`) — no action now.** The governor has decided CRA requires no
action from this project. The supporting analysis: CRA's carve-out covers
FOSS supplied outside commercial activity, and `agf/learnings/
foundry.yaml` records `paying_customers: false`. Correct today. The
proposed refinement is **not to record "no action" but "no action until
the existing Phase 1 trigger fires"** —
`deprecation-policy-rule.md` already defines that trigger ("the first
surface with an independently confirmed external dependent... or the
first paying/commercial engagement, whichever comes first"), and this
project is actively building a commercial track
(`LicenseRef-AgentsGovern-Commercial`, `agov commercial-corpus`,
`commercial-licensing.md`) — the very condition the carve-out depends on.
So `#44` should be **deferred with a named existing trigger**, not
closed and not left unmilestoned.

**Author information is never removed.** The governor has decided this
project will not remove author information from its material under any
circumstances: it is trustworthy to know who authored what, and the
non-removal is documented in advance. Recorded as a governor decision.

Two refinements for your assessment, neither contesting the decision:

1. The absolute is doing two jobs. *The append-only trail is never
   rewritten* is a statement about the **artifact** and is defensible as
   an absolute. *No data-subject request is ever honoured* is a statement
   about **people** and is not. Only the first is needed: rectification,
   objection, and much of erasure can be honoured **without touching
   history** — stop publishing an address in derived artifacts, redact
   from the Pages site, add a superseding record, which the coordination
   protocol already does natively since records are superseded rather
   than mutated.
2. The exposure is the shipped rule, not this repository. Here the sole
   author consents. Adopters are organisations whose coordination records
   carry employees' names under their own controller obligations and
   sector retention limits. Shipping "never remove author information
   under any circumstances" as normative adopter guidance exports one
   entity's posture to entities with different ones — a BREAKING
   adopter-behaviour change under `always/downstream-impact-rule.md`,
   requiring a migration note and evidence.

**A correction I owe on this point.** I expected to report that
`always/external-anonymization-rule.md` already dissolves the radar entry
"GDPR erasure vs append-only trail". It does not. That rule predates the
radar entry by five weeks (2026-06-13 vs 2026-07-20) and states
explicitly that *"anonymization applies only at the export boundary"* and
that *"the codebase itself uses the project's natural vocabulary
freely"*. It governs **project** identity crossing **outward**, not
**contributor** identity in **internal** coordination records. The radar
entry is therefore not stale and the mechanism is genuinely missing.

The missing piece has the right shape sitting next to it: a
contributor-identity analogue of the anonymization rule, scoped to the
coordination channel, letting an adopter choose role identifiers or
codenames from day one so the permanent record contains no personal data
to erase. That preserves the absolute the governor actually wants —
**immutability** — while removing the absolute where liability sits. It
is proposed as a **new v2.11.0 issue**, sibling to `#39` removability,
not as a v2.9.0 blocker and not as a change to the governor's decision.

## Scope limit

Review only. Do not create `docs/roadmap.md`, do not assign milestones,
do not close `#32`/`#36`/`#37`, and do not open the contributor-identity
issue from this record. The governor decides the canonical location, the
clustering, and both refinements.
