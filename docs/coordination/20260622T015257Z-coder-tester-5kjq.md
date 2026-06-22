---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260622T015257Z-coder-tester-5kjq
timestamp: 2026-06-22T01:52:57Z
model: codex-gpt-5
immutable: true
references-to: []
topic: "Gate 2 handoff: release artifacts implementation boundary"
requires-ack: false
artifacts_owned:
  - agf/docs/site/reference/release-artifacts.md
  - docs/coordination/20260622T015257Z-coder-tester-5kjq.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the release artifacts wording that separates public release surfaces from current packaging implementation details.
    - Decide whether to merge the docs-only MR.
  must_read:
    - agf/docs/site/reference/release-artifacts.md
    - docs/coordination/20260622T015257Z-coder-tester-5kjq.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: release artifacts implementation boundary

Branch: `docs/release-artifacts-implementation-boundary`

Summary:

- Reframes `reference/release-artifacts.md` so the top of the page leads with
  the two published release surfaces:
  - framework source archives;
  - Rust `agov` adopter tooling archives.
- Labels Elixir/Mix commands as current source-archive packaging implementation
  details, not adopter-facing tooling requirements.
- Keeps the factual CI state intact: framework source archives are still built
  by Mix-backed packaging jobs; GitLab Releases publish Rust `agov` tooling
  archives, not Elixir/ERTS `agov` packages.

Validation:

- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` via `uvx` with docs
  dependencies: passed.
- `cd tooling/rust && cargo run --quiet -p agov -- validate`: passed.
- `git diff --check`: passed.

Ready for Gate 2 review.
