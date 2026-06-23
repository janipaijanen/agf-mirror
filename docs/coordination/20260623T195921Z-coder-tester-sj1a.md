---
kind: handoff
from: coder-tester
to: governor-reviewer
model: codex-gpt-5
id: 20260623T195921Z-coder-tester-sj1a
timestamp: 2026-06-23T19:59:21Z
immutable: true
references-to:
  - 20260623T194245Z-planner-architect-aahs
  - 20260623T194816Z-coder-tester-t8rm
topic: rust-only-guidance-cleanup-gate2
requires-ack: false
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - >-
      Review MR for active Rust-only guidance cleanup and decide whether to
      merge as Gate 2 or request a broader historical documentation sweep.
  must_read:
    - docs/coordination/20260623T195921Z-coder-tester-sj1a.md
  do_not_touch: []
artifacts_owned:
  - AGENTS.md
  - tooling/agov
  - tooling/rust/crates/agov/src/main.rs
  - tooling/elixir/lib/agov/brief_generator.ex
  - docs/reference/coord-visibility.md
  - docs/reference/tooling-token-roi.md
  - agf/skills/
  - agf/docs/governance/
  - agf/docs/site/
  - agf/ADOPTING.md
  - agf/agents/MANIFEST.yaml
  - agf/rules/
---

# Gate 2 handoff: remove active Mix guidance from agent/operator surfaces

Governor-reviewer requested that no "run mix" guidance remain in code,
documentation available from Git, or AI guidance files.

I made a focused cleanup pass over active operator and AI-facing surfaces:

- `tooling/agov` no longer falls back to legacy tooling when a Rust command is
  missing; it reports the missing Rust command as a tooling gap.
- Rust and legacy brief generators now print
  `run agov coord status --roles for details`, not the old Mix hint.
- `AGENTS.md`, `docs/reference/*`, active `agf/skills/*`, governance docs,
  `agf/ADOPTING.md`, and relevant rules/manifests now use Rust-shaped `agov`
  commands for active guidance.
- The exact `run mix` phrase was removed from the current checkout outside
  excluded immutable/bootstrap areas.

Verification performed:

```bash
git grep -n -i 'run mix' -- . ':!docs/coordination/**' ':!agents-govern/**' ':!tmp/**'
```

Result: no matches.

```bash
git grep -n -i -E 'run mix|mix agov|Elixir/Mix remains|Mix reference tasks|Mix-only|cd tooling/elixir|from `tooling/elixir/`|from tooling/elixir' -- AGENTS.md tooling/agov tooling/rust/crates/agov/src/main.rs tooling/elixir/lib/agov/brief_generator.ex docs/reference agf/skills agf/docs/site agf/docs/governance agf/rules agf/agents/MANIFEST.yaml agf/ADOPTING.md ':!agf/docs/site/search/search_index.json' ':!agf/docs/site/**/*.html'
```

Result: no matches.

```bash
./tooling/agov coord brief --id-format readable --from coder-tester --to governor-reviewer --kind note --topic rust-only-guidance-cleanup-smoke
```

Result: generated ownership hint says
`run agov coord status --roles for details`.

```bash
cargo test --manifest-path tooling/rust/Cargo.toml -p agov
```

Result: passed, 29 tests total across the CLI crate test targets.

Residual note: historical `docs/plans/**` still contain many old
`mix agov.*` references because those files are design/history records. I only
changed two exact `run mix` strings there. A full historical-plan rewrite should
be treated as release-hardening/editorial work so we do not silently distort
old decision context.
