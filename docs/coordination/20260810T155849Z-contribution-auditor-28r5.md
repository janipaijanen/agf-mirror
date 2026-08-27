---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260810T155849Z-contribution-auditor-28r5
timestamp: 2026-08-10T15:58:49Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260810T154213Z-coder-tester-4psk
  - 20260810T150120Z-contribution-auditor-rp7m
  - 20260810T052006Z-contribution-auditor-7kjh
  - 20260810T144155Z-contribution-auditor-4q71
topic: mr1382-mr1383-corrected-head-review
requires-ack: false
artifacts_owned:
  - docs/coordination/20260810T155849Z-contribution-auditor-28r5.md
---

# MR !1382 corrected-head and MR !1383 first-pass review

Coverage: COMPLETE for both exact heads.

Authority: advisory contribution-auditor review only. The Governor retains
sole Gate 2 and merge authority.

The v2.5.0 release and forward-sync are complete, but the fresh-context
due-diligence plan is still unwritten. That leaves a separate evidence-
containment dependency: neither candidate may become part of the public source
tree used by the evaluator before the plan freezes a pre-candidate source
cutoff or the evaluator freezes its own report.

## MR !1382 — COMPLETE / BLOCK

Scope: MR !1382 exact head
`80655d514567879cc5eb8047c803115ba747838a`, merge-base
`eb1690651d5a75e76a40430cbd0ef291988259b5`, reviewed against
`origin/main` `a469c50f2af08f700eb13549733d71c2bb72d1e3`.

Recommendation: BLOCK.

Prior findings: B-DD-FILE-2, B-DD-FILE-3, and B-DD-FILE-4 are resolved.
B-DD-FILE-1 is only partially resolved. One new containment blocker is open.

### B-DD-FILE-1 — unsupported independence wording remains

Check: provenance-and-review-topology

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: verifiable-claims-rule and `rp7m`'s accepted resolution condition

Finding state: open

Risk disposition: none

The README now preserves the exact prompt and candidly discloses the missing
timestamp, response identifier, model/mode, and mutual-blinding attestation.
That correctly limits the supported characterization to separately prompted,
cross-vendor review with incomplete context provenance. However, filer-authored
surfaces still make the stronger claim that the correction says it removed:

- the candidate commit subject says “two independent external AI” reports;
- `k5jd`'s heading says “Two independent external AI” assessments;
- `k5jd` and the README say the Governor “independently prompted” them;
- both raw-report wrapper headers say “in response to an independent prompt”;
- the README categorically says neither system had access to the internal
  session/MRs, while also acknowledging there is no mutual-blinding or
  product-context attestation.

The self-description “Independent” inside each verbatim response may remain as
raw evidence, provided the wrapper explicitly labels it as the model's own
unverified characterization. The project-authored commit, wrapper, README, and
coordination language must not repeat it as established provenance.

Resolution condition: amend every project-authored surface to the supported
wording, for example “separately prompted, cross-vendor reports relayed by the
Governor; session/model/blinding metadata unavailable.” Replace categorical
no-context-access claims with what is observable or human-attested. Preserve
the raw responses verbatim and distinguish their self-labels from project
findings. Rerun exact-head CI and request a fresh review.

### B-DD-FILE-5 — merging now defeats the promised evaluator blinding

Check: fresh-context-containment

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `7kjh` method items 1, 2, and 12; `rp7m` containment disposition

Finding state: open

Risk disposition: none

The candidate says the reports must be hidden from the fresh evaluator, but
also says no sequencing hold remains and requests ordinary immediate Gate 2.
Those statements cannot both be true while the evaluator is required to
inspect an anonymous clone of public `main`: merging !1382 would put the two
reports and their synthesis in that public source before the fresh-context
plan has even frozen its evidence SHA.

Resolution condition: retain an explicit merge hold until either (a) the
fresh-context evaluator freezes its report, or (b) an approved Gate 1 plan
freezes an exact pre-!1382 source SHA and a Pages/repository evidence procedure
that cannot expose this candidate. Correct the MR body and `k5jd` gate-status
language. Gate 2 content review may proceed, but merge/publication to `main`
must respect the chosen containment proof.

## MR !1383 — COMPLETE / BLOCK

Scope: MR !1383 exact head
`11e483658be6569f4203d786e0d574c8c9bab3eb`, merge-base
`eb1690651d5a75e76a40430cbd0ef291988259b5`, reviewed against
`origin/main` `a469c50f2af08f700eb13549733d71c2bb72d1e3`.

Recommendation: BLOCK.

### B-DD-LEARN-1 — a placeholder is not the adopter's codename

Check: learning-anonymization-and-identity

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: external-anonymization-rule, `ADOPTING.md` codename intake contract,
and learning.schema.json's mandatory-pseudonym semantics

Finding state: open

Risk disposition: none

`pending-codename-20260810a` explicitly says its filename and
`project.codename` value are not the adopter's chosen or stable pseudonym.
The public intake contract requires the adopter's codename as the only project
handle crossing the codebase boundary; a validator accepting any non-empty
string does not prove that semantic requirement.

The proposed future repair is also invalid: `supersedes` relates learning
entries within the same file, and the validator requires the referenced entry
to exist earlier in that file. It is not a file-rename pointer and cannot
legitimately supersede an entry stranded under another project file.

Resolution condition: before merge, obtain the submitter's codename and amend
the unpublished candidate so filename and `project.codename` agree, or remove
the unnamed submission and every two-adopter/convergence claim from this MR.
Do not intentionally publish a placeholder to create a correction obligation.

### B-DD-LEARN-2 — independent-adopter provenance is not durably established

Check: learning-provenance-and-corroboration

Check status: reviewed

Outcome: needs-review

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: verifiable-claims-rule and the project's evidence-status discipline

Finding state: open

Risk disposition: none

The files and `sqkg` repeatedly describe “two independent real adopters” and
“meaningful corroboration.” The durable candidate evidence is a coder-tester
statement that the Governor received two emails. It includes no Governor
attestation that these are distinct external adopter projects or actors and
does not distinguish submitter-reported command results from independently
reproduced evidence. Confidential email content need not be published, but the
strong public provenance claim still needs a safe authoritative basis.

Resolution condition: obtain a codename-safe Governor record attesting what is
actually known: separate submissions, whether the projects/actors are distinct
and external to the framework authors, and which facts are submitter-reported.
Otherwise narrow all language to “two learning snapshots relayed by the
Governor,” avoid independent-adopter/corroboration claims, and label unarchived
command outputs as submitter-reported. Keep each entry at `single-adopter`;
do not infer multi-adopter or longitudinal confidence from counts of commands.

### B-DD-LEARN-3 — public merge conflicts with the blinded evidence cutoff

Check: fresh-context-containment

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `7kjh` method items 1, 2, and 12

Finding state: open

Risk disposition: none

The MR correctly observes that these learning records directly affect the
fresh evaluator's “external traction” question and says they are held out of
the evaluator's input. It then removes the sequencing hold and requests an
ordinary merge. Publishing the records on `main` before the evaluator's source
cutoff is frozen would make them discoverable in the exact public source the
evaluator must inspect.

Resolution condition: use the same containment choice as B-DD-FILE-5: hold
merge until the evaluator freezes its report, or first approve a plan that
pins a pre-candidate source SHA and prevents later public-source material from
entering the evidence set. Correct the MR body and `sqkg` sequencing claims.

## Evidence and live state

- Both MRs are open, non-draft, target `main`, conflict-free, discussions
  resolved, and GitLab reports `mergeable`.
- !1382 exact-head pipeline `2747886372` is SUCCESS; its sole blocking job
  `15813011284` passed.
- !1383 exact-head pipeline `2747892481` is SUCCESS; all five non-allow-failure
  jobs `15813061208` through `15813061212` passed.
- Current target advance from each merge-base is only `4psk`, the review
  request itself; it touches neither candidate. Synthetic merges succeed and
  `git diff --check` is clean.
- Current-schema `agov validate` passes on each exact-head worktree with only
  pre-existing warnings. Focused coordination validation passes for `k5jd`
  and `sqkg`; !1383's learning and infosec validators pass mechanically.
- Mechanical/schema success does not resolve unsupported provenance,
  codename semantics, or the public-source containment contradiction.

No candidate file, commit, MR description, branch, or Gate state was mutated
by this review.
