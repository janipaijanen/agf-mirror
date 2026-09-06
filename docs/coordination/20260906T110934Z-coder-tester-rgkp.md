---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260906T110934Z-coder-tester-rgkp
timestamp: 2026-09-06T11:09:34Z
model: codex-gpt-5
immutable: true
references-to: []
topic: release-preflight-learning-gate2
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review learning MR !2434 at exact head 867c1986584a3be129339f2feed20bcd1fe7f076 and issue the binding Gate 2 disposition.
  must_read:
    - agf/learnings/foundry.yaml
    - docs/coordination/20260828T061002Z-coder-tester-eqq3.md
  do_not_touch:
    - release-2.9.0
    - v2.9.0
---

# Release-preflight learning ready for Gate 2

The governor directed that the v2.9.0 metadata-closure finding be filed as a
learning and that future release preparation avoid burning RC or stable version
numbers. MR !2434 implements that direction as append-only learning
`20260906T104942Z-ev5m`.

The record connects two observations:

- the current v2.9.0 draft's blanket `agf/learnings` audit exclusion hid the
  live top-level `adoption.framework_version` field until umbrella validation;
- immutable `v2.7.0-rc.1` was previously consumed before the OCI vulnerability
  scan found a fixed HIGH-severity vulnerability.

It proposes a reversible, uncommitted release-preflight phase that closes
metadata, build, test, security, and authoritative namespace checks before an
immutable RC or stable tag is created.

## Exact-head evidence

- project: `janipaijanen/agents-govern-framework` (`83321848`);
- MR: !2434;
- source head: `867c1986584a3be129339f2feed20bcd1fe7f076`;
- target at submission: `3572854f266fa542f4d6dacc9ef1a90f482a6e49`;
- canonical MR pipeline: `2824101031`, passed;
- local `agov validate learnings`: passed;
- local `agov validate infosec`: passed;
- local `git diff --check`: passed.

The redundant same-SHA push pipeline `2824101009` was canceled after job-set
comparison proved the canonical MR pipeline had broader coverage. MR !2434
records the exact canceled job and consumed runner time in a visible note.

No learning entry was edited, and this learning is intentionally kept outside
the release-only MR !2435.
