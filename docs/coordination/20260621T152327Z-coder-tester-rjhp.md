---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T152327Z-coder-tester-rjhp
timestamp: 2026-06-21T15:23:27Z
immutable: true
references-to: []
topic: compact-agf-command-gate2
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the project `/compact-agf` command update.
    - Decide whether the AGF-native coordination-note save path should replace the previous private Claude memory path.
  must_read:
    - .claude/commands/compact-agf.md
    - docs/reference/session-resume.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: compact-agf command

## What changed

The existing `.claude/commands/compact-agf.md` command no longer writes session
state to a private Claude memory path. It now instructs the agent to save
pre-compaction state as an ordinary validated coordination message under
`docs/coordination/`.

The command records the role, branch, worktree, latest commit, dirty tree,
active request, completed work, validation, open work, must-read files, and
false assumptions to avoid. It explicitly tells agents not to rely on
background processes, timers, terminal sessions, or model memory for resume.

`docs/reference/session-resume.md` now points Claude Code users to
`/compact-agf` before `/compact`.

## Why

The previous command wrote to a local Claude memory file outside the repository
audit trail. That does not satisfy AGF's cold-start requirement for other agents
or reviewers. Coordination notes are already validated, versioned, reviewable,
and readable by the existing session-resume flow.

## Validation

- `git diff --check`: passed.
- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`: passed.
- Coordination ID generated with the Rust `agov coord brief` implementation:
  `20260621T152327Z-coder-tester-rjhp`.

Environment note: this fresh worktree prints a local `mise.toml` trust warning
on shell startup. The repo-local `./tooling/agov` wrapper did not run in that
environment, so the coordination ID was generated with:

```bash
cd tooling/rust
cargo run -q -p agov -- coord brief --id-format readable --from coder-tester --to governor-reviewer --kind handoff --topic compact-agf-command-gate2
```

## Review focus

- Confirm that `/compact-agf` should write to `docs/coordination/` rather than
  local Claude memory.
- Confirm `kind: note` is the right default for ordinary compaction saves, with
  `kind: handoff` reserved for saves that create a concrete downstream
  obligation.
- Confirm the command should stop after reporting readiness for `/compact`.
