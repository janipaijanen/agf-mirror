---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260810T052006Z-contribution-auditor-7kjh
timestamp: 2026-08-10T05:20:06Z
model: Codex GPT-5
immutable: true
references-to:
  - "20260810T045847Z-contribution-auditor-t1jj"
  - "20260810T050834Z-coder-tester-h8mz"
  - "20260810T051051Z-coder-tester-p3vk"
  - "20260810T051437Z-contribution-auditor-5krd"
topic: fresh-context-due-diligence-gate1-planning
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "After the v2.5.0 metadata-forward-sync completion record is visible on main, author a new planner-owned Gate 1 plan for the genuine fresh-context public due-diligence assessment."
    - "Submit the plan for contribution-auditor review and a fresh binding Governor Gate 1 decision before any assessment begins."
  must_read:
    - "docs/coordination/20260810T045847Z-contribution-auditor-t1jj.md"
    - "docs/coordination/20260810T050834Z-coder-tester-h8mz.md"
    - "docs/coordination/20260810T052005Z-contribution-auditor-h7h4.md"
    - "agf/rules/always/verifiable-claims-rule.md"
    - "agf/rules/workflow/merge-request-description-rule.md"
    - "agf/docs/site/reference/maturity-vocabulary.md"
    - "agf/docs/site/reference/review-findings.md"
  do_not_touch:
    - ".claude/settings.local.json"
    - "refs/tags/v2.5.0"
    - "release/2.5"
    - "release-2.5.0"
---

# Planning commission

The human Governor has instructed Codex to proceed with the previously
proposed genuine simulated-insomnia due-diligence method. The assessment may
start only after the mandatory v2.5.0 forward-sync is merged, its completion
record is visible, this new plan is independently reviewed, and the Governor
records a binding Gate 1 approval.

MR !1366 remains a same-session draft and must not be edited into the
replacement. Preserve it as process evidence until the replacement is visible;
then its owner may close it as superseded.

# Required method contract

The plan must freeze all of the following:

1. **Genuine context discontinuity.** The primary evaluator is a newly spawned
   agent/session with no conversation history or inherited turns. Its prompt
   contains only the approved method, public URLs, exact evidence cutoff, and
   output contract. It must not receive MR !1366, `p3vk`, their findings, or
   private session summaries before freezing its own report.
2. **Public-only evidence.** Use an anonymous clean clone of the public source
   at an exact `main` SHA after forward-sync, public Pages, anonymous GitLab
   issues/MRs/releases, downloadable release assets, public package/OCI
   metadata, and other cited public primary sources. Do not use authenticated
   maintainer APIs, local credentials, private conversation, or uncommitted
   workspace state.
3. **Exact cutoff and reproducibility.** Record UTC cutoff, commit/tag/release
   identities, URL, access time, command/query, extracted fact, source class,
   and confidence for every material claim. Archive only lawful, non-secret
   evidence needed for reproducibility.
4. **Safe sensitive-material boundary.** If public source appears to expose a
   credential or sensitive value, report only class and location, reproduce no
   value in the report or coordination, and route immediately to
   infosec-sentinel and governor-reviewer.
5. **Assessment pillars.** Cover product and architecture; code quality and
   test evidence; governance design versus enforceability; role independence
   and key-person concentration; security and software supply chain; licensing
   and IP clarity; release/operations; documentation and public-page UX;
   adoption/market/commercial evidence; and material risks, mitigations, and
   30/90/180-day recommendations. State unavailable evidence rather than
   guessing financial, customer, legal, or security facts.
6. **Claim discipline.** Separate observed fact, inference, recommendation,
   and unavailable evidence. Enumerate evidence behind every count. Cite or
   hedge every comparison/superlative. Apply AGF capability maturity and
   evidence-status vocabulary where relevant.
7. **Consultancy-style, no impersonation.** The document may use a disciplined
   professional-services due-diligence structure, but must say it is a
   simulated external assessment, not an audit, assurance engagement,
   penetration test, legal opinion, valuation, or work by/affiliated with KPMG,
   EY, McKinsey, or any other firm.
8. **Fixed scoring rubric.** Define dimensions, scale, weights (if any),
   evidence thresholds, confidence, and materiality before evaluation. Do not
   turn the score into an assurance claim or conceal unknowns in an average.
9. **Exact durable outputs.** Reserve literal, timestamped paths under
   `docs/reviews/` for the report, evidence log, and machine-readable finding
   register; name their owners and formats. No `<timestamp>` placeholders may
   remain at Gate 1.
10. **Independent verification.** After the evaluator freezes its report,
    activate fact-checker for every factual, quantitative, legal/licensing,
    comparative, market, and release claim; use web-ux-reviewer for the public
    Pages assessment; use contribution-auditor for complete exact-head method,
    evidence, and scope review. The suspended independent-implementation-
    reviewer role is not a dependency.
11. **Validation and publication.** Require strict docs build/link checks,
    repository validation, `git diff --check`, exact-head pipeline, governed MR
    description, and binding Governor Gate 2 before merge. The assessment
    author cannot supply its own independent review.
12. **Post-report reconciliation only.** After the independent report and
    fact-check freeze, compare it with MR !1366/`p3vk`. Classify each candidate
    as confirmed, narrowed, rejected, or not assessed. Convert only confirmed
    and Governor-selected remediations into bounded child plans. v2.5.1 may
    include only child work with independent Gate 1/Gate 2 authority; no report
    finding self-authorizes implementation.

# Required plan consultations

Declare and obtain the schema-required infosec-sentinel and
contribution-auditor consultations. Also activate fact-checker and
web-ux-reviewer in the staffing/method sections. Documentation-writer status
must be explicitly reconsidered because the output is substantial public
prose; if it remains skipped, give a bounded reason and assign report ownership
without violating role jurisdiction.

# Requested next visible state

Once forward-sync completion is durable, publish a planner-owned plan branch,
literal output paths, full six-section MR description, validated plan, and an
exact-head handoff to contribution-auditor and governor-reviewer. Do not start
the fresh evaluator before Gate 1.

