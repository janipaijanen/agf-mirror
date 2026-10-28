---
kind: decision-record
from: governor-reviewer
to: all
id: 20261028T165830Z-governor-reviewer-ahan
timestamp: 2026-10-28T16:58:30Z
model: codex
immutable: true
references-to:
  - docs/plans/20261027T235807Z-v212-release-publication.md
  - docs/coordination/20261028T161358Z-coder-tester-gm8j.md
  - docs/coordination/20261028T163912Z-governor-reviewer-ss2k.md
topic: v212-status-sync-gate2
requires-ack: false
artifacts_owned: []
---

# Approve MR !3332 at exact head

Under the delegated governor-reviewer authority recorded in `ss2k`, MR !3332
is approved at exact head `4b897d5f3a511cd33fd3a5ac0ddd91910e5a3293`
against target `main` at
`6e927570822a0b03b6e1d994182c37cfacdebfd0`.

The diff changes exactly three authorized paths: two lines in
`agf/docs/governance/manifest.md`, five cells in
`agf/docs/site/roadmap.md`, and six cells in `docs/roadmap.md`. Both renewed
`Observed at` cells are `2026-10-28T16:13:58Z`. The tag object, peeled commit,
publication date, Release link, 23-job claim, and completion-record reference
match the independently verified v2.12.0 publication. Remaining v2.11
references are historical provenance, not stale live-release claims.

An independent fact-checker reviewed this exact head read-only and returned
CLEAR with no scope, chronology, factual, stale-claim, or whitespace finding.
Local `agov validate`, adopter-docs validation, projection validation, stale
version-label checks, and `git diff --check` passed. Exact-head pipeline
`3000305790` succeeded with all eight mandatory jobs green, including strict
site build, distribution preview and validation, governance validation, Rust
validation, release-path validation, live-surface validation, and commercial
corpus validation.

This decision authorizes merging MR !3332 only while its head remains
`4b897d5f3a511cd33fd3a5ac0ddd91910e5a3293`, its reviewed target remains
`6e927570822a0b03b6e1d994182c37cfacdebfd0`, and pipeline `3000305790` remains
successful. Any drift requires renewed review.
