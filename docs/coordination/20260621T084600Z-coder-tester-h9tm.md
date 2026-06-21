---
id: 20260621T084600Z-coder-tester-h9tm
kind: completion
from: coder-tester
to: governor-reviewer
timestamp: 2026-06-21T08:46:00Z
immutable: true
references-to: []
topic: mise-glab-tooling
requires-ack: false
artifacts_owned: []
model: gpt-5-codex
completion:
  status: complete
  completed_task:
    - Added GitLab CLI installation to mise configuration so repository tooling can rely on glab.
    - Pinned glab to 1.103.0 in mise.toml and mise.lock.
    - Kept only the direct glab tool entry; did not commit the duplicate ubi provider entry from the local worktree drift.
  artifacts:
    - mise.toml
    - mise.lock
---

# Completion: add glab to mise tooling

Added `glab` to the repository mise toolchain so agents and contributors can
install the GitLab command-line client through the project environment rather
than relying on an undeclared local binary.

The local drift had both `glab` and `ubi:gitlab-org/cli` entries. I kept the
direct `glab = "1.103.0"` declaration and its lock metadata, because both
entries provide the same CLI and duplicating them would make the environment
less predictable.

## Verification

- `MISE_TRUSTED_CONFIG_PATHS=$PWD mise ls --current glab` reported
  `glab 1.103.0`.
- `MISE_TRUSTED_CONFIG_PATHS=$PWD mise exec -- glab --version` reported
  `glab 1.103.0 (c724bea5)`.
- `git diff --check` passed.

The local shell emits a mise trust warning for this linked worktree before
commands run; it did not prevent mise from resolving or executing `glab`.
