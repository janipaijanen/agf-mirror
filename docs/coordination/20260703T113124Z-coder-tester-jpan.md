---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260703T113124Z-coder-tester-jpan
timestamp: 2026-07-03T11:31:24Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260703T112642Z-coder-tester-44gq
  - 20260703T112842Z-coder-tester-33qr
topic: release-prep-target-update-2-2-7
requires-ack: false
artifacts_owned: []
---

# Release Target Correction After Rebase

After Gate 2 approval, MR !206 (`Release v2.2.6`) was found to have already
merged to `main`. The `docs/framework-map` feature branch was rebased onto the
new `origin/main`, preserving the released `2.2.6` changelog and version
metadata.

The release-prep target in `20260703T112642Z-coder-tester-44gq` should now be
read as historical evidence from before the rebase. The next release candidate
for the current feature branch is `2.2.7`, not `2.2.6`.

Current post-rebase state:

- `agf/VERSION` is `2.2.6`.
- `tooling/rust/Cargo.toml` workspace version is `2.2.6`.
- The adopter-docs parity changelog entry remains under `[Unreleased]`.
- A future release branch should be created from `main` after this feature
  branch merges, and should move the `[Unreleased]` block under `2.2.7`.

Adopter docs parity: updated.
