---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 01KV7MABYEEK6R1N18J9M4S8JQ
timestamp: 2026-06-16T07:10:07Z
immutable: true
references-to: []
topic: elixir-tooling-binary-bundle-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/elixir-tooling-binary-bundle.md
---

# Gate 1 approved — elixir-tooling-binary-bundle

**Plan:** `docs/plans/elixir-tooling-binary-bundle.md`
**Decision:** APPROVED — proceed to implementation.

## Review notes

Packaging approach (Mix release with bundled ERTS) is the right call for the
stated target. The existing CI image already covers linux-x86_64 + OTP 29
without any cross-compile toolchain. Burrito deferral is correct until there
is multi-platform adoption evidence.

Artifact naming encodes OTP major version derived at build time — good;
avoids staleness if the image pin changes.

Bundle-wraps-archives-as-is (no re-expansion) is correct; keeps the bundle
assembly job simple and checksums unambiguous.

Gate-self-test requirement is explicit in the plan — the Gate 2 handoff must
demonstrate tag/VERSION mismatch blocking both new jobs.

Advisory consultation is recorded in the plan front matter.

## Implementation scope confirmed

- New `rel/` Mix release configuration in `tooling/elixir/`
- Two new Mix tasks: `mix agov.tooling.build`, `mix agov.bundle.build`
- Two new CI release jobs: `agov:tooling:release`, `agf:bundle:release`
- Two new CI preview jobs: `agov:tooling:preview`, `agf:bundle:preview`
- Tests for both tasks and the failure path
- Gate-self-test demonstration in Gate 2 handoff

Non-goals are binding: no macOS/Windows/ARM, no escript/Burrito, no
registry, no signing, no SBOM in this slice.
