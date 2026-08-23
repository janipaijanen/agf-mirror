---
id: review-topology-declaration
status: pending-gate1
topic: review-topology-declaration
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Requesting independent contribution-auditor review before Gate 1.
      I (planner-architect) author this plan. The originating issue
      (#82, "mirror-review" evidence) was triaged on 2026-07-28 with a
      routing note directing its content toward issues #45/#47/#49/#77
      — this plan does not follow that routing (see "Why this plan does
      not wait on #45/#47/#49/#77" below), which is itself a scope
      decision worth an independent second look.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Documentation-only change to two adopter-facing Pages pages (the
      AI Evaluation Dossier and the philosophical-lenses concept page).
      No code, credential, or security-surface change.
---

# Plan: require review-topology declaration and a source-identity preflight in external AI review guidance

## Round 1 correction (fixes contribution-auditor's exact-head review, `pssr`)

`20260823T205558Z-contribution-auditor-pssr.md` reviewed this plan
alongside three others and recommended BLOCK, 2 material findings + 1
advisory, on this plan specifically. Verified and fixed:

- **`CA-LRB-TOP-1`** — the original design let "blinded independent
  review" stand as if "did not see another reviewer's answer" were
  sufficient for independence. It is not: two reviews can still share
  the same implementing session, prompt, supplied framing, evidence
  selection, or model lineage/vendor while neither has literally seen
  the other's output. Calling that "independent" risks recreating the
  exact overclaim this plan exists to prevent. Fixed: the field is
  renamed `blinded-to-prior-answers` (narrower, accurate label for what
  is actually being tested), and a separate, explicit independence-
  disclosure requirement is added naming the material dimensions
  (session, prompt/framing, model family/vendor, evidence selection).
  Terminology deliberately kept distinct from issue #47's own "record
  reviewer independence in the artifact" effort rather than silently
  defining a second, weaker framework-wide meaning for "independence" —
  see the revised Design §2 below.
- **`CA-LRB-TOP-2`** — the original design said blinded reviews "count
  as independent cross-model corroboration for the framework's own
  evidence-confidence levels," which conflates two different things:
  the External Review template's own text already says it is *not* an
  adopter Learning Record, and `learning.schema.json`'s confidence
  levels (`single-adopter`/`multi-adopter`/`longitudinal`/`industry-
  corroborated`) describe adopter/event provenance, not model count
  within one review exercise. As written, the plan could be read as
  license to promote a single adopter's evidence to `multi-adopter` by
  running several models in one sitting. Fixed: Design §2 now states
  explicitly that review topology never changes which confidence level
  an adopter/event's evidence sits at and does not by itself increment
  `confidence.evidence_count` — it is orthogonal metadata about *how*
  a review was conducted, not a lever on adopter-evidence strength. A
  separate cross-model-corroboration contract, if wanted, is out of
  scope for this plan and would need its own schema/invariant change.
- **`CA-LRB-TOP-A1`** (advisory) — "required" overstated a documentation-
  only, unenforced field as a guarantee. Fixed: "What this plan does
  NOT do" now states plainly that omission is detectable only by manual
  review (Level 2 guidance, per `implementation-honesty-rule.md`'s
  maturity model), not a validated or universally-present declaration.

Implements the actionable portion of GitLab issue
[#82](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/82)
("mirror-review" evidence record, filed 2026-07-28).

## Why this plan does not wait on #45/#47/#49/#77

#82 was triaged same-day with a disposition comment routing "review
topology/source-identity work through #47, #49, and #77" and "nominal
accountability as a Theoretical candidate under #45." Checked directly,
not assumed: all four target issues (#45, #47, #49, #77) are themselves
in `release::unmilestoned-backlog` with no plan, same as #82 was. The
routing moved the stall, it did not resolve it — nearly a month later,
none of the four has produced movement either. #47/#49/#77 are also
substantially broader in scope than #82's actual ask (#47 is "record
reviewer independence in the artifact" generally across the whole
framework; #49 is "instruction files/skills/MCP servers as a governed
supply chain"; #77 is "cross-repository scientific and promotional
intake" — each a larger design effort in its own right).

This plan implements #82's own first entry directly and narrowly
(review-topology declaration + source-identity preflight for external
AI review specifically), rather than waiting for the larger #47/#49/#77
efforts to eventually subsume it. If and when #47/#49/#77 are
implemented, they can supersede or fold in this narrower mechanism —
that is a future compatibility question for whoever picks those up, not
a reason to leave #82's concrete, boundable ask stalled indefinitely.

#82's second entry (the "Nominal accountability" philosophical-lens
corollary) is correctly left at Theoretical status under #45, per the
issue's own explicit request ("keep the proposed nominal-accountability
material at Theoretical status unless later operational evidence
justifies promotion") — that is not a stall, that is the evidence bar
working as designed, and this plan does not implement it.

## The problem

A sequential external-AI review (three AI services reviewing in
sequence, each seeing the prior model's answer, ranking, and critique)
was mistaken for independent cross-model corroboration. It was not:
later reviewers tended to accept the supplied framing, rationalise
assigned strengths and weaknesses, and repeat factual claims until
required to re-fetch the exact release and canonical source pages. One
review round confused the legacy `/agent-governance/` documentation
lineage with the current `/agents-govern-framework/` lineage and
presented the resulting vocabulary discrepancy as a correction — a
mistake only exact release/source identity checking caught.

The AI Evaluation Dossier (`agf/docs/site/ai-evaluation-dossier.md`)
already has an "Identity Check" section (canonical repository,
published docs, current version) and an "Output Format" section
requiring "Identity confirmed" as a checkbox — but neither section
distinguishes *how* the review was conducted, so a sequential-critique
review and a genuinely independent review currently produce
indistinguishable output, and nothing prompts a reviewer to check for a
legacy-vs-current documentation lineage split specifically.

## Design

### 1. Extend "Identity Check" with source-lineage handling

Add one sentence to the existing "Identity Check" section (line ~64):
when a project has both a legacy and a current documentation lineage
(as this one does — legacy `/agent-governance/`, current
`/agents-govern-framework/`), state explicitly which lineage is
canonical for the version under review, and require the reviewer to
confirm they are not citing the other lineage's vocabulary or structure
as authoritative.

### 2. New required fields in "Output Format": review topology + independence disclosure (revised, fixes `CA-LRB-TOP-1`, `CA-LRB-TOP-2`)

Add two required fields to the existing "Output Format" section's
bullet list (alongside "Scope reviewed" and "Identity confirmed"):

- **Review topology** — declare one of: **blinded-to-prior-answers**
  (has not seen another reviewer's answer, ranking, or critique —
  deliberately *not* labelled "independent," since not having seen a
  prior answer is necessary but not sufficient for independence),
  **sequential critique** (responds to an earlier answer or
  assessment), or **adjudicated synthesis** (compares competing answers
  against identified primary sources).
- **Independence disclosure** (required whenever `blinded-to-prior-
  answers` is declared) — state, for each of these dimensions, whether
  it was shared with any other review in the same exercise: implementing
  session, prompt/framing text, model family or vendor, and evidence
  selection (which pages/files were provided vs. independently
  retrieved). A review sharing any of these is not independent on that
  dimension, regardless of its topology label.

**This field is orthogonal to adopter evidence strength, not a lever on
it (fixes `CA-LRB-TOP-2`).** The External Review issue template already
states it produces something distinct from an adopter Learning Record.
Review topology and the independence disclosure describe *how a given
review was conducted*; they never change which `confidence` level
(`single-adopter` / `multi-adopter` / `longitudinal` / `industry-
corroborated`, per `agf/schemas/learning.schema.json`) an adopter's or
event's own evidence sits at, and declaring `blinded-to-prior-answers`
does **not** by itself increment a learning record's
`confidence.evidence_count`. Running several models against the same
material in one exercise is one event, not multiple adopters or
multiple longitudinal observations, however many models were involved
or how independently each was blinded. A genuine cross-model-
corroboration contract that *does* affect adopter confidence levels is
out of scope for this plan and would need its own schema/invariant
change, not an informal attachment here — and terminology here is kept
deliberately distinct from issue #47's own "record reviewer
independence in the artifact" effort, which is a separate, broader
mechanism this plan does not attempt to pre-empt or redefine.

### 3. `External Review` issue template: add the same field

`.gitlab/issue_templates/External Review.md` gets the same "Review
topology" field added, so a reviewer filing directly from that template
(the path the dossier's own "Submit Your Review" section points to)
declares it at submission time, not only when following the dossier
prose.

## What this plan does NOT do

- Does not implement the "Nominal accountability" philosophical-lens
  corollary (stays Theoretical under #45, correctly, per the issue's
  own request).
- Does not implement #47 (recording reviewer independence in every
  artifact, framework-wide), #49 (governed instruction-file/skill/MCP
  supply chain), or #77 (cross-repository intake design) — all three
  remain open, larger efforts, unaffected by this plan.
- Does not add any mechanical validator enforcing the review-topology
  declaration — this is guidance for a human/external reviewer filling
  out a template, not a machine-checked artifact field. A future
  learning record's own schema-level `confidence` fields already exist
  for evidence-count enforcement; this plan does not add new schema.
  **Correction (round 1, fixes `CA-LRB-TOP-A1`):** because nothing
  mechanically enforces it, an omitted declaration is detectable only
  by manual review, not guaranteed present — this is Level 2
  (Surface-defined) guidance per `implementation-honesty-rule.md`'s
  maturity model, not a validated or universally-enforced contract.
  Do not describe the field elsewhere as "required" in a way that
  implies mechanical enforcement.
- Does not retroactively correct any past external review that lacked
  this declaration.

## Files expected to change

This plan's own diff (Gate 1 candidate): only this plan file.

On the implementation branch, after Gate 1 approval:

- `agf/docs/site/ai-evaluation-dossier.md` — "Identity Check"
  source-lineage sentence; "Output Format" Review-topology field.
- `.gitlab/issue_templates/External Review.md` — matching field.

No code, no schema, no validator, no CI change.

## Test strategy

Documentation-only; no unit tests apply.

- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella): no new findings beyond pre-existing,
  unrelated warnings.
- Manual check: render the dossier page locally (or via the docs build,
  if run as part of CI for `agf/docs/site/**` changes) and confirm the
  new field reads clearly in context, not as a disconnected insertion.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan does not touch a listed trigger path in the current
`downstream-impact-rule.md` (`agf/rules/**/*.md`, `agf/agents/*.yaml`,
`tooling/rust/`, `agf/schemas/*.json`, `agf/templates/`, `agf/dist/`,
`agf/docs/site/reference/environment-variables.md`,
`docs/coordination/agov-config.yaml`, `AGENTS.md` — checked directly
against the rule's current text, not assumed from an earlier cached
reading). `agf/docs/site/ai-evaluation-dossier.md` and
`.gitlab/issue_templates/External Review.md` are adopter/reviewer-
facing but neither is a listed trigger path. Documenting the
classification here for transparency even though the rule's own screen
does not fire:

```
Impact:
  Foundry:          EDITORIAL/ADDITIVE. Guidance for external reviewers
                     gains a new required declaration; no existing
                     obligation is removed or narrowed.
  Direct adopters:  ADDITIVE. Adopters running their own external
                     reviews gain clearer guidance; nothing required of
                     adopters who do not solicit external AI review.
  Downstream:       No change.
  Migration:        Not required (ADDITIVE, not BREAKING).
  Evidence:         GitLab issue #82 (mirror-review, 2026-07-28), one
                     external review exercise, single-adopter evidence
                     per the issue's own "Evidence-strength statement."
```

## Rollback

Pure documentation addition to two files. Reverting is a one-commit
revert with no schema, data, or code implications.
