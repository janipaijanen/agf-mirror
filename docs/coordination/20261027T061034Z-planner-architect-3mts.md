---
kind: note
from: planner-architect
to: all
id: 20261027T061034Z-planner-architect-3mts
timestamp: 2026-10-27T06:10:34Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261027T053114Z-planner-architect-tks3
topic: bridge-human-standing-decision-gate1-and-old-mr-closure
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3275
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3276
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3281
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3282
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bridge record: Human standing decision on two Gate 1 approvals and the closure of eight old merge requests

**Status: bridge record, provisional.** It transcribes a live instruction from the Human Governor on 2026-10-27 and is
written by the planner-architect, who authored MR !3275, !3281 and !3282. It is not an independent decision. It binds
nothing until the conditions below are met and a `governor-reviewer` record cites it for each exact head.

## What the Human said

"When CLEAR and landed (3281, 3282), then approved Gate 1 on !3275 and !3276, plus the closure decision on the eight old MRs."

## How the author reads it (to be corrected by the Human if wrong)

The instruction is a standing conditional approval. It takes effect only when **both** conditions hold:
1. MR !3281 (radar count correction) is CLEAR by a non-author reviewer and merged to `main`; and
2. MR !3282 (closure proposal v3, record `8qse`) is CLEAR by a non-author reviewer and merged to `main`.

When both hold, the Human's decisions are:
- Gate 1 is approved for the plan on MR !3275 and for the plan on MR !3276, each **at the exact head that is CLEAR and
  green at that moment**. A later head voids the approval for that plan; the earlier conditional approval on head
  `686f06bf` did not vest for this reason (see `rdpm`), and this one is bound the same way.
- The closure of the eight listed old merge requests (!459, !923, !967, !998, !1000, !1451, !1718, !1722) is authorized
  as the batch in MR !3282, using its authorization digest, subject to the work order's pre-execution re-read: any item
  whose state, head, branch, note count or notes digest differs from the baseline is dropped from the batch.

## Not authorized by this record

Implementation of either plan; admission of either plan into a release or any freeze; a version; promotion of any
`CANDIDATE` to `IN`; deletion of any branch; closure of !2679, !3066 or any other merge request; a merge of the two plan
merge requests (they merge with status `pending-gate1` as before, and their status fields are updated afterwards by a
separate metadata change); and any change to the heads named above.

## Next steps

A `governor-reviewer` record, written by someone other than the author of the plan concerned where possible, issued when the two
conditions hold and citing the exact heads and the batch digest. Then the pre-execution re-read, the closures with the
closure note, a completion record, and the ledger reconciliation merge request.
