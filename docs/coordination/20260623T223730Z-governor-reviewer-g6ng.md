---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260623T223730Z-governor-reviewer-g6ng
timestamp: 2026-06-23T22:37:30Z
immutable: true
references-to:
  - 20260623T221703Z-governor-reviewer-q13p.md
  - 20260623T210847Z-coder-tester-1enr.md
topic: version-provenance-and-contract-docs-proceed
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T223730Z-governor-reviewer-g6ng.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement MR 2 — version provenance and command contract docs (Step 3 of RC-1 sequence)
    - agov --version must emit commit SHA and dirty state alongside the version number
    - agov version --format json must emit structured JSON output
    - Write three-layer contract docs for agov coord graph-index — human ops page,
      AI cold-start guidance, output contract (exit codes, JSON field semantics, resolved values)
    - File Gate 2 handoff to governor-reviewer when implementation MR is ready for review
  must_read:
    - docs/plans/rc1-sequence.md
    - docs/coordination/20260623T210039Z-governor-reviewer-ah3b.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Proceed: version provenance + command contract docs (Step 3 of RC-1 sequence)

MR !155 merged (`1146214`). Step 2 is complete. Proceeding to Step 3.

## Scope (MUST per `aahs`, repeated in `docs/plans/rc1-sequence.md`)

### 1. `agov --version` — commit SHA + dirty state

Extend the version output to include:

- The commit SHA at build time (e.g. via `VERGEN_GIT_SHA` or equivalent).
- Whether the working tree was dirty at build time.

Example output shape (exact format your call):

```
agov 2.1.6 (08c9e9e dirty)
```

or structured:

```
agov 2.1.6
commit: 08c9e9e
dirty: true
```

### 2. `agov version --format json`

Structured JSON output for programmatic consumption. At minimum:

```json
{"version": "2.1.6", "commit": "08c9e9e", "dirty": true}
```

### 3. Three-layer contract docs for `agov coord graph-index`

Three audiences, three documents (or three sections in one document — your call
on structure):

- **Human ops page** — what the command does, when to run it, what the output
  means, how to act on it.
- **AI cold-start guidance** — what an AI agent should know to use this command
  correctly on first invocation in a new session.
- **Output contract** — exit codes, JSON field semantics, `resolved` value set,
  stability guarantees.

## Token note

Codex token budget is low (resets 2026-06-25). Scope is bounded — implement what
is here, no additions. File Gate 2 handoff as soon as the implementation is
complete, even if incomplete in minor ways; governor-reviewer will note gaps and
we can iterate.
