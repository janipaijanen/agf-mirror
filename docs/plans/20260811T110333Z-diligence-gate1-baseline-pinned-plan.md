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
  rules, memory, or git state the way an `Agent`-tool spawn would.
  Round-1 review (`BP-DD-3`) correctly found an earlier draft of this
  section overclaimed that fact as "nothing to disclose because there
  is nothing inherited" — package acquisition alone cannot establish
  what a Gemini conversation setting, connected app, or account-level
  memory setting is doing. Section 3/4 below state only the narrower,
  provable fact: no AGF workspace/session context is *deliberately
  supplied* beyond the frozen package and prompt, and the invocation
  surface itself is recorded as evidence rather than assumed.
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
1), the evidence universe, acquisition method, and package layout
(Section 2), the frozen evaluator prompt, output schema, and
invocation-surface checklist (Section 3), the honest
context-disclosure statement (Section 4), and the exactly-two-candidate
forward role assignment — Candidate A (package) and Candidate B
(results), each separately Gate-2-gated — (Section 5) below. This
authorization covers Candidate A and Candidate B's existence and
sequencing; it does **not** itself construct any package, spawn or
contact any evaluator, or authorize either candidate's Gate 2 — each
of Gate 2 A and Gate 2 B is a separate binding Governor decision made
when its candidate is actually ready for review, not granted in
advance by this document. This document does not lift MR !1382/!1383's
containment holds — per the Governor decision, those holds lift only
when a replacement Gate 1 plan pins the baseline below **and**
receives binding Gate 1 approval, which this submission requests but
does not grant itself.

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

**B — Release metadata**: the raw GitLab Release API response for
`v2.5.0` itself (tag name, release date, description, and the 13 asset
link entries — name, URL, link type; the API exposes no checksum
field of its own). The downloaded bytes of this API response are
hashed into the package manifest (Section 2's manifest, below) exactly
like every other evidence file — the API response's *authenticity* is
established by the manifest hash, not by a checksum GitLab does not
publish for it.

**C — Release assets**: the exact 13 published links named in Section
1, downloaded from the GitLab generic-package registry. Acquisition
**fails closed** (produces no package) if the live Release does not
have exactly these 13 links, or any one is missing, duplicated,
renamed, or unreachable. Verification differs honestly by asset
shape, corrected here after round-1 review (`BP-DD-1`) found the plan
had claimed a uniform checksum check none of the 9 non-archive assets
actually have:

- The **4 payload archives** (`agov-2.5.0-linux-x86_64.tar.gz`,
  `agov-2.5.0-linux-aarch64.tar.gz`, `agf-2.5.0.tar.gz`,
  `agf-2.5.0.zip`) are each verified against their own paired
  `<name>.sha256` file — the only 4 of the 13 assets that have one.
- The **5 provenance JSON files** (`agov-oci-2.5.0-provenance.json`
  plus one paired provenance file per payload archive) have no
  `.sha256` sibling of their own. Each is instead parsed, and its
  declared source commit, tag, artifact name/kind, and embedded
  archive hash (or, for the OCI provenance file, OCI manifest digest)
  is cross-checked against the corresponding authoritative object
  already pinned in Section 1 (the tag/commit) or verified above (the
  archive's own `.sha256`-checked hash). A mismatch on any field fails
  acquisition.
- The **4 `.sha256` files** themselves have no meta-checksum; their
  content is exactly what verifies the payload archives above, and
  their own bytes are hashed into the package manifest like every
  other evidence file.
- The OCI container image itself is **not** acquired by this plan —
  only its provenance JSON (which is). The report's limitations
  section (Section 3) must state plainly that the OCI provenance
  claim is evidence as received, but its registry digest is not
  independently reverified against the live registry by this
  acquisition.

**D — Public Pages**: a bounded, same-origin snapshot of
`https://janipaijanen.gitlab.io/agents-govern-framework/` (the exact
base URL configured in `agf/docs/build/mkdocs.yml`'s `site_url`,
corrected here after `BP-DD-2` found the plan had named no URL at
all), fetched at one fixed acquisition timestamp:

- **Scope**: every HTML page reachable by same-origin recursive link
  traversal starting from the base URL, plus every same-origin asset
  (CSS, JS, image) those pages directly reference. External links
  (any URL outside the base URL's origin) are recorded as link text
  and target URL only — never fetched or followed.
- **Redirects**: same-origin redirects are followed once and the
  final resolved URL is what is recorded; a redirect leaving the
  origin is treated as an external link (not followed).
- **Errors**: a non-2xx response for any in-scope URL is recorded in
  the manifest as a failed fetch for that URL — it does not abort the
  whole Pages acquisition, but the report's limitations section must
  list every such failure.
- **URL-to-file mapping**: each fetched URL is written to a file path
  mirroring its URL path under the package's `pages/` subtree (e.g.
  `https://janipaijanen.gitlab.io/agents-govern-framework/rules/` →
  `pages/rules/index.html`), with a per-file SHA-256 recorded in the
  package manifest alongside the shared acquisition timestamp.

**Package layout and manifest (frozen; corrected here after
`BP-DD-2`)**: the assembled evidence package is one directory tree,
committed to the results/package candidate named in Section 5:

```
evidence-package/
  product-archive/         # Class A, extracted read-only from the git-archive tar
  release-metadata.json    # Class B, the raw API response bytes, unmodified
  release-assets/          # Class C, the 13 downloaded files, unmodified
  RELEASE-VERIFICATION.md  # Class C's per-asset verification method and result (see Class C above)
  pages/                   # Class D, the URL-mirrored HTML/asset tree
  MANIFEST.sha256          # one line per file above: sha256  relative/path
```

Release archives (Class C) are supplied **raw** — not extracted —
inside `release-assets/`; the evaluator is told which of them are
themselves archives and may reference their manifests/checksums as
evidence without this plan requiring their contents to be unpacked.
`RELEASE-VERIFICATION.md` is a generated (not downloaded) evidence
file recording, per asset, which of Class C's three verification
methods applied and the result — this is what resolves "checksum-
verified" honestly per file, rather than the prompt or report
asserting a uniform check that does not exist. `MANIFEST.sha256` is
the single per-file hash ledger for every file in the tree (including
`RELEASE-VERIFICATION.md` itself), generated once at acquisition time
and never edited after. This entire `evidence-package/` tree, plus
`MANIFEST.sha256`, is what is transmitted to the evaluator (Section
3) — every citation the
evaluator is permitted to make must resolve to a path inside this
tree, with no live network access required to verify any of it.

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
> metadata, (C) the release's published assets — verified per the
> notes in `RELEASE-VERIFICATION.md` (each file's own verification
> method is recorded there; not every asset has an independent
> checksum, and that is explained, not hidden), and (D) a timestamped
> snapshot of the public documentation site. Cite every finding by
> evidence class and path (e.g., "A: agf/rules/workflow/tdd-rule.md"
> or "C: release-assets/agf-2.5.0.tar.gz"). A citation that does not
> resolve to an actual path in `MANIFEST.sha256` is not acceptable.
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
> insufficient-evidence-to-conclude}`); `findings-register.json`
> (matching the frozen schema below); and `evidence-log.md` (one line
> per register entry, citing the exact evidence-package path it is
> drawn from).
>
> State plainly, as the first line of `report.md`, whether you have
> any prior awareness of this project from training or other sources,
> and if so, that your findings below are nonetheless scoped strictly
> to the provided evidence package.

**`findings-register.json` schema (frozen; added here after round-1
review found no field schema existed, `BP-DD-4`)** — a JSON array,
one object per material finding:

```json
{
  "id": "string, unique within the register, e.g. \"F-01\"",
  "dimension": "one of the six dimension names above",
  "claim": "string, one-sentence statement of the finding",
  "evidence_class": "one of A|B|C|D",
  "evidence_path": "string, resolves inside MANIFEST.sha256",
  "confidence": "one of high|medium|low",
  "stated_conclusion": "one of the five Conclusion terms above -- identical string across every entry"
}
```

**Consistency invariants**, checked at reconciliation (Section 5):
every entry's `stated_conclusion` is byte-identical to `report.md`'s
own Conclusion term; every `evidence_path` resolves to a real path in
the package's `MANIFEST.sha256`; every `dimension` value is one of the
six named above; the register is non-empty (at least one finding, or
the report's Conclusion must be `insufficient-evidence-to-conclude`
with that absence itself explained in `report.md`).

**Invocation-surface checklist (frozen; corrected here after
round-1 review found package acquisition alone cannot establish
conversation-level facts, `BP-DD-3`):** coder-tester records the
following as invocation evidence (Section 3's raw-response
preservation, below) at the moment of submission — this is what
actually supports any isolation claim, not the acquisition method:

- Gemini product/surface used (e.g. the consumer web app, AI Studio,
  or API), and the displayed model/version string.
- The new-conversation identifier or start time, confirming this is a
  conversation with zero prior turns.
- Whether a custom Gem or saved system-instruction persona is active
  — **must be no**; if the surface does not allow confirming this,
  record that inability as a limitation.
- Whether any connected app, extension, or plugin is enabled for this
  conversation, and whether browsing/search/live-retrieval tooling is
  enabled — disabled where the surface permits; recorded as a
  limitation where it cannot be disabled or verified.
- The account-level memory/personalization setting, if the surface
  exposes it as visible/inspectable state.
- The exact attachment set given to the evaluator (the package tree
  from Section 2, nothing else).

**Limitations, stated honestly in the plan and required in the
report's own limitations section:** the *deliberately supplied* input
is exactly the evidence package and prompt in Section 2/this section
— nothing else is intentionally given. This plan does **not** claim
the evaluating model has no training-time awareness of this project
(an unprovable claim about model weights), does not claim the
provider retains no memory across separate conversations on the same
account (a provider-operational fact outside this project's control),
and — corrected here — does not claim package acquisition alone
proves the conversation had no system instructions, connected tools,
or account-level context; the invocation-surface checklist above,
not the acquisition method, is what supports whatever isolation claim
the final report makes. The prompt above requires the evaluator to
disclose any prior awareness rather than the plan asserting its
absence.

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
provider-blind, or otherwise stronger than what the invocation-surface
checklist (Section 3) actually records — no AGF workspace/session
context is *deliberately supplied* beyond the frozen package and
prompt, and that recorded checklist, not an assumption about what
"external conversation" mechanically implies, is the evidence for any
isolation claim. This replaces MR !1403's "simulated, disclosed-context"
framing entirely — that framing existed only because the prior design
used a same-session sub-agent; it does not apply here and must not be
carried forward into any implementation of this plan.

## 5. Forward role assignment (after Gate 1 only)

Round-1 review (`BP-DD-4`) found the prior version of this section
left the branch/MR topology ambiguous — one branch could not
plausibly own both pre-submission package work and post-response
report artifacts, since the response does not exist until after
submission. This plan authorizes, at Gate 1, exactly **two** governed
candidates in sequence — no more, no fewer, and no step of either may
be taken without its own named gate below:

**Candidate A — the package candidate.** New branch, owned by
coder-tester. Contains: the assembled `evidence-package/` tree
(Section 2's layout), `MANIFEST.sha256`, `RELEASE-VERIFICATION.md`,
and the frozen prompt text (Section 3) copied verbatim into the branch
as `evaluator-prompt.md`, plus a submission checklist covering the
invocation-surface checklist (Section 3). Gated by: exact-head
contribution-auditor implementation review, then binding **Governor
Gate 2 A**. Gate 2 A authorizes exactly one thing beyond merging
Candidate A: the single external Gemini submission described in
Section 3/4, performed immediately after Gate 2 A using the merged
package unmodified.

**Candidate B — the results candidate.** New branch, opened only
after the Gemini submission has actually happened (never before —
per `coordination-publication-rule.md`'s lifecycle-semantics
principle, a record of a not-yet-occurred event is a false claim).
Owned by coder-tester. Contains, immutably: the evaluator's complete
verbatim raw response and the invocation metadata recorded at
submission time (Section 3), committed once and never edited in
place — a correction to a preserved raw response is a new file, not
an edit, mirroring `immutable-tags-rule.md`'s append-only principle.
Separately, in the same candidate: the derived `report.md`,
`findings-register.json` (matching Section 3's frozen schema), and
`evidence-log.md`, produced by coder-tester from the raw response.
Gated by: fact-checker (every citation in `report.md` resolves to a
`MANIFEST.sha256` path), contribution-auditor (reconciliation —
`findings-register.json`'s consistency invariants, Section 3, all
hold), then binding **Governor Gate 2 B**. Gate 2 B is the completion
gate for this entire assessment.

**Build-engineer** owns any CI change either candidate turns out to
need. The default and preferred design has **no CI change at all** —
per the Governor decision, any CI need must be separately justified
and governed, never introduced inside either candidate's MR.

No implementation of either candidate begins before this plan's own
binding Gate 1. No step of Candidate B begins before Candidate A's
Gate 2 A. No submission to Gemini happens more than once under this
plan's authorization — a second submission (e.g. to correct a defect
found after the fact) requires a plan revision and fresh Gate 1, not
an ad hoc resubmission.

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
