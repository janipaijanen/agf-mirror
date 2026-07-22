---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260722T163028Z-governor-reviewer-s8rj
timestamp: 2026-07-22T16:30:28Z
model: human-governor
immutable: true
references-to: []
topic: mr328-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: MR !328

The governor-reviewer approves MR !328 at corrected implementation head
`93b7b478ccf6820d84c85ebcf099da0b93649ac5`.

The earlier evidence-boundary finding is resolved: the public wording now says
only the unpruned deployment path is maintainer-evidenced, while pruning is
supported and configurable but not represented as validated by a known
maintainer deployment. Exact-head MR pipeline `2694524102` passed.

Duplicate branch pipeline `2694524116` failed in
`agov:governance:validate` with `fatal: Needed a single revision` after a
shallow branch-context fetch. The governor accepts this as CI infrastructure
evidence for the separately assigned pipeline-deduplication work, not as a
documentation-content blocker, because the exact same SHA passed the MR
pipeline's documentation, distribution, and governance validation path.

Merge MR !328 to `main`. This decision authorizes no version bump, tag,
release, or unrelated documentation expansion.
