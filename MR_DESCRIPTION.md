# docs: restore embedded figure legends on main

## What happened

The embedded-legend figure fixes were merged on `master`, but GitLab Pages is
publishing from `main`. The live site therefore kept serving the older SVGs
without embedded legends even though the corrected figures existed on the other
branch.

## What changed

- Restored embedded legends for every public docs figure on `main`.
- Updated both source figures and published site-copy SVGs:
  - `adoption-exit-flow`
  - `coordination-channel`
  - `evidence-lifecycle`
  - `gate-pipeline`
  - `governance-loop`
  - `learning-loop`
  - `operating-responsibility-map`
- Preserved the newer `main` wording in `adoption-exit-flow` while restoring
  the legend layer.
- Kept the final branch diff free of legacy seq-only coordination files that do
  not validate against current coordination schema.
- Added a current Gate 2 handoff for this restore branch.

## Verification

- `mkdocs build --strict`: clean.
- `git diff --check`: clean.
- All seven `agf/docs/site/reference/figures/*.svg` files contain embedded
  `Legend` text.
- All seven `agf/docs/figures/*.svg` source-copy files contain embedded
  `Legend` text.
- Coordination handoff validates with current `agov coord validate`.

## Review Focus

Please verify:

- [ ] The live Pages-target branch receives the embedded-legend SVGs.
- [ ] Each public reference figure has its legend inside the figure itself.
- [ ] The `adoption-exit-flow` figure keeps the newer `main` wording.
- [ ] No legacy Elixir/sequence-only coordination files are introduced by this
      restore.
