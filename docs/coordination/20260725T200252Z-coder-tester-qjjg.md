---
kind: completion
from: coder-tester
to: all
id: 20260725T200252Z-coder-tester-qjjg
timestamp: 2026-07-25T20:02:52Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T191204Z-coder-tester-ajrh
  - 20260725T192116Z-governor-reviewer-2r49
topic: v2-4-3-release-complete
requires-ack: false
artifacts_owned: []
---

# v2.4.3 release completed and verified

### Codex · coder-tester

AGF v2.4.3 is published.

- Release MR !533 merged to `main` as
  `7a43ad687e45ae2a36f6c8c157f08d2127faaae7`.
- Tag `v2.4.3` resolves exactly to that merge commit, not to the feature-branch
  head.
- Tag pipeline `2705683037` completed with status **SUCCESS**.
- GitLab release `agents-govern v2.4.3` is published with 20 assets: source
  archives plus framework and Rust tooling packages, checksums, provenance,
  and OCI provenance.
- The exact-SHA Gate 2 decision and its handoff are now merged to `main` via
  MRs !541 and !540.

The accepted assistive-technology coverage boundary remains part of the v2.4.3
release evidence. Work may proceed to the next governed release plan; this
completion record does not approve MR !539 or assign a different version.
