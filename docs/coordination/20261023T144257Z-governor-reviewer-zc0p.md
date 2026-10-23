---
kind: decision-record
from: governor-reviewer
to: planner-architect,coder-tester
id: 20261023T144257Z-governor-reviewer-zc0p
timestamp: 2026-10-23T14:42:57Z
model: human-governor-via-codex-transcription
immutable: true
references-to:
  - 20261023T143824Z-planner-architect-jdmh
  - 20261023T143243Z-coder-tester-geav
  - 20261023T141727Z-governor-reviewer-251g
  - 20261023T140948Z-contribution-auditor-shqm
topic: gate1-approved-rule-trigger-recall-corpus
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3167
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 approved: rule-trigger recall corpus

The Human Governor replied, "Gate 1 approved. proceed", to the exact-head
confirmation request recorded in `jdmh`. This record supplies the binding
governor-reviewer decision before any plan-status change or implementation
starts.

Gate 1 is **APPROVED** for plan
`20261023T140050Z-rule-trigger-recall-corpus` at MR !3167 exact head
`c676b2fee51b77c5589e809a84f57113fa2736ea`. Coder-tester review `geav` is
CLEAR and exact-head pipeline `2981584718` succeeded. The corrections required
by `251g` and audit `shqm` are accepted.

Approved scope is the plan as written:

- a foundry-local corpus, manual protocol, result schema, and projection
  manifest at the four declared paths;
- route-selection smoke evidence and the fixed 45-call actual-open smoke;
- no runner and no blocking-CI model calls;
- no statistical claim run unless the governor separately requests it;
- Codex authors the hold-out quarter and signs the protocol as reproducible.

Planner-architect may now apply only the mechanical Gate 1 metadata update
(`status: approved`, this record as `gate1_approved_by`, and this record's UTC
timestamp as `gate1_approved_at`) to MR !3167. The plan MR may merge after that
exact metadata-only head passes its required pipeline. Implementation may then
start under the approved scope. Any substantive plan change returns to Gate 1.

This decision does not approve Gate 2, the implementation merge, a claim-gate
run, CI wiring, a runner, a release, or any change outside the four declared
paths.
