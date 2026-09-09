---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260909T102151Z-governor-reviewer-rqqh
timestamp: 2026-09-09T10:21:51Z
model: human-via-codex
immutable: true
references-to:
  - 20260909T043713Z-governor-reviewer-djh1.md
  - 20260909T051907Z-coder-tester-ebgh.md
  - 20260909T051913Z-coder-tester-sjk2.md
  - 20260909T051931Z-coder-tester-4ms4.md
topic: propeller-stage4-waiver-correction-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Approve the bounded !2597 pipeline correction

After the diagnosed !2597 pipeline failure and the explicit request for a
minimal scope decision, the human governor stated: “!2597 is approved.” In
the immediately preceding context, Codex announced that it would treat this as
authorization for the requested corrective waiver and pipeline rerun, not as
Gate 2 on a failed SHA.

The approved correction is limited to:

- rewording the implementation branch's mixed-range review-handoff commit so
  it does not claim the protected `coordination:` fast path; and
- adding one `Adopter docs: not applicable` waiver under `agf/CHANGELOG.md`
  Unreleased, explaining that the activation is foundry-only and does not
  activate adopter projects.

No version bump or feature changelog entry is authorized. Corrected head
`ea9924ef32bb05b5c9889b174046ed79bcb87f51` must complete its canonical
pipeline and exact-head independent reviews before a separate binding Gate 2
decision. Runtime state and heartbeat creation remain post-merge actions.

MR !2600 was already merged before the governor's instruction to assist with
it; its durable output is the `pkbt` decision bridge and `drh5` diagnosis.
