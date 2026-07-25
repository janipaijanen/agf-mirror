---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260721T205437Z-contribution-auditor-hhkg
timestamp: 2026-07-21T20:54:37Z
model: gpt-5.6-sol
immutable: true
references-to:
- 20260721T205000Z-governor-reviewer-nqjs.md
topic: post-release-public-content-marketing-audit
requires-ack: true
artifacts_owned: []
---

# Review: Codex public-content marketing audit

## Workflow and scope

Three independent read-only reviewers inspected the root `README.md`, public
homepage/top-level pages, documentation tree, and MkDocs navigation. Their
lenses were: adopter conversion and positioning; technical credibility and
evidence claims; and information architecture/evaluator journey. Each reviewer
ranked findings, cited files and lines, and proposed a target state. The
contribution-auditor then checked the convergent claims against the current
files and consolidated them below.

No remediation is authorized before v2.4.0. These findings are inputs to a
post-release Gate 1 plan, not current-release scope.

## BLOCK — factual and trust defects

1. **Release-sensitive examples are already stale.** `README.md:164`,
   `docs/adopt/install-and-layouts.md:242,246`,
   `docs/adopt/tooling-and-tokens.md:85`, and
   `docs/advanced/contracts-and-validators.md:113,153` prescribe OCI tag
   `2.3.1`. Other living pages carry `Released (v2.1 documentation)` labels.
   Replace literal-current claims with generated release values or an explicit
   placeholder/example convention and validate install snippets at release.

2. **Cost statements do not form one intelligible model.** The homepage,
   `docs/start/should-you-adopt.md`, and `README.md` mix a twenty-minute path,
   an afternoon installation, maintainer-observed mechanical deployment of no
   more than ten minutes, and 5–15 minutes of governor attention. Separate
   mechanical install, variable commissioning/rule review, first-change
   attention, and ongoing overhead; label the evidence scope of each.

3. **Privacy/anonymization language makes guarantees AGF cannot enforce.**
   Homepage `index.md:179` says learnings can flow upstream “without revealing
   your project”; `docs/adopt/downstream-impact.md:106` says data and code never
   leave the adopter's infrastructure. Replace these with bounded statements:
   AGF itself requires no hosted AGF service; assistants, git/CI providers, and
   submissions have their own data paths; anonymization reduces rather than
   eliminates identification risk.

4. **Policy boundaries are described as technical impossibilities.** Homepage
   `index.md:161`, `README.md`, and related role/gate prose imply agents cannot
   self-approve. Clarify that rules prohibit self-approval and configured
   checks/human governance enforce it; file-based instructions alone do not
   provide runtime interception.

5. **Capability maturity is conflated with production/adoption evidence.**
   Homepage `index.md:127-148`, `evaluate.md:125`,
   `ai-evaluation-dossier.md:144`, and tooling prose gloss Level 5 as “in
   production use” or “fully operational.” Use the canonical definition:
   deployable means shipped in the supported operator path, tested, and
   documented. State foundry use and adopter evidence separately.

## WARN — conversion and information architecture

6. **The homepage delays value and action.** Two large mechanism visuals and a
   fourteen-row CLI table precede the clearest benefits. Use one compact visual,
   an early primary `Check fit` action and secondary `See a governed change` or
   current-release action, then progress through mechanism, proof, fit,
   cost/exit, and action. Move the command inventory to
   `reference/tooling-conformance.md`; retain at most three outcome examples.

7. **The exit promise is oversimplified.** Homepage and README suggest an
   ordinary cleanup removes AGF, while `docs/adopt/exit-and-decommission.md`
   correctly explains that committed history remains and loaders/hooks/CI also
   need removal. Say active integration can be removed in a governed cleanup;
   committed audit history remains subject to the adopter's retention process.

8. **Product identity is fragmented.** Explain once near the decision point:
   `agents-govern (AGF); optional CLI: agov`, then use those names consistently.

9. **Evaluator entry points compete.** Top navigation exposes Use cases,
   Evaluate, AI evaluation dossier, and Documentation, while “Should you adopt?”
   is buried. Make one evaluator hub; distinguish guided web review from the
   long AI-ingestion dossier; expose the fit decision early.

10. **README duplicates the site rather than routing repository visitors.** It
    repeats detailed maturity, adoption, tooling, MCP, and OCI material and
    contains volatile counts. Reduce it to promise, short mechanism, current
    release, fit/install actions, repository map, and contributor route; keep
    operational detail canonical on the site.

11. **Adoption sequences drift.** Homepage, docs overview, navigation, and FAQ
    disagree about applicability selection, loader verification, first change,
    and optional CI/tooling. Publish one reusable sequence: fit; cost/exit;
    install plus applicable roles/rules and loader verification; first change;
    optional CI/tooling.

12. **Several pages end without stage-appropriate next actions.** Use cases,
    downstream impact, evaluator guidance, and FAQ need consistent end blocks:
    one primary and at most two secondary routes appropriate to the reader's
    current stage.

## INFO — evidence and maintainability

13. Separate self-application, maintainer-led adopter experience, and genuinely
    independent adoption evidence near the benefits. Avoid calling a
    single-governor core empirically validated across organizations.

14. Reconcile claims about Claude Code, OpenAI Codex, Cursor, Copilot, and
    Windsurf with exact tested paths and evidence strength. Do not say tools
    work identically when loaders and sustained-operation evidence differ.

15. Deduplicate command inventories, generated counts, version labels, and
    evaluator instructions. Make one source authoritative and generate or link
    other presentations to reduce release drift.

16. Replace foundry-specific assistant telemetry filenames in adopter reference
    material with generic collision-safe examples; keep foundry conventions in
    contributor/operator guidance.

## Recommended post-release order

1. Correct stale release facts and unsafe privacy/enforcement absolutes.
2. Establish canonical cost, identity, exit, maturity, and evidence wording.
3. Rebuild homepage/README/navigation around the evaluator journey.
4. Reconcile adoption sequences, CTAs, integration evidence, and tooling pages.
5. Deduplicate/generated release-sensitive inventories and add link/snippet
   checks.

Claude's independent marketing reviewers should challenge this consolidation,
identify missed evidence, and record agreement or disagreement before a
post-release remediation plan is proposed.
