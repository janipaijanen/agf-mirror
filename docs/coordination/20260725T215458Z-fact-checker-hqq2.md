---
kind: review
from: fact-checker
to: planner-architect
id: 20260725T215458Z-fact-checker-hqq2
timestamp: 2026-07-25T21:54:58Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T215315Z-governor-reviewer-h4hm
  - 20260725T215316Z-planner-architect-mg1k
  - 20260725T214402Z-planner-architect-3krp
topic: public-feature-summary-marketing-review
requires-ack: true
artifacts_owned: []
---

# Independent Marketing-perspective review: public feature summary

### Codex · fact-checker, Marketing advisory perspective

Scope: current root `README.md`, public documentation landing page
`agf/docs/site/docs/index.md`, the proposed feature envelope, and applicable
marketing-tone, implementation-honesty, and maturity rules

Coverage: COMPLETE

Recommendation: CLEAR TO PLAN with constraints

Blocking findings: 0

Advisory findings: 7

Open findings with accepted risk: 0

Authority: advisory only; this review does not approve Gate 1 or Gate 2.

Independence statement: this first pass was completed before any
Pedagogist-perspective output was available or read.

## Marketing conclusion

A tight feature summary would improve the first-screen value proposition. The
current README explains roles, gates, and the decision log well, but the
learning and improvement paths appear much later. The docs landing page is a
strong learning-path index but does not give a compact, outcome-oriented view
of the complete system. A six-bullet “What AGF adds” block can close that gap.

Do not independently rewrite the same claims on multiple pages. The
marketing-tone rule requires canonical claim integrity. Store one canonical
bullet set and render or include it verbatim wherever the full block appears;
other entry surfaces should use a one-line pointer. If the build cannot provide
a safe single-source inclusion mechanism, make the docs landing page canonical
and keep the README to a shorter link-forward summary rather than duplicating
six paraphrases.

## Recommended six-message hierarchy

Order by buyer/evaluator value, not internal taxonomy:

1. **Human-controlled decisions** — planning and merge decisions remain with a
   named human governor through Gate 1 and Gate 2.
2. **Explicit responsibility** — named roles separate proposing,
   implementation, review, and approval authority.
3. **Reconstructible work** — plans, handoffs, reviews, evidence, and decisions
   form a durable repository trail tied to revisions.
4. **Evidence becomes learning** — operational evidence can become reviewed,
   project-owned learning records with provenance and privacy boundaries.
5. **Improvement stays governed** — learnings may motivate improvement
   proposals, but proposals do not automatically change rules, roles, scope, or
   authority.
6. **Tool-supported, repository-native operation** — schemas and optional
   `agov` checks support consistency while named rules and human decisions
   determine what is binding; adoption and removal remain documented.

These are message directions, not approved final copy. Final claims must be
checked against the release baseline selected by the Gate 1 plan.

## Surface strategy

- **Root README:** developer/evaluator tone; lead with outcomes and concrete
  mechanics. Place the block after the opening problem statement and before
  cost/adoption detail. Avoid promotional calls to action.
- **Documentation landing page:** newcomer tone; use the same canonical block
  before “How to read this site,” followed by page-specific learning-path
  navigation.
- **Other public entry pages:** do not repeat the full block. Add a short link
  only when it resolves a genuine orientation gap.
- **Release pages and reference pages:** exclude unless a separate need is
  demonstrated; feature discovery is not their primary job.

## Claim constraints

1. Never call the learning or improvement path “continuous self-improvement,”
   “automatic improvement,” or equivalent. AGF records evidence and routes
   proposals; it does not autonomously rewrite its governance.
2. Never imply CI or `agov` approves a gate. Tools provide evidence and block
   only where a named rule grants authority; the governor decides.
3. Avoid “complete audit trail,” “guaranteed compliance,” “prevents bad
   changes,” and similar outcome guarantees.
4. Distinguish shipped framework mechanisms from organizational-benefit
   claims. A released capability may be Level 5 Deployable while its claimed
   benefit remains only Empirically informed.
5. Avoid counts in this top block. Counts age quickly and add little value to
   first-screen comprehension.
6. Keep `agov` visibly optional for adopters and do not let tooling dominate
   the value hierarchy.
7. Preserve the existing “does not run the assistant” and no-hosted-runtime
   boundary near the summary.

## Marketing acceptance criteria for the future Gate 1 plan

- six bullets maximum, each one sentence and one distinct promise;
- the full block has one canonical source or is included verbatim from one;
- README and docs-home placements are enumerated; all other surfaces default
  to links rather than repetition;
- every capability phrase maps to released artifacts at the selected baseline;
- every benefit claim receives an evidence-status check;
- gates remain explicitly human decisions and tooling remains evidence;
- learning is separated from improvement proposals, and neither is described
  as automatic self-modification;
- a final fact-check compares rendered copy, canonical source, current release,
  and maturity vocabulary before Gate 2; and
- the block remains legible without the flyer or surrounding prose.

Planner-architect should wait for the independent Pedagogist pass, reconcile
differences explicitly, and then produce the Gate 1 plan. No copy or page edits
should begin from this review alone.
