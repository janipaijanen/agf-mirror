# TODO.md × Sol High assessment — synthesis and prioritized action list

**Status:** documentation change under Gate 2 review (this MR). The backlog
below remains **advisory** — a proposed prioritization, not a governor
decision. Every non-trivial item still needs its own Gate 1 plan before
implementation; approving this document approves the prioritization
reasoning, not the individual changes it names.
**Author:** coder-tester (claude-sonnet-5)
**Date:** 2026-07-20; revised 2026-07-20 after Codex's critique (below)
**Sources:**
- `docs/TODO.md` (three open project TODOs: v1.x verification audit, Rust
  implementation health, token ROI zero-savings)
- `docs/reviews/agf-framework-assessment-codex-sol-high.md` (external
  assessment by `gpt-5.6-sol` at high reasoning effort, merged to `main`
  via MR !251, commit `b967b50`)

## Revision note

Codex reviewed the first version of this document
(`docs/coordination/20260720T200018Z-coder-tester-psht.md`, addressed to
`planner-architect`) and filed two BLOCKs and several WARNs. This revision
agrees with and applies essentially all of it:

- **Release-sequencing contradiction (BLOCK):** the first version labeled
  the MCP `dir` defect P0 but still recommended cutting v2.3.0 before
  fixing it. Resolved by an explicit choice, not a deferred tension: v2.3.0
  ships without the fix, but `agov-mcp`'s documented status is downgraded
  to Level 4 Validated with the limitation disclosed (MR !258) — the
  "exclude from Deployable claims" branch Codex offered as one of two valid
  resolutions. See P0 below.
- **Artificial P0/P2 split (BLOCK):** the fix's fixtures and CLI/MCP parity
  test were never actually split in the underlying Gate 1 plan
  (`docs/plans/mcp-coordination-dir-contract-fix.md` always specified them
  atomically); the split existed only in this document's summary. Fixed
  below — one P0 item, not two.
- **Lanes instead of executability-ordering (WARN):** the original P0–P3
  used "can an agent execute this" as a proxy for "how important is this,"
  which inverted the assessment's own conclusion that external validation
  evidence — not framework breadth — is the next unit of value. Restructured
  into four lanes below, each with its own internal ordering, per Codex's
  proposal.
- **Positioning canonized too early (WARN):** "cross-vendor, Git-native
  assurance protocol" was Codex's own informed recommendation, not a
  decided category. Softened to an audit-and-remove-overclaiming action;
  the replacement phrase itself is left as a governor/product decision.
- **Historical-evidence axes (WARN):** clarified below that origin
  (v1.x-vs-v2) and schema-contract validity (current-vs-legacy) stay as two
  separate fields/submodels, unified only in reporting — not collapsed into
  one scheme.
- **Token ROI miscategorized (WARN):** moved out of the "human decision"
  tier into the governance-evidence lane; it is a concrete, already-assigned
  technical task, not a product judgment call.
- **Stale/unsupported claims (WARN):** the Sources section and the P0-2 note
  now correctly state the assessment merged via a reviewed MR (!251), not a
  direct push to a protected branch; a decision-record now exists for the
  pending release instruction (see "Durable release decision" below)
  instead of relying on an unrecorded chat instruction; a full risk
  disposition table (below) replaces the earlier unqualified "synthesizes
  the assessment's risks" claim.
- **Role-only routing (INFO):** not fixed here — tracked as its own
  proposed learning record (`agf/learnings/foundry.yaml`,
  `20260720T201034Z-w6zo`) and an open joint-design discussion with Codex.

## Durable release decision

The governor's instruction to cut a v2.3.0 minor release exists in this
session's chat, which is not a durable artifact per
`coordination-message-rule`. A `kind: decision-record` coordination message
recording it is filed alongside this revision
(`docs/coordination/`, see the companion MR) so the instruction is citable
without relying on session memory.

## Risk disposition table

Every numbered risk from the Sol High assessment's "Risks / gaps /
inconsistencies" section, mapped to an action or an explicit no-action
disposition — replacing the earlier, narrower claim that this document
synthesizes "the assessment's named risks."

| # | Risk (assessment's own heading) | Disposition |
|---|---|---|
| 1 | Organizational benefit remains largely dogfooding evidence | P1 — begin external pilot recruitment (below) |
| 2 | Governance can degrade into ceremony / governor bottleneck | No separate action item. Structural risk inherent to the single-human-governor model this project has chosen; monitor via the existing audit cadence (`workflow/deputy-governor-rule.md` already exists as an opt-in mitigation, currently unused — a future item if bottleneck becomes acute, not now) |
| 3 | Enforcement is intentionally partial | No action — this is a documented, accepted design property (`agf/docs/site/docs/advanced/contracts-and-validators.md` already states it), not a gap to close |
| 4 | Default mental model too large for first adoption | P3 — minimal starter profile (below) |
| 5 | Historical contract drift weakens health reporting | P1 — two-axis historical-evidence plan (below) |
| 6 | MCP `dir` path contract permits wrong-scope success | P0 (below) |
| 7 | Source/docs/release truth require reconciliation | P0 — partially resolved this session (MR !258); general release-truth invariant remains P1 (below) |
| 8 | Implementation concentration and conformance gaps | P2 — Rust implementation health assessment (below) |
| 9 | Native platforms can absorb adjacent capabilities | Folds into P1's positioning audit — no separate action |
| 10 | Commercial moat currently limited | No action item — business-strategy observation, not an engineering or governance task; relevant context for P3's product-validation thresholds, not a standalone action |

---

## P0 — release integrity

Blocking or directly resolving a claim this project is about to make in a
release.

### P0-1. MCP coordination-directory contract — fix, atomically bundled

**What:** Rename the MCP `dir` field to `coordination_dir` across all six
affected tools (`agov_session_resume`, `agov_handoff_next/take/complete`,
`agov_coord_post`, `agov_coord_list`), keep `dir` as a deprecated alias with
an explicit conflict-note when both are supplied, add a structural warning
on `agov_session_resume` for the likely-wrong-directory case, add resolved
provenance — **together with** its fixtures (omitted/correct/repo-root/
absolute/nonexistent/alternate-layout paths) and a direct CLI/MCP semantic
parity test, as one atomic change. Full design in `docs/plans/
mcp-coordination-dir-contract-fix.md` (pending Gate 1, MR !255).

**Basis:** Sol High finding #6 / Proposal 1, corroborated firsthand this
session during `.mcp.json` setup. Codex's critique (BLOCK 2) is why the
fixtures and parity test are listed as part of this single item, not a
separate later tier — if this fix ships as release evidence, the tests
proving it work are the same evidence, not optional hardening.

**Release relationship (resolved, not deferred):** this fix does **not**
block v2.3.0. Instead, `agov-mcp`'s documented status is downgraded from
"Deployable" to "Level 4 Validated" with the limitation disclosed (MR
!258, merged/pending alongside this revision) — the release ships honestly
without the fix, rather than the fix being rushed to beat the release or
the release silently overclaiming. This fix proceeds on its own Gate
1-then-implementation timeline.

### P0-2. Sol High assessment sequencing — RESOLVED

Was flagged as an open dependency (assessment unreviewed, branch had no
MR); resolved when the human governor merged it via a reviewed merge
request, **MR !251** (commit `b967b50`) — not a direct push to the
protected branch, which GitLab's branch protection would reject in any
case. Left in place as a record that the dependency was named and closed.

### P0-3. General release-truth invariant

**What:** Sol High Proposal 3 — a release invariant checking that
`agf/VERSION`, README claims, generated docs, changelog entries, archive
contents, and public release metadata all describe the same artifact
surface, checked mechanically rather than by manual review each time.

**Basis:** finding #7. This session found and fixed one instance by hand
(the `agov-mcp` status/changelog gap, MR !258) — exactly the kind of drift
a mechanical check would catch automatically next time.

**Why P0 not P1:** listed under release integrity because it's the general
form of a defect this session just found in a specific instance; not
blocking v2.3.0 (the specific instance is already fixed), but the next
release after v2.3.0 should not rely on another manual catch.

---

## P1 — governance and evidence integrity

Not release-blocking, but load-bearing for whether the framework's
maturity and capability claims stay honest, and — per Codex's correction —
including the assessment's single most important recommendation, which the
first version of this document under-weighted by treating it as a
"someone else's job" P3 item.

### P1-1. Begin external pilot recruitment and measurement, in parallel

**What:** Sol High Proposals 6 and 7 — recruit toward ≥5 independent
pilots covering ≥50 governed changes, working toward the longer-term
thresholds (10 repos, 250 changes, 3 paying engagements, ≥60% continuation,
2 case studies, 1 security-sensitive adopter) before making scaled-product
or enterprise claims.

**Basis:** the assessment's central finding — "organizational benefit
remains largely dogfooding evidence" — is not a nice-to-have alongside
engineering work, it is the thing the assessment says matters most.
Demoting it to a low-urgency tier because an agent can't personally recruit
pilots (the first version's error, per Codex's WARN) confuses who executes
a task with how important it is.

**What an agent can still do now:** recruitment and measurement design
itself — drafting pilot selection criteria, the measurement plan (install
time, governor minutes, plans revised at Gate 1, defects caught at Gate 2,
decision-reconstruction time, fast-track rate, abandonment time), and an
outreach template — is agent-executable groundwork. The governor owns
actual recruiting and relationship decisions; the agent can prepare the
instrument.

### P1-2. Two-axis historical-evidence disposition model

**What:** One Gate 1 plan producing a disposition scheme that keeps two
axes as **separate fields**, unified only in reporting and migration
strategy — not collapsed into a single scheme:

1. **Origin** (`docs/TODO.md`'s axis): `verified-agf-v2` /
   `inherited-v1x-unverified` / `adapted-from-v1x` / `retired-for-agf-v2`,
   applied to rules, docs, and learnings inherited from v1.x.
2. **Schema-contract validity** (Sol High finding #5's axis):
   `current-valid` / `historical-valid` / `legacy/unversioned` / `invalid`,
   applied to coordination records and plans against the currently-active
   schema (329 historical coordination records and 11/76 plans currently
   fail current-schema validation for reasons that may be entirely
   expected — pre-dating the contract — not indicative of anything wrong).

**Basis:** both sources independently reached related conclusions without
seeing each other. `workflow/concept-modelling-rule` applies directly:
these are two distinct axes on different artifact types (rules/docs/
learnings vs. coordination records/plans) that a single combined field
would conflate.

### P1-3. Audit and soften unqualified positioning claims

**What:** Audit `README.md`, `agf/docs/site/index.md`, and other
external-facing copy for language implying "complete enterprise agent
governance" or unqualified superiority over adjacent tools (GitHub,
GitLab, OPA, in-toto, NIST AI RMF), per `verifiable-claims-rule` and
`docs/marketing-tone-rule` — both of which already require this regardless
of the assessment.

**What this does NOT do:** adopt "cross-vendor, Git-native assurance
protocol" (or any other specific phrase) as the new canonical line. That
was Codex's own informed recommendation, not a validated category
decision — per Codex's own correction, ideally informed by adopter
interviews, and "framework" may be safer than "protocol" until cross-vendor
conformance is independently demonstrated. The audit is P1; picking the
replacement phrase is a governor/product decision, not scheduled here.

### P1-4. Token ROI zero-savings investigation

**What:** `docs/TODO.md`'s third item — determine whether `coord-status`,
`coord-brief`, `coord-stale`, etc. reporting `0 saved` tokens is by-design
(no configured baseline) or a silent reporting gap.

**Basis:** moved here from the first version's P3 per Codex's correction —
this is a concrete, already-assigned technical task (`docs/TODO.md` names
coder-tester and planner-architect as owners), not a human/product
judgment call. `README.md`'s "Token savings with the Rust tooling" section
publishes a maturity table including `agov token roi report` at Level 5
Deployable; if the zero-savings rows reflect an undercounting bug rather
than an intentional no-baseline design, that's a gap under a claim already
being made publicly, which is exactly what governance-evidence integrity
should catch before P2-tier "nice to fix eventually" work.

---

## P2 — engineering maintenance

Structural, sequenced after P0/P1, not blocking any current claim.

### P2-1. Rust implementation health assessment

**What:** `docs/TODO.md`'s Rust health assessment scope: module
boundaries, parsing/normalization duplication across coordination/plan/
learning/graph-index code, test architecture for stable JSON output,
whether version provenance needs centralizing, where machine-readable
command contracts should live.

**Basis:** corroborated independently by Sol High finding #8 (CLI entry
point ~4,200 lines, thin re-export module boundaries, duplicated CLI/MCP
argument contracts) without having read `docs/TODO.md`. Two independent
sources naming the same concentration risk is why this is scoped now
rather than left indefinitely deferred.

### P2-2. Remaining CLI/MCP lifecycle conformance

**What:** Beyond the fixtures already bundled into P0-1 (which cover the
`dir`/`coordination_dir` contract specifically), broader lifecycle
conformance — an MCP-only claim/handoff lifecycle test suite comparing
MCP-observed state against CLI-observed state across more than the path
contract alone.

**Basis:** Sol High Proposal 2's broader scope, net of what P0-1 already
covers.

---

## P3 — product decisions informed by pilots

Genuinely requiring human product/business judgment; sequenced after P1's
pilot recruitment begins generating evidence to inform them, not before.

### P3-1. Minimal starter profile as the primary first-adoption product

Sol High Proposal 5 — a reduced default onboarding profile (three roles,
two gates, four message kinds, one validator, one CI example) versus the
current full system. Real trade-offs (what counts as "core," whether a
reduced profile undersells the actual differentiator) that need the human
governor's judgment, informed by what P1-1's pilots actually reveal about
onboarding friction — deciding this before any pilot evidence exists would
be guessing.

### P3-2. Final canonical positioning phrase

Once P1-3's audit removes overclaiming and P1-1's pilots produce real
adopter-facing evidence, decide the permanent canonical descriptor (see
P1-3 — not decided here).

### P3-3. Commercial validation thresholds and scaling decision

Sol High Proposal 7's specific numeric thresholds (10 repos, 250 changes,
3 paying engagements, etc.) as a formal go/no-go gate for scaled-product
investment — a business decision informed by P1-1's pilot results, not an
engineering task.

## What this document does not do

It does not re-litigate the Sol High assessment's factual claims (that's
Gate 2 review of the assessment MR itself, already resolved — P0-2). It
does not commit to timelines beyond the explicit v2.3.0 sequencing
decision in P0-1. It is a priority ordering and a rationale, offered for
further critique.
