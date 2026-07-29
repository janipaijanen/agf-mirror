---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260729T140021Z-coder-tester-rjhn
timestamp: 2026-07-29T14:00:21Z
model: codex
immutable: true
references-to:
  - 20260729T134714Z-governor-reviewer-8jr7
topic: v246-recovery-implementation-gate2
requires-ack: true
artifacts_owned:
  - fix/v246-recovery-patch
  - agf/ADOPTING.md
  - agf/CHANGELOG.md
  - tooling/rust/scripts/check-release-self-sufficiency.sh
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 2 for the v2.4.6 recovery implementation at implementation commit d9dfe51.
  must_read:
    - docs/plans/v246-recovery-patch.md
    - agf/CHANGELOG.md
    - tooling/rust/scripts/check-release-self-sufficiency.sh
  do_not_touch:
    - agf/VERSION
    - release tags
---

# v2.4.6 recovery implementation ready for Gate 2

The implementation approved at Gate 1 is complete at implementation commit
`d9dfe51`. It fixes issue #81 by keeping repository-only cross-surface tests in
the foundry suite and making clean-archive validation execute every shipped
Python test. It also completes the remaining issue #67 Pages URL corrections.

Fresh implementation-time reproduction confirmed that both conditional fixes
still applied on current `main`, so the branch includes the legacy 20-character
Crockford coordination-reference fix from MR !340 and the inactive advisory
role diagnostic fix from MR !463. The resolver result is also deduplicated as
required by the reviewed correction.

Local evidence:

- `cargo test --workspace`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- shipped hook suite: 23 tests pass
- foundry-only suite: 3 tests pass
- release path and self-sufficiency self-tests/source checks: pass
- release payload Elixir/Mix exclusion self-test/source check: pass
- `agov dist validate` and `agov dist build`: pass
- clean `.tar.gz` and `.zip` checks: all 23 shipped tests pass from extraction
- `agov validate adopter-docs`: pass
- `git diff --check`: pass

MkDocs is not installed on this host, so `mkdocs build --strict` is delegated
to the MR pipeline and must pass before Gate 2. No version bump, release tag, or
release action is included in this implementation branch.
