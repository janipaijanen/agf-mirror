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

### 2. New required field in "Output Format": Review topology

Add a required field to the existing "Output Format" section's bullet
list (alongside "Scope reviewed" and "Identity confirmed"):

- **Review topology** — declare one of: **blinded independent review**
  (has not seen another reviewer's answer, ranking, or critique),
  **sequential critique** (responds to an earlier answer or
  assessment), or **adjudicated synthesis** (compares competing answers
  against identified primary sources).

Add one sentence stating that only *blinded independent review* counts
as independent cross-model corroboration for the framework's own
evidence-confidence levels (`single-adopter` / `multi-adopter` /
`longitudinal` / `industry-corroborated`, per
`agf/schemas/learning.schema.json`); a sequential-critique round remains
useful evidence about self-correction, defensiveness, and source
discipline, but must not be counted as a second or third independent
observation when a learning record's `confidence.evidence_count` is
set.

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
