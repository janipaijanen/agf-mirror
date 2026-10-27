---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261027T153529Z-governor-reviewer-hpn2
timestamp: 2026-10-27T15:35:29Z
model: codex
immutable: true
references-to:
  - 20261027T061034Z-planner-architect-3mts
  - 20261027T054450Z-planner-architect-8qse
  - 20261027T152503Z-governor-reviewer-1rdd
  - 20261027T152725Z-governor-reviewer-s63q
  - 20260910T133145Z-governor-reviewer-q862
topic: authorize-eight-park-radar-closures
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/459
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/923
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/967
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/998
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1000
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1451
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1718
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1722
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Authorize closure of eight preserved park-radar merge requests

This record durably applies the Human Governor's standing decision bridged by
`20261027T061034Z-planner-architect-3mts` on MR !3284 at exact head
`b088880a98d143417107525a1fdc444406a52bc6`. The Human said:

> When CLEAR and landed (3281, 3282), then approved Gate 1 on !3275 and
> !3276, plus the closure decision on the eight old MRs.

Both closure conditions are satisfied. MR !3281 was independently reviewed
and merged as `5ebb03be04368a0dfa25b35a44875118720dbabe`. MR !3282 was
independently reviewed and merged as
`914495f38a8e39ddb06c64d4d5a23695e7a70625`, publishing proposal `8qse`.

The Human Governor therefore authorizes closing MRs !459, !923, !967, !998,
!1000, !1451, !1718 and !1722 as `park-radar`, using the exact closure-note
text in `8qse`. The binding authorization digest over
`{iid,state,disposition,note}` is
`fcb0776461780c88633183eb4e3d5be0497a32f23a9a0cb39f4cd67e4b1727ab`.
The richer evidence baseline digest is
`ec8fa8f2f2ffad509a49becb672974a5f41dd80f989800032a229a0e59c74d19`.

Immediately before execution, the operator MUST re-read each merge request's
state, exact head, source branch, non-system note count, and complete digest
over every non-system note's `[id, updated_at]`. Any changed item is dropped
from the batch and returned for review; unchanged items may close with their
exact authorized notes.

Every source branch is retained. No branch, tag, issue, or other merge request
may be deleted or closed. MRs !2679 and !3066 are expressly excluded. The
branch-only leftovers accepted by this decision remain exactly those disclosed
in `8qse`, including MR !459's thirteen coordination records plus proposed
learning `20260723T231109Z-w7k2`, MR !1718's `branches.csv`, and MR !967's
`docs/sources.md` edit.

After execution, publish a completion record with the per-item result and
perform the ordinary reviewed ledger reconciliation. This decision grants no
release admission, `IN` promotion, version, freeze, tag, release, deployment,
or implementation authority for MRs !3275 or !3276.
