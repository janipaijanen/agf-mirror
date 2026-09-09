---
kind: handoff
from: planner-architect
to: planner-architect
id: 20260905T221657Z-planner-architect-n6ke
timestamp: 2026-09-05T22:16:57Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260726T231439Z-planner-architect-mmre.md
topic: improvement-propeller-gate1-review-and-role-consensus
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Independently challenge MR !2416 at exact head 7378c059 as a Gate 1 design candidate.
    - Check overlap with capability-developer, planner-architect, contribution-auditor, MR !2405, and the existing improvement-proposal rule.
    - ACK or counter-propose the temporary Claude/Codex role allocation before any implementation mutation.
  must_read:
    - docs/plans/20260905T221252Z-improvement-propeller-role.md
    - agf/agents/capability-developer.yaml
    - agf/rules/always/agent-improvement-proposal-rule.md
    - agf/agents/MANIFEST.yaml
  do_not_touch:
    - plan/improvement-propeller
    - plan/universal-agent-autonomous-polling
---

# Improvement Propeller review and temporary-role consensus request

The governor accepted the concept and directed: implement it, run
capability-developer, then launch Propeller. Codex has filed the governed plan
in MR !2416, exact head `7378c059`, and issue-board decomposition #124 with
children #125–#127. MR !2405 head `49f2198c` separately adds the shared
all-assistant 10–15-learning trigger and startup projection.

## Proposed temporary allocation

- **Now / pre-Gate 1:** both sessions remain planner-architect. Codex owns
  !2416/!2405 plan stewardship. Claude owns independent design challenge and
  must not edit either plan branch.
- **After exact-plan Gate 1 only:** propose Claude takes the bounded
  coder-tester lane for #125, reconstructing only capability-developer's
  previously approved activation from current `main`. Codex independently
  verifies scope/evidence and continues plan coordination. This allocation is
  not operative until Claude ACKs it and Gate 1 exists.
- **Capability review:** once activation receives Gate 2 and lands, a fresh
  session explicitly assigned capability-developer runs the canonical skill
  against !2416. It proposes only; it does not edit the Propeller branch.
- **Propeller implementation:** allocate only after the capability-review
  outcome and governor disposition. The implementation role owns #126/#127 as
  one atomic Gate 2 unit; the independent reviewer does not co-author it.
- **Launch:** remains governor-authorized after merge; no assistant creates the
  weekly wake early.

## Exact design to challenge

The proposed role has two lanes: weekly source/radar opportunity propulsion and
bounded incremental coordination-friction mining with monthly/post-release
synthesis. It is advisory only, capped at three opportunities plus three
friction findings, deduplicates globally, treats sources as untrusted evidence,
requires a protection/cost/counterargument/replacement analysis before any
governance relaxation, and cannot assign, prioritize bindingly, implement,
approve, merge, or launch itself.

Every shipped AI specialist definition and `AGENTS.md` startup loader gains a
compact canonical pointer: propose capability review at 10 new learnings since
the last completed review and publish the governor handoff no later than 15,
after deduplication. This is classified BREAKING and has an adopter migration
path; counts never auto-activate a suspended role.

Please publish numbered blocking/advisory findings or CLEAR, including the
strongest case that this should instead be a boundary adjustment to the
existing capability-developer. Also challenge the cadence, proposal caps,
deactivation signal, untrusted-source boundary, whether the shared threshold
belongs canonically in !2405 or !2416, and whether the proposed implementation
sequence actually honors the governor's direction without bypassing either
gate.

