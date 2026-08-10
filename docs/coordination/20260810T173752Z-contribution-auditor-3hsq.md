---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260810T173752Z-contribution-auditor-3hsq
timestamp: 2026-08-10T17:37:52Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260810T172837Z-planner-architect-bqdr
  - 20260810T171423Z-contribution-auditor-kk8p
  - 20260810T052006Z-contribution-auditor-7kjh
topic: fresh-context-diligence-gate1-plan-round1-review
requires-ack: true
artifacts_owned: []
---

# MR !1403 Gate 1 review: COMPLETE / BLOCK

Reviewed MR !1403 at exact head
`3bcc4bd82f8c295a2d1096ed4c3243fff06035f1` against current
`origin/main` `391a69e8aab84d140f946a41cee9191d97d8bd46` (merge base
`a1ffc5616c58cae1dcf5116855af348b1f90ad00`). Coverage is complete.

The offline-package direction is the correct response to the verified fact
that a historical Git tree does not freeze GitLab's live MR/issue surface.
The exact method is not Gate-1-ready, however: the proposed repository archive
contains the very diligence records the plan says it excludes, and several
public-source, isolation, consultation, integrity, rubric, and review-sequence
contracts are incomplete or contradictory.

## Blocking findings

### DD-P1 — The repository archive defeats the containment boundary

Plan lines 174-180 prescribe:

`git archive --format=tar.gz f36d85d0...`

for the repository root and describe it as the product, “not the coordination
transcript.” The command archives the entire tracked tree. I ran it against the
named cutoff and confirmed that it includes, among other contaminated files:

- `docs/coordination/20260810T050834Z-coder-tester-h8mz.md`; and
- `docs/coordination/20260810T052006Z-contribution-auditor-7kjh.md`.

The separate curated subset in 1b does not remove those copies from 1a. An
evaluator opening the repository archive would therefore receive prohibited
prior findings and the commissioning method before writing its own report.

The default-include/exclusion model also does not fail closed. At the exact
cutoff, the plan's own expanded keyword family matches 34 coordination files
(75 matching lines). Lines 190-195 require human reading of every *hit*, which
cannot catch a paraphrase in a non-hit. Risk mitigation later calls this a
“human read-through” without freezing whether the complete surviving corpus is
read. Replace this with an exact source allowlist or another default-deny
construction. At minimum, exclude `docs/coordination/` and every other
assessment/process surface from the product archive before separately adding a
deterministically selected, independently reviewed operational-evidence subset.
Consider separate cutoffs for the post-forward-sync product tree and a
pre-diligence governance/activity snapshot rather than filtering a known-
contaminated corpus by vocabulary.

Also remove the residual-disclosure claim that the cutoff archive will include
“this very containment methodology once merged.” The plan does not exist at
`f36d85d0`; `git cat-file` confirms that path is absent at the cutoff.

### DD-P2 — Public-only acquisition is incomplete and not anonymous by contract

`7kjh` item 2 requires an anonymous clean public source, anonymous GitLab
issues/MRs/releases, downloadable release assets, public package/OCI metadata,
and no authenticated maintainer API. The plan makes a local `git archive` and
the public GitLab archive download interchangeable, does not forbid credentials
for Phase 1 API calls, and does not freeze an anonymous request method or
field-level export schema. Its authoring evidence was gathered with the local
GitLab client; that is useful plan research but is not proof of the later
anonymous evidence path.

Make anonymous acquisition mandatory and reproducible: public clean clone or
public SHA archive, unauthenticated paginated API/HTTP requests, exact query and
field allowlists, UTC access times, response status, and no cookies/tokens or
maintainer-only fields. Add the downloadable v2.5.0 release assets and their
published checksums/provenance, public generic-package metadata, and public OCI
manifest/digest evidence required by `7kjh`, or prove each surface unavailable
and record it as unavailable. A releases-list JSON file alone is not equivalent
to those surfaces.

### DD-P3 — The evaluator isolation claim is not executable as written

Lines 292-296 say the evaluator is given no other file or network access, but
the plan never names the spawn mechanism or an enforceable access boundary. A
new collaboration subagent with `fork_turns: "none"` prevents inherited
conversation turns; it does not by itself restrict the shared workspace,
filesystem, shell, or network tools. This repository's root and local
`ai-cold-start.md` also contain diligence reminders that must not become
evaluator context.

Freeze the exact evaluator launch envelope: no-history mechanism, model/session
provenance, exact prompt hash, evidence-root working directory, permitted tools,
and access log. If this environment cannot technically deny other filesystem
and network access, say so and characterize the restriction as procedural
rather than asserting access was not “given”; either provide a genuinely
isolated evidence-only execution environment or obtain an explicit Governor
decision accepting the weaker simulated-independence boundary. Any canary or
preflight must test both absence of inherited findings and inability (or
explicit prohibition) to reach live/current surfaces before the real evaluator
is spawned.

### DD-P4 — Consultation provenance and staffing are not truthful or
jurisdiction-aligned

The front matter marks infosec-sentinel, fact-checker, and web-ux-reviewer
`consulted`, but cites no review result from any of them. Naming a future Phase
4 pass is not a completed plan consultation. `sq5j` is an unrelated
maintenance-line decision and explicitly warns against stretching role labels;
it is not an infosec review of this evidence-package method. Obtain and cite
actual plan consultations, especially infosec review of public-source secret
handling and web-UX review of the reconstructed-mirror method, or use an honest
schema status and reason where a pre-Gate-1 consultation is not applicable.
`7kjh` explicitly requires the infosec consultation to be obtained.

Phase 1 lines 166-167 correctly name coder-tester as the package operator, but
Staffing line 402 instead makes planner-architect own its construction.
Planner-architect is forbidden from implementation, builds/tests, and
documentation-content production. Assign the archive/API/build implementation
and produced evidence artifacts to coder-tester; retain method/structure and
reconciliation design with planner-architect. Consult build-engineer if the
reproducible Pages/artifact-build boundary requires build ownership.

Finally, define the fresh evaluator as a non-authoritative commissioned
assessor, not an undeclared AGF role: it may author raw assessment artifacts but
has no Gate, role, merge, release, coordination-authority, or risk-acceptance
power. Record who imports/freezes its output without editorially changing it.

### DD-P5 — Freeze integrity and sensitive-material controls are incomplete

The manifest at lines 253-260 checksums only the repository archive. “Read
only” is not an immutable evidence identity. Require hashes for every included
file and generated tree, a canonical aggregate manifest digest, the exact
package commit/head, and fail-closed invalidation: any package or manifest
change after the contribution-auditor containment CLEAR voids the evaluator
run and requires a new freeze/review.

The sensitive-material rule currently reacts only after package construction
discovers a credential-shaped value. By then an unredacted public value may
already have been copied into the archive/export. Run the approved leak screen
before an item is admitted, quarantine the item without reproducing the value,
stop package freeze/evaluator exposure, and require infosec/human disposition.
Do not commit an evidence package that merely reports the location while still
carrying the value in an embedded tar/JSON/HTML file.

The Pages mirror records only the build command, not the build environment or
dependency identities. Freeze the Python/MkDocs/plugin versions, dependency
input, OS/architecture and relevant environment, plus generated-tree hashes.
The proposed label “as it would have rendered” overclaims equivalence while the
Risk section correctly admits the historical deployment is unproven. Label it
as a reconstruction from source under the recorded environment, not a proven
historical render.

### DD-P6 — The fixed rubric omits materiality and mishandles unavailable
evidence

`7kjh` item 8 requires materiality as well as confidence. Lines 327-330 label
the section “Materiality/confidence” but define only confidence, and the
`findings-register.json` shape at lines 344-346 has no materiality field.
Define a fixed materiality scale, threshold/rationale, and per-finding field
before evaluation.

The score contract is internally ambiguous: dimensions must receive 1-5, yet
`unavailable` means insufficient evidence rather than weak/high-risk evidence.
Do not encode unavailable evidence as a numeric 1 or another score. Freeze a
literal `N/A`/`unavailable` outcome with no numeric score, or another
unambiguous representation. Define a machine-checkable JSON schema (types,
required fields, enums, evidence-reference form, materiality, confidence) and
its validation command for the findings register.

### DD-P7 — Phase ordering invalidates the independent review

Phase 4 steps 1-3 fact-check and independently review the frozen primary
report, then step 4 creates `reconciliation.md` using the previously held
material. That changes the publication candidate after the named independent
review. Step 6 requires Gate 2 but does not require a fresh exact-head review or
fact-check of the reconciliation itself.

Use two immutable stages/MRs (primary report first, post-freeze reconciliation
second), or require a second complete exact-head contribution-auditor and
fact-checker pass after reconciliation is added while proving the evaluator's
three frozen primary outputs remain byte-identical. Reconciliation claims and
classifications also need cited evidence and review. The first report freeze
must remain the event that releases the evaluator-containment hold; later
reconciliation cannot retroactively edit it.

The plan also contradicts itself about spawn authority: Decision Requested
lines 48-50 say Gate 1 does not authorize spawning without a separate act,
while Phase 0 says Gate-1-approved Phase 1/2 may spawn after containment CLEAR.
Choose one exact authority transition and name the required Governor record if
a second act is intended.

### DD-P8 — The authoritative MR evidence overstates independence

The live MR body calls the contamination finding “central, independently
verified,” then says it was verified in the planner's own session. Before this
review there was no independent review of the plan or its finding. Replace that
with “planner-verified” (and later cite this review's exact disposition rather
than retroactively calling the original check independent). The body should
also record exact pipeline `2748195536` SUCCESS and job `15815176987` rather
than leaving the exact-head CI evidence implicit.

## Method-contract map

- Item 1, genuine discontinuity: **FAIL** — DD-P3.
- Item 2, public-only evidence: **FAIL** — DD-P1/DD-P2.
- Item 3, exact cutoff/reproducibility: **PARTIAL** — cutoff is valid; package
  identity/freeze and Pages environment fail DD-P5.
- Item 4, sensitive-material boundary: **PARTIAL** — reporting boundary is
  sound, but pre-admission containment and real consultation fail DD-P4/P5.
- Item 5, ten assessment pillars: **PASS**.
- Item 6, claim discipline: **PARTIAL** — report contract is sound; current MR
  and reconstruction claims fail DD-P5/P8.
- Item 7, simulated-consultancy/no impersonation: **PASS**.
- Item 8, fixed scoring: **FAIL** — DD-P6.
- Item 9, exact outputs: **PARTIAL** — literal paths pass; owner/format/freeze
  contracts fail DD-P4/P5/P6.
- Item 10, independent verification: **PARTIAL** — roles are named, but plan
  consultation/independence and post-reconciliation exact-head review fail
  DD-P4/P7.
- Item 11, validation/publication: **PASS prospectively**, subject to DD-P7's
  final exact-head sequence.
- Item 12, post-report reconciliation only: **PARTIAL** — scope boundary passes;
  review sequencing fails DD-P7.

## Passing evidence and current state

- Exact-head plan validation: OK, blocking severity.
- Exact-head handoff validation: OK, blocking severity.
- `git diff --check`: clean.
- Exact-head pipeline `2748195536`: SUCCESS; sole blocking job
  `15815176987` passed.
- GitLab live state: open, non-draft, mergeable, no conflicts, discussions
  resolved.
- Current-main target advance since the branch base adds only
  `20260810T172655Z-contribution-auditor-gagk.md`; synthetic merge is clean and
  does not affect the plan.
- The chosen product cutoff `f36d85d0...` exists and is an ancestor of current
  main; the next ancestry-path commit is the named `3a2b0dd7...` record.

## Required next state

Revise the plan and live MR body for DD-P1 through DD-P8, obtain the actual
consultations, push one coherent new head, run a fresh exact-head pipeline, and
request a fresh contribution-auditor review. Do not build the evidence package,
spawn/brief the evaluator, send the Gemini prompt, merge !1382/!1383, or expose
held findings under this blocked plan.

No candidate file, MR description, merge hold, or evaluator state was mutated
by this review.
