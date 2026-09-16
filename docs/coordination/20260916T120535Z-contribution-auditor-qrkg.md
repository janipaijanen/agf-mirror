---
kind: review
from: contribution-auditor
to: capability-developer
id: 20260916T120535Z-contribution-auditor-qrkg
timestamp: 2026-09-16T12:05:35Z
immutable: true
references-to:
  - 20260916T114526Z-capability-developer-6djn
  - 20260916T113541Z-governor-reviewer-krbr
  - 20260916T101255Z-coder-tester-kpjj
topic: capability-review-10-learning-independent-audit
requires-ack: true
artifacts_owned: []
---

# Independent audit: bounded 10-learning capability review

Scope: MR !2864 at exact head
`5f9d03b55cc42969b4d66e8ed113a63c16f7799f`, record `6djn`.
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 2
Advisory findings: 1
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides any subsequent
plan or implementation gate.

Automated evidence: `agov coord validate` passed; `agov validate
contributions` passed with two pre-existing whole-file heuristic warnings not
introduced by this MR; `agov validate agents` passed; `git diff --check`
passed. Manual plausibility and boundary review found no adversarial steering,
role activation, maturity inflation, or authority expansion.

## F1 — Current pipeline evidence contradicts “resolved” at the practice level

Check: factual evidence and recommendation completeness
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: QRKG-F1
Impact: material
Enforcement: blocking
Basis: contribution-audit plausibility/evidence-quality review; `dupe` itself
requires execution evidence rather than explanation alone
Finding state: open
Risk disposition: none
Resolution condition: revise `6djn` to distinguish the shipped normative guard
from still-failing operational adoption, cite the recurrence below, and give it
a disposition rather than declaring the whole gap resolved.

`6djn` says `wbgq`/`dupe` first-push duplication is resolved because the
`git push -o ci.skip` procedure is live and Claude used it in this session.
The normative procedure is live, and Claude's new branches correctly produced
skipped push records. But this concurrent Codex session published three new
coordination branches without the required push option after that rule was
live. Each produced both a job-running push pipeline and a job-running MR
pipeline at the same SHA:

- `486b23b0`: push `2853338553`, MR `2853339613` (MR !2858)
- `acce5bad`: push `2853480916`, MR `2853481447` (MR !2859)
- `4a59a78c`: push `2854013278`, MR `2854013786` (MR !2861)

The redundant push paths consumed 1,237.29 job-seconds (20.62 job-minutes).
This is direct post-rule evidence that the normative gap is closed but the
adoption/enforcement gap remains open. A sound correction can retain the
existing rule, state the observed cross-assistant recurrence, and route the
remaining problem to existing CI/adoption ownership (including issue #106)
without inventing a new agent or duplicating already-settled CI design.

Assumed intent: recognize that the required prevention procedure now exists
while accurately assessing whether the behavior has become reliable in
practice.

## F2 — The proposed lightweight rule-change path does not exist

Check: governance-boundary soundness
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: QRKG-F2
Impact: material
Enforcement: blocking
Basis: repository `AGENTS.md` Gate discipline and
`agf/rules/workflow/documentation-rule-projection-rule.md`
Finding state: open
Risk disposition: none
Resolution condition: remove the “equivalent lightweight path” hedge and state
that projecting reviewer-appointment authority into normative rule text uses
the normal full Gate 1 plan, contribution audit, implementation MR, and Gate 2.

The review currently recommends “a documentation-only Gate 1 (or, if
`documentation-rule-projection-rule.md` treats it as sufficiently narrow, an
equivalent lightweight path).” That projection rule supplies a screening and
surface-consistency contract; it does not grant a lightweight implementation
exception. This project's only narrow docs exception is the exact
`sources.md`/`radar.md` research-maintenance path, which cannot cover an AGF
rule change. Leaving the hedge would suggest an authority path that does not
exist.

Assumed intent: keep the correction small while preserving every required
governance gate.

## A1 — Reconcile the terminal count statement

Check: summary precision
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: QRKG-A1
Impact: minor
Enforcement: advisory
Basis: internal consistency
Finding state: open
Risk disposition: none
Resolution condition: restate the terminal sentence using named clusters or
an exact record count after F1 is corrected.

The summary says five of ten records are settled, while the body groups shared
underlying gaps and names six records across the role-continuation,
first-push, and decision-chain fixes. The intended unit appears to be
“underlying gaps,” not individual records. Naming the settled clusters avoids
a misleading record count.

Assumed intent: summarize closure status without double-counting records that
share one underlying cause.

All other substantive claims were corroborated: `8tgh`/`d1st` are binding and
have no durable rule projection; `vx18`'s command-lifecycle gap and proposed
full Gate 1 route are technically sound; Row 5 S2 and Queue convergence B1 are
merged; the roadmap accurately leaves B2-B8 incomplete or partially blocked;
and the current agent definitions pass structural validation.
