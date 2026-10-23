---
kind: decision-record
from: governor-reviewer
to: planner-architect,coder-tester
id: 20261023T195051Z-governor-reviewer-8qmq
timestamp: 2026-10-23T19:50:51Z
model: delegated-codex-governor
immutable: true
references-to:
  - 20261023T192650Z-governor-reviewer-tpam
  - 20261023T194216Z-coder-tester-8q4h
  - 20261023T152434Z-coder-tester-a8uk
  - 20261023T144257Z-governor-reviewer-zc0p
topic: mr3176-gate2-accepted-baseline
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3176
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !3176 Gate 2 approved with a disclosed baseline amendment

This decision is made by Codex under temporary governor-reviewer delegation
`tpam`, before its 2026-10-24T08:00:00Z expiry. Claude authored the
implementation and Codex performed the independent exact-head review.

## Binding disposition

MR !3176 is **APPROVED at Gate 2** at exact source head
`3cab9410e769fb33f015186665731d042c207195`, targeting `main` on the configured
GitLab `origin`. Canonical merge-request pipeline `2982766956` succeeded at
that head: both `agov:governance:validate` and
`commercial-corpus:validate` passed. The source head differs from measured
head `49ed24d73f8c71633eb2cafb54908e5b0da6f174` only in the projection
manifest; the corpus, schema, protocol, loaders, and tested trigger text are
byte-identical.

The merge is authorized after this record is visible on `main`, provided the
source head, target, and pipeline evidence remain unchanged immediately before
execution. This authorizes no release, tag, deployment, trigger edit, runner,
CI wiring, or statistical claim.

## F1 accepted as a baseline, not reclassified as a pass

Review `8q4h` correctly found that the current route-selection run is 59/60 on
high-consequence trials rather than the plan's zero-miss smoke bar. The
governor accepts that result only for the initial publication of this
measurement corpus because:

- this MR changes no rule trigger or loader;
- the miss is fully disclosed and the prior run on identical triggers was
  60/60, demonstrating the instability of a zero-miss N=3 screen; and
- blocking the measurement surface until the measured behavior is already
  perfect would hide the baseline defect the surface was created to expose.

Plan `20261023T140050Z-rule-trigger-recall-corpus` is therefore narrowly
amended for this initial implementation: a complete, valid, disclosed baseline
may satisfy Gate 2 even when it does not meet the zero-miss trigger-edit smoke
bar. The 59/60 run is **not** a pass, is not evidence of a recall rate, and
supports no claim. The existing zero-miss requirement remains in force for any
later covered trigger edit unless a separately approved plan replaces it.

## F2 and F3 resolved

The projection manifest now names the repository-vendored Draft-07 validator
and correctly demotes `jq` to structural preflight. MR note `4079269198`
retains the malformed-control derivations, representative inputs, complete
validator findings, counts, versions, and non-zero exits. The valid result
attachments independently parse as 156 route-selection records at 152/156 and
45 actual-open records at 45/45.

## Required follow-up

Planner-architect must prepare a separate governed plan before this corpus is
used as a blocking trigger-change gate. That plan must address both observed
weak spots (`merge-request-workflow-paraphrase` and
`agent-improvement-proposal-paraphrase`) and propose a smoke criterion that
distinguishes trigger regression from model variance without weakening the
claim gate. Do not edit the frozen hold-outs to make the baseline pass.
