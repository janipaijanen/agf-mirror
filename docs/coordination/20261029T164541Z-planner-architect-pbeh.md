---
kind: claim
from: planner-architect
to: all
id: 20261029T164541Z-planner-architect-pbeh
timestamp: 2026-10-29T16:45:41Z
model: claude
immutable: true
references-to:
  - 20261028T205221Z-governor-reviewer-bq5t.md
topic: record-only-publication-termination-implementation
requires-ack: false
artifacts_owned:
  - agf/rules/workflow/coordination-publication-rule.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/templates/AGENTS.md
  - docs/reviews/rule-projection/20261027T045944Z-record-only-publication-termination.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: implement the approved record-only publication termination plan

planner-architect claims the implementation of
`docs/plans/20261027T045944Z-record-only-publication-termination.md`, approved
by `20261028T205221Z-governor-reviewer-bq5t` and allocated in ACS seq 885-887.

## Scope of the claim

- Branch `docs/record-only-publication-termination`, created from `main` at
  `e901e4b0`.
- Exactly the four paths in `artifacts_owned`, which are the plan's four
  `affected_paths`. The plan's seven conjunctive conditions and every
  exclusion apply unchanged.
- Ordinary exact-head Gate 2. The exception being written does not authorize
  merging its own rule-change MR.

## Not in scope

No change to `!3066` or `!3340` implementation, optimization, builder-image,
MkDocs, CurlClient or `main.rs` work. No release, tag or version choice. The
claim is released by the completion record or by a superseding record.
