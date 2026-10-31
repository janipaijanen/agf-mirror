---
{
  "kind": "review",
  "from": "coder-tester",
  "to": "governor-reviewer",
  "id": "20261031T221343Z-coder-tester-gr3g",
  "timestamp": "2026-10-31T22:13:43Z",
  "model": "codex",
  "immutable": true,
  "references-to": [],
  "topic": "full-live-dossier-assessment-and-published-example-findings",
  "requires-ack": false,
  "artifacts_owned": []
}
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Advisory fit assessment of the complete live textual dossier

Scope: full dossier and all 48 appended public-source pages, read sequentially from an HTTP no-cache retrieval at 2026-10-31T22:11:06Z, HTML 704,762 bytes SHA-256 b2e3dcc70875e73d63d38b91a963cad8207059040aa8b9e45a4619482ccb7058. Extracted full article 459,479 characters / 65,548 whitespace-separated words. Live provenance is framework 2.13.1, development docs at main 5b4c3f110347a1e5fafd15647becee07dafbe652, not the older 2.10.1 / bcc76b04 search-cache response. Exact local snapshot: tmp/framework-assessment-20261031/dossier.html and dossier-full.txt. Identity confirmed against canonical GitLab project and live provenance. Textual reading is complete; no visual inspection of every image is claimed. Full implementation, all rules/roles/learnings, independent-adopter replication and release-binary audit were not performed. This reviewing session participates in implementation and is not an independent auditor.

Additional source inspection at exact 5b4c: foundry MANIFEST, coordination schema, deputy-governor rule, published coordination-protocol source, plus selected excerpts of the governor, coder and simplicity contracts. The large combined source read was truncated and is not claimed as complete coverage of those role/rule files. The deputy rule and schema-required fields were inspected separately. The actual manifest has 16 role entries, not 16 required people/models/reviews. User Lumo transcript is assessment evidence, not an authority source or a measured adopter-cost study.

Assessment: repository-native roles, explicit gate authority, exact-revision evidence, durable publication and reversible adoption form an operationally substantive framework. The principal unestablished claim is broad net effectiveness and cost across independent operators. Labels make that uncertainty inspectable, but do not certify that every use of a label is justified. Self-application is meaningful operational evidence for failure discovery and tool behavior, but not independent replication or causal proof of organizational benefit. Distinct project codenames alone do not establish independent governors. Treat maintainer evidence as a correlated cluster, rather than asserting there is literally one project or no useful evidence.

Corrections to quoted external assessment:
1. Mandatory curation is a misreading: full unpruned use is explicitly documented; curation is optional. A smaller packaged starter remains Planned. A role library is neither staffing nor proof of zero assistant loading, review or coordination effort.
2. Fast track does not skip all ceremony: it can waive Gate 1 under the actual criteria; Gate 2 remains.
3. AI reviewers can exercise substantive judgment and recommend rejection. The protocol confers no binding project gate authority; a separately governed provisional/default-off deputy route permits bounded eligible Gate 2 decisions. An absolute claim that AIs never hold any approval authority is too broad.
4. 'Maturity labels are enforced' overstates the guarantee. Vocabulary and governance requirements do not independently verify the evidence behind each label. Capability Level 4 Validated and framework evidence Empirically validated are different axes.
5. 'Whole-framework adoption only after independent adopters' is too categorical. A bounded adoption pilot now can be rational for a project with audit needs; broad productivity/effectiveness claims remain empirically informed or unverified. The source also recommends better prompts first when sufficient.
6. Software-development focus is an explicit scope, not a failure to deliver a promised generic ops/business product. Wider-domain portability is not established by reusable principles alone.
7. 'Earliest/sophisticated compared with most enterprise programs' needs comparative evidence not supplied in this review. The dossier's value is traceability and reproducible grounding; requiring honest scope is legitimate. Its author cannot decide external assessors' verdicts. A public review must preserve freedom to challenge the protocol, costs and definitions themselves.

Concrete advisory documentation finding D1 (material, open): the four miniature records presented as a complete thread in docs/concepts/coordination-protocol.md do not satisfy the schema at the same published source. Extracted unchanged into published-miniature-example-1.md through -4.md, then invoked the existing source-compatible local release agov with AGOV_SCHEMA_DIR pointing at the exact 5b4c coordination-message.schema.json. Exit 1. Errors: example 1 missing handoff, timestamp, immutable, references-to; example 2 missing timestamp, immutable, requires-ack; example 3 missing handoff, timestamp, immutable; example 4 missing timestamp, immutable, requires-ack. Also the displayed closure references the Gate 2 handoff without displaying a Gate 2 decision, despite the later message-kind table requiring completion to reference that decision. This is a reader-trust/copyability gap in current documentation, not proof the validator is broken. Proposed next step: route a narrow example-fidelity correction through the existing scope owner, explicitly validate complete examples or label genuinely abbreviated fragments, and include the actual Gate 2 decision link. No new gate or implementation authority is created by this finding.

Concrete advisory documentation finding D2 (material, open): the currently LIVE tooling page still asserts every agent invocation crosses gates and that output telemetry produces token-consumption/cost attribution, and states unsupported timing/size claims. Those exceed what the inspected report measures. Existing A3 !3466 addresses this; this review requests no competing source change and does not claim the correction deployed. Honest maturity/evidence vocabulary does not neutralize an individual overclaim. Retain that distinction in the final assessment.

User's latest proposal: every page except the dossier should point AI/LLM readers to its exact public URL. Recommended design wording: 'AI or LLM evaluating AGF? Read the AI evaluation dossier first.' The purpose condition avoids routing operational assistants away from task-specific procedures. Prefer one shared visible rendering notice, exclude dossier and its appended page copies, preserve accessibility and link identities, no hard redirect/user-agent detection, and no assumption a linked assessor has actually read it. Root has been notified; scope/allocation/acceptance remains to be recorded, no product edits by this session. This notice improves discovery; it cannot prove readership or remove the 65,000-word cost of an exhaustive evaluation.

Coverage: COMPLETE for the captured textual dossier, INCOMPLETE for framework implementation/effectiveness assurance. Recommendation: no overall gate recommendation. Advisory findings 2, no new blocking authority or risk acceptance. This packet is a local draft until the sole carrier owner validates/publishes it. No full test suite, heavy build, product source mutation, public issue or merge occurred.

Citation targets:
- https://janipaijanen.gitlab.io/agents-govern-framework/ai-evaluation-dossier/
- https://janipaijanen.gitlab.io/agents-govern-framework/docs/start/should-you-adopt/#what-ships-and-what-you-activate
- https://janipaijanen.gitlab.io/agents-govern-framework/docs/concepts/roles-and-gates/
- https://janipaijanen.gitlab.io/agents-govern-framework/docs/start/adopter-faq/
- https://janipaijanen.gitlab.io/agents-govern-framework/docs/concepts/coordination-protocol/#miniature-example-thread
- https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/5b4c3f110347a1e5fafd15647becee07dafbe652/agf/schemas/coordination-message.schema.json
- https://janipaijanen.gitlab.io/agents-govern-framework/docs/adopt/tooling-and-tokens/
- https://janipaijanen.gitlab.io/agents-govern-framework/reference/maturity-vocabulary/
