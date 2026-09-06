---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260906T090930Z-coder-tester-g7mh
timestamp: 2026-09-06T09:09:30Z
model: gpt-5
immutable: true
references-to: []
topic: v290-atomic-runner-routing-gate2
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2430
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review exact head 91cb81067f14b6f5b250382367abf3a177f60b4b of MR !2430 and make the binding Gate 2 decision.
  must_read:
    - docs/plans/20260905T145109Z-route-all-jobs-gate-accompaniment.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2430
  do_not_touch:
    - .claude/settings.local.json
---

# v2.9 atomic runner-routing implementation ready for Gate 2

MR !2430 reconstructs the reviewed ordinary-runner routing from !2362 and
native mandatory Linux packaging from !2392, then adds the six approved
`[dind]` overrides and both gate-accompaniment regression guards. The exact
implementation head is `91cb81067f14b6f5b250382367abf3a177f60b4b` against
`main` at `9fc2f63a83ee0a4965963be7297ae6ee3d47888d`.

Canonical MR pipeline `2823963799` is terminal green: 22 required jobs passed
and the two approved Windows preview jobs remain manual/allowed-failure. The
initial same-SHA push pipeline `2823963746` was verified as the narrower
duplicate and canceled with duration 0; it was not used as acceptance
evidence.

Runner execution evidence:

- `agov:oci:mr-build-scan` job `16330500985` selected runner `55975189`
  (`dind`) and passed. Executor preparation completed without the former
  duplicate-`/cache` error, the nested image build succeeded, and Trivy
  reported zero vulnerabilities.
- `rust:tooling:oci-pull-test` job `16330500998` selected runner `55975189`
  and passed its real Podman/registry and composed evidence tests.
- Every observed ordinary job selected runner `55908035` (`priv runner`);
  no ordinary job selected the DinD runner.
- Immediately before the run, both runner API records were active, online,
  and unpaused. Runner `55975189` had `tag_list=[dind]` and
  `run_untagged=false`; runner `55908035` retained
  `tag_list=[podman,linux,amd64]`.

Local exact-tree evidence is recorded in the MR description: full Rust
workspace tests green, CI-needs suite 11/11, both live-registry tests green,
`agov gate change-check --base-ref origin/main` green, `git diff --check`
green, and GitLab CI lint valid with no warnings. RED before implementation
named exactly the six missing DinD overrides; mutation controls reject an
ordinary self-tag and an unresolved live gate path.

Gate 2 remains a binding governor-reviewer decision. Do not merge merely from
this handoff or the green pipeline.
