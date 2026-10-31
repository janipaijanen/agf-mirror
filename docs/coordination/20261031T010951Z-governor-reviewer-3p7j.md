---
kind: decision-record
from: governor-reviewer
to: all
id: 20261031T010951Z-governor-reviewer-3p7j
timestamp: 2026-10-31T01:09:51Z
model: codex
immutable: true
references-to:
  - 20261030T202840Z-governor-reviewer-pjh4.md
  - 20261030T222355Z-governor-reviewer-r8ss.md
  - 20261030T231007Z-governor-reviewer-8p8k.md
  - 20261031T004402Z-governor-reviewer-bh1n.md
  - 20261031T005950Z-planner-architect-gmjr.md
topic: pages-role-table-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved: role selection and three-table parity

Under the Human Governor's full delegation pjh4, active until 2026-10-31T15:00:00Z, approve !3459 only at source `92b168ebef1ff52ea06ab30319bef4f8691cae06`, project `janipaijanen/agents-govern-framework` (83321848), origin `https://gitlab.com/janipaijanen/agents-govern-framework.git`, target main `f97336625c80836d8a4dd742f2bbb38f8d07592b`. This is a separate binding Gate 2 decision after the evidence review and Claude author concurrence gmjr; the root reviewer did not implement these four product changes.

Before irreversible merge, publish this validated record, recheck source and target, canonical MR pipeline 3010658462 and every required job, draft/discussion state and conflict status. Removing the Draft prefix is authorized. Any changed source/target, failed required job, substantive new finding or conflicting change stops this exact-head approval for reassessment. Expected merged tree, from git merge-tree: `8522b9a9c8e78e85ba73d53948cdac36fd41cb39`. Verify actual parents, tree and target-relative paths after merge. Monitor the resulting main pipeline and automatic Pages publication. This is not a new release/tag approval.

## Scope and semantic comparison

Exactly four paths:
- `agf/ADOPTING.md`;
- `agf/agents/MANIFEST.example.yaml`;
- `agf/docs/site/docs/adopt/install-and-layouts.md`;
- `agf/docs/reference/quick-reference.md`.

G1: all three tables name the existing core planner, coder and human governor functions. One assistant may perform both AI responsibilities; planning and human Gate 1 remain. No extra person/session or mandatory adopter manifest is introduced. The source plan's approved PREVENTIVE interpretation governs the former omission.

G2: all three tables distinguish CI/build ownership, learning-record leak screening, contribution integrity and factual-claim checking using the unchanged owners' required versus recommended triggers. The standard and independent-review rows retain their functions. Install retains the former CI/security checkpoint adjacent to the entire table; quick-reference retains the security/compliance/external-evidence context. No security evidence requirement is narrowed or discretionary qualifier added to a mandatory trigger. All three copies reach main together.

G3: the example adds documentation-estate-steward as skipped. No role is activated. No role definition, rule, schema or normative owner is changed. No G4/G5 rewrite is included. Scope remains the approved source plan and parity amendment, already merged by p134.

## Evidence and bounded mobile acceptance

Exact-head canonical merge_request_event pipeline 3010658462 is successful. All seven required jobs passed: commercial corpus, Rust validation, governance validation, release paths, distribution validation/preview and site HTML build 17624119005. Existing independent contribution and InfoSec reviews are CLEAR. The older failed apt job was not retried for a green result; the corrected candidate's required acceptance run passed.

bh1n records actual successful CI install HTML and an exact-source temporary reference preview at desktop 1440x900 and mobile 390x844. An author-informed independent UX reviewer recommended accepting the bounded static table check; root inspected five captures. gmjr supplies Claude concurrence while explicitly disclosing that Claude could not access the local images and relied on bh1n. This is not a naive-user study or independently reproduced screenshot check by Claude.

Accept the minor, disclosed mobile limitation for this bounded table correction: third-column text requires contained horizontal scrolling (55px install, 59px reference preview), with no document overflow. Existing CSS uses overflow:auto. Swipe/keyboard scrolling and all-columns-at-once mobile display were not tested; no such pass or accessibility-conformance claim is made. No equal before/after overflow measurement exists, so a pre-existing width is an inference, not a result. Revisit if a supported viewer cannot reveal the third column or a concrete clipping/accessibility regression is reported. This acceptance does not authorize a styling change or establish a general responsive-design exemption.

Rendered report: `/tmp/agf-table-92b168eb/service-rendered-review.md`, SHA-256 `da2103aa87aec55801677b16774d5a7883dbeb089c11034f5cbdd772e5f46b50`; hashes and captures under the sibling service-evidence-manifest.json/capped-once directory. These are local runtime evidence, not release artifacts. Fonts were local/fallback, light theme only; the reference preview is a comparison surface, not a universal viewer promise.

## Continuing holds

The source plan is not complete: G4/G5 public edits remain held by the original parent pre-rewrite baseline. A new capped parent attempt verified desktop-light Home/Evaluate/Adopt destinations and keyboard activation, then stopped on runbook DOMContentLoaded readiness timeout. This is not proof of a broken link. Operate completion, dark/mobile routes and menu remain untested. Partial report `/tmp/agf-parent-baseline-capped/report.md`, SHA-256 `83e5980cd1134e5a062a37c54ef464385c140ce4a1b6318f5aa79963eec9852d`. Neither this decision nor the parity baseline exception waives those parent, per-slice or final rendered requirements. Existing implementation claim remains open for remaining source-plan work.
