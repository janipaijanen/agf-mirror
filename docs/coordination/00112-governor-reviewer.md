---
kind: decision-record
from: governor-reviewer
to: all
seq: 112
timestamp: 2026-06-12T20:41:40Z
immutable: true
references-to:
  - 00105-coder-tester.md
  - 00106-coder-tester.md
  - 00110-coder-tester.md
topic: disposition-foreign-tooling-commits-before-v2-merge
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Decision: land tooling-fix commits on master before v2 structural review

Governor-reviewer disposition for the foreign commits identified in seq 106 and
requested in the Gate 2 handoff seq 110:

Cherry-pick the three Codex tooling-fix commits to `master` first, then review
and merge `feat/agents-govern-2.0-structure` as its own structural decision.

## Completed cherry-picks

The following commits were cherry-picked to `master`:

- `c0f43a8` — `learning: record handoff queue resolution mismatch`
- `fbaf98b` — `fix: make handoff next resolution-aware`
- `8a56c7c` — `coordination: complete handoff next resolution fix (seq 105)`

This makes seq 105 and the `handoff.next` fix visible on master before the v2.0
structural branch is reviewed.

## Additional learning/rule hardening

The repeated wrong-branch pattern was also recorded and hardened:

- `1cf06e4` — `learning: require active branch preflight`

That commit adds learning `L007` and
`spec/rules-template/always/active-branch-preflight-rule.md`.

## Verification

```text
mix test test/agov/handoff_reader_test.exs
8 passed

mix test
107 passed (1 doctest, 106 tests)

agov-validate-learnings learnings/foundry.yaml
0 errors, 0 warnings

agov-validate-infosec learnings/foundry.yaml
0 findings
```

Smoke check:

```text
mix agov.handoff.next governor-reviewer
Source: 00035-planner-architect.md
```

The stale seq 100 handoff no longer appears from master.
