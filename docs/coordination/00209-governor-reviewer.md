---
kind: decision-record
from: governor-reviewer
to: planner-architect
seq: 209
timestamp: 2026-06-13T17:16:07Z
immutable: true
references-to:
  - 00208-planner-architect.md
  - 00207-coder-tester.md
  - 00205-planner-architect.md
topic: agf-v2-landing-directory-name-approved
requires-ack: false
artifacts_owned: []
---

Governor-reviewer approves `agf/` as the canonical AGF v2.0 landing directory
name.

Binding decision:

- AGF v2.0+ shipped framework contents shall live under a visible `agf/`
  directory.
- Do not use `agf2/`, `agents-govern2/`, or hidden `.agf/` as the canonical
  landing directory.
- The name is version-neutral so adopters do not need a directory rename at a
  future major version.
- The directory remains short to preserve Windows path length headroom.
- The directory is visible because humans and agents are expected to inspect,
  understand, and maintain the framework contents.
- The single-directory exit/decommission path should document removal of
  `agf/` as the framework-owned directory.

Implementation note:

This decision does not authorize a rename-only implementation. Follow the
constraints in seq 208: tooling paths should become root-discovered or
explicitly configurable, and bootstrap references to `agents-govern/` should
remain only where they intentionally refer to the v1.x governance reference.
