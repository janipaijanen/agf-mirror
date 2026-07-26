---
kind: handoff
from: coder-tester
to: contribution-auditor
id: 20260726T223557Z-coder-tester-kmmd
timestamp: 2026-07-26T22:35:57Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T183242Z-contribution-auditor-5hsk
  - 20260726T184011Z-governor-reviewer-t3q7
topic: agov-binary-freshness-exact-head-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Audit MR !638 at exact implementation SHA 3beea54826fac04de7ddb5c51aa6459e2a493b76 for conformance with the approved plan and return CLEAR/BLOCK evidence for human Gate 2.
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

# Audit exact-head stale-binary and packaging controls

MR !638 is frozen at implementation SHA
`3beea54826fac04de7ddb5c51aa6459e2a493b76`. The implementation rejects
stale, dirty, malformed, and provenance-incompatible repo-local binaries;
adds full provenance parity for `agov` and `agov-mcp`; rebuilds and verifies
Linux package bytes before archive creation; and invalidates Cargo's cached
provenance when a tracked freshness-sensitive source changes.

Local validation passed the Rust workspace tests and clippy, wrapper freshness
suite, adopter-documentation and infosec validators, and the packaging negative
suite. The negative suite reports `archive creation not reached`. An isolated
reproduction also rejected the actual old `agov 2.2.6` binary at short commit
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
triggered. Therefore the approved plan's positive report for every supported
release target is not represented as complete: please state whether this is a
Gate 2 blocker requiring a governed exception, while independently reviewing
the implemented code and available exact-head evidence.

Do not merge, create release state, run either Windows job, or make a binding
gate decision.
