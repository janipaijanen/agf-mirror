# Claude independent backlog ranking — frozen freeze-only base

> Independence statement: this pass was performed against freeze-only
> commit `46784e042ca1452bb8687b742f9739d2b83894de` (MR !1480) only.
> Codex's classification/ranking head `edcbba8bb6c19d61a95150f3c3aa689a000a6d26`
> was **not read** before this document was frozen. Any resemblance to
> Codex's conclusions is convergent, not copied.

Frozen at (source): `2026-08-12T21:09:35Z`, base `origin/main`
`1ecbd744e8f8c44472830ac2bc2ca429dd7a9529`. This pass performed
`2026-08-13`, working entirely from the frozen table data — no new
live GitLab queries were made, to stay comparable to Codex's same
frozen baseline.

## Method and honest scope limits

- **MRs, issues, learnings, radar**: classified individually below —
  I have titles, states, and (for many) direct session knowledge from
  this project's own coordination history.
- **123 source-registry URLs**: NOT read individually. Classifying by
  title/URL pattern into thematic clusters only. This is a disclosed
  coarse pass, not equivalent-depth review — flagging this explicitly
  per the plan's completeness requirement rather than presenting a
  false precision. One exact duplicate found without opening either
  link: SOURCE-036 and SOURCE-048 are both
  `https://www.iso.org/standard/42001`.
- Prior work folded in as evidence, not re-derived: this session's own
  low-IID cleanup batches 1 (`rjhs`, MR !1471, 9 MRs) and 2 (`p633`,
  MR !1473, 10 MRs) already produced verified, exact-head-checked
  dispositions for 19 of the 44 MRs. Those findings are reused here
  rather than re-investigated.

## Cluster map (MRs + issues + learnings, by outcome)

### C1 — Gate/authority enforcement is schema-unenforced (integrity)

**Evidence**: LEARN-045 through LEARN-051 (schema-unenforced Gate1/
Gate2 authority and record immutability; unauthenticated attribution
gap in `from:` field; naive "approved-by-non-empty-check" is unsafe;
maturity vocabulary not applied to own gate claims; Goodhart risk in
multi-model review relay; requesting review doesn't mechanically block
merge; memory-derived identifiers instead of authoritative lookup).
Issues **#34** (hard block on Gate 2 without governor approval
artifact) and **#35** (sender identity/attestation for channel
messages) are the tracker objects these learnings point at. This
session's own `nqp8` incident (Codex publishing a Gate-1 handoff
`from: planner-architect`) is a **fresh, live occurrence of exactly
what LEARN-046 already documented** — the attribution gap is not
theoretical, it recurred today.

**Classification**: P0 integrity. This is the strongest P0 candidate
in the entire backlog — six converging learning records plus a live
recurrence, addressing the mechanism that makes every other gate
decision in this project trustworthy. Unblock value is high (a fix
here retroactively strengthens confidence in every past Gate 1/Gate 2
decision going forward) and the risk of doing nothing is not
hypothetical — it already happened twice (the earlier attribution-gap
incidents behind LEARN-046, and `nqp8` today).

### C2 — agov-mcp / cold-start tooling gaps (issues #84, #85, #86, #87, #88, #89, #90)

**Evidence**: MR-806 (plan for #86, #88), MR-807 (plan for #87, #90).
Both plans state "independently reproduced and confirmed by
contribution-auditor," both are `mergeable`, non-draft, non-conflict,
pipeline `success`. #84, #85, #89 have no MR yet — bugs in `agov coord
list`/`agov coord visibility` affecting cold-start discovery,
including one (#89: default scan ignores remote-tracking-only branches
after fetch) that is directly the failure shape LEARN-036
("polling-is-a-fetched-cross-surface-action-loop-not-a-local-status-check")
already names.

**Classification**: P1 next bounded lane. Two Gate-1-ready plans
already exist and are independently confirmed; the remaining three
issues (#84/#85/#89) are small, well-described bugs in the same
subsystem. High unblock value for every future agent cold-start in
this project (including this session's own experience), low
implementation risk, already-reduced review cost since two of five
plans are done. Natural to batch as one lane: "agov-mcp / cold-start
visibility parity."

### C3 — CI cost, latency, and pipeline-dedup (issue #53 and family)

**Evidence**: issue #53 (open, **19 notes** — the most-discussed open
issue in the frozen set), MR-459 (measurement-only plan), MR-810
(completed read-only longitudinal assessment: 3,154 pipelines, 10,933
jobs, 804 MRs, 90 issues, already graded for causal corroboration),
issue #101 (CI cache policy — MR-931 already reviewed a renewed
implementation), issue #106 (cancelled/duplicate pipeline frequency
assessment), MR-921/925 (first-push pipeline race documentation),
LEARN-035 ("duplicate-pipeline-source-waste-is-a-positive-build-
engineer-activation-signal"), LEARN-032
("ci-workflow-rule-checks-push-head-title-not-full-commit-range").

**Classification**: P2 validated follow-up, with one P1 sub-item.
The assessment work (MR-810) is already done and thorough — that's
the hard part. What's missing is a *decision* built on it. MR-810
itself (read-only, already reviewed) is close to a trivial merge — I'd
treat merging MR-810 as P1 (cheap, unblocks everything downstream that
cites it), and treat "act on its findings" as P2 pending a Governor
read of the assessment.

### C4 — Learning-record backlog: unmerged, not dead

**Evidence**: MR-349, 407, 461 (draft), 465, 739, 776, 800 (draft),
829, 850, 932 — 10 learning-only MRs, most 2-4 weeks old, most with
low note counts (0-4), all still showing `checking`/`mergeable` merge
state in the frozen snapshot. Batch 1/2 of this session's own low-IID
pass already verified: of MRs in this exact numeric range, only 2 of
19 were actually dead (content already landed elsewhere); the rest —
including several in this cluster — were confirmed live, unmerged,
substantive content with no supersession signal.

**Classification**: P1 next bounded lane, disposition = **bulk merge
after a single completeness pass**, not closure. This is the opposite
lever from what an initial skim might suggest: these aren't backlog to
delete, they're backlog to *land*. A single reviewer doing one pass —
confirm `agov validate learnings` + `agov validate infosec` pass, no
conflicting learning ID, no stale reference — could plausibly clear
most of these in one sitting. Low risk (learning-only, no code/schema
changes), real value (each one is a documented incident sitting
unrecorded in the canonical corpus).

### C5 — Rules/governance canonicalization and hygiene

**Evidence**: MR-654 (.claude/rules vs agf/rules reconciliation plan —
confirmed still live in batch 2, `.claude/rules/` is a real separate
tree on `main`, not a symlink), MR-430 (tooling documentation-impact
rule), MR-998 (learning-record field-enforcement audit — directly
relevant to the many `severity: unknown` learnings visible in this
inventory itself), issue #43 (governance amendment workflow), issue
#39 (removability CI drill).

**Classification**: P2 validated follow-up. Real, bounded, not urgent.
MR-998 has a mild multiplier effect: fixing schema enforcement here
would make future backlog-inventory passes like this one more
reliable (fewer "severity: unknown" rows).

### C6 — Regulatory/compliance research and product-boundary questions

**Evidence**: MR-967 (AI regulation gap analysis, **12 notes** —
actively discussed, `mergeable`), MR-973 (post-v2.5.0 regulatory
reassessment TODO), MR-1032 (fact-check clearance for 967/981), issues
#40 (compliance-export generator), #41 (buyer-side AGF), #44 (CRA
entity structure), #94 (post-v2.4.6 refactor + framework compatibility
proposal), #100 (expectation-signals proposal), #103 (local-model
recipe, already has a critical assessment from coder-tester —
`jqa5`, still pending), plus the large regulatory portion of the
source registry (GDPR, AI Act, CRA, NIS2, ISO 42001, NIST SSDF/RMF,
SLSA, PROV-DM, CycloneDX Agent-BOM — roughly SOURCE-028 through
SOURCE-050, SOURCE-089/090).

**Classification**: P2/P3 mixed. MR-967 is the most mature single
artifact (12 notes, live discussion) — P2. The rest is genuinely
discovery-stage: real regulatory landscape mapping, not yet actionable
product decisions, and the plan itself (`docs/plans/...`) explicitly
scopes this backlog pass as read-only, so no compliance-generator
commitment belongs in this round's portfolio. P3 discovery lane.

### C7 — Actor landscape / OpenClaw and adjacent agentic frameworks

Covered separately below (OpenClaw tiering section) — this cluster
spans issue #104, RADAR-013/014/015, MR-915/916/918, and roughly
SOURCE-059 through SOURCE-087 (TaG, Gategraph, ai_glue, agentic-os,
adversarial-review, BMAD-METHOD, AI-Governor-Framework,
systemprompt-core, agentic-control-framework, Spec Kit, Agent RuleZ,
ai-review, ThumbGate, SteerPlane, agent-passport-system, Agentlas-OS,
openfirma, certiv.ai).

### C8 — Coordination-channel scalability

**Evidence**: issue #33 (concurrency semantics for the coordination
channel), issue #76 (optimize `agov coord visibility` for
branch-heavy repositories — this exact project is now a live example:
44 open MRs, hundreds of coordination files), RADAR-014 (external
context graph vs canonical repository trail — CRDT-adjacent sources
SOURCE-118 through SOURCE-121 plausibly relevant here, unverified).

**Classification**: P2 validated follow-up, rising. This project's own
growth (this inventory pass took real effort partly *because* of scale
issue #76 names) is itself evidence this cluster's priority is
increasing, not stable.

### C9 — Intelligence-tradecraft / analytic-uncertainty research

**Evidence**: SOURCE-106 through SOURCE-111 (ICD 203/206/208,
UK intelligence-assessment uncertainty guidance, JDP 2-00),
SOURCE-094-096 (Coldcard entropy-failure writeups — different topic,
possibly miscategorized in `docs/sources.md` or linked to a different
concern about entropy/randomness in identifier generation; I can't
tell from URL alone and am not asserting a link).

**Classification**: P3 discovery. Plausibly feeds RADAR-007
(verification independence tiers) and general epistemic-hygiene
practice for how this project's own reviews state confidence — but
speculative without reading the sources themselves.

### C10 — Everything else in the source registry (bulk P3/discovery)

The remaining ~60 source-registry entries (competitor/adjacent
governance tooling not already covered in C7, general AI-governance
commentary, local-first/offline-agent blog series, NASA LLIS lessons-
learned infrastructure, misc.) are classified **P3 discovery lane, bulk**
— not individually justified. This is the honest boundary of what a
title/URL-only pass can support.

## Priority-banded portfolio (recommended)

| Band | Item | Why |
|---|---|---|
| **P0 integrity** | C1: gate/authority enforcement (issues #34, #35; learnings 045-051) | Schema-unenforced authority + a live recurrence this session (`nqp8`). Everything else in this backlog is downstream of trusting gate decisions. |
| **P1 next bounded lane (a)** | C2: agov-mcp/cold-start parity (#84, #85, #86, #87, #88, #89, #90; MR-806, MR-807) | Two plans already Gate-1-ready and independently confirmed; three small bugs remain in the same subsystem. |
| **P1 next bounded lane (b)** | C4: bulk-merge the learning-record backlog (MR-349/407/461/465/739/776/800/829/850/932) | Low risk, real value, already shown by batch 1/2 to be mostly live not dead — the lever is landing, not closing. |
| **P1 (cheap, near-free)** | Merge MR-810 (CI longitudinal assessment) | Read-only, already reviewed, unblocks C3's decision step. |
| **P2 validated follow-up** | C3 remainder (CI decision), C5 (rules hygiene), C6 (MR-967 specifically), C8 (coordination scalability) | Real, evidenced, not urgent enough to jump the P1 lane. |
| **P3 discovery/radar** | C6 remainder, C7 (OpenClaw tiers ≥1), C9, C10 | Genuine discovery-stage; correctly out of this round's implementation scope per the plan's own read-only boundary. |
| **Close/archive candidate** | None identified beyond the 2 already closed in batch 1 (!340, !463) | I did not find additional demonstrably-obsolete MRs in this frozen set beyond what batches 1/2 already closed. This may be a real disagreement point with Codex's pass — see below. |

## OpenClaw / new-agentic-actor tiering

Per the plan's explicit tier framework, and treating containment as
the current default:

- **Tier 0 (observer/divergence detector)**: no objection found in the
  frozen evidence to discovery-priority consideration. Lowest-risk
  tier by construction — read-only, no authority.
- **Tier 0.5 (evidence-preserving sync relay)**: plausible discovery
  candidate, but I found no frozen evidence (learning, coordination
  record, or source) demonstrating this project has validated the
  *evidence-preserving* property claimed by the tier name — flagging
  as an open question, not a recommendation either way.
- **Tier 1 (bounded dispatcher) and higher**: no frozen evidence
  supports promoting past discovery. RADAR-013 (governance
  proxy/LLM-gateway layer) was **already explicitly rejected in
  2026-07** under the removability principle — that rejection is
  directly on point for any Tier ≥1 OpenClaw proposal and should be
  cited if this comes up again, not re-litigated from scratch.
- Issue **#103** (local-model recipe) is adjacent but distinct: it's
  about running *this project's own* AGF stack on a local model, not
  about adopting an external agentic actor. I would not conflate #103
  with OpenClaw tiering — they answer different questions (can AGF run
  without a cloud LLM vendor, vs. should AGF integrate a third-party
  autonomous agent). #103 already has a coder-tester critical
  assessment (`jqa5`) pending response — that's a live, specific,
  bounded item, unlike the broader OpenClaw discovery question.

## Answers to the plan's five framing questions (pre-reconciliation)

Recorded here **before** reading Codex's ranking, so the comparison in
the follow-up reconciliation note is genuine, not retrofitted.

1. **P0 semantics for #34/#35**: yes, P0 integrity — see C1 above. This
   is my strongest, most-evidenced P0 call in the whole pass.
2. **#102 vs #107 ordering**: #102 (Elixir/Mix exclusion check removal)
   is a narrow, bounded CI-config cleanup with a stated precondition
   ("once safe to remove") — it's a small P2/P3 item, not urgent on
   its own evidence. #107 (governed coordination-to-learning candidate
   discovery) is more structurally significant — it's about *how this
   project notices its own learnings*, which is upstream of C4's
   entire backlog-of-unlanded-learnings problem and adjacent to C1's
   integrity concerns (a learning that never gets discovered can't
   inform a gate decision). I'd rank **#107 ahead of #102**: #107 has
   higher leverage (process-improving) even though #102 is easier.
3. **Implementation vs administrative truth for #84/#85/#89**: all
   three read as genuine implementation gaps (real bugs in `agov coord
   list`/`agov coord visibility`), not administrative/paperwork items
   — each has a concrete, falsifiable failure description in its
   title. I found no evidence any of the three is already fixed or
   administrative-only; treating all three as live P1 implementation
   work within cluster C2.
4. **OpenClaw Tier 0/0.5 vs #103**: answered above — I treat these as
   separate questions, not a single tradeoff. #103 is unrelated to
   OpenClaw tiering.
5. **Unique-evidence vs closeable stale MRs**: this session's batches 1
   and 2 already did exact-head verification on 19 of the 44 MRs and
   found only 2 demonstrably dead. I did not re-verify the remaining
   25 at exact-head level for this pass (would require live queries,
   which I avoided to stay on the frozen snapshot) — for those 25, my
   classification above is based on title/state/note-count signal
   only, not a supersession check. This is a real limitation: I cannot
   rule out additional closeable MRs in the un-rechecked 25. Flagging
   this as the most likely place my pass and Codex's diverge.

## Explicit deferrals (not decided by this pass)

- Whether to actually implement anything in P0/P1 — this pass is
  classification only, per the plan's read-only scope.
- Exact merge order within C4's bulk-learning-merge lane.
- Whether #103's local-model recipe assessment should block or inform
  the OpenClaw discovery question (I found no evidence either way).
- The ~123 source-registry entries' individual dispositions — see
  method note above; C10's bulk P3 treatment is not a substitute for
  actually reading them.
