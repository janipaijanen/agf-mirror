---
{
  "id": "20261031T152858Z-pages-communication-authority-a2",
  "status": "approved",
  "topic": "communication-authority-clarification-current-pages-work",
  "affected_paths": [
    "agf/docs/site/docs/concepts/coordination-protocol.md",
    "agf/docs/site/reference/figures.md",
    "agf/docs/site/docs/concepts/roles-and-gates.md",
    "agf/docs/figures/src/coordination-channel.dot",
    "agf/docs/figures/coordination-channel.svg",
    "agf/docs/site/reference/figures/coordination-channel.svg",
    "docs/plans/20261031T152858Z-pages-communication-authority-a2.md",
    "docs/reviews/rule-projection/20261031T152858Z-pages-communication-authority-a2.yaml",
    "docs/reviews/pages-structure-and-voice/20261031T152858Z-pages-communication-authority-a2.md"
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
    "20261030T160040Z-forge-evidence-rule"
  ],
  "advisory_consultation": [
    {
      "role": "infosec-sentinel",
      "status": "consulted",
      "reason": "Exact scope/body reviewed at plan SHA d62bb0bf and projection SHA 2581e107; final delta CLEAR. Report 5fb2cf87aae121573498e46ea9dbd05de23e4413f1627d87c54f2737114ce12e. Root separately inspected named ref/open-MR overlap evidence; limitations remain disclosed."
    },
    {
      "role": "contribution-auditor",
      "status": "consulted",
      "reason": "Exact scope/body reviewed at plan SHA d62bb0bf and projection SHA 2581e107; final delta CLEAR. Report contribution reviewer final exact-delta response; no pending scope finding. Root separately inspected named ref/open-MR overlap evidence; limitations remain disclosed."
    }
  ],
  "gate1_approved_by": "20261031T160614Z-governor-reviewer-jaa4",
  "gate1_approved_at": "2026-10-31T16:06:14Z"
}
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages amendment A2: communication authority, using existing figures

Governance mode: STRICT. Gate 1 approved by published governor decision 20261031T160614Z-governor-reviewer-jaa4 (carrier commit 18371d0a), binding the reviewed scope at f36a9a4d. This paragraph and lifecycle metadata are the only post-review changes authorized by jaa4; all actual implementation and Gate 2 conditions remain. Jani explicitly instructed inclusion of the Codex/Claude consensus in the current work order. Published allocation habh (20261031T151346Z-governor-reviewer-habh) assigns Claude the six paths above; root handles this supplement and independent reviews. Parent rp8p and A1 remain unchanged; this adds only necessary canonical figure-source paths omitted from the parent's original affected-path list. No new figure or reader page is proposed.

The reader question is: how does a request become visible work, who acts next, and what actually grants approval? Existing gate-handoff-sequence, gate-pipeline, governed-change-flow and coordination-channel figures already cover the sequence. Another diagram would duplicate them and increase drift.

## Exact work

- In coordination-protocol, add a short tool-neutral clarification adjacent to the existing sequence and correct only matching categorical authority prose. Conversation, receipt acknowledgements, reviewer concurrence and passing checks do not by themselves approve a change. Approval requires a governed, published decision by the authorized actor (role or principal), for that gate, under the applicable rules; message kind alone is insufficient. The gate owner determines valid decision forms. Reader-facing prose may say authorized decision-maker without adding unexplained jargon. Publication means visibility on the project's agreed shared channel, not mandatory public internet disclosure.
- Correct the matching categorical statement in reference/figures.md and, only where confirmed, the frozen roles page. Narrowly reopen those authority sentences; do not rewrite the roster, activation, deputy rules, other safeguards or unrelated explanations. Respect each applicable gate's actual allowed decision forms rather than declaring every acknowledgement binding or forbidden.
- Correct coordination-channel.dot's false exclusive-kind description and regenerate both named SVG copies with the existing render_svg.py pipeline. Preserve visible geometry, labels, palette, legend, titles, licensing and layout unless a separately reviewed fidelity correction proves necessary. A confirmed categorical visible label may receive the smallest reviewed wording correction; then recheck its affected geometry/readability rather than claiming label invariance. Source and both copies must remain synchronized. No new image, generator, dependency or full-site redesign.
- Claude's Day-to-day hygiene bullet/link belongs to the existing operation branch and claim, outside its A1 counted range. It is not duplicated in the separate clarity MR. The reserved forge-rule section remains untouched.

## Ownership and sequence

Claude must reference habh and this approved supplement in a published exact-path claim/ack before expanded product edits. Existing jgtt is reported but must be verified. The bounded clarity MR starts from actual main after setup merges; preserve source/target identities and compare every open MR/claim again. Root retains Home/nav and independent review coordination. Do not merge any second main change while a Pages-bearing pipeline is running. No role, schema, permission, gate, release/version or canonical governance rule changes are authorized. Preserve the deputy-governor-rule.md default-off, locally scoped Gate 2 exception, red list, severity ceiling, expiry and no-self-approval boundary; do not erase or widen it.

## Verification and completion

- Validate this plan, its six-row projection manifest and fresh plan/ref/MR/claim collision coverage. Parent dependencies acknowledge inherited overlaps only; no unrelated live overlap is waived. Canonical figure paths and site copies ship under the current release manifest's agf/docs/figures/** and agf/docs/site/** globs.
- Review exact source against unchanged applicable workflow, publication, governor-role, tool-authority and coordination-schema owners. Preserve every original URL/anchor and qualify example versus binding contract. Independently inspect any resulting exact-head diff; author concurrence is not governor authority.
- Use existing figure render/metadata checks and strict docs/link/provenance checks. Prove unchanged visible SVG geometry/labels where only descriptions change, source/copy synchronization, and actual archive inclusion. Run the required canonical MR pipeline; no paid retry solely for green.
- Inspect affected captions/prose at 1440x900 and 390x844 in light/dark with existing bounded QA controls, correct readable context/focus/destinations and actual figure descriptions. Record actual evidence and limitations; no inherited browser-health or whole-site/user-study claim.
- Re-run original A1 counters and necessary answer-key/findability checks. Caps, ranges and all parent residuals stay unchanged. Gate 2, Codex/Claude consensus where required, main CI and actual Pages verification remain separate. Final parent release is still contingent on remaining operation, Home/nav and estate work.

Rollback: stop before mutation if scope/authority conflicts. Revert an implemented candidate through normal gated review if needed; retain claims, approvals and evidence history. No irreversible history rewrite or automatic scope extension.
