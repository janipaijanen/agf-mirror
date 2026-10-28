---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261025T205749Z-governor-reviewer-kb8t
timestamp: 2026-10-25T20:57:49Z
model: gpt-5
immutable: true
references-to:
  - 20261025T204657Z-planner-architect-adtq.md
  - 20261025T204905Z-governor-reviewer-k172.md
  - 20261025T195702Z-governor-reviewer-8gsh.md
  - 20261025T181659Z-governor-reviewer-q88m.md
topic: supersedes-adopter-docs-gate2-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved: adopter documentation for `supersedes`

Under the temporary full governor delegation in
`20261025T181659Z-governor-reviewer-q88m`, I approve MR !3222 at exact head
`063ced333fdd940de7b037e4b012a91584cc4d33` for merge to `main`.

The three-file documentation diff matches approved plan
`20261025T192020Z-supersedes-adopter-docs`: the normative rule states the six
validity checks, chain and tie-break behavior, advisory authority boundary, and
the four verified read surfaces; the Pages explanation gives a minimal valid
example and links to the owner; the changelog closes the adopter-docs gap.

Exact-head merge-request pipeline `2987140641` is successful with all seven
jobs green. Its first `site-html-build` attempt failed only because DNS could
not resolve GitLab during the final tag fetch, after its content checks passed;
one same-head retry, job `17451960313`, succeeded. Strict projection,
adopter-docs, infosec, and diff checks passed again against the current target
tip after coordination MRs !3224 and !3225 advanced `main`.

The contribution audit found no absence-bias, governance-relaxation,
plausibility, boundary-integrity, or rule-soundness issue in the final head.
Review `20261025T204539Z-governor-reviewer-rkqj` followed the wrong helper and
was superseded by correction `20261025T204905Z-governor-reviewer-k172`; the
actual CLI `pending_handoffs` path confirms the approved handoff-view behavior.
There are no open findings or accepted risks.

This decision authorizes merging only the reviewed exact head above. It does
not authorize a release, tag, or deployment.
