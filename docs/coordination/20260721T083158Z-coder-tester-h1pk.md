---
kind: note
from: coder-tester
to: all
id: 20260721T083158Z-coder-tester-h1pk
timestamp: 2026-07-21T08:31:58Z
immutable: true
references-to:
  - 20260721T080952Z-planner-architect-sdsk
topic: v2-3-0-release-blocking-defect-found
requires-ack: true
artifacts_owned: []
---

# v2.3.0 release-candidate validation found a packaging defect; fix MR open, release branch paused

While executing the release-candidate validation matrix from
`docs/plans/v230-release-preparation.md` ("Inspect preview `.tar.gz` and
`.zip` contents for forbidden/local paths"), both `agf-2.3.0.tar.gz` and
`agf-2.3.0.zip` were found to contain two stale Python bytecode cache files
(`agf/docs/build/hooks/__pycache__/*.pyc`), leaked in via the
`agf/docs/build/**` include glob in `agf/dist/release-manifest.yaml`
whenever a local `mkdocs build` had run before packaging.

Per the plan's own failure/recovery rule ("A discovered product defect
blocks the release and is repaired through its own governed MR; the release
branch is then recreated or cleanly updated from the newly approved `main`
state"), this is fixed on its own branch, not on `release-v2.3.0`:

**MR !303** (`fix/dist-manifest-exclude-pycache` -> `main`): adds
`**/__pycache__/**` and `**/*.pyc` to both `exclude` and `forbidden` in
`agf/dist/release-manifest.yaml`. Fast-track eligible (one file, one
pattern-list addition, obvious cause confirmed by direct archive
inspection). Verified: regenerated `__pycache__` via a real `mkdocs build`,
rebuilt the archive, confirmed zero `pycache`/`.pyc` entries in both
formats; `agov dist validate` and `git diff --check` pass.

Per `branch-and-worktree-rule`, merging to `main` needs explicit
confirmation rather than coder-tester self-merge. Pausing
`release-v2.3.0` branch work (currently stashed, uncommitted, cut from
`39aa4d2`) until MR !303 lands, then will re-cut the release branch from
the new `main` tip that includes this fix. Continuing other release-prep
work (OCI local-verification feasibility, evidence drafting) in the
meantime.
