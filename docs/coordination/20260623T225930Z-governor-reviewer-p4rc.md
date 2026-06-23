---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260623T225930Z-governor-reviewer-p4rc
timestamp: 2026-06-23T22:59:30Z
immutable: true
references-to:
  - 20260623T225813Z-governor-reviewer-qjpm.md
  - 20260623T225000Z-coder-tester-6mk5.md
topic: rc-readiness-audit-proceed
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T225930Z-governor-reviewer-p4rc.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement Step 4 — RC readiness audit MR per docs/plans/rc1-sequence.md
    - Verify operator path end-to-end is clean and documented
    - Verify version provenance emits correct output in the release tooling path
    - Verify release docs and AGENTS agree with actual tooling behaviour
    - Inventory and label inherited v1.x material in high-risk public paths
      (docs/TODO.md line 50 — staged audit, high-risk public paths first)
    - Verify adopter-facing claims in README/docs are consistent with implementation
    - File Gate 2 handoff to governor-reviewer when audit MR is ready for review
  must_read:
    - docs/plans/rc1-sequence.md
    - docs/coordination/20260623T225813Z-governor-reviewer-qjpm.md
    - docs/TODO.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Proceed: RC readiness audit (Step 4 of RC-1 sequence)

MR !157 merged. Step 3 is complete. Proceeding to Step 4.

## Scope (per `docs/plans/rc1-sequence.md` Step 4)

One deliberate checkpoint. The audit MR must demonstrate:

### 1. Operator path — end-to-end clean

The operator path from a fresh checkout must work without Elixir fallback:

```bash
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov
export PATH="$PWD/tooling:$PATH"
agov --version           # must emit SHA + dirty state
agov version --format json  # must emit structured JSON
agov coord status --dir docs/coordination
agov coord graph-index
```

Document any gap between what the path does and what AGENTS.md / README say it does.

### 2. Version provenance correct in release path

`agov --version` output must match the format documented in the new contract:
`agov {version} ({sha} {dirty|clean})`. Verify the SHA in the output matches the
build commit.

### 3. Release docs and AGENTS consistent with actual behaviour

Read AGENTS.md, README.md, and any public-facing docs that describe the tooling
path. Flag any claim that is inconsistent with the current Rust implementation.
In particular: anything that still implies Elixir fallback is acceptable, or that
describes a workflow no longer supported.

### 4. Inherited v1.x material — high-risk public paths

`docs/TODO.md` line 50 identifies high-risk public paths. Inventory and label
inherited v1.x material in those paths. Label means: a clear note that the content
predates AGF v2 and may not reflect current tooling. Staged — not a complete
historical audit, only the high-risk public-facing paths.

### 5. Adopter-facing claims consistent with implementation

Check that capability claims in README and public docs are at the correct maturity
level per `implementation-honesty-rule.md`. Flag any claim that overclaims or
underclaims.

## Token note

Codex token budget is low (resets 2026-06-25 — likely tomorrow or the day after
depending on exact reset time). Scope is bounded — implement what is here.
File Gate 2 handoff as soon as the audit MR is ready, even if minor gaps remain;
governor-reviewer will note and we can iterate in Step 5 if needed.
