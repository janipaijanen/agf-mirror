---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T203226Z-coder-tester-kgmk
timestamp: 2026-06-21T20:32:26Z
immutable: true
references-to:
  - 20260621T202628Z-coder-tester-bjdh.md
topic: compact-agf-context-hygiene-docs
requires-ack: false
artifacts_owned:
  - .claude/commands/compact-agf.md
  - agf/docs/site/docs/operate/day-to-day.md
  - docs/reference/session-resume.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the documentation update that names `/compact-agf` in Context hygiene.
  must_read:
    - .claude/commands/compact-agf.md
    - agf/docs/site/docs/operate/day-to-day.md
    - docs/reference/session-resume.md
  do_not_touch:
    - .claude/settings.local.json
---

# Handoff: compact-agf Context hygiene docs

## Summary

The public Context hygiene section described the durable-state behavior but did
not name the shared `/compact-agf` instruction. That made the workflow harder
for humans to discover and harder for different AI surfaces to follow
consistently.

This branch:

- documents `/compact-agf` in the public Day-to-day operation page as the shared
  instruction to save AGF state to `docs/coordination/` before compacting;
- tells readers to run `agov session resume` after restart, model switch, or
  cold start;
- updates `docs/reference/session-resume.md` so `/compact-agf` is described as
  a tool-neutral convention for Claude, Codex, and other AI agents, even though
  the concrete project command definition currently lives under `.claude/`.
- removes stale Mix-first session-resume guidance from
  `docs/reference/session-resume.md`;
- removes the Mix fallback from `.claude/commands/compact-agf.md`; when the
  wrapper is unavailable, the command now tells agents to build/run the Rust
  implementation directly.

## Verification

- Pass: `./tooling/agov coord validate docs/coordination/20260621T203226Z-coder-tester-kgmk.md`
  - Advisory only: recommended `model` field is not set.
- Pass: `git diff --check`
- Pass: `/home/devi/.local/share/mise/installs/uv/0.11.20/uv-x86_64-unknown-linux-musl/uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`
- Pass: `/home/devi/.cargo/bin/cargo run --quiet --manifest-path tooling/rust/Cargo.toml -p agov -- session resume --base-branch main`
