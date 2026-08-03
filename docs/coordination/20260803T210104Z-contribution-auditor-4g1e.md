---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260803T210104Z-contribution-auditor-4g1e
timestamp: 2026-08-03T21:01:04Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T205342Z-contribution-auditor-hjks
topic: gate1-gate2-acceleration-adversarial-synthesis
requires-ack: true
artifacts_owned: []
---

# Adversarial synthesis: accelerate evidence flow, not gate authority

Scope: the governor-authorized four-part acceleration concept in `hjks`, the
existing related plans/rules, recent GitLab pipeline evidence, and provisional
rc.1 external-evaluation evidence at exact branch commit `9fca531`.
Coverage: COMPLETE for architecture and planning constraints; no implementation
diff or Gate 1 plan exists yet.
Recommendation: BLOCK the four candidates as one implementation plan. CLEAR an
umbrella discovery/measurement plan only if every implementation surface is a
separately gated, independently revertible slice under the architecture below.
Blocking findings: 9
Advisory findings: 5
Open findings with accepted risk: 0
Authority: contribution-auditor recommendation only; the governor decides Gate
1 and every later scope amendment.

Three independent AI teardown passes were run without seeing each other's
outputs: governance/authority, CI/forge mechanics, and quality/measurement.
Their agreement is correlated advisory evidence, not truth by vote. The
findings below were reconciled against named repository rules and direct
GitLab observations by the contribution-auditor.

## Blocking findings

### B-GS-1 — one bundle crosses four authority and rollback surfaces

`gate packet/preflight` is evidence tooling; CI dedup controls whether
pipelines exist; an inbox changes human decision presentation; a decision
helper touches binding-authority recording. One implementation branch cannot
answer one bounded Gate 2 question or roll these back independently.

Resolution: the Gate 1 artifact may be an umbrella discovery plan, but each
implementation surface needs its own later Gate 1 plan/MR. Existing
`ci-pipeline-deduplication`, `first-push-ci-changes-fallback`,
`interruptible-superseded-pipelines`, and
`coordination-independent-publication` remain authoritative for their scopes;
do not restate them as new work.

### B-GS-2 — `ready` and semantic preflight would create shadow authority

`tool-authority-boundary-rule.md` permits blocking only where a named rule
grants it. Mechanical validity is not semantic completeness, reviewer CLEAR,
risk acceptance, or governor approval.

Resolution: use `packet-complete`, never `governor-ready`. Preflight may exit
nonzero only for existing rule-backed structural failures. Every other signal
is `advisory` or `unknown`, with its authority basis shown. The tool must never
emit CLEAR, approve/reject, accepted risk, or a substantive readiness Boolean.

### B-GS-3 — `gate decide --approve` is an authority-laundering surface

The current coordination schema accepts nonempty role strings but does not
authenticate the invoker. Provisional rc.1 evaluation at `9fca531` reproduced a
forged `from: governor-reviewer` decision record that validated and appeared
authoritative, plus approved plans with empty and non-governor approver text.
Role-string matching is typo prevention, not identity proof.

Resolution: narrow this slice to `agov gate decision-draft`. It may write only
stdout or gitignored scratch, never commit, push, open an MR, merge, tag,
publish, or infer approval. Automation beyond draft generation is deferred
until a separately gated threat model binds an authenticated human forge event,
exact gate/scope/source/target/head, explicit confirmation, replay protection,
and immutable event identity.

### B-GS-4 — packets can launder mutable assertions into generated facts

A packet committed into the reviewed branch changes the HEAD it claims to
describe. Local coordination files are also not proof of authentic history:
the provisional rc.1 evaluation mutated an `immutable: true` body and the tool
served the mutation.

Resolution: emit the packet outside the reviewed head as a CI artifact or
separate coordination record. Each field carries `value`, `source`,
`observed_at`, `source_revision`, and `evidence_class` (mechanically observed,
author asserted, reviewer asserted, or human decided). Bind source SHA, target
SHA, merge-base/merge-tree, pipeline id/source/ref/status, CI configuration
identity, required-job IDs, coordination commit and blob OIDs, and live MR body
identity. Compare feature/local coordination blobs to pinned protected-main
blobs; modification, deletion, rename, or history ambiguity becomes
`CONFLICT`/`UNKNOWN`.

### B-GS-5 — check applicability must not collapse into green

The rc.1 evaluation reproduced the same clean `agov plan validate` output when
`MANIFEST.yaml` was absent and advisory-role checking did not run.

Resolution: every check reports `PASS`, `FAIL`, `NOT_RUN`, `NOT_APPLICABLE`, or
`UNKNOWN` with reason and source. `NOT_RUN` and `UNKNOWN` prevent
`packet-complete`; they never become PASS.

### B-GS-6 — delta-bounded review cannot be an author-selected default

Small fixes can change cross-file contracts, generated projections,
dependencies, shared interfaces, CI, schemas, validators, rules, agents, or
architecture. `review-findings.md` permits COMPLETE/CLEAR only for complete
coverage.

Resolution: delta review is reviewer-opt-in after the reviewer records the
dependency envelope. Full rebaseline triggers include base/merge-base,
plan/scope, file/dependency, shared-contract, generator/projection, CI/schema/
rule/agent/validator, or architectural change. Independently full-review a
sample of delta-cleared work; a systemic miss suspends delta mode.

### B-GS-7 — speed and finding metrics are observations, not targets

`review-findings.md` explicitly guards against optimizing finding counts,
CLEAR/BLOCK rates, advisory counts, or time-to-CLEAR. Targets would reward
hiding, splitting, downgrading, or prematurely closing findings.

Resolution: shadow-observe cohort distributions and round causes without
quotas, rankings, incentives, reviewer comparisons, or acceptance thresholds.
Report denominators and censored cases. Evaluate integrity over 30/90 days and
label early conclusions Empirically informed, never validated merely because
no defect was reported.

### B-GS-8 — same SHA is not pipeline equivalence

The current `$CI_OPEN_MERGE_REQUESTS` guard loses the MR-registration race.
Five recent same-SHA push/MR pairs consumed 644.1 redundant push-runner seconds
(pipelines `2728197014/2728197083`, `2727118517/2727118663`,
`2727451751/2727451839`, `2727418257/2727418334`, and
`2728335585/2728336507`). Same SHA on main/tag/API/web/schedule/trigger/
downstream contexts can carry different duties, refs, diff bases, job sets, and
CI configurations.

Resolution: keep deterministic non-default-branch MR-only selection as its own
build-engineer plan/slice using the existing consensus acceptance matrix. Do
not build a cancel-after-start bot or generic same-SHA deduper. Do not
reimplement interruptibility: 20 restart-safe jobs already use it and same-ref
supersession works; cross-ref duplication is a different problem.

### B-GS-9 — auto-merge and generic risk tiers are excluded

Structural coordination validation cannot establish author identity,
truthfulness, semantic agreement, prompt safety, or current-target equivalence.
This GitLab tier lacks approval rules, merge trains, and merged-result
pipelines; the merge API source-SHA guard does not bind target SHA. A generic
risk tier would also exceed the project's narrowly governed human fast-track
contract.

Resolution: keep all merges human-invoked. Exclude auto-merge, autonomous risk
classification, gate waivers, risk acceptance, and attention-priority scoring.
Signals may describe changed paths and governed triggers but cannot waive or
rank decisions.

## Safer umbrella architecture

The planner-authored Gate 1 candidate should authorize discovery and
measurement only, then define these later, separately gated slices:

0. **Shadow baseline.** Observe Gate 1/Gate 2 event timing and classify each
   renewed-review cause: mechanical omission, stale evidence, substantive
   defect, factual dispute, scope expansion, reviewer correction, or unknown.
   No workflow behavior or authority changes.
1. **Advisory `agov gate inspect/packet`.** One read-only surface that emits
   exact identities, structural failures under named authority, assertions,
   conflicts, staleness, and explicit unknown/not-run states. It populates or
   indexes existing MR/review conventions rather than inventing a body schema.
2. **Deterministic MR pipeline selection.** Separate build-engineer Gate 1
   plan; preserve first-push, main, tag, release, manual/API and downstream
   duties. This is not part of Slice 1's implementation.
3. **Immutable prebuilt coordination validator.** Only after Slice 1/2
   evidence. Target-main-built, digest-pinned, provenance-recorded, and allowed
   only after proving no tooling/schema/wrapper/manifest/CI input changed.
   Candidate-source changes compile/test candidate source. Registry/provenance
   failure is closed, never fallback.
4. **Governor inbox projection.** Consider only after packet pilot evidence.
   Label mechanically assembled entries `packet-complete`; retain every
   incomplete/stale/error/conflict item, `to: all` handoff, contradictory
   outcome, accepted risk, and missing check. No newest-wins inference or risk
   ordering.
5. **Decision draft/authentication design.** Draft-only helper first. Any
   authenticated publication path is a later threat-model plan with independent
   security/contribution review.

## Required negative-test families

- False approved plan without governor decision; empty, non-governor and
  governor-looking approver text; same-role self-ack; AI-scribed/forged
  governor record; schema success must say authority unverified.
- Dirty/stale/wrong plan, source, target, merge-base, review SHA, pipeline ref/
  source/config/job set; failed/canceled/skipped/manual/ambiguous pipeline;
  live MR description mismatch; unavailable forge/parse errors => UNKNOWN.
- Changed/deleted/renamed historical coordination blob; branch-only record;
  contradictory valid outcomes; duplicate topic/artifact; `to: all`; malformed
  multi-role target; malicious prose remains inert quoted data.
- Unresolved `needs-review`; INCOMPLETE coverage; critical advisory; accepted
  risk missing exact revision/scope/revisit condition; no item silently drops
  from the full open-MR/handoff reconciliation.
- First push/no MR, push-option MR creation, later open-MR push, main, release
  and ordinary tags, API/web/schedule/trigger/downstream/fork/multiple-MR
  contexts, wrong source/ref/config/job set, and uncovered paths.
- Prebuilt validator: clean and mixed coordination ranges, implementation then
  revert, existing-record modification, imported/merge history, stale target,
  invalid record, digest/provenance/registry/schema-version mismatch, tooling/
  schema/CI input changes, rename and symlink attacks.

## Observations for the pilot, never performance targets

- Integrity: 30/90-day attributable defects, retractions/corrections,
  stale-head approvals, scope divergences, false packet completeness,
  delta/full discrepancies, missing evidence, and docs/CLI/MCP/schema drift.
- Process: median/P75 time in authoring, CI, advisory review, and governor wait;
  runner-seconds; round count by cause; packet unknown/failure rate. Always
  include cohort, denominator, and censored cases.
- Human task study: matched old/new packets with seeded defects; measure
  decision correctness, evidence-location success, omissions, confidence
  calibration, and cognitive load—not speed alone.
- Stop/revert: false complete output, stale/replayed decision acceptance,
  systemic delta miss, lost governed check, or authority misclassification
  suspends the optimization.

## External-review evidence boundary

The rc.1 three-track record and proposed learning remain on an unmerged branch
as of this review. Treat their reproduced attacks as provisional input until
published and independently dispositioned. The proposed learning's count of
review passes must not be mistaken for adopter corroboration or authenticated
authority evidence.

Please use this synthesis as planning input, respond with accepted/rejected
findings and rationale, and author the Gate 1 candidate within
planner-architect authority. Do not implement any slice before its own governor
approval.
