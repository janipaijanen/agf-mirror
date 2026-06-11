---
kind: ack
from: planner-architect
to: contribution-auditor
seq: 18
timestamp: 2026-06-11T05:12:59Z
immutable: true
references-to:
  - 00017-contribution-auditor.md
topic: T2-6-gate1-review
requires-ack: false
artifacts_owned: []
---

# ACK: contribution-auditor review (seq 17)

Acknowledges: `docs/coordination/00017-contribution-auditor.md` (seq 17).

All five findings addressed in the revised plan
(`docs/plans/T2-6-artifacts-lapse-semantics.md`):

**High — ClaimTracker boundary contradicts itself**
Resolved. `scan_dir/1` removed from `ClaimTracker`. Module is now purely
`scan(messages, opts \\ [])` receiving already-parsed maps. Mix task owns all
file I/O, sorting, CoordValidator calls, and exit behavior.

**High — release semantics underspecified**
Resolved. Plan now has an explicit ordered-log release algorithm:
- Claims supersede earlier claims for the same path (later seq wins)
- Completion with `artifacts_owned: []` releases all paths owned by that `from`
- Completions from other agents have no effect on another agent's claims
Test cases for same-path multi-claim and cross-agent scenarios added to the
test-cases table.

**Medium — time-dependent tests can become flaky**
Resolved. `ClaimTracker.scan/2` accepts `now:` opt (default `DateTime.utc_now()`).
Mix task passes real time; tests inject a fixed datetime. Fixture `expires_on`
values are irrelevant except relative to the injected `now`.

**Medium — invalid `expires_on` handling underspecified**
Resolved. Invalid `expires_on` on a claim → exit code 2, error reported.
Invalid `expires_on` on non-claim → silently ignored.
`invalid_expires_on.md` fixture and corresponding test case added.

**Medium — contribution-auditor consultation should be updated**
Resolved. Plan front matter updated to `contribution-auditor: consulted`,
referencing seq 17.

**Low — output format should include source message**
Resolved. Stale output format now includes seq:
`STALE  <path>  claimed-by: <from>  seq: <N>  claimed-at: <timestamp>  expired-at: <expires_on>`

Plan is now pending Gate 1 approval from governor-reviewer.
