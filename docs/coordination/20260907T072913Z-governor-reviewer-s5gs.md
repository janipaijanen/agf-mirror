---
kind: decision-record
from: governor-reviewer
to: all
id: 20260907T072913Z-governor-reviewer-s5gs
timestamp: 2026-09-07T07:29:13Z
model: human
immutable: true
references-to:
  - 20260907T045703Z-coder-tester-jqrm.md
  - 20260907T061405Z-planner-architect-7dkk.md
topic: agf-wide-role-assumption-handshake-effective-immediately
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# AGF-wide design enhancement: role-assumption handshake — effective immediately

Issued by the governor-reviewer as a binding in-session directive
(2026-09-07), transcribed to coordination as instructed ("AGF wide design
enhancement. MUST be effective immediately. MUST fall into every AIs
cold-start instructions, if no better place exists. MUST be handled through
coordination.").

## The rule (normative, effective on publication of this record)

When one AI assistant (the **initiator**) sends a coordination message — or
any request for action — that **assumes or asserts** another AI in the
discussion holds a particular role:

1. **The receiving assistant, if it does not recognize holding that role,
   MUST communicate that back** through coordination — an explicit record
   stating the role it actually holds and that it does not hold the assumed
   one. Silence, or being busy in another role, is **not** the same as being
   an unreachable peer and MUST NOT be treated as one by the initiator.

2. **The initiator MUST then explicitly ask the receiver to *temporarily*
   take that role** — a proper temporary-role-allocation proposal under the
   concurrent-session handshake (naming the exact task, artifacts/lane,
   authority basis, start/end conditions, and the role to resume) — before
   routing the matter to the governor as an unreachable-peer or
   disagreement escalation.

3. Only after step 2's temporary allocation is proposed **and** the
   receiver has had a real opportunity to accept, decline, or counter may
   the initiator treat the lane as blocked and escalate. A direct governor
   assignment may still settle the allocation at any point, but the
   affected sessions still acknowledge it so the execution lane is
   unambiguous.

This binds every active AI development session in every AGF-governed
project, immediately.

## Canonical home and discoverability

- **Canonical normative owner:**
  `agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`,
  the "Concurrent-session role allocation" section. That section already
  owns the temporary-role handshake; this enhancement adds the explicit
  "receiver MUST communicate back" and "initiator MUST then request
  temporary allocation before escalating" steps it is currently missing.
- **Cold-start fallback (required):** because this MUST be discoverable at
  cold start, a compact pointer to this rule/section goes into the root
  `AGENTS.md`, the distributable `agf/templates/AGENTS.md`, and every
  shipped specialist definition under `agf/agents/*.yaml`, at the same
  structural location and in the same bounded-batch manner as the universal
  polling / capability-review trigger pointer. Specialist files do not copy
  the algorithm.
- The full projection is a Gate 1 rule-refinement (planner-architect
  handoff filed alongside this record). Until that projection merges, **this
  decision-record is the effective normative source** and every active
  session follows it now.

## Downstream impact

BREAKING under `downstream-impact-rule.md` — it adds a required
communication step and a required pre-escalation step to inter-assistant
coordination behavior. Migration: adopters running concurrent AI sessions
adopt the handshake immediately; single-session adopters are unaffected in
practice until they run concurrent sessions. No schema, tooling, CI, or
release-metadata change is authorized by this record; a mechanical check,
if later proposed, is its own Gate 1 with blocking authority and a negative
test.

## Motivating evidence

- `jqrm` (2026-09-07): Codex, assuming a concurrent Claude session held
  planner-architect for RUW/v2.10, published a temporary-allocation
  proposal; the Claude session was active as contribution-auditor and did
  not reply; Codex escalated an "unreachable peer" via !2474 rather than
  first receiving an explicit "I do not hold that role" and then explicitly
  requesting the temporary allocation.
- The same shape recurred across the `232s` / `s1kq` / `6hqm` build /
  capability / infosec review routing, resolved only by a direct governor
  "proceed as proposed" assignment.
- Issue #121 (session-level addressing) tracks the underlying mechanism;
  this handshake is the behavioral discipline that operates correctly under
  today's role-only addressing.

## Ack

Every active AI session acknowledges this record on its next poll and
applies the handshake from now on.
