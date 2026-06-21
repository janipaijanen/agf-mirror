---
id: 20260621T140113Z-coder-tester-p9cw
kind: handoff
from: coder-tester
to: planner-architect
timestamp: 2026-06-21T14:01:13Z
immutable: true
references-to:
  - docs/plans/rust-agov-complete.md
  - docs/coordination/20260621T135434Z-planner-architect-c6vp.md
topic: ci-rationalization-and-windows-rust-preview
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
model: gpt-5-codex
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the CI scoping and Windows x64 Rust preview artifact slice.
    - Confirm whether this is acceptable as the first packaging/CI step before
      the larger Rust `dist build`, `tooling build`, and `bundle build` port.
    - Route the next implementation slice toward replacing Elixir dist/tooling
      packaging jobs with Rust commands.
  must_read:
    - .gitlab-ci.yml
    - docs/coordination/20260621T135434Z-planner-architect-c6vp.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# CI Rationalization and Windows Rust Preview Slice

Codex implemented the first packaging/CI follow-up on `feat/rust-packaging-ci`.

Implemented behavior:

- path-scoped `site-html-build` and `pages` so coordination-only changes do not
  build the docs site;
- path-scoped `elixir:test` to `tooling/elixir/**` and `.gitlab-ci.yml`;
- path-scoped Elixir framework dist preview/validation jobs to `agf/**` and
  `.gitlab-ci.yml`;
- path-scoped Elixir tooling and bundle preview jobs to their relevant source
  trees;
- kept Linux Rust preview artifacts automatic;
- added a separate manual `rust:tooling:build:windows:preview` job for
  `x86_64-pc-windows-gnu` and the Windows `agov.exe` preview artifact.

Validation run:

- GitLab CI lint API:
  - `valid: true`
  - `errors: []`
  - `warnings: []`

Notes:

- This is not the full Rust packaging port. It is an immediate CI rationalization
  and Windows preview-binary availability step.
- Because this branch changes `.gitlab-ci.yml`, its own MR pipeline will still
  run the broad job set. The savings appear on subsequent Rust-only,
  docs-site-only, and coordination-only branches.
- The Windows preview job is manual so every normal MR does not pay the extra
  cross-build cost. Release preparation can trigger it when a Windows x64
  artifact is needed.
- The larger remaining work is to implement Rust `agov dist build`,
  `agov tooling build`, and `agov bundle build`, then remove the Elixir packaging
  jobs from CI entirely.
