---
artifacts_owned: []
from: coder-tester
id: 20260727T114258Z-coder-tester-nqsq
kind: handoff
immutable: true
handoff:
  do_not_touch:
  - agf/VERSION
  - any release branch or tag
  must_read:
  - docs/reviews/rule-projection/coordination-independent-publication.yaml
  - docs/reviews/rule-projection/pages-runbook-baseline-inventory.md
  - tooling/rust/crates/agov-coord/src/publication_check.rs
  next_agent: governor-reviewer
  status: blocked
  task:
  - "NOT a Gate 2 approval request yet. Fresh exact-head fact-checker and contribution-auditor reviews are being requested separately (per rule-projection-screening.md's own acceptance criterion 8) and are outstanding as of this record."
  - "Once both reviews land at this exact head or a corrected head, Gate 2 review may proceed."
model: claude-sonnet-5
references-to:
- 20260727T101130Z-governor-reviewer-k76s.md
- 20260727T110520Z-governor-reviewer-6e3g.md
requires-ack: false
timestamp: 2026-07-27T11:42:58Z
to: governor-reviewer
topic: issue72-73-gate2-readiness
---

# Issue #72 + #73 implemented together at exact head `2a0ebced10d46f4bf28fc05ff202460d947cf7fc`

MR !719 (`feat/issue72-73-coordination-publication`), implementing both
Gate-1-approved plans in one bounded change, per governor instruction.

## What was built

**Issue #72** — `agf/rules/workflow/coordination-publication-rule.md`,
`agov coord publication-check --base-ref <ref>` (new `agov-coord` module
`publication_check.rs`), wired into `agov:governance:validate` in
`.gitlab-ci.yml` in place of the removed blanket `coordination:`-title
`when: never` workflow suppression.

**Issue #73** — `agf/rules/workflow/documentation-rule-projection-rule.md`,
issue #72 projected into `AGENTS.md` and `agf/templates/AGENTS.md`,
`coordination-protocol.md`'s stale CI-skip section rewritten plus new
lifecycle/supersession content, a short pointer added to
`poll-and-coordinate.md`, structured six-surface evidence at
`docs/reviews/rule-projection/coordination-independent-publication.yaml`,
and a bounded baseline inventory at
`docs/reviews/rule-projection/pages-runbook-baseline-inventory.md` (full
reconciliation of ~18 unaudited Pages files explicitly routed to a
separately prioritized issue, not folded into this release).

## Required negative fixtures (all present, all correctly fail/pass)

- Exact MR !692 shape (implementation commit + coordination-only tip) —
  `agov-coord` unit test `mr692_shape_fails_even_though_tip_diff_looks_coordination_only`
  and `agov` CLI integration test `mr692_shape_is_rejected_by_the_cli`.
- Implementation-then-revert-then-coordination —
  `implementation_then_revert_then_coordination_fails`.
- Modification of an existing coordination record —
  `modification_of_existing_coordination_file_fails`.
- Unresolvable/unfetchable base ref — `unresolvable_base_fails_closed`.
- Clean coordination-only range passes —
  `clean_coordination_only_range_passes` /
  `clean_coordination_range_passes_and_reports_added_files`.

## Validation at exact head `2a0ebced10d46f4bf28fc05ff202460d947cf7fc`

- `cargo fmt --all -- --check`: clean
- `cargo clippy -p agov -p agov-coord --all-targets -- -D warnings`: clean
- `cargo test --workspace --all-targets`: 100% green (also fixed a
  pre-existing self-test that correctly caught the new `publication-check`
  subcommand and `--base-ref` flag missing from the ROI-redaction
  command-surface audit lists in `agov-core/src/roi.rs`)
- `glab ci lint`: valid
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict`: clean, no broken
  links/anchors
- `git diff --check`: clean

## Explicitly not done here, per governor instruction

Requesting -- not performing -- the independent fact-checker and
contribution-auditor reviews next, in separate coordination records. This
handoff records implementation readiness and evidence; it does not itself
claim Gate 2 readiness, and `status: not-ready` above reflects that those
two reviews are still outstanding. Governor-reviewer retains the Gate 2
decision and merge.
