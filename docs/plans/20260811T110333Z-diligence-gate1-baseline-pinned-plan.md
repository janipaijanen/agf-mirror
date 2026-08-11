---
id: diligence-gate1-baseline-pinned-plan
status: pending-gate1
topic: one-time-external-due-diligence-assessment-baseline-pinned
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      This plan is commissioned directly by binding Governor decision
      `20260811T105210Z-governor-reviewer-mh8p`, which superseded MR
      !1403 after `khkg` (round 10, COMPLETE/BLOCK) found planner-
      architect had crossed its own role boundary by writing
      implementation code, test fixtures, and a CI change. This
      document contains no code, no fixtures, and no CI change — plan
      text only. A fresh exact-head Gate 1 review is requested before
      Governor Gate 1; per the decision, this document does not
      pre-approve itself.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The evidence universe (Section 2) is, by construction, entirely
      first-party public content this project already publishes: the
      tagged v2.5.0 release archive, published release assets, release
      metadata, and this project's own public Pages site. There is no
      untrusted or arbitrary candidate file in this design, so the
      candidate-file scanning apparatus MR !1403 built (and which
      `khkg` correctly found undermined the role boundary) is not
      needed here. If a future revision of this plan introduces any
      third-party or untrusted input, this waiver must be revisited.
  - role: fact-checker
    status: waived
    reason: >-
      Applies to the *evaluator's* claims after Phase 2 (post-Gate-2,
      per Section 5), premature pre-Gate-1. Every fact this plan itself
      relies on (the pinned tag/commit, the release date, the asset
      count) was independently verified against live Git and GitLab
      API state before being written here — see Section 1.
  - role: web-ux-reviewer
    status: waived
    reason: >-
      Applies to Pages-derived findings after the evaluator's report is
      produced (post-Gate-2, per Section 5), premature pre-Gate-1.
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: one-time external due-diligence assessment of AGF v2.5.0 (baseline-pinned replacement for MR !1403)

## Why this plan exists, and how it differs from MR !1403

MR !1403 (10 rounds, `docs/plans/20260810T172503Z-fresh-context-diligence-gate1-plan.md`)
attempted an *in-session, disclosed-context, simulated* assessment: a
same-process `Agent`-tool sub-agent, which automatically inherits this
repository's ambient rules, memory, and git state, answering a
due-diligence questionnaire while disclosing that inherited context at
a class level. That design required an entire apparatus — a canary
pre-flight to detect context injection, a candidate-file scanner to
screen an assembled package before publication, a "sensitive-material
boundary" escalation path — to make the simulation honest. Building
that apparatus is what led planner-architect to write source code,
test fixtures, and a CI change under its own authorship: role
violations `khkg` (round 10, COMPLETE/BLOCK) correctly identified, and
which the binding Governor decision
(`20260811T105210Z-governor-reviewer-mh8p`) resolved by superseding MR
!1403 with **no retroactive authorization**.

This replacement plan is a genuinely different, simpler shape:

- The evaluator is a **new, external conversation** (Gemini, outside
  this session, outside this repository's tooling), given only a
  frozen package of already-public files. It is not a same-session
  sub-agent, so it does not automatically inherit this project's
  rules, memory, or git state the way an `Agent`-tool spawn would —
  there is nothing to disclose because there is nothing inherited.
  This plan states that mechanical fact plainly and does not claim
  anything stronger (Section 4).
- The evidence universe (Section 2) excludes coordination records,
  plans, learnings, prior due-diligence reports, MR discussions, and
  issue prose by construction — this is a **one-time** assessment of
  the *shipped v2.5.0 product*, not a reusable evidence-processing
  pipeline (that is a separate, future, its-own-Gate-1 concern; see
  issue #108).
- Because the evidence universe is 100% first-party, already-published
  content this project controls, there is no untrusted candidate file
  to scan, so no scanner is built (see `infosec-sentinel` waiver
  above).
- This document is planner-owned specification only: pinned baseline,
  allowlisted evidence universe, frozen evaluator prompt and output
  contract, and forward role assignment. It contains no shell/Python
  implementation, no executable fixtures, no build/test execution, and
  no CI configuration change, per the Governor decision's explicit
  instruction.

## Decision requested

Authorize, via binding Governor Gate 1, the pinned baseline (Section
1), the evidence universe and its acquisition method (Section 2), the
frozen evaluator prompt and output contract (Section 3), the honest
context-disclosure statement (Section 4), and the forward role
assignment (Section 5) below. This authorization is for **Phase 1
(evidence acquisition and packaging) and Phase 2 (evaluator
submission) only**, both performed by coder-tester after Gate 1 on a
new implementation branch, per Section 5. This document does not
itself construct the package, spawn or contact any evaluator, or lift
MR !1382/!1383's containment holds — per the Governor decision, those
holds lift only when a replacement Gate 1 plan pins the baseline below
**and** receives binding Gate 1 approval, which this submission
requests but does not grant itself.

## 1. Pinned immutable baseline

Independently verified against live Git and GitLab API state on
2026-08-11 (commands and raw output below are the verification
record, not implementation — no file is created or modified by
running them):

- **Annotated tag object**: `b9215bd65aec59bea45f1576d789d00ca3f3e4ae`
  (`git cat-file -t b9215bd65aec59bea45f1576d789d00ca3f3e4ae` →
  `tag`).
- **Peeled commit**: `f1932a1a486b772ca4353c7fa71c2279132fe0dc`
  (`git cat-file -p b9215bd6...` → `object
  f1932a1a486b772ca4353c7fa71c2279132fe0dc`, `type commit`, `tag
  v2.5.0`).
- **GitLab Release**: `v2.5.0`, `released_at: 2026-08-10T04:48:20.144Z`
  (`glab api projects/83321848/releases/v2.5.0`), with exactly 13
  published assets:
  `agov-2.5.0-linux-x86_64.tar.gz`,
  `agov-2.5.0-linux-x86_64.tar.gz.sha256`,
  `agov-2.5.0-linux-x86_64.tar.gz-provenance.json`,
  `agov-2.5.0-linux-aarch64.tar.gz`,
  `agov-2.5.0-linux-aarch64.tar.gz.sha256`,
  `agov-2.5.0-linux-aarch64.tar.gz-provenance.json`,
  `agov-oci-2.5.0-provenance.json`,
  `agf-2.5.0.zip`, `agf-2.5.0.zip.sha256`,
  `agf-2.5.0.zip-provenance.json`,
  `agf-2.5.0.tar.gz`, `agf-2.5.0.tar.gz.sha256`,
  `agf-2.5.0.tar.gz-provenance.json`.
- **Public Pages**: the live site is a separately timestamped
  observation, fetched at acquisition time (Phase 1) and labeled with
  that fetch timestamp in the evidence package. It is explicitly
  **not** tag-immutable — Pages content can change independently of
  the tag — and must never be presented to the evaluator or in any
  report as if it were.

Per `immutable-tags-rule.md`, the tag itself is never moved or
recreated; if a defect is found after Gate 1 approval, the correction
is a new plan revision citing a new baseline, not a rewrite of this
one.

## 2. Evidence universe (allowlisted by construction)

Four evidence classes, each a positive allowlist — nothing outside
these lists is ever read into the package. No coordination record,
plan file, learning record, prior due-diligence report, MR discussion,
or issue is part of the evidence universe, at any stage.

**A — Product/source archive** (git archive of the pinned commit
`f1932a1a...`, path-scoped):
`README.md`, `CHANGELOG.md`, `agf/LICENSE`, `agf/VERSION`,
`agf/ADOPTING.md`, `agf/agents/`, `agf/rules/`, `agf/schemas/`,
`agf/templates/`, `agf/docs/site/`, `agf/docs/build/`,
`agf/docs/governance/`, `tooling/`, `.gitlab-ci.yml`. This is the
"pinned public Rust/tooling source needed for the technical
assessment" the Governor decision names — it is what a code-quality
or architecture judgment can actually be made from, without pulling in
this project's own internal governance material (`docs/coordination/`,
`docs/plans/`, `docs/reviews/`, `agf/learnings/` are all outside this
list by construction, not by a separate exclude step).

**B — Release metadata**: the GitLab Release API object for `v2.5.0`
itself (tag name, release date, description, the 13 asset
name/URL/checksum entries) — Section 1's own verification record is
the frozen shape of this evidence class.

**C — Release assets**: the 13 published assets listed in Section 1,
downloaded from the GitLab generic-package registry, each checksum-
verified against its published `.sha256` file before being included.

**D — Public Pages**: a fetch of the live public Pages site,
timestamped at fetch time, labeled per Section 1's immutability note.

No bespoke scanner is built for this acquisition. Every byte in every
evidence class originates from this project's own tagged release, its
own release metadata, or its own public Pages — there is no
third-party or user-supplied content anywhere in this universe.

## 3. Evaluator prompt and output contract (frozen)

The evaluator is given exactly the following instructions, verbatim,
plus the assembled evidence package from Section 2. Coder-tester may
not alter this text when implementing Phase 2; any change requires a
plan revision and fresh Gate 1.

> You are performing a due-diligence assessment of the AGF project,
> using **only** the evidence package provided in this conversation as
> your source. Do not use any other knowledge you may have about this
> project, its team, or its history — if you recognize the project,
> disclose that recognition explicitly in your report but base every
> finding strictly on the provided evidence package.
>
> The evidence package contains: (A) the public source/documentation
> tree at the pinned release commit, (B) the release's own published
> metadata, (C) the release's published, checksum-verified binary/
> archive assets, and (D) a timestamped snapshot of the public
> documentation site. Cite every finding by evidence class and path
> (e.g., "A: agf/rules/workflow/tdd-rule.md" or "C:
> agf-2.5.0.tar.gz"). A citation that does not resolve to an actual
> path in the package is not acceptable.
>
> Answer these six dimension questions, each scored 1–5 or
> `unavailable` (a dimension the package does not address is always
> `unavailable`, never guessed): **product_architecture**,
> **code_quality_tests**, **governance_enforceability**,
> **key_person_concentration**, **security_supply_chain_licensing**,
> **adoption_market_release_ops**.
>
> Write three outputs: `report.md` (executive summary, one subsection
> per dimension citing evidence, a material-risks section, and a
> Conclusion section using exactly one of `{recommend-adopt,
> recommend-pilot, recommend-wait, recommend-decline,
> insufficient-evidence-to-conclude}`); `findings-register.json` (one
> entry per material finding, each citing an evidence path, with
> `stated_conclusion` matching `report.md`'s Conclusion); and
> `evidence-log.md` (one line per register entry, citing the exact
> evidence-package path it is drawn from).
>
> State plainly, as the first line of `report.md`, whether you have
> any prior awareness of this project from training or other sources,
> and if so, that your findings below are nonetheless scoped strictly
> to the provided evidence package.

**Limitations, stated honestly in the plan and required in the
report's own limitations section:** this is a genuinely fresh,
external conversation with no tool access to this repository and no
prior turns in this conversation — that much is mechanically true and
verifiable by the acquisition method itself (Section 2's evidence
package is the only input given). This plan does **not** claim the
evaluating model has no training-time awareness of this project (an
unprovable claim about model weights), and does not claim the
provider retains no memory across separate conversations on the same
account (a provider-operational fact outside this project's control).
The prompt above requires the evaluator to disclose any such prior
awareness rather than the plan asserting its absence.

**Raw-response preservation:** the evaluator's complete raw response
(not only the three named output files) is saved verbatim alongside
them, with the exact prompt text, evidence-package manifest hash, and
submission timestamp, as invocation metadata. Unlike MR !1403's
same-session design, there is no sensitive-material boundary concern
here — the evaluator was given no session-internal content to
possibly leak in the first place — so the raw response requires no
redaction before being committed as evidence.

## 4. Honest context-disclosure statement

This assessment is described, in the plan, in the evaluator prompt,
and in any resulting report, as: a new external conversation, given
only the evidence package in Section 2, asked to assess AGF v2.5.0
from that package alone. It is **not** described as memory-erased,
provider-blind, or otherwise stronger than the mechanical fact that no
repository tool access or prior conversation turns exist for this
submission. This replaces MR !1403's "simulated, disclosed-context"
framing entirely — that framing existed only because the prior design
used a same-session sub-agent; it does not apply here and must not be
carried forward into any implementation of this plan.

## 5. Forward role assignment (after Gate 1 only)

1. **Coder-tester**, on a new implementation branch, owns: literal
   acquisition of evidence classes A–D (Section 2), checksum
   verification of class C, allowlist assembly into the frozen package
   layout, and the report/register/log artifacts once the evaluator
   response is received. No implementation begins before binding Gate
   1 approval of this plan.
2. **Contribution-auditor** performs an exact-head implementation
   review of coder-tester's acquisition/assembly work before Gate 2.
3. **Build-engineer** owns any CI change this work turns out to need.
   The default and preferred design has **no CI change at all** — per
   the Governor decision, any CI need must be separately justified and
   governed, never introduced inside this plan or its implementation
   MR.
4. Only after binding Gate 2: submit the frozen package (Section 2)
   and prompt (Section 3) to a new external Gemini conversation,
   preserve the raw response and invocation metadata (Section 3), then
   perform fact-check (every `report.md` citation resolves to an
   actual evidence-package path — fact-checker) and reconciliation
   (contribution-auditor cross-checks `findings-register.json` against
   `report.md`'s Conclusion for consistency) before the assessment is
   considered complete.
5. This plan does not authorize, and no implementation of it may
   perform, any step beyond acquisition/assembly (Phase 1) and
   submission/preservation (Phase 2) without a further binding Gate 2
   decision for each subsequent phase.

## Risks and rollback

- **Risk**: the pinned tag/commit/release could be superseded by a
  later version before implementation begins. **Mitigation**: Section
  1's baseline is re-verified (same three commands) immediately before
  Phase 1 begins; if it has changed, this is a plan revision (new
  baseline, fresh Gate 1), not a silent substitution.
- **Risk**: Pages content drifts between acquisition and report
  writing. **Mitigation**: Section 2's fetch timestamp is fixed at
  acquisition; the evaluator is given that one fetch, not a live URL.
- **Rollback**: if Gate 1 does not approve this plan, no
  implementation exists to roll back — this document is the entire
  artifact. If Gate 2 review of coder-tester's implementation finds
  defects, the implementation branch is corrected and re-reviewed
  before evaluator submission; the evaluator is never contacted with
  an unreviewed package.

## Out of scope

- Does not build reusable due-diligence evidence-processing or
  candidate-file validation tooling — tracked separately as issue
  #108, requiring its own Gate 1 plan.
- Does not decide v2.5.1 issue #102 planning, a separate lane.
- Does not lift MR !1382/!1383's merge holds — those lift only per the
  Governor decision's stated condition (this plan's baseline pinned
  **and** binding Gate 1 approved), not automatically upon submission.
- Does not treat any script, fixture, or transcript from MR !1403 as
  authoritative — per the Governor decision, that material is
  non-authoritative prototype/debugging evidence only.
