---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260623T230927Z-coder-tester-gerh
timestamp: 2026-06-23T23:09:27Z
immutable: true
model: gpt-5-codex
references-to:
  - 20260623T225930Z-governor-reviewer-p4rc
topic: rc-readiness-audit-gate2
requires-ack: false
artifacts_owned:
  - docs/audits/rc1-readiness-audit.md
  - AGENTS.md
  - README.md
  - agf/docs/site/index.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/site/reference/tooling-conformance.md
  - agf/ADOPTING.md
  - agf/framework.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR for RC readiness audit follow-up from p4rc.
    - Confirm operator path, docs consistency, and v1.x material labels are sufficient for the RC sequence.
  must_read:
    - docs/audits/rc1-readiness-audit.md
    - AGENTS.md
    - README.md
    - agf/docs/site/reference/tooling-conformance.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# RC Readiness Audit Gate 2

Step 4 from `20260623T225930Z-governor-reviewer-p4rc` is ready for Gate 2.

Summary:

- Verified the supported operator path with a local release Rust build and repo-local launcher.
- Confirmed version provenance surfaces report the current commit through `agov --version` and `agov version --format json`.
- Added `docs/audits/rc1-readiness-audit.md` with operator-path evidence, docs consistency checks, inherited v1.x inventory, adopter-facing claim check, and residual risks.
- Updated AGENTS/README/public docs so `version`, `coord graph`, and `coord graph-index` are represented in the documented Rust surface.
- Labelled high-risk inherited v1.x public material in `agf/ADOPTING.md` and `agf/framework.md` as adapted from v1.x and only partially verified for AGF v2.
- Aligned project branch examples in `AGENTS.md` from `master` to `main`.

Validation performed:

- `cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov`
- `agov --version`
- `agov version --format json`
- `agov coord status --dir docs/coordination`
- `agov coord graph-index --output tmp/rc1-graph-index.json`
- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov`
- `./tooling/agov coord validate docs/coordination/20260623T230927Z-coder-tester-gerh.md`
- `git diff --check`

Docs build caveat: `python -m mkdocs build -f agf/docs/build/mkdocs.yml --strict` could not be run in this checkout because `mkdocs` is not installed (`No module named mkdocs`).
