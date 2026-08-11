---
id: diligence-gate1-baseline-pinned-plan
status: approved
topic: one-time-external-due-diligence-assessment-baseline-pinned
gate1_approved_by: "20260811T161420Z-governor-reviewer-514h.md"
gate1_approved_at: "2026-08-11T16:14:20Z"
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
      Corrected here after round-4 review (`R4-DD-2`) found the prior
      wording of this sentence overclaimed a review of *every* response
      — Section 5's actual contract is **trigger-only**: coder-tester's
      bounded manual-plus-tool check runs on every response, but
      infosec-sentinel is engaged only if that check finds a trigger,
      not as a review step every response passes through regardless of
      outcome. This Gate 1 plan does not, and cannot, fabricate a
      pinned infosec-sentinel consultation in advance of content that
      does not exist yet; what it commits to is that *if* Section 5's
      check ever fires, infosec-sentinel's actual involvement at that
      point is mandatory before Candidate B proceeds further — a
      conditional precondition, not a review of every run.
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

## Outcome: ABANDONED after Gate 1 approval, before completion

Gate 1 was approved (below, `gate1_approved_by`/`gate1_approved_at`) after
seven review rounds. Candidate A (implementation) then acquired the full
evidence package and built the six-attachment `.tar.gz`/JSON transport
this plan specifies. The human Governor performed the one authorized
synthetic preflight; it **failed** — the evaluator (Gemini Pro) could not
open the attached archive at all
(`docs/coordination/20260811T175332Z-contribution-auditor-rtgg.md`,
finding `PF-1`). Per this plan's own text, that outcome required a plan
revision and fresh Gate 1 before any further Gemini submission.

Rather than invest further in a heavier, more heavily-governed replacement
transport design, the human Governor directed dropping this effort. Three
independently-run, ungoverned prompts (Gemini, Perplexity, a fresh Grok
run) were tried directly against live public URLs instead, outside any
plan/gate process — Gemini's output was unusable (fabricated, wrong
tech stack and file structure); Perplexity and Grok were partially to
substantially grounded, with specific fact-checked errors. The Grok run
is filed as a lightweight addition alongside the existing external
assessments in
`docs/reviews/20260810T134228Z-external-ai-diligence-grok-copilot/`,
following that filing's existing verbatim-relay-plus-corrections pattern,
not this plan's Candidate A/B apparatus.

This plan, Candidate A's branch (`candidate-a/diligence-gate1-baseline-pinned`,
MR !1440, closed without merging), and the six built transport attachments
are retired. No further Gemini submission, Candidate B, or plan revision
is authorized under this document. It is retained, unedited below, as the
historical record of the approved-but-abandoned design and the review
process that produced it.

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
`R2-DD-2` point 4; simplified here after round-4 review found the
submission-time re-fetch/re-verify step it required created an
unresolvable manifest-scope mismatch, `R4-DD-1` point 2)**: the four
payload archives are **not** duplicated as permanent git blobs in this
repository — they are already durably published, immutable evidence at
the GitLab Release (Section 1), and re-committing multi-megabyte
binaries into git history has a real, unnecessary storage cost. Only
their small text evidence (the 4 `.sha256` files, the 5 provenance JSON
files, and `RELEASE-VERIFICATION.md`, below) is committed to the
durable path. Their acquisition-time verification (Class C's fail-closed
acquisition, above) is the durable evidence of their integrity — round
4 removes the payload archives from the Gemini transport entirely (see
below), so there is no submission-time re-fetch step: the archives are
acquired and verified exactly once, at Candidate A package-assembly
time, and never touched again.

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
`RELEASE-VERIFICATION.md` is a generated (not downloaded) evidence
file recording, per asset, which of Class C's three verification
methods applied and the result — this is what resolves "checksum-
verified" honestly per file, rather than the prompt or report
asserting a uniform check that does not exist. The 4 payload archives
are **not** transmitted to the evaluator at all (removed from the
Gemini transport here after round 4, `R4-DD-1`) — see the Gemini
transport list below for what is.

**Gemini transport (frozen; added here after round-2 review found it
unspecified, `R2-DD-2` point 4; consolidated here after round-3 review
found the prior 18-unit design exceeds Gemini Apps' published 10-file-
per-prompt limit, `R3-DD-1` — see [Google Gemini Apps Help — Upload and
analyze files](https://support.google.com/gemini/answer/14903178),
independently confirmed live: up to 10 files per prompt, 100 MB per
file; corrected here after round-4 review found the 7-unit design's
manifest contract unresolvable and its transport shape unverified,
`R4-DD-1`; ordering corrected here after round-5 review found these
built after Gate 2 A rather than reviewed by it, `R5-DD-1`)**: the
evaluator is given exactly **6** file attachments, each built once at
Candidate A package-assembly time — **before** Gate 2 A reviews and
names them, not at real-submission time — and uploaded unmodified at
the real submission (Section 5):

1. `product-archive.tar.gz` — created from the committed
   `product-archive/` tree, byte-for-byte from those already-committed
   files (no regeneration from source).
2. `release-metadata.json` — the committed file, unmodified.
3. `release-assets-text.tar.gz` — bundling the 9 committed small
   Class C text files (4 `.sha256` + 5 provenance JSON) and
   `RELEASE-VERIFICATION.md` (10 members total) into one created
   container, byte-for-byte from those already-committed files.
4. `pages.tar.gz` — created from the committed `pages/` tree.
5. `MANIFEST.sha256` — the committed file, covering the pre-packaging
   member paths of attachments 1, 3, and 4 above.
6. `submission-manifest.json` — generated once, after attachments 1–5
   are final, the bridge between `MANIFEST.sha256` (which hashes
   pre-packaging member files) and what Gemini actually receives (3
   packaged containers: attachments 1, 3, and 4). For each of those
   three: the container's own filename and SHA-256, and one entry per
   member file giving its in-container path and its pre-packaging
   SHA-256 — which must be byte-identical to that same path's entry in
   `MANIFEST.sha256`. For attachment 2 (`release-metadata.json`):
   filename and SHA-256 only, uploaded unmodified. `submission-
   manifest.json` does **not** include an entry for itself — corrected
   here after round-4 review found the prior text both required and
   forbade self-hashing, `R4-DD-1` point 1, the same logical
   impossibility `MANIFEST.sha256`'s own sibling rule already resolves.
   Its own post-generation filename and SHA-256 are instead recorded in
   `invocation-metadata.json` (Section 5), generated after it, at the
   real submission.

The four payload archives are **removed from this transport entirely**
(round 4, `R4-DD-1` point 2 and point 4): they are never transmitted to
the evaluator. Every verification signal a due-diligence read can act
on — hash, provenance, and cross-check against the pinned tag/commit —
is already fully present in `RELEASE-VERIFICATION.md` and the 9 small
text files (attachment 3); the evaluator is never asked or expected to
open the raw binaries in the first place, so sending untouched
multi-megabyte archives added real transport/format risk (per round 4's
finding, below) without adding evidence a report can cite that is not
already available. This also removes the `MANIFEST.sha256`-scope
mismatch round 4 found: every member path any attachment now claims a
hash for is, without exception, a path actually committed inside
`evidence-package/` and present in `MANIFEST.sha256`.

Packaging (attachments 1, 3, 4) **is** a byte-level transformation of
the underlying files into a container — corrected here after round-3
review found the plan's prior claim that no bytes are transformed to be
false for exactly these attachments (`R3-DD-1`). What is preserved, and
what `submission-manifest.json` exists to prove, is that every *member
file's* pre-packaging bytes are unchanged and independently
re-verifiable against `MANIFEST.sha256` — not that the container bytes
themselves equal any pre-existing hash. Attachments 2, 5, and 6 are
transferred unmodified with no packaging step.

**Citation resolution (corrected here after round-4 review found the
frozen prompt text unchanged despite the packaging transformation,
`R4-DD-1` point 3; corrected again here after round-6 review found the
prompt still withheld the attachment-to-evidence mapping needed to cite
across a six-attachment, two-ledger transport, `R6-DD-1`)**: the
evaluator's citation contract (Section 3) is unaffected in *form* by
transport packaging — the evaluator still cites by evidence class and
the underlying committed file's path (e.g. "C: release-
assets/agf-2.5.0.tar.gz.sha256"), exactly as if reading the committed
`evidence-package/` tree directly. The frozen prompt (Section 3) now
tells the evaluator explicitly which of the six attachments maps to
which evidence root, and that `MANIFEST.sha256`/`submission-
manifest.json` are delivery/integrity ledgers, not citable product
evidence — correcting the prior claim that the evaluator was never told
about attachment structure at all, which round 6 found incompatible
with a multi-attachment, two-ledger transport. At Gate 2 B,
fact-checker's citation check becomes two steps, not one:
(a) the cited path exists in `MANIFEST.sha256`, as before, and (b) via
`submission-manifest.json`, the packaged container the evaluator
actually received contained that exact path with byte-identical
content — confirming transport packaging did not silently substitute or
corrupt what the evaluator saw.

**Transport-shape feasibility and the governed preflight (added here
after round-4 review found no authoritative support that Gemini Apps
unpacks and reasons over `.tar.gz` containers, `R4-DD-1`; fully
specified here after round-5 review found the preflight ungoverned —
no frozen fixture, no frozen expected answers, no submission-count
correction, no size check, `R5-DD-2`)**: this plan does not assume the
chosen `.tar.gz` container shape works without verification, and does
not spend the one authorized real AGF submission finding out. This plan
authorizes exactly **two** Gemini submissions, of two different
classes, each singular and never retried:

1. Exactly one **synthetic transport preflight** (below), before
   Gate 2 A.
2. Exactly one **real AGF assessment submission** (Section 4/5), after
   Gate 2 A.

Corrected here: Section 5's "no submission to Gemini happens more than
once" means no submission *of either class* happens more than once —
not that only one submission total exists across the plan. A failure of
either class (a failed preflight, or a triggered/flagged real response)
requires a plan revision and fresh Gate 1, never an ad hoc resubmission
of that class.

**Preflight fixture (frozen, literal content — not left to coder-tester
to invent)**: two throwaway, non-AGF text files, bundled into
`preflight.tar.gz` (the same container shape as the real transport,
mirroring its risk):

- `preflight/alpha.txt`, containing exactly the bytes
  `preflight-fixture-alpha-e5f2` (no trailing newline).
- `preflight/beta/gamma.txt`, containing exactly the bytes
  `preflight-fixture-gamma-91ac` (no trailing newline) — nested one
  directory deep, specifically to test subdirectory member visibility.

Each member's SHA-256 is recorded before bundling; `preflight.tar.gz`'s
own SHA-256 is recorded after.

**Preflight prompt (frozen, verbatim)**:

> You will receive one file attachment, `preflight.tar.gz`, a
> gzip-compressed tar archive. List the exact relative path and the
> exact literal file contents of every file it contains. Do not
> summarize or paraphrase file contents — quote them verbatim.

**Preflight pass condition (frozen, mechanical — no evaluator judgment
involved, unlike the real assessment)**: the raw response's text
contains, verbatim, both exact relative paths (`alpha.txt` and
`beta/gamma.txt`, allowing for a leading unpacked-root prefix such as
`preflight/`) and both exact file contents
(`preflight-fixture-alpha-e5f2` and `preflight-fixture-gamma-91ac`).
Any deviation — a missing member, truncated or paraphrased content, an
incorrect path — is a **fail**, not a partial pass; there is no scoring,
only pass/fail.

**Preflight invocation governance**: run in its own new, zero-prior-turn
conversation, distinct from the real submission's conversation, on the
same Gemini product/surface/model that will receive the real
submission. The chosen product/surface/model, the new-conversation
identity, and the pass/fail outcome are recorded in `transport-
preflight.md`, under Candidate A's package. The preflight's raw response
is captured verbatim and goes through the same fail-closed
pre-publication check (Section 5) as the real assessment's response
before being committed to `transport-preflight.md` — it is arbitrary
third-party output too, even though its content is synthetic.

**Real-attachment size governance (closes the Pages-bound/100 MB-limit
gap round 5 found, `R5-DD-2`)**: each of the six real attachments' exact
byte size, once packaged, is checked against Gemini's documented 100 MB
per-file limit at Candidate A package-assembly time (before Gate 2 A,
per the ordering fix in Section 5, below). Section 2's Class D Pages
acquisition bound (500 pages / 200 MiB raw fetched bytes / depth 6) is a
bound on *raw fetched content*, not on the packaged `pages.tar.gz`
attachment — if the packaged attachment exceeds 100 MB despite the
raw-fetch bound, that is a fail-closed acquisition failure requiring a
plan revision (e.g. a tighter Pages bound), never a silent violation or
an ad hoc workaround.

If the preflight does not pass exactly as specified, or any real
attachment exceeds 100 MB, Candidate A's transport design is not
executable as frozen — that is a fresh finding requiring a plan revision
and fresh Gate 1, not a design coder-tester improvises around.
Candidate A's package (Section 5) reserves the literal filename
`submission-checklist-template.md` for the blank invocation-surface
checklist (Section 3); Candidate B (Section 5) reserves
`submission-checklist-completed.md` for the filled-in record at the
real submission, kept explicitly distinct per round 5's finding that
the two were not previously distinguished.

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
> metadata, (C) checksum and provenance evidence **about** the
> release's published assets — **not the assets' own binary contents**
> (see the explicit limitation below) — verified per the notes in
> `RELEASE-VERIFICATION.md` (each file's own verification method is
> recorded there; not every asset has an independent checksum, and
> that is explained, not hidden), and (D) a timestamped snapshot of the
> public documentation site.
>
> You will receive exactly six file attachments, mapped as follows:
> `product-archive.tar.gz` contains Class A paths under
> `product-archive/`; `release-metadata.json` is the single Class B
> path `release-metadata.json`; `release-assets-text.tar.gz` contains
> Class C paths under `release-assets/`, plus `RELEASE-VERIFICATION.md`;
> `pages.tar.gz` contains Class D paths under `pages/`; `MANIFEST.sha256`
> and `submission-manifest.json` are delivery/integrity ledgers used to
> verify the other four attachments' contents — they are not
> independently sourced product evidence, and neither is a valid
> `evidence_path` citation in `findings-register.json`. Regardless of
> which attachment a file arrived in, cite it by evidence class and its
> underlying path within that attachment's mapped root, not by
> attachment or container name (e.g., "A:
> agf/rules/workflow/tdd-rule.md" or "C:
> release-assets/agf-2.5.0.tar.gz.sha256"). A citation that does not
> resolve to an actual path in `MANIFEST.sha256` is not acceptable.
>
> **You do not receive, and cannot inspect, the release's actual
> compiled/packaged binary archives or its OCI container image.** Class
> C gives you only their published checksums and provenance metadata
> (declared source commit, tag, artifact kind, and archive/image
> digest) — never their binary contents. You cannot verify or describe
> what is inside those archives or that image, their runtime behavior,
> or platform-specific parity from this evidence; do not claim to have
> inspected them. State this limitation explicitly in `report.md`'s
> limitations section whenever your findings touch code quality,
> supply chain, or release-artifact completeness.
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
— nothing else is intentionally given. As the frozen prompt above now
states explicitly (corrected here after round 5, `R5-DD-3`), the
evaluator never receives the release's actual compiled/packaged binary
archives or its OCI container image — only checksum and provenance
metadata about them — and cannot inspect their contents or runtime
behavior from this evidence. This plan does **not** claim
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
Class C's 4 payload archives are *not* committed and, since round 4,
never transmitted to the evaluator at all), `MANIFEST.sha256`,
`RELEASE-VERIFICATION.md`, and the frozen prompt text (Section 3)
copied verbatim into the branch as `evaluator-prompt.md`.

**Corrected here after round 5 found Gate 2 A reviewed source, not the
actual upload bytes, `R5-DD-1`**: Candidate A additionally contains the
**exact, final, immutable transport artifacts** — `product-archive.tar.gz`,
`release-assets-text.tar.gz`, `pages.tar.gz`, and `submission-manifest.json`
— built once, at Candidate A package-assembly time, byte-identical to
what the real submission will upload. There is no submission-time
regeneration: the real submission (below) uploads exactly these
already-committed, already-reviewed bytes. Candidate A also contains
`submission-checklist-template.md` (the blank invocation-surface
checklist, Section 3) and `transport-preflight.md` (the governed
synthetic preflight, Section 2, its fixture/prompt/pass-condition frozen
there and its outcome recorded here once run).

Gated by: exact-head contribution-auditor implementation review —
verifying all **six attachments** are present (corrected from "four
transport artifacts" here after round 6, `R6-DD-A1` — Section 2's
size check and the binding Gate 2 A decision both already covered all
six; this sentence had lagged), that `submission-manifest.json`'s
member hashes match `MANIFEST.sha256`, that each of the six attachments
is ≤100 MB (Section 2), and that `transport-preflight.md` records a
pass under Section 2's frozen preflight — then binding **Governor
Gate 2 A**. Gate 2 A's approval
names the exact reviewed identities: each attachment's filename and
SHA-256, the attachment count (6), and each attachment's exact byte
size. Gate 2 A authorizes exactly one thing beyond merging Candidate A:
the single real external Gemini submission described in Section 2
(transport)/Section 3/Section 4, uploading precisely the bytes Gate 2 A
named — no regeneration, no substitution — in a new, zero-prior-turn
conversation distinct from the preflight's. Any byte, member, surface,
or model change from what Gate 2 A named invalidates that approval and
requires a fresh Gate 2 A before submission.

**Candidate B — the results candidate.** New branch, opened only
after the Gemini submission has actually happened (never before —
per `coordination-publication-rule.md`'s lifecycle-semantics
principle, a record of a not-yet-occurred event is a false claim).
Owned by coder-tester. Reserved durable paths, all under
`docs/diligence/v2.5.0-baseline/results/`:

- `raw-response.md` and `invocation-metadata.json` — the evaluator's
  complete verbatim raw response and the invocation metadata recorded
  at submission time (Section 3), committed once and never edited in
  place. A correction to either is a new, separate file (e.g.
  `raw-response-correction-1.md`), never an edit to the original,
  mirroring `immutable-tags-rule.md`'s append-only principle.
- `submission-manifest.json` and `submission-checklist-completed.md` —
  **reserved here after round 5, `R5-DD-1`**: a copy of the exact
  `submission-manifest.json` Gate 2 A named and the real submission
  used (already durably committed in Candidate A, but copied here so
  Candidate B's reconciliation evidence is self-contained), and the
  filled-in invocation-surface checklist recorded at the real
  submission — explicitly distinct from Candidate A's
  `submission-checklist-template.md`.
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
`MANIFEST.sha256` path **and**, via `submission-manifest.json`, to a
packaged container member that byte-matches that path — added here
after round 4, `R4-DD-1` point 3; **and**, added here after round 5,
`R5-DD-3`, `report.md` contains no claim of having directly inspected
the payload archives' or OCI image's binary contents or runtime
behavior — any such claim is a fact-check rejection, not a passable
finding), contribution-auditor (reconciliation —
`findings-register.json`'s consistency invariants, Section 3, all
hold), then binding **Governor Gate 2 B**. Gate 2 B is the completion
gate for this entire assessment.

**Build-engineer** owns any CI change either candidate turns out to
need. The default and preferred design has **no CI change at all** —
per the Governor decision, any CI need must be separately justified
and governed, never introduced inside either candidate's MR.

No implementation of either candidate begins before this plan's own
binding Gate 1. No step of Candidate B begins before Candidate A's
Gate 2 A. Per Section 2's two-submission-class correction (round 5,
`R5-DD-2`), no submission *of either class* — the synthetic preflight or
the real assessment submission — happens more than once under this
plan's authorization; a second submission of either class (e.g. to
correct a defect found after the fact, or to retry a failed preflight)
requires a plan revision and fresh Gate 1, not an ad hoc resubmission.

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
