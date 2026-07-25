---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260723T023620Z-contribution-auditor-ghsd
timestamp: 2026-07-23T02:36:20Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T022449Z-governor-reviewer-qkks
  - 20260723T022733Z-governor-reviewer-3d27
topic: open-issue-portfolio-triage
requires-ack: false
artifacts_owned: []
---

# INFO — Codex open-issue portfolio triage

## Current release boundary

The v2.4.0 main pipeline has succeeded and the GitLab Release exists with the
expected asset set. Complete any remaining public-surface verification and the
release completion record, but the planning queue may now advance.

## Act now

| Issues | Action | Lead and review |
|---|---|---|
| #54 | Consolidate unique coordination records from historical branches, then close/delete only after post-merge verification. This is the first housekeeping action because it reduces ambiguity for every later cold start. | coder-tester (Claude); contribution-auditor (Codex) checks the manifest/diff; governor merges |
| #56 with #51/#55 | Start the bounded v2.4.1 Gate 1 plan now. Inventory pages/figures once and leave reusable evidence for v2.4.2. | planner-architect (Claude); Codex audits Gate 1 and exact-SHA Gate 2; governor decides |
| #53 | Start the measured pipeline/release-latency assessment during genuine waits. Do not implement before Gate 1. The build-engineer is suspended in the current manifest, so it cannot be assigned as lead unless the governor explicitly reactivates it; until then Claude/Codex work under active roles. | Claude measures/plans; coder-tester maps local parity; Codex challenges false-green/coverage risk; governor decides role activation and trade-offs |
| #58 | Plan immediately after or alongside late v2.4.1 planning so both use one page/figure inventory; implement only after v2.4.1. | Claude plans; Claude/Codex may take non-overlapping implementation slices; Codex audits provenance/attribution; governor decides |

## Close or archive after a final evidence link

| Issue | Recommendation | Reason |
|---|---|---|
| #18 | Close | Learning was ingested as `20260625T071500Z-c1im`; its historical OCI gap is separately superseded by shipped distribution work. |
| #19 | Close | Learning `20260625T072500Z-svch`, validator, tests, and docs landed through !186. |
| #32 | Close as shipped | `agov coord post` and handoff take/complete are present in v2.4.0. Follow-on concurrency/watch work belongs to #33/#36. |
| #37 | Close as shipped | `agov-mcp` 2.4.0 is released and locally smoke-tested; defects or additional transports need new/narrow issues. |
| #17 | Feed still-applicable proposals into #58, preserve the external review as evidence, then close the intake issue | It is a version-2.2.4 evidence packet, not an indefinitely active implementation tracker. |

Do not close #30 merely because `build-engineer` is suspended. The issue exposes
a real contract/usability question: the schema admits role strings while the
semantic validator restricts advisory roles through the active manifest and
reports an “unknown role.” Decide and document that contract, improve the error
or implementation as needed, then close.

## Small operational backlog

| Issue | When | Lead |
|---|---|---|
| #31 | After #54, in a genuine wait; reconcile the requested `improvement`/`bug` templates with the four existing differently named templates rather than adding duplicates blindly. | coder-tester; governor fast-track/full-gate decision after sizing |
| #30 | v2.5 tooling-contract triage, unless a current plan is blocked by it | planner-architect + coder-tester; contribution-auditor checks schema/semantic consistency |
| #33 | v2.5 coordination-foundation pass | planner-architect specifies merge/rebase/concurrency guarantees; coder-tester verifies readable-ID collision behavior |
| #36 | Narrow to the unshipped watch/notification remainder: `coord list --since` already ships, `coord watch` does not. Schedule only after #33. | coder-tester after approved plan |
| #38 | After #33/#36 produce a stable experiment protocol; do not test a moving coordination surface. | planner-architect designs; independent assistants execute conditions; academic/fact advisory review; Codex audits claims |
| #39 | Independent v2.5 candidate after patch releases; docs partially cover exit, but the tested removal guarantee/drill remains the differentiator. | planner-architect, coder-tester, contribution-auditor |

## v2.5 governance dependency chain

Do not implement #34–#49 as a grab-bag. Use this order:

1. **Define the safety claim:** #42 threat model first. Without it, enforcement
   design cannot say what failure or adversary it handles.
2. **Define accountable and attributable decisions:** jointly discover #35,
   #45, and #47. Keep #58's visible assistant byline separate from #35's
   authentication/attestation problem.
3. **Enforce only the defined contract:** #34 follows #42/#35/#45. Mechanical
   blocking authority must cite a named rule and must not turn advisory model
   output into approval authority.
4. **Govern the inputs:** #49 follows the minimum identity/provenance model;
   start with hashes/declared surfaces and avoid premature dependence on an
   unstable Agent BOM proposal.
5. **Govern evolution:** pair #43 amendment/sunset with #46 incident-to-learning
   loop after the threat/control vocabulary is stable.
6. **Add throttles only from evidence:** defer #48 until #53 and usage telemetry
   show which velocity failure actually needs a semantics-free brake.

Suggested ownership: Claude/planner-architect decomposes and plans; Codex as
contribution-auditor performs adversarial contract review; fact-checker checks
external research claims; infosec-sentinel reviews identity, credential, and
provenance surfaces; Jani/governor retains every binding choice. A future
advisory risk-specialist may be activated for high-consequence risk analysis
but receives no gate authority.

## External/compliance lane

| Issues | Disposition | Ownership |
|---|---|---|
| #40/#41 | Keep as a paired later research/product lane. Start only after the underlying gate, identity, threat, and export semantics are stable; otherwise the generator would package unproven claims. | fact-checker + domain/legal specialist research; planner plans; coder implements; governor decides |
| #44 | Keep as a trigger/watch decision, not a software release feature. Re-verify current law/guidance before any reliance and obtain qualified legal advice if an entity is contemplated. | governor owns the entity decision; fact-checker monitors primary sources; legal specialist when triggered |

## Portfolio recommendation

The immediate sequence is **finish v2.4.0 verification → #54 and v2.4.1 Gate 1
→ v2.4.1 → v2.4.2**, while #53 research uses waits. The most coherent later
v2.5.0 character is not “all governance ideas”; it is a bounded trust chain:
declared threat model → accountable/independent evidence → enforceable gate.
Coordination performance, removability, experimentation, and compliance should
remain independently gated workstreams or milestones.
