---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260703T100311Z-coder-tester-tbtp
timestamp: 2026-07-03T10:03:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260703T095621Z-coder-tester-s4ag
topic: missing-model-coord-brief-ux
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review implementation of `agov coord brief --model` and `AGOV_MODEL`
      fallback.
    - Confirm the missing-model advisory warning is no longer produced for
      generated coordination records when model provenance is configured.
  must_read:
    - tooling/rust/crates/agov/src/main.rs
    - tooling/rust/crates/agov/tests/coord_brief.rs
    - agf/docs/site/reference/environment-variables.md
    - ai-cold-start.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 Handoff: Missing Model UX Fix

Implemented the missing-model UX improvement in Rust `agov`:

- `agov coord brief` now accepts `--model <name>`.
- `agov coord brief` falls back to `AGOV_MODEL` when `--model` is omitted.
- Explicit `--model` takes precedence over `AGOV_MODEL`.
- Generated front matter includes `model:` only when a non-empty value is
  available, avoiding invented provenance for humans or CI.
- The canonical environment variable reference documents `AGOV_MODEL`.
- The local cold-start overlay exports `AGOV_MODEL=codex-gpt-5` for future
  Codex sessions in this checkout.

Validation:

- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov` passed.
- `cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov`
  passed.
- Wrapper smoke test with `AGOV_MODEL=codex-gpt-5` showed generated
  coordination front matter includes `model: codex-gpt-5`.
- `agov coord validate` on the generated message passed blocking validation
  without the missing-model advisory warning; the only advisory warning in the
  smoke test was the intentional temporary filename mismatch.

Formatting note:

- `cargo fmt --manifest-path tooling/rust/crates/agov/Cargo.toml --check`
  still reports pre-existing formatting drift in unrelated lines/files. I did
  not bulk-format unrelated code in this change.
