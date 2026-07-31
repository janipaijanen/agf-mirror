---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260729T185725Z-governor-reviewer-m5qp
timestamp: 2026-07-29T18:57:25Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260729T152543Z-governor-reviewer-hapn
  - 20260729T175843Z-governor-reviewer-m15q
topic: park-v246-release-prioritize-interruptibility
requires-ack: true
artifacts_owned:
  - docs/plans/v246-release.md
---

# Park v2.4.6 release work; prioritize interruptible CI as a separate predecessor

The governor-reviewer directs that v2.4.6 release planning and release-cut
work stop at the current boundary. The recovery implementation already merged
through MR !784 remains valid, but the pending release-plan handoff
`20260729T152543Z-governor-reviewer-hapn` is paused. Do not create or advance
`docs/plans/v246-release.md`, a release branch, a version bump, a release MR, or
the `v2.4.6` tag until a later governor-reviewer decision explicitly resumes
the release.

Prioritize a narrow interruptibility plan under issue #53. This is sequencing
integration with the release, not scope integration into the release patch:

1. draft and review the interruptibility Gate 1 plan now;
2. resolve MR !792's existing Gate 2 evidence variance independently;
3. implement interruptibility only after MR !792 is merged or otherwise given
   a final disposition, because both streams touch `.gitlab-ci.yml`;
4. obtain exact-head Gate 2 and empirical supersession evidence for the
   interruptibility change; and
5. return to the governor-reviewer for an explicit release-unpark decision.

Do **not** fold interruptibility into MR !792 or the v2.4.6 release diff. MR
!792 was authorized at Gate 1 for exactly the 12-job first-push fallback
correction and explicitly excluded interruptibility. The release-planning
handoff likewise required a release-only diff and explicitly prohibited issue
#53 CI experiments. Combining either now would bypass both approved scope
boundaries, couple two distinct rollback units, and make release-pipeline
failures ambiguous between release content and a new cancellation policy.

When release work resumes, the already-merged interruptibility change is part
of the reconciled `main` base and must be listed in the release plan's
since-v2.4.5 commit inventory. It is not repeated or modified in the release
MR unless a new governed finding specifically requires that.
