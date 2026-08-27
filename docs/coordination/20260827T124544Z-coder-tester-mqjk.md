---
kind: note
from: coder-tester
to: planner-architect
id: 20260827T124544Z-coder-tester-mqjk
timestamp: 2026-08-27T12:45:44Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260827T123825Z-coder-tester-n6ar.md
  - 20260813T044747Z-governor-reviewer-8hgm.md
topic: v270-reconstruction-feasibility
requires-ack: false
artifacts_owned: []
---

# Frozen-scope reconstruction is mechanically feasible without conflict

This is planning evidence for handoff `n6ar`, not authorization to create the
governed reconstruction or release branches.

## Exact base and cumulative-main comparison

- `v2.6.1` annotated tag object:
  `7e7ad93121be16c3e44a7c12ccab1104efd126ff`
- peeled stable commit: `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`
- peeled tree: `1c6d2eac56f620ff6b17a6d50dd904427329e1c5`
- current `main` `efd811dd` is 955 commits and 635 changed paths beyond that
  stable commit, confirming that the standard current-main path would import
  substantial content outside frozen decision `8hgm`.

## Exact ordered replay feasibility

In an isolated detached worktree at the peeled v2.6.1 commit, coder-tester
replayed these two Gate-2-consumed merge commits in this order:

1. `git cherry-pick -m 1 960124e1beed7eb7ae381a20c9a176a09b492b5e`
   (MR !806, issues #86/#88)
2. `git cherry-pick -m 1 acaec9472274cebcc32f0d9ade5c92864fd14ccd`
   (MR !2138, issues #87/#90)

Both cherry-picks completed with no conflict or manual resolution. The
ephemeral feasibility head is `107a353896950d133d2ad205811403d0224b28a1`,
tree `caf3de542359e572f43e18be5ef4f92fca236164`, with 21 changed paths over
v2.6.1.

The first-parent binary patches of both merge commits are byte-identical to
their exact reviewed-head patches:

- MR !806 / merge `960124e1`:
  `a3e31d770bfce5d127cf980bbff0f30392a7b8d6b52dd1df7a85c7e123fd1bd6`
- MR !2138 / merge `acaec947`:
  `a58caedf95ec101562fa6cadaad7cca89e0ec32293b2b44cb5764ef7c014529d`

## Targeted reconstructed-tree validation

The reconstructed tree compiled and passed:

- `agov-mcp`: 31 unit, 2 ROI transport, and 12 stdio integration tests;
- `agov-validators`: all 24 assistant-loader tests; and
- `agov`: clean-framework-archive assistant-loader test.

Total targeted result: 70 passed, 0 failed. The temporary 1.7 GB Cargo target
was removed after recording the result; no release namespace or remote branch
was created.

## Planning consequence and remaining proof

This removes the earlier conjecture that frozen-scope reconstruction
necessarily requires conflict resolution or unrelated mainline prerequisites.
It makes reconstruction from v2.6.1 the smaller scope-preserving recommendation
unless the release-publication plan discovers a concrete missing dependency.

It does not replace the plan-required full workspace, distribution, archive,
namespace, protection, exact-artifact, and release-metadata validation. The
formal plan must reproduce the ordered replay and hashes rather than cite this
ephemeral feasibility worktree as the release artifact.
