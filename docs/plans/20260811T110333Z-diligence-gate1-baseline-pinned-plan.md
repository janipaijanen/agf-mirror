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
    status: waived
    reason: >-
      Corrected here after round-3 review (`R3-DD-2`) found
      `not-applicable` wrong in kind: this project's only playback
      mechanism for infosec-sentinel is CI-automated
      (`agf/agents/MANIFEST.yaml`: "ci (agov validate infosec, on
      commit)"), and `agov validate infosec`'s actual, current scope is
      narrow — it scans `agf/learnings/*.yaml` free-text fields for
      secret patterns; it has no scope over plan text or, more to the
      point, over content that does not exist yet. Run live against
      this plan branch's current tree immediately before this revision
      (`agov validate` → `OK agov.validate.infosec`, scanning its one
      in-scope file, `agf/learnings/foundry.yaml`): clean, but that run
      says nothing about Section 5's future third-party evaluator
      response, because no mechanism this project has today scans that
      content class at all. A pinned "applicable" consultation result
      for content that is not yet acquired cannot honestly be obtained
      before Gate 1 — so this entry is **waived**, on the same footing
      as `fact-checker` and `web-ux-reviewer` below: the real
      consultation is a Gate 2 B precondition (Section 5), not a Gate 1
      one. What is frozen now, at Gate 1, is the handling rule itself —
      Section 5's fail-closed pre-publication check (bounded manual
      review plus whatever secret/sensitive-content tooling this
      project already has available, e.g. `detect-secrets`/
      `gitleaks`-class per `security-hardening-rule.md`; no bespoke
      scanner) — and the commitment that `raw-response.md` is not
      committed until that check runs and any trigger is routed to
      infosec-sentinel and governor-reviewer, per Section 5's own text.
      A real, applicable infosec-sentinel review of the actual response
      is a named precondition of Candidate B proceeding past that
      check, not something this Gate 1 plan can fabricate in advance of
      the content it would review.
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
field of its own), fetched from
`https://gitlab.com/api/v4/projects/83321848/releases/v2.5.0`
(the authoritative HTTPS endpoint). Corrected here after round-2
review (`R2-DD-1`) found the plan conflated integrity with
authenticity: the response's **provenance** record is the exact
endpoint above, the HTTP response status (must be `200`), and the
acquisition timestamp — recorded alongside the response bytes as
`release-metadata.json`. `MANIFEST.sha256` (below) records only the
**post-acquisition integrity** of that already-provenance-recorded
file (that its bytes have not changed since acquisition) — it does
not and cannot establish that the bytes originated from GitLab in the
first place; that is what the recorded endpoint/status/timestamp is
for.

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

- **Scope (bounded; corrected here after round-2 review found
  "same-origin" alone unbounded, `R2-DD-2` point 1)**: every HTML page
  whose path starts with the exact prefix `/agents-govern-framework/`
  on host `janipaijanen.gitlab.io`, reachable by recursive link
  traversal starting from the base URL, plus every same-prefix asset
  (CSS, JS, image) those pages directly reference. A URL differing
  only by query string or `#fragment` is canonicalized by dropping
  both before comparison — two URLs identical except for query/fragment
  are the same page, fetched once. A path collision (two distinct
  source URLs canonicalizing to the same local file path) fails
  acquisition rather than silently overwriting one file with another.
  The crawl stops at whichever bound is reached first: 500 pages, 200
  MiB total fetched bytes, or link depth 6 from the base URL — reaching
  a bound is recorded as a limitation in the report, not a silent
  truncation. A same-prefix asset or link outside `/agents-govern-
  framework/` (any other path on the same host, or any other host
  entirely) is recorded as link text and target URL only — never
  fetched or followed.
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

**Durable repository path (frozen; added here after round-2 review
found none reserved, `R2-DD-2` point 3)**: Candidate A (Section 5)
commits its package identity artifacts under
`docs/diligence/v2.5.0-baseline/` — a fresh, dedicated top-level
location, distinct from and unrelated to the superseded MR !1403's
`docs/plans/support/fresh-context-diligence-gate1/` (non-authoritative
per the Governor decision; not reused).

**Retention choice for Class C's binary archives (frozen; added here
after round-2 review offered this as the required either/or,
`R2-DD-2` point 4)**: the four payload archives are **not** duplicated
as permanent git blobs in this repository — they are already durably
published, immutable evidence at the GitLab Release (Section 1), and
re-committing multi-megabyte binaries into git history has a real,
unnecessary storage cost. Only their small text evidence (the 4
`.sha256` files, the 5 provenance JSON files, and
`RELEASE-VERIFICATION.md`, below) is committed to the durable path.
The archives themselves are re-fetched immediately before Gemini
submission (Section 5) and their freshly-fetched bytes re-verified
against the already-committed `.sha256`/provenance evidence before
upload — if a re-fetch does not match, submission is refused and this
is a fresh finding, not silently proceeded past.

**Package layout and manifest (frozen; corrected here after `BP-DD-2`
and, for the manifest self-reference defect, `R2-DD-2` point 2)**: the
assembled evidence package is one directory tree under
`docs/diligence/v2.5.0-baseline/evidence-package/`:

```
evidence-package/
  product-archive/         # Class A, extracted read-only from the git-archive tar
  release-metadata.json    # Class B, the raw API response bytes, unmodified
  release-assets/          # Class C, ONLY the 9 small text files (4 .sha256 + 5 provenance JSON) --
                            # NOT the 4 payload archives themselves, per the retention choice above
  RELEASE-VERIFICATION.md  # Class C's per-asset verification method and result (see Class C above)
  pages/                   # Class D, the URL-mirrored HTML/asset tree
MANIFEST.sha256             # sibling of evidence-package/, NOT inside it --
                             # one line per file INSIDE evidence-package/: sha256  relative/path
```

`MANIFEST.sha256` is a **sibling** of `evidence-package/`, not a file
inside it — round-2 review correctly found that a manifest hashing
"every file in the tree, including itself" is a logical impossibility
(a file's hash cannot include its own hash). It lists every file
inside `evidence-package/` and nothing else; it is generated once,
after every other file in the tree is final, and never edited after.
Release archives, when re-fetched for submission per the retention
choice above, are supplied **raw** — not extracted — as transient
submission-time attachments; the evaluator is told which of them are
themselves archives and may reference their manifests/checksums
(already in the committed `evidence-package/`) as evidence without
this plan requiring their contents to be unpacked.
`RELEASE-VERIFICATION.md` is a generated (not downloaded) evidence
file recording, per asset, which of Class C's three verification
methods applied and the result — this is what resolves "checksum-
verified" honestly per file, rather than the prompt or report
asserting a uniform check that does not exist.

**Gemini transport (frozen; added here after round-2 review found it
unspecified, `R2-DD-2` point 4; consolidated here after round-3 review
found the prior 18-unit design exceeds Gemini Apps' published 10-file-
per-prompt limit, `R3-DD-1` — see [Google Gemini Apps Help — Upload and
analyze files](https://support.google.com/gemini/answer/14903178),
independently confirmed live: up to 10 files per prompt, 100 MB per
file)**: the evaluator is given exactly **7** file attachments — the
prior design's 9 individual Class-C text files and 4 individual payload
archives are each bundled into one container, below, specifically to
fit this limit:

1. `product-archive.tar.gz` — freshly created at submission time from
   the committed `product-archive/` tree, byte-for-byte from those
   already-committed files (no regeneration from source).
2. `release-metadata.json` — the committed file, unmodified.
3. `release-assets-text.tar.gz` — **new, bundling the 9 committed
   small Class C text files (4 `.sha256` + 5 provenance JSON) and
   `RELEASE-VERIFICATION.md` (10 members total) into one freshly
   created container**, byte-for-byte from those already-committed
   files.
4. `release-assets-payloads.tar.gz` — **new, bundling the 4 payload
   archives (each independently confirmed under 3.5 MB, well inside
   Gemini's 100 MB per-file limit), re-fetched and re-verified per the
   retention choice above, into one freshly created container.** The
   evaluator is told these 4 members are themselves archives and may
   cite their manifests/checksums/provenance (already fully described
   in attachment 3) as evidence without this plan requiring the
   evaluator to unpack them.
5. `pages.tar.gz` — freshly created at submission time from the
   committed `pages/` tree.
6. `MANIFEST.sha256` — the committed file, covering the pre-packaging
   member paths of attachments 1, 3, 4, and 5 above.
7. `submission-manifest.json` — **new (added here per `R3-DD-1`)**:
   generated fresh at submission time, the exact bridge between
   `MANIFEST.sha256` (which hashes pre-packaging member files) and
   what Gemini actually receives (4 freshly packaged containers). For
   each of attachments 1, 3, 4, and 5: the container's own filename and
   SHA-256 as uploaded, and one entry per member file giving its
   in-container path and its pre-packaging SHA-256 — which must be
   byte-identical to that same path's entry in `MANIFEST.sha256`. For
   attachments 2, 6, and 7: filename and SHA-256 only (uploaded
   unmodified or, for this file itself, generated once and not
   self-hashed, mirroring `MANIFEST.sha256`'s own sibling rule above).
   A citation the evaluator makes (e.g. "C:
   release-assets/agov-2.5.0-linux-x86_64.tar.gz.sha256") resolves by:
   look up the path in `submission-manifest.json`'s member entries to
   find its container and pre-packaging hash, then confirm that hash
   matches `MANIFEST.sha256`'s entry for the same path.

Packaging (attachments 1, 3, 4, 5) **is** a byte-level transformation
of the underlying files into a container — corrected here after
round-3 review found the plan's prior claim that no bytes are
transformed to be false for exactly these attachments (`R3-DD-1`).
What is preserved, and what `submission-manifest.json` exists to prove,
is that every *member file's* pre-packaging bytes are unchanged and
independently re-verifiable against `MANIFEST.sha256` — not that the
container bytes themselves equal any pre-existing hash. Attachments 2,
6, and 7 are transferred unmodified with no packaging step. The prompt
(Section 3) tells the evaluator which numbered attachment corresponds
to which evidence class, and every citation the evaluator is permitted
to make must resolve to a path recoverable from `submission-manifest.json`
and `MANIFEST.sha256` together, with no live network access required to
verify any of it. Candidate A's package (Section 5) reserves the
literal filename `submission-checklist.md` for the invocation-surface
checklist (Section 3) coder-tester records at submission time.

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
> or "C: release-assets/agf-2.5.0.tar.gz.sha256"). A citation that does
> not resolve to an actual path in `MANIFEST.sha256` is not acceptable.
>
> Answer these six dimension questions, each scored 1–5 or
> `unavailable` (a dimension the package does not address is always
> `unavailable`, never guessed): **product_architecture**,
> **code_quality_tests**, **governance_enforceability**,
> **key_person_concentration**, **security_supply_chain_licensing**,
> **adoption_market_release_ops**.
>
> Write three outputs: `report.md` (executive summary, one subsection
> per dimension citing evidence — each subsection headed by exactly the
> dimension name, e.g. `### product_architecture`, with the literal
> line `Score: {1-5 or unavailable}` immediately following the heading
> — a material-risks section, and a Conclusion section using exactly
> one of `{recommend-adopt, recommend-pilot, recommend-wait,
> recommend-decline, insufficient-evidence-to-conclude}`);
> `findings-register.json` (matching the frozen schema below); and
> `evidence-log.md` (one line per register entry, citing the exact
> evidence-package path it is drawn from).
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
(not only the three named output files) is captured verbatim, with the
exact prompt text, evidence-package manifest hash, and submission
timestamp, as invocation metadata. Corrected here after round-3 review
found the prior text asserted no sensitive-material boundary applies at
all (`R3-DD-2`) — that is true of what the evaluator was *given* (no
session-internal content to leak, since the input evidence is entirely
first-party public material per Section 2), but not of what the
evaluator *returns*: the response itself is third-party, arbitrary
external LLM output, and Section 5's fail-closed pre-publication
handling rule — not an assumption of safety — is what governs whether
and how it is committed. The raw response is captured to a local,
uncommitted location immediately at submission time; it is committed as
`raw-response.md` only after Section 5's check passes, or, if a trigger
is found, only the safe class/location/hash evidence Section 5
describes is committed instead.

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
coder-tester. Contains, under `docs/diligence/v2.5.0-baseline/`
(Section 2's durable path): the committed subset of the
`evidence-package/` tree (Section 2's layout and retention choice —
Class C's 4 payload archives are *not* committed), `MANIFEST.sha256`,
`RELEASE-VERIFICATION.md`, and the frozen prompt text (Section 3)
copied verbatim into the branch as `evaluator-prompt.md`, plus a
submission checklist covering the invocation-surface checklist
(Section 3). Gated by: exact-head contribution-auditor implementation
review, then binding **Governor Gate 2 A**. Gate 2 A authorizes
exactly one thing beyond merging Candidate A: the single external
Gemini submission described in Section 2 (transport)/Section 3/
Section 4, performed immediately after Gate 2 A using the merged
package and the Section 2 retention/re-verification procedure for
Class C's archives.

**Candidate B — the results candidate.** New branch, opened only
after the Gemini submission has actually happened (never before —
per `coordination-publication-rule.md`'s lifecycle-semantics
principle, a record of a not-yet-occurred event is a false claim).
Owned by coder-tester. Reserved durable paths, both under
`docs/diligence/v2.5.0-baseline/results/`:

- `raw-response.md` and `invocation-metadata.json` — the evaluator's
  complete verbatim raw response and the invocation metadata recorded
  at submission time (Section 3), committed once and never edited in
  place. A correction to either is a new, separate file (e.g.
  `raw-response-correction-1.md`), never an edit to the original,
  mirroring `immutable-tags-rule.md`'s append-only principle.
- `report.md`, `findings-register.json` (matching Section 3's frozen
  schema — additional fields beyond the frozen 7 are permitted and
  preserved verbatim if the evaluator includes them, but only the
  frozen 7 are validated; `id` and `evidence_path` must each be a
  non-empty string, `id` unique within the register), and
  `evidence-log.md` — each omitted entirely, not populated with
  coder-authored substitute content, if the byte-faithful derivation
  rule below finds it not cleanly extractable.
- `extraction-status.md` — **reserved here after round-3 review found
  the derivation-failure path had no coder-owned location of its own,
  `R3-DD-3`** — coder-tester's own record of which of the three files
  above were cleanly extracted and which were omitted and why. Always
  present once Candidate B exists, regardless of whether any omission
  occurred.

**Byte-faithful derivation (frozen; added here after round-2 review
found "produced from the raw response" permitted undefined rewriting,
`R2-DD-4` point 3; corrected here after round-3 review found the
failure path itself authored substitute evaluator content, contradicting
the very next sentence, `R3-DD-3`)**: coder-tester extracts `report.md`,
`findings-register.json`, and `evidence-log.md` from the raw response
byte-for-byte (copy, not paraphrase, summarize, correct, or
supplement). If the raw response does not contain a cleanly extractable
version of any of the three — malformed JSON, a missing Conclusion
term, or similar — that is a **preservation-worthy failure**, not a gap
coder-tester fills: the affected file (`report.md`,
`findings-register.json`, and/or `evidence-log.md`) is **omitted
entirely** rather than populated with any coder-authored value — no
Conclusion is set, no empty register is manufactured as a stand-in for
a real one. The raw response is preserved regardless. Instead,
coder-tester records the mechanical failure — which file(s), which
extraction step, and why — in a separate, explicitly coder-owned
artifact, `extraction-status.md`, reserved here alongside
`raw-response.md`/`invocation-metadata.json` under
`docs/diligence/v2.5.0-baseline/results/`. Coder-tester never authors a
substitute finding, score, or conclusion that the evaluator itself did
not produce, in `report.md`/`findings-register.json`/`evidence-log.md`
or anywhere else presented as if it were evaluator output;
`extraction-status.md` is unambiguously coder-tester's own status
report, never mistakable for the evaluator's. Gate 2 B's binding
Governor decision (below), informed by `extraction-status.md` when
present, determines the assessment's outcome when extraction fails —
this plan does not pre-decide it as `insufficient-evidence-to-conclude`
or any other specific value.

**Score anchors (frozen; added here after round-2 review found none
defined, `R2-DD-4` point 1; completed here after round-3 review found
only 1/3/5 defined, `R3-DD-3`)**: for every dimension, `1` = no
supporting evidence found or the evidence indicates a severe,
unmitigated gap; `2` = minimal evidence — a partial or tangential
capability exists but with major, largely unaddressed gaps; `3` =
partial evidence, some real capability with material gaps; `4` =
substantial evidence — a materially complete capability with only
minor, specifically-identified gaps; `5` = strong, directly-cited
evidence of a mature, complete capability for that dimension.
`unavailable` remains distinct from `1` — `1` is evidence *of absence
or severe weakness*, `unavailable` is *absence of evidence either way*.
The relation between the six dimension scores and the report's single
Conclusion term is the evaluator's own judgment call, not a mechanical
formula this plan prescribes — the reconciliation check (below)
verifies internal consistency (the stated conclusion appears
identically everywhere it must), not that the scores mathematically
justify the conclusion. **Score placement (frozen; added here after
round-3 review found no literal location specified, `R3-DD-3`)**: each
dimension's score appears exactly once, as the first line of that
dimension's subsection in `report.md`, in the exact literal form
`Score: {1-5 or unavailable}` — the subsection heading itself names
only the dimension (e.g. `### product_architecture`), immediately
followed by the `Score:` line, so reconciliation can locate every score
by a fixed textual position rather than inferring presentation.

**Fail-closed handling of the raw response before publication (frozen;
added here after round-2 review found the evaluator's response is
untrusted third-party content, `R2-DD-3`)**: before `raw-response.md`
is committed, coder-tester runs a bounded manual review plus whatever
secret/sensitive-content detection tooling this project already has
available (no bespoke scanner is built for this). If a trigger is
found: the raw content is **not** committed or quoted anywhere,
including in a coordination record or MR description; only the
trigger's class, its approximate location in the response, and a hash
of the full raw response are recorded, and the finding is routed to
infosec-sentinel and governor-reviewer for a decision before Candidate
B proceeds further. Because Section 5's "no submission happens more
than once" rule still holds, a triggered response is **not** silently
resubmitted to Gemini for a "clean" retry — the Governor decides
whether the assessment is abandoned, the flagged content is handled
under a separate governed disclosure process, or some other
resolution, exactly as `deputy-governor-rule.md`'s red list already
treats security-boundary decisions as non-delegable.

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
