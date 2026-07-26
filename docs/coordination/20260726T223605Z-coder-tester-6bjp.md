---
kind: handoff
from: coder-tester
to: infosec-sentinel
id: 20260726T223605Z-coder-tester-6bjp
timestamp: 2026-07-26T22:36:05Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T181327Z-planner-architect-khq3
  - 20260726T184011Z-governor-reviewer-t3q7
topic: agov-binary-freshness-exact-head-infosec-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: infosec-sentinel
  status: ready
  task:
    - Perform the independent Gate 2 infosec review of MR !638 at exact SHA 3beea54826fac04de7ddb5c51aa6459e2a493b76 and return CLEAR/BLOCK evidence.
    - Assess local candidate probing, scoped dirty-state derivation, override containment, embedded-provenance inspection, and fail-before-archive behavior.
    - Explicitly classify the deferred Windows preview provenance evidence rather than treating an unrun manual job as passing.
  must_read:
    - docs/plans/agov-binary-freshness-and-release-integrity.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/638
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2707056136
  do_not_touch:
    - rust:tooling:preview:windows-package
    - rust:tooling:build:windows:preview
    - agf/VERSION
    - Git tags and release artifacts
---

# Infosec review of exact-head execution and release boundary

MR !638 is frozen at implementation SHA
`3beea54826fac04de7ddb5c51aa6459e2a493b76`. Please review the trust boundary
end to end: the wrapper's provenance probe and ancestry/path checks; its
handling of tracked versus unrelated dirt; the explicit development override
and CI prohibition; the linker-retained embedded provenance records; the
tracked-source Cargo invalidation added at the final SHA; and deterministic
verification of cross-built and staged bytes before archive creation.

Local validation passed the Rust workspace tests and clippy, wrapper freshness
suite, adopter-documentation and infosec validators, and the packaging negative
suite. The negative suite reports `archive creation not reached`. An isolated
reproduction rejected the actual stale `agov 2.2.6` binary at short commit
`7dd69050fb1f` before session resume. The disclosed latency sample measured
about 10.1 ms added per `--version` invocation and about 20.5 ms (3.0%) per
`session resume` invocation on the test machine.

The exact-head ordinary pipeline `2707056136` passed; preview job `15541210058`
also passed. Its immutable job artifact was independently downloaded: both archive
hash sidecars match, both provenance JSON files identify exact source
`3beea54826fac04de7ddb5c51aa6459e2a493b76`, and native execution plus
aarch64 marker inspection agree on version `2.4.4`, `dirty:false`, and
`source_dirty:false`. The complete fields and hashes are published in MR !638.
The governor has instructed that `rust:tooling:preview:windows-package` must not
be operated until an external request exists. Neither Windows manual job was
triggered. The approved plan's positive report for every supported release
target is therefore explicitly incomplete. Please state whether that evidence
gap blocks Gate 2 or needs a governed exception; do not infer Windows success
from Linux evidence.

Do not merge, create release state, run either Windows job, or make a binding
gate decision.
