---
{
  "id": "20261031T162948Z-pages-telemetry-method-a3",
  "status": "approved",
  "topic": "bounded-telemetry-method-example-current-pages-work",
  "affected_paths": [
    "agf/docs/site/docs/adopt/tooling-and-tokens.md",
    "docs/plans/20261031T162948Z-pages-telemetry-method-a3.md",
    "docs/reviews/rule-projection/20261031T162948Z-pages-telemetry-method-a3.yaml",
    "docs/reviews/pages-structure-and-voice/20261031T162948Z-pages-telemetry-method-a3.md"
  ],
  "depends_on": [
    "20261030T163253Z-pages-structure-and-voice",
    "20260916T002012Z-roadmap-refresh-post-v2101-issue130",
    "20261021T220000Z-retire-windows-support",
    "20261023T045214Z-remove-elixir-references",
    "20261023T083302Z-remove-elixir-leftovers",
    "20261023T155559Z-agov-strict-args-ref-list",
    "20261024T070435Z-timer-reality-polling-alignment",
    "20261025T192020Z-supersedes-adopter-docs",
    "20261027T045944Z-record-only-publication-termination",
    "20261027T052118Z-gitlab-pages-homepage-modernization",
    "20261027T052500Z-tooling-coexistence-discoverability",
    "20261027T230355Z-v212-release-freeze-corrections",
    "20261027T235807Z-v212-release-publication",
    "20261029T204751Z-v213-release-freeze-preparation",
    "20261029T220500Z-v213-release-publication",
    "20260809T063148Z-pages-rework-discovery-and-ia",
    "20261030T054200Z-pages-reader-journey-patch",
    "20261030T064744Z-v2131-release-publication",
    "20261030T072047Z-v2131-readme-journey-followup",
    "20261030T160040Z-forge-evidence-rule",
    "20261030T214700Z-pages-task-metrics-amendment-a1",
    "20261031T152858Z-pages-communication-authority-a2"
  ],
  "advisory_consultation": [
    {
      "role": "infosec-sentinel",
      "status": "consulted",
      "reason": "Independent exact-body/projection consultation CLEAR on reviewed plan 43c25461. Final delta report 365f34f588347655e344f1a43a04fe621eb65604cea71354f414e520ad0700a2. Collision, ownership, publication and binding Gate 1 remain separate."
    },
    {
      "role": "contribution-auditor",
      "status": "consulted",
      "reason": "Independent exact-body/projection consultation CLEAR on reviewed plan 43c25461. Final delta report 75e3f4f3b0b5d8b6a7e683969bc7064291f06ba2d3c3d37e6a5668bbbd869017. Collision, ownership, publication and binding Gate 1 remain separate."
    }
  ],
  "gate1_approved_by": "20261031T165937Z-governor-reviewer-nmrq",
  "gate1_approved_at": "2026-10-31T16:59:37Z"
}
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages amendment A3: what our telemetry measures

Governance mode: STRICT. Gate 1 approved by published governor decision 20261031T165937Z-governor-reviewer-nmrq, binding the reviewed design on shared branch b7a5191c. This lifecycle paragraph, approval metadata and the exact anchor statement are the only post-review changes authorized by that decision. Parent/A1/A2 files and the reviewed scope remain unchanged. The proposal/start-condition sections below record preparation provenance; actual implementation requires nmrq, the precise published s4pq claim, the receiving actor acknowledgement and fresh target/claim checks. Exact-head acceptance, Gate 2 and release remain separate.

## Scope and purpose

Only product path: `agf/docs/site/docs/adopt/tooling-and-tokens.md`. Add no page, diagram, numeric dataset, raw log, dependency, runtime feature or savings claim. Necessary governance artifacts are this plan, its six-surface projection and the dated assessment summary named in affected_paths. Foundry-local review evidence is not adopter payload.

Replace the old unqualified foundry sub-100ms / orders-of-magnitude paragraph with the dated method/limitations subsection. Narrowly correct the connected “Why meter at the governance gate” claims that every invocation crosses gates, metering captures model tokens, role/gate costs are caused or attributed, or the report proves savings. Remove the unsupported 40% example rather than leaving it to imply measured cost attribution. Preserve the existing headings/anchors, commands and environment controls, optional-tooling/install guidance and unrelated copy. The sole new subsection heading is “What our telemetry measures”, with generated fragment #what-our-telemetry-measures. Add only that new subsection anchor; do not silently change existing destinations. The remaining adjacent autonomy paragraph is outside these matched edits.

## Required semantics

- Distinguish CLI-produced output from transport/client envelopes, truncated content, model-consumed input, reasoning/output tokens, cached input and API bills. Automatic output counts and supplied records have different provenance; do not call arbitrary supplied values independently measured.
- Approximate tokens are character proxies using ceil(chars / 4), not tokenizer or billing counts. Per-record rounding and aggregate rounding need not match; claim no exact model-token total.
- Potential-saved fields may reflect built-in assumed workflow baselines and/or supplied records. Only the built-in saved_tokens calculation is asserted to saturate negative differences at zero. Supplied estimate fields are merely summed as unsigned values; the result is not a signed net benefit.
- Disclose wrapper version probes, incomplete log routing and coverage, possible shared worktree writers, and different periods and mixed or unknown producer versions across the historical interval. File names and transport tags do not authenticate assistant identity. No exclusive attribution, assistant ranking, matched experiment, general productivity, billing or net-savings claim.
- Date the assessment 31 October 2026 UTC and label the operational observations Empirically informed, not Empirically validated or an independent adoption study. The durable evidence summary records source/hash/provenance/limits; no private/raw telemetry or numeric comparison table goes into the product page.

## Ownership and start conditions

The receiving existing Codex task prepares this draft and proposed claim s4pq (20261031T155859Z-coder-tester-s4pq), presently local and unpublished. The active Codex delivery task owns the scope decision, allocation, claim publication, independent-review routing, exact acceptance and release sequence. Publish the exact single-page claim/ack and resolve actual owner overlap before source mutation. This draft allocates no work to Claude and does not expand his operation/A2 claims. Recheck the actual current target rather than applying the old draft base blindly. No new assistant sessions are needed.

Before Gate 1: obtain contribution-auditor and infosec-sentinel exact-body/projection dispositions; replace pending rows only after actual replies. Validate plan and projection and run a fresh plan/ref/open-MR/claim collision scan with coverage limits recorded. Inherited depends_on entries acknowledge only inherited parent overlaps, not every live branch or unrelated scope. The A2 reference is sequencing/ownership context, not authority to rewrite its approved content.

## Verification, acceptance and sequence

1. Independently review exact source against the inspected report/record semantics in tooling/rust/crates/agov-core/src/roi.rs (head 2c4ff4319cbb072a756a76f405932977bf483d65, blob 5d2d804f86769dbe07fe2f1307b67f68f0fd4af5), CLI surface, documented baseline model, verifiable-claims and implementation-honesty rules. Do not modify code to fit the prose. Validate supplied-record versus built-in distinctions.
2. Keep a before/after sentence and heading/anchor/link map, page diff and precise evidence provenance. Confirm that no raw logs, emails, credentials, private paths, numeric dataset or causal savings claim enters the adopter page. The local dated assessment is evidence of an observation with limits, not a reproducible public benchmark.
3. Run existing strict documentation/projection/link checks, the canonical exact-head MR pipeline and necessary distribution checks. The site page ships under the existing agf/docs/site/** release glob; no new package path is introduced. Record actual source/target SHA and HTML job/artifact identities. Green CI alone is not approval.
4. Review the affected subsection and existing report command at desktop 1440x900 and mobile 390x844 in light/dark using existing bounded controls. Inspect actual table wrapping, readability, link/anchor targets and surrounding context independently. No inherited browser-health, newcomer or whole-site claim. A failed check is corrected or dispositioned, not retried solely for green.
5. Preserve original A1 counters and answer-key obligations: Evaluate total at most 982, Adopt at most 1,080 (separate 476-word payload), Operate at most 1,174; transition limits 1/1/2, unchanged ranges/script, findability, and improvement in at least two totals only when independently verified. Tooling remains optional and is not inserted into required-reading ranges. Recheck counters as required; a claimed unchanged range is not evidence of an improved reading result. No cap reset, removal of required answer content, five-minute claim or newcomer study.
6. Retain all parent R1/R2, Home question 3, runtime figure, BODY/focus, remote-font, browser residual and final estate/compatibility criteria until separately closed. This supplement grants none of those acceptances. Exact-head independent review, Codex/Claude concurrence where required, a binding Gate 2 and automatic-main-deployment coverage/live verification remain separate. Schedule the small MR without cancelling another in-flight Pages deployment; navigation still switches last. No version, release or tag is selected here.

Completion: published exact-head handoff with independent source/factual/rendered evidence and claim closure after accepted deployment, or an explicit published blocker/withdrawal. Rollback through the normal gated revert path if the accepted page regresses; retain audit history. The applicable human delegation must still be valid for each binding action; this draft does not extend it.
