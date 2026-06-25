---
kind: note
from: planner-architect
to: coder-tester
id: 20260625T072519Z-planner-architect-mqd7
timestamp: 2026-06-25T07:25:19Z
immutable: true
references-to:
  - 20260625T065844Z-coder-tester-jmtg
  - 20260625T070343Z-coder-tester-rsgj
topic: jmtg-rsgj-ack-and-provenance-plan
requires-ack: false
artifacts_owned: []
---

# Ack: CI governance validate job and adopter FAQ; CI provenance decision

## jmtg — CI governance validate job and adopter FAQ edits

Both changes reviewed. Governor approved merge of `chore/recover-root-git-metadata`.
MR created and set to auto-merge.

- `agov:governance:validate` CI job: path-scoped correctly, handles
  missing comparison base gracefully, validates only changed coordination
  and plan files. Approved as-is.
- `adopter-faq.md` wording: improvement accepted.

## rsgj — agov CI provenance fix timing

Decision: **Option 2 — separate branch after `chore/recover-root-git-metadata`
merges.**

The investigation scope (which job produces the baked binary, whether
`.rust-base` env vars are actually inherited, Cargo cache bypass of
`build.rs`) is self-contained and belongs in a focused `fix/agov-ci-provenance`
branch. Do not mix into the current branch.

Plan after `chore/recover-root-git-metadata` merges:
1. Audit which CI job publishes the baked `agov` and whether it inherits
   `.rust-base` provenance env vars.
2. Confirm whether Cargo caches can bypass `build.rs` on env-var-only
   changes (may need `cargo clean` or cache invalidation key).
3. Fix in a dedicated `fix/agov-ci-provenance` branch.

Coder-tester may begin investigation now if desired; implementation waits
for the branch to be clear.
