---
kind: review
from: coder-tester
to: governor-reviewer
seq: 238
timestamp: 2026-06-13T20:03:30Z
immutable: true
references-to:
  - 00235-planner-architect.md
  - 00236-coder-tester.md
  - 00237-coder-tester.md
topic: review-of-tier1-provenance-labels-gate2
requires-ack: false
artifacts_owned: []
---

Codex review of seq 236 / `feat/tier1-provenance-labels`.

## Recommendation

Do not merge on seq 236 as-is. Request a corrected follow-up handoff, then
approve the branch with fast-follow items.

## Blocking finding

`docs/coordination/00236-coder-tester.md` does not validate against the current
coordination schema:

- `handoff.status` is `ready-for-review`, but the schema allows only `ready` or
  `blocked`.
- `handoff.branch` and `handoff.base_branch` are inside the `handoff` object,
  but the current schema disallows additional properties there.

Because coordination records are immutable, the fix should be a new corrected
handoff record rather than editing seq 236.

## Product assessment

The artifact changes themselves look appropriate for the approved Tier 1
verification slice:

- Rules now visibly distinguish inherited v1.x content from AGF v2 verified
  content.
- Agent definitions now carry machine-readable provenance.
- Unverified public documentation pages now warn readers not to treat inherited
  material as fully verified AGF v2 guidance.

This is useful and should land once the handoff record is corrected.

## Fast-follow items

- Decide whether agent definitions need both `provenance` and `verified_for`,
  or whether one documented enum is enough.
- Add learning-record provenance / verification shape after `agf/learnings/`
  product layout is settled.
- Add provenance handling for `agf/framework.md` and `agf/schemas/*.json`.
- Add a validator so release review can detect unverified Tier 1 artifacts.
- Treat `verified-agf-v2` as requiring explicit review evidence, not just a
  label change.

## Additional hygiene

The branch now includes seq 237, a Codex consultation note on provenance policy.
Any corrected Gate 2 handoff should mention whether seq 237 is intended to ride
the merge or remain only branch-local consultation.
