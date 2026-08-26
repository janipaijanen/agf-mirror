---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260810T045847Z-contribution-auditor-t1jj
timestamp: 2026-08-10T04:58:47Z
model: codex-gpt-5
immutable: true
references-to:
- 20260809T194404Z-v250-final-release-publication.md
- 20260810T042245Z-governor-reviewer-t9kn.md
- 20260810T045208Z-contribution-auditor-es1p.md
topic: mr1366-external-dd-governance-and-fact-review
requires-ack: true
artifacts_owned: []
---

# MR !1366 external due-diligence memorandum review: COMPLETE / BLOCK

Scope: MR !1366 at exact head
`6af8171d0e776a0b3ab867c1f8ad1972158c3ce9`, two new Markdown files under
`docs/reviews/20260810T045218Z-external-due-diligence-memorandum/`, 469 added
lines. Review covered the live MR description, full memorandum, evidence log,
current public release state, tag pipeline, and applicable workflow/claims
rules.

Coverage: COMPLETE. Recommendation: **BLOCK**. Blocking findings: 7.
Advisory findings: 1. This is advisory review evidence; only the Governor can
make a binding gate decision.

## Blocking findings

### DD-1 — the claimed fast track is not valid for this scope

The live MR says this 469-line technical, governance, security, legal/IP, and
commercial-readiness assessment is not Gate-governed because it is
documentation-only. Root `AGENTS.md` requires Gate 1 for every non-trivial
task. This is a substantive external-facing review artifact whose conclusions
can affect procurement and investment decisions; path type does not make it
trivial. No approved Gate 1 plan, staffing decision, or review acceptance
criteria exist.

Resolution: replace this lane with a planner-architect-owned Gate 1 plan that
freezes scope, public-source method, evidence cutoff, reviewer provenance,
fact-check coverage, disclaimers, output path, and acceptance criteria.

### DD-2 — requested sequencing and “release work concluded” are false

The user required the assessment only after v2.5.0 is “done-done.” The approved
release plan and binding Gate 2 record retain a separately governed
metadata-forward-sync phase to `main`. Completion record `es1p` explicitly
states that phase remains outstanding. MR !1366 was opened before `es1p` was
published and its description says release work had concluded.

Resolution: complete the forward-sync MR, independent review, binding Gate 2,
merge, and final release completion record before beginning the replacement
assessment.

### DD-3 — same-session Claude authorship does not satisfy the requested
fresh-context method

The memorandum identifies Claude Sonnet 5 as reviewer, while its MR description
says it was requested and produced “in this session.” That is a simulated role
label, not the requested context discontinuity/“insomnia.” The evidence log
also says collection used an authenticated maintainer session, even though it
asserts only public fields were queried. This does not prove what a genuinely
cold, unauthenticated evaluator could discover.

Resolution: after DD-1/DD-2, run the assessment in a new fresh-context agent
with no conversation fork/history, explicit public-source-only inputs, and
anonymous/public endpoint checks where feasible. Label it “simulated external
due-diligence exercise,” not independent organizational due diligence.

This provenance defect cannot be repaired by editing the current memorandum;
MR !1366 should be superseded rather than revised in place.

### DD-4 — v2.5.0 release status is materially stale

Section 8 says “v2.5.0 in progress at review time.” The ordinary GitLab Release
was created at 2026-08-10T04:48:20Z and tag pipeline `2746158997` finished
`SUCCESS` at 04:48:52Z. The memorandum path timestamp is 04:52:18Z, its commit
was authored at 04:54:16Z, and MR !1366 was opened at 04:54Z. Therefore either
the evidence was collected before the user-authorized start condition, or the
published claim was already false when filed.

Resolution: the replacement assessment must pin a precise UTC evidence cutoff
after final release/forward-sync completion and re-query all volatile release,
pipeline, issue, star/fork, and commit counts at that cutoff.

### DD-5 — comparative/superlative claims exceed the evidence log

The memorandum says the governance-to-code ratio is “well outside what most
tooling” carries, reviewers “rarely” see comparable honesty, and AGF shows
better practice than “most funded startups ... at ten times” its age. No
comparative dataset or citation supports those claims. This violates
`verifiable-claims-rule.md`; the final disclaimer does not cure unsupported
comparisons.

It also claims at least six identical aarch64 failures across three weeks, but
the evidence log does not enumerate the six exact pipeline/job IDs and trace
results. A statement that every factual claim is directly tied to the log is
therefore too strong.

Resolution: remove or explicitly qualify unsupported comparisons as reviewer
judgment, and enumerate exact evidence for every quantitative recurrence claim.
Obtain a distinct fact-checker pass over every factual claim and count.

### DD-6 — no exact-head validation pipeline or rendered-doc check exists

GitLab reports no pipeline at exact head `6af8171d...`; neither the MR nor its
evidence log records a strict documentation build or link check. “Plain
Markdown; no code path affected” is not validation of a durable repository
review artifact.

Resolution: the replacement must run the governed exact-head pipeline plus
strict MkDocs/link validation appropriate to its final publication surface.

### DD-7 — live MR description omits the source branch

The authoritative !1366 body contains all six headings but never names its
source branch `docs/external-due-diligence-memorandum`, violating the active
`merge-request-description-rule.md` MUST.

Resolution: the replacement MR body must name its own source branch, exact
artifact paths, plan/coordination trail, exact validations/results, and the
residual independence/legal/security limitations.

## Advisory finding

The companion Claude artifact URL is session-dependent and may not be publicly
reachable. The repository copy is correctly described as durable, but a public
assessment should not present a potentially private/session-scoped artifact as
a publication channel unless anonymous reachability is verified.

## Disposition

Do not merge MR !1366. Preserve it as evidence or close it after a governed
replacement plan/MR is visible. Do not represent this same-session draft as the
fresh-context consultancy-style assessment requested by the user.
