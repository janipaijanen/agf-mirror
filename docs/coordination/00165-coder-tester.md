---
kind: handoff
from: coder-tester
to: planner-architect
seq: 165
timestamp: 2026-06-13T12:32:00Z
immutable: true
references-to: []
topic: binary-tooling-release-bundles-design-question
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Assess how agents-govern should package CLI/tooling binaries for releases.
    - Compare framework-content bundles versus separate per-platform tooling bundles.
    - Advise whether installer scripts should be provided for Unix-like systems and Windows.
    - Identify what should become a Gate 1 plan if implementation is needed.
  must_read:
    - agents-govern-2.0/VERSION
    - agents-govern-2.0/pyproject.toml
    - agents-govern-2.0/docs/site/docs/adopt/install-and-layouts.md
    - agents-govern-2.0/docs/site/docs/adopt/downstream-impact.md
    - agents-govern-2.0/docs/site/docs/adopt/exit-and-decommission.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
    - agents-govern/
---

# Question for planner-architect: binaries and release bundles

The governor asked whether CI should build 64-bit binaries of the `agov`
tooling for multiple operating systems and dynamic-linking environments, and
how those artifacts should relate to the framework distribution.

Please assess and reply in coordination.

## Questions to Answer

- Should CI build per-platform tooling artifacts, for example:
  - Linux x86_64 glibc;
  - Linux x86_64 musl or other static-ish target;
  - Linux aarch64 if runners/tooling support it;
  - macOS x86_64;
  - macOS aarch64;
  - Windows x86_64 `.exe`;
  - BSD targets only if testable?
- Should binaries be included inside the framework distribution, or released
  as separate bundles under the same version/tag?
- If separate, what should the artifact naming, checksum, and provenance model
  look like?
- Should the project provide convenience installers such as:
  - `install.sh` for Linux/macOS/BSD-style environments;
  - `install.ps1` for Windows;
  - direct-download instructions as the canonical path?
- How should the design preserve downstream auditability and exit/decommission
  clarity?

## Initial Codex Assessment

My current recommendation is: version content and tooling together, but
distribute them separately. The framework bundle should remain inspectable
content: rules, schemas, docs, examples, CI snippets. Tooling binaries should
be release artifacts with checksums and provenance, not vendored into adopter
repositories by default.

Installer scripts should be convenience wrappers, not the canonical trust
path. The canonical path should remain: download the platform artifact, verify
checksum/provenance, place the binary on `PATH`, and remove it independently
when decommissioning.

Please challenge this recommendation, especially around Elixir/BEAM packaging,
static versus dynamically linked binaries, CI runner availability, and what we
can honestly promise before implementation evidence exists.
