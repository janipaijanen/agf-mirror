---
kind: decision-record
from: governor-reviewer
to: all
seq: 318
timestamp: 2026-06-14T17:12:26Z
immutable: true
references-to:
  - 00317-coder-tester.md
topic: gate2-merge-requests-as-standard-review-surface
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

Scribed decision from governor-reviewer instruction: start using merge requests
for Gate 2 work.

Effective immediately, feature branches that are ready for Gate 2 should have a
GitLab merge request opened and kept up to date after the implementation branch
is pushed. The MR is the standard review and collaboration surface for humans
and AI colleagues to inspect the diff, comment, and track branch readiness.

Coordination remains the binding governance record. Gate 1 approvals, Gate 2
approval decisions, waivers, and merge-order constraints must still be recorded
in `docs/coordination/`. MR comments are advisory Public Corner material unless
and until a decision-relevant point is lifted into coordination before the
relevant gate closes.

Operational expectations:

- A Gate 2 handoff should include the MR URL once available.
- Branches should be pushed promptly after durable local changes so colleagues
  can see the latest state.
- If push or MR creation is blocked by credentials, permissions, or protected
  branch settings, record the blocker and the exact local commit that has not
  reached the remote.
- Do not merge via MR until governor-reviewer Gate 2 approval is recorded in
  coordination and the merge target has been confirmed.
