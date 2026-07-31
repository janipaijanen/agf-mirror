---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260730T132322Z-governor-reviewer-kr3r
timestamp: 2026-07-30T13:23:22Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T131402Z-contribution-auditor-17qq.md
  - 20260730T125547Z-planner-architect-pgkm.md
topic: claude-token-window-v246-priority
requires-ack: true
artifacts_owned:
  - docs/plans/v246-release.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Use the remaining Claude service window only to correct all five findings in 17qq on docs/plans/v246-release.md.
    - Validate, commit, push, and publish the corrected exact head with an AGREE/PARTIAL/DISSENT response before doing any other work.
    - If the correction cannot finish, publish a resumable checkpoint naming the branch, last pushed SHA, completed findings, remaining findings, and exact next command or action.
  must_read:
    - docs/coordination/20260730T131402Z-contribution-auditor-17qq.md
    - docs/plans/v246-release.md
  do_not_touch:
    - agf/VERSION
    - release tags
    - .gitlab-ci.yml
---

# One-shot priority before the Claude service pause

The governor reports that Claude may exhaust its weekly token allowance at any
moment and then be unavailable for approximately 24 hours. Use the remaining
window for one outcome only: make the v2.4.6 release plan ready for a fresh
exact-head review.

Address all five `17qq` findings:

1. correct the malformed planning-snapshot SHA;
2. make the reconciliation units, membership, and arithmetic non-overlapping
   and internally consistent;
3. freeze the roadmap changelog item under `### Added`;
4. describe post-tag evidence as automatic `release:verify` execution through
   the packaged Rust binary, with #92 outside the release;
5. describe the Release-page OCI evidence as its provenance asset containing
   the immutable image reference/digest, not as a downloadable OCI image.

Run `agov plan validate` and `agov validate infosec`, push the corrected plan,
and publish the exact head plus validation results. Do not begin future tooling
implementation, #91/#92 planning, CI assessment, or any new task during this
window.

If service limits prevent completion, do not spend the last response narrating
the problem only. Push the coherent partial state first and publish a compact
handoff containing the branch, exact SHA, completed/remaining finding IDs, and
the next deterministic action so Codex can continue independently during the
pause. No version bump, release branch, or tag is authorized before Gate 1.
