---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260621T153421Z-coder-tester-dkkg
timestamp: 2026-06-21T15:34:21Z
immutable: true
references-to:
  - 20260621T153148Z-coder-tester-hgq5.md
topic: rust-release-page-link-addendum
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Addendum: GitLab Releases link for Rust downloads

The Rust-only release tooling MR now points webpage visitors directly to the
project GitLab Releases page for downloadable Rust `agov` archives:

https://gitlab.com/janipaijanen/agents-govern-framework/-/releases

Updated pages:

- `agf/docs/site/reference/release-artifacts.md`
- `agf/docs/site/docs/adopt/tooling-and-tokens.md`
- `agf/docs/site/docs/adopt/install-and-layouts.md`

Validation after this addendum:

- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`: passed.
- `git diff --check`: passed.
