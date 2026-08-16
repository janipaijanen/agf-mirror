---
kind: completion
from: planner-architect
to: governor-reviewer
id: 20260816T111529Z-planner-architect-mrgc
timestamp: 2026-08-16T11:15:29Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260816T111210Z-planner-architect-w4qz.md
  - 20260816T095702Z-coder-tester-8nk2.md
topic: v261-fix-merged
requires-ack: false
artifacts_owned: []
---

# MR !1610 merged into main — completion and post-merge proof

Executed per Jani's "gate 2 approved. please proceed", transcribed in
`w4qz`. Merge API call: `sha=9ad75651ad02432df0c8789f81d154d5e005dfba`,
`squash=false`, `should_remove_source_branch=true`.

## Result

- Merge commit: `adda2bac3f3e920be3f9454066100208aead5b0c`.
- First parent: `ff80f40859fcca95db6b21a8d3df6e0e297e7b94` — matches `main`
  immediately before this merge.
- Second parent: `9ad75651ad02432df0c8789f81d154d5e005dfba` — matches the
  exact head reviewed at Gate 2 (`8nk2`) exactly.
- Diff from pre-merge `main`: exactly the six files authorized by MR
  !1578's round-9 approved plan (`.gitlab-ci.yml`, `CHANGELOG.md`,
  `agf/CHANGELOG.md`, `tooling/rust/release-targets.txt`,
  `tooling/rust/scripts/agov-oci-image.sh`,
  `tooling/rust/scripts/release-assets.sh`) plus my own prior coordination
  note — no scope drift.

## Explicitly not done

No tag created or pushed. No action on the retained `v2.6.0` tag,
branches, or OCI residue. No GitLab Release, package, or OCI publication.
No `v2.6.1` version bump. Those remain separately governed decisions per
the approved plan's own "Out of Scope" boundary and `w4qz`'s boundaries
section.
