---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260703T102424Z-coder-tester-esjh
timestamp: 2026-07-03T10:24:24Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260703T100311Z-coder-tester-tbtp
topic: adopter-docs-parity-hard-gate
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the new hard gate for adopter-facing documentation parity.
    - Confirm `agov validate adopter-docs` is appropriate as a release
      preparation blocker.
  must_read:
    - tooling/rust/crates/agov-validators/src/adopter_docs.rs
    - tooling/rust/crates/agov/src/main.rs
    - tooling/rust/crates/agov/tests/validate.rs
    - agf/rules/always/downstream-impact-rule.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/ADOPTING.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
    - agf/docs/site/docs/operate/review-and-release.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 Handoff: Adopter Docs Parity Hard Gate

Implemented the adopter-facing documentation parity gate requested by the
governor.

Summary:

- Added `agov validate adopter-docs`.
- Included the new validator in umbrella `agov validate`.
- The validator compares changes from the latest reachable `v*` tag through
  `HEAD`.
- If adopter-relevant trigger paths changed, the check requires
  downstream-facing docs/templates in the same range or an
  `agf/CHANGELOG.md` waiver marker.
- Reference-only docs do not satisfy the gate.
- No reachable `v*` tag produces an advisory skip rather than a blocking
  failure.

Adopter docs parity: updated.

Downstream-facing coverage was updated in:

- `agf/ADOPTING.md`
- `agf/docs/site/docs/adopt/tooling-and-tokens.md`
- `agf/docs/site/docs/advanced/contracts-and-validators.md`
- `agf/docs/site/docs/operate/review-and-release.md`

Rule and release policy updates:

- `agf/rules/always/downstream-impact-rule.md` now requires a Gate 2
  `Adopter docs parity` note for adopter-relevant framework/tooling changes.
- `agf/rules/workflow/release-via-merge-request-rule.md` now requires
  `agov validate adopter-docs` and a release changelog adopter-docs marker for
  adopter-relevant release entries.

Validation:

- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov-validators adopter_docs`
  passed.
- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov --test validate`
  passed.
- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov-validators -p agov`
  passed.
- `cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov`
  passed.
- `./tooling/agov validate adopter-docs` passed.
- `./tooling/agov validate` passed with existing advisory warnings for
  transitional learning IDs and copied `.claude/rules` drift.
- `rg -n "Adopter docs parity|adopter-docs|Adopter docs:" agf/rules agf/docs/site agf/ADOPTING.md`
  found the expected downstream-facing coverage.

Notes:

- The validator is intentionally path-level evidence, not semantic docs
  judgment. Governor-reviewer still decides whether the documentation is
  sufficient.
- Existing uncommitted `AGOV_MODEL` work in `main.rs`, `coord_brief.rs`,
  `environment-variables.md`, `foundry.yaml`, and earlier coordination files
  was present before this handoff. I preserved it and layered this change on
  top.
