---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260905T234835Z-coder-tester-hh5m
timestamp: 2026-09-05T23:48:35Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260905T224802Z-governor-reviewer-ksbs.md
  - 20260905T233121Z-coder-tester-bs3m.md
  - 20260905T233910Z-contribution-auditor-e8gj.md
topic: capability-developer-activation-gate2
requires-ack: true
artifacts_owned:
  - agf/agents/MANIFEST.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and decide Gate 2 for MR !2420 exact head e6630feb.
  must_read:
    - agf/agents/MANIFEST.yaml on MR !2420
    - docs/coordination/20260905T233910Z-contribution-auditor-e8gj.md on MR !2418
  do_not_touch:
    - feat/capability-developer-activation unless returning a finding to coder-tester
---

# Gate 2 handoff: capability-developer activation

MR !2420 exact head `e6630feb71ff3fd23c93c151d88036ef7cb27dcc`
is ready for binding Gate 2 review. It changes one file and one manifest entry,
reconstructing only the capability-developer activation freshly authorized by
`ksbs`; stale MR !658 remains evidence and contributes no unrelated diff.

Evidence:

- canonical MR pipeline `2823376541`: success, all six jobs green;
- `agov validate agents`: all 14 agent definitions pass;
- aggregate `agov validate`: blocking checks pass; warnings are pre-existing
  and unrelated;
- `git diff --check`: pass;
- independent different-model review `e8gj`: **NO OBJECTION**;
- MR is mergeable against current `main` at the poll immediately preceding
  this handoff.

Downstream impact: **ADDITIVE, foundry-local**. The authoritative project
manifest activates an existing shipped role. `MANIFEST.example.yaml` remains
skipped, so adopters receive no activation or behavior change.

Review advisories are dispositioned as follows:

1. `AGENTS.md` still summarizes capability-developer as suspended. It is not
   authoritative and is outside `ksbs`'s one-manifest reconstruction scope, so
   it is not smuggled into !2420. A bounded consistency follow-up must land
   immediately after activation and before any capability-developer invocation;
   until then the role must not start.
2. `played_by` intentionally says “AI advisory (explicitly assigned for each
   capability-review invocation)”. The governor remains the separate binding
   human role; this avoids inventing permanent human co-staffing not present in
   either the old approved patch or current direction.
3. The missing impact classification is supplied above.

Gate 2 approval authorizes merging exactly this head. It does not authorize the
subsequent AGENTS consistency MR, capability-review execution before that sync,
Propeller implementation/activation, heartbeat launch, tag, or release.
