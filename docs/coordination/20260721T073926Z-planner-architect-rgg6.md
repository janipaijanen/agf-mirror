---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260721T073926Z-planner-architect-rgg6
timestamp: 2026-07-21T07:39:26Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260720T201654Z-coder-tester-sb14
  - 20260720T222023Z-coder-tester-ksgr
  - 20260721T062347Z-coder-tester-dkmj
  - 20260721T073721Z-governor-reviewer-q77n
topic: v2-3-0-release-preparation-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/v230-release-preparation.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - review and approve, revise, or reject the v2.3.0 release-preparation plan
  must_read:
    - docs/plans/v230-release-preparation.md
    - docs/coordination/20260720T222023Z-coder-tester-ksgr.md
  do_not_touch:
    - agf/VERSION
    - tooling/rust/Cargo.toml
    - tooling/rust/Cargo.lock
---

# Gate 1 handoff: v2.3.0 release preparation

The plan converts the release NO-GO dossier into an executable checklist with
frozen scope, release-only branch boundaries, pre-tag packaged-binary and OCI
evidence, immutable-tag recovery semantics, and public post-release checks.

Do not approve implementation until !289 and the v2.2.6 history disposition are
handled as specified. MR !293 and MR !294 are explicitly excluded and cannot
delay v2.3.0.

Claude / Contribution Auditor is separately asked for a requires-ack critical
review. Gate 1 remains a human governor decision after that review is visible.
