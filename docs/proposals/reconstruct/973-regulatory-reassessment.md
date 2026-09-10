<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# !973: regulatory reassessment reconstruction proposal

Status: proposed; governor disposition and independent review pending.
Capability maturity: Level 1 Planned. Evidence: empirically informed retention
proposal; no present-day regulatory applicability finding.
Owner of this draft: Codex, temporary planner-architect under m3hj.
Independent reviewer requested: Claude, contribution-auditor.

## Decision requested

Select **reconstruct**, preserving the unique assessment contract below, rather
than close-invalid. Alternatively approve park-radar with this full contract
retained and an explicit reassessment trigger. The latest QC-1 draft proposes
park-radar for both source MRs; this proposal does not override that ledger.
A passed earliest-start trigger is not evidence that a reassessment is invalid.

Before activating reassessment, the governor must select its baseline: either
historical v2.5.0 as originally requested, or a named released version and peeled
commit selected for a contemporary reassessment. Never silently change versions.
The reconstruction does not itself add assessment work to v2.11 release scope.

## Preserved scope and authority

The source assigns planner-architect to scope/plan, fact-checker to citation and
applicability verification, infosec-sentinel to security/privacy dimensions, and
the governor to dispositions and roadmap decisions. Thus it is not an unowned
TODO. These are retained proposed responsibilities, not new player activations.

Preserve the three-axis output: filled / could-fill / cannot-fill, crossed with
verified regulatory status, and tied to shipped capability evidence. Distinguish
structural exclusions from unbuilt capabilities. Preserve four deliverables:
updated or dated successor analysis, business-readable disposition table,
scoped candidate shortlist for later Gate 1, and verification of every dated
claim. All regulations, dates and applicability statements in the appendix are
historical unverified source text, not findings or legal advice from this draft.

## Proposed execution, after its own Gate 1

1. Resolve MR !967 and docs/plans/ai-regulation-requirements-gap-analysis.md
   against the approved baseline. If that source exists only on an unmerged
   branch, preserve and review it; do not pretend it is an accepted main artifact.
2. Freeze baseline tag/commit, assessment date, jurisdiction and intended adopter
   audience with the governor. Record explicit exclusions and evidence owners.
3. Fact-checker verifies current primary legal/standards sources and applicability;
   InfoSec reviews scoped security/privacy rows. No conclusions from old dates.
4. Produce the four retained deliverables with a row-to-source evidence trail,
   five-level capability maturity, evidence status and uncertainty. Governor
   dispositions determine any later roadmap or release commitments.

## Acceptance and preservation before source closure

- Claude independently checks fidelity to the complete source section below,
  baseline choice, role boundaries and proposed disposition against QC-1.
- The governor decides the disposition; if reconstruct is selected, this retained
  contract is published on a shared branch and reviewed as the visible carrier.
- Draft authorship does not approve the eventual assessment plan. Any execution
  receives its own Gate 1, acceptance evidence and independent reviews.
- For this proposal, require the retained contract to merge before source closure;
  this is deliberately stricter than mere branch visibility. The closure step
  retains QC-4 exact-state, digest, independent review and governor authorization.
- Keep !973 open until those conditions pass; update QC-1 through its owner, not
  this drafting branch. No forge mutation is part of this proposal.

Review date proposed for an unactivated assessment: 2026-10-10 UTC. Earlier
review trigger: governor selection of a baseline or a public applicability claim
that depends on this assessment. The date is a proposal, not a scheduled job.

## Source provenance and complete retained text

Source: MR !973, commit `3541f00bcbeca936e330a9c103a7c7783e28be50`,
`docs/TODO.md`, section `Post-v2.5.0 Regulatory Requirements Re-Assessment`.
The section bytes below have SHA-256 `e3973b5c54b027629405d948258baab09443a9af3d260d97729e53434b17f516`
(UTF-8, including the source trailing newline). This fenced historical appendix
is retained evidence, not current authority or verified applicability.

```markdown
## Post-v2.5.0 Regulatory Requirements Re-Assessment

Status: open TODO, gated on release. Owner: planner-architect for the Gate 1
re-assessment plan; fact-checker for citation/applicability verification;
infosec-sentinel for the security/privacy-dimension rows; governor-reviewer
for disposition and any resulting roadmap items.

**Trigger condition — do not start before all three are true:** v2.5.0 is
released and tagged, the release is published (GitLab release + distribution
archives available), and adopter-facing documentation for v2.5.0 is updated
and live. This TODO is explicitly deferred until after that point — it is a
re-assessment of AGF's *shipped* state, not a pre-release design exercise.

### Problem

`docs/plans/ai-regulation-requirements-gap-analysis.md` already inventories
every AI regulation/governance-standard requirement found in the source
registry (`docs/sources.md`) against AGF + `agov`'s current capability, using
the five-level maturity scale. That document is a snapshot taken mid-v2.5.0
development; several rows will move once v2.5.0 actually ships (e.g. any
compliance-export or documentation work landing in the release), and the
regulations themselves keep moving (several rows in that document are
explicitly flagged `[dates: verify before citing]`). A one-time snapshot goes
stale the moment it's read as current.

### Scope

Once the trigger condition is met, re-run the gap analysis against the
released v2.5.0 state, split three ways per requirement:

1. **Already filled** — the maturity level now genuinely supports the
   requirement (cite the shipped artifact: a released command, a published
   doc page, a merged validator — not just a plan or intent).
2. **Could be filled** — a concrete, scoped path exists (a named issue, a
   plan, a follow-on adapter already identified in the gap-analysis
   document) but is not yet built.
3. **Cannot be filled** — a structural boundary, not a backlog item (e.g.
   Layer 4 runtime-behaviour enforcement, which `agf/docs/adopt/
   compliance-profile.md` declares out of scope for every compliance tier
   by design). State this honestly as a permanent exclusion, not a future
   todo, per `implementation-honesty-rule.md`.

Split further by regulation status, since "effective" and "probably
effective" carry different urgency:

- **Effective now** — regulations already in force as of the reassessment
  date (re-verify each date; several are cited from a 2026-07 snapshot and
  may have moved, including the AI Act Art. 26 "Omnibus VII" deferral noted
  as "agreed but not yet law" at drafting time).
- **Probably effective / scheduled** — regulations with a known future
  effective date (CRA reporting duties from 2026-09-11, PLD transposition by
  2026-12-09, etc.) — assess AGF's trajectory toward readiness, not just
  current-moment compliance.
- **Proposed / voluntary / standards-track** — NIST AI RMF, ISO 42001, SLSA,
  CycloneDX Agent-BOM, IETF WIMSE/AIMS, etc. — track maturity of the
  standard itself alongside AGF's maturity against it, since several are
  explicitly immature or still drafts.

### Candidate output

1. An updated `docs/plans/ai-regulation-requirements-gap-analysis.md` (or a
   dated successor snapshot, if the living-document update pattern turns out
   to be the wrong shape) reflecting the released v2.5.0 state.
2. A three-column disposition table (filled / could-fill / cannot-fill) that
   a business reader can act on without reading the full per-requirement
   detail, per `documentation-audience-rule.md`'s business-user framing for
   anything published under `docs/`.
3. A prioritized shortlist of "could be filled" items worth a Gate 1 plan
   next, distinct from items correctly left as backlog.
4. Explicit confirmation (or correction) of every date this document
   currently carries a recheck caveat on.
```
