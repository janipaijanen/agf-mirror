---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260810T172655Z-contribution-auditor-gagk
timestamp: 2026-08-10T17:26:55Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260802T063735Z-planner-architect-assm
topic: openclaw-governor-spinoff-future-assessment
requires-ack: false
artifacts_owned: []
---

# Future assessment: isolated OpenClaw-governor spin-off

The governor asked that a possible spin-off supporting OpenClaw as a governor
or first-tier governor-like component be retained for future assessment. The
durable tracker is GitLab issue #104:

<https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/104>

This is radar only: Level 1 Planned / Theoretical, unmilestoned, and not an
authorization to plan implementation, install OpenClaw, provision credentials,
alter roles or Gates, create a spin-off repository, or add release scope. It
must not displace or become coupled to the current AGF release and roadmap
lanes.

The supplied comparative AI assessments disagree on basic OpenClaw topology,
remote-harness, ACP, node, session-continuation, and release-channel facts.
Treat all such statements as research leads. A promoted assessment must verify
exact versions against primary OpenClaw, Claude CLI, and Codex CLI sources
before using them as plan premises.

Issue #104 deliberately separates five authority tiers: observer, dispatcher,
human governor proxy, policy-bounded machine governor, and autonomous governor.
The recommended first experiment, if later authorized, is the dispatcher tier:
OpenClaw may route already-authorized work and status but has no Gate 1, Gate 2,
merge, release, risk-acceptance, or role-status authority. Any higher tier needs
an explicit threat model, authenticated content-bound human decisions, and
likely a hard product boundary from AGF.

When the governor explicitly promotes this radar item, planner-architect owns a
staged Gate 1 program with contribution-auditor, infosec-sentinel,
build-engineer, and fact-checker participation: discovery/threat model first,
then architecture and product-boundary decision, then a synthetic-data proof
of concept for the lowest approved tier, then independent security/evidence
review. Do not collapse those stages into one implementation plan.

No acknowledgement or action is requested now. This record exists so a future
Claude/planner session can recover the intent and its release-containment
boundary without relying on chat memory.
