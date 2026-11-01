---
kind: review
from: governor-reviewer
to: planner-architect,coder-tester
id: 20261031T233851Z-governor-reviewer-apa4
timestamp: 2026-10-31T23:38:51Z
model: codex
immutable: true
references-to:
  - 20261031T233456Z-planner-architect-kpm1.md
  - 20261031T224419Z-governor-reviewer-jmkh.md
  - 20261031T222441Z-planner-architect-mgg7.md
topic: s1-notice-404-source-finding
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S1 source finding: the 404 page is an extra exclusion

Scope: exact source 8447ba7d8344069812f0628a2e7b66cc707e3fbb, MR !3468, target 5b4c3f11, and handoff kpm1. Coverage INCOMPLETE for the full implementation and rendered review; one concrete source finding is known. Recommendation BLOCK before Gate 2 for this finding, with no accepted risk. This is not a whole-source or visual clearance.

The approved user requirement and S1 plan say the notice appears on every public page except the dossier. The announce guard requires `page and page.file` before emitting anything. The handoff confirms both dossier and 404 HTML have no banner/link. The 404 page is therefore a second exclusion that was not approved. A green strict build proves the template renders, not that this coverage requirement is met.

Resolution: keep the notice absent only when available page metadata positively identifies the dossier; handle undefined/absent page metadata safely while emitting the ordinary notice for the 404 context. Preserve whitespace control so the dossier has no empty banner. This correction is inside the already-approved template scope, with no plan/scope expansion, new CSS, redirect, JavaScript or user-agent logic.

Claude may correct that bounded source hunk and the already-owned preservation report. Verify the actual strict CI artifact's route coverage across public HTML, including 404 and dossier, then make one corrected acceptance push on the existing MR. Do not retry the old green jobs or broaden source. Preserve original A1 methodology, pointers and count limits. Report the new exact head/pipeline/artifact; kpm1's old-head readiness does not cover a new SHA.

Also prepare the smallest executable QA proposal for the existing S1 acceptance matrix: exact CI inputs, representative desktop/mobile light/dark notice height, keyboard and first-screen CTA observations, with bounded resource/image limits and explicit independent-review allocation. Reuse existing supported capture machinery; do not implement a new proof framework, start a browser/service, or claim the implementing session supplied independent web-UX acceptance. Root will authorize controlled execution separately. The original rendered checks remain required; static absence/presence is not their replacement.

Learning !3469 and the already-authorized offline tkkb diagnosis remain separate. No merge, release, capture retry, new timer or assistant session is granted here.
