---
kind: handoff
from: contribution-auditor
to: build-engineer
id: 20260803T235714Z-contribution-auditor-rnh2
timestamp: 2026-08-03T23:57:14Z
model: gpt-5.4
immutable: true
references-to:
  - 20260803T233746Z-build-engineer-h2dg.md
topic: agov-wrapper-fingerprint-variable-collision
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: build-engineer
  status: ready
  task:
    - Assess the reproduced wrapper variable-collision defect for Gate 1 planning.
    - Keep the local-native and CI-prebuilt-artifact concerns as separate acceptance paths.
  must_read:
    - tooling/agov
    - tooling/rust/tests/agov-wrapper-freshness.sh
    - docs/coordination/20260803T233746Z-build-engineer-h2dg.md
  do_not_touch:
    - .claude/settings.local.json
---

# Handoff: reproduced `agov` wrapper fingerprint rebuild loop

During the poll, two consecutive `./tooling/agov version --format json`
invocations with no intervening source change both rejected and rebuilt the
release binary. The source fingerprint script returned
`3c9713c382f2150108b92dfbfe2bccabbb37edeb3ca733acbf4769eafefb994b`, while
the supposedly rebuilt binary continued embedding
`cd7bc3b1f6b30b7d9a51f7053836091da9af8868e350af54e9c95be153bf340f`.

The defect is a POSIX-shell variable collision in `tooling/agov`:
`run_rust` stores the expected digest in `source_fingerprint`, then calls
`binary_is_fresh`; that function assigns the inspected binary's digest to the
same global shell variable before returning failure. The subsequent
`AGOV_BUILD_SOURCE_FINGERPRINT="$source_fingerprint" cargo build ...` therefore
instructs Cargo to re-embed the stale digest. A manual build with the expected
digest re-ran the build script and embedded `3c9713...` correctly, confirming
the causal mechanism.

This is distinct from `h2dg`'s CI bootstrap optimization. Local interactive use
should remain the repo-local native, fingerprint-checked binary; the wrapper
needs collision-proof variable names and a regression test that begins with a
wrong embedded digest and verifies one rebuild converges. CI should separately
prototype the pinned prebuilt-binary artifact proposed by `h2dg`.

The same evidence is visible on issue #53, note `3639150564`. No tooling or CI
change was made in this diagnostic pass; implementation requires its own
approved scope.
