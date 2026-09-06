---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260905T224802Z-governor-reviewer-ksbs
timestamp: 2026-09-05T22:48:02Z
model: human
immutable: true
references-to:
  - 20260905T222844Z-planner-architect-5q4s.md
  - 20260905T223840Z-contribution-auditor-2b6h.md
topic: improvement-propeller-gate1-and-capability-activation
requires-ack: true
artifacts_owned:
  - docs/plans/20260905T221252Z-improvement-propeller-role.md
  - agf/agents/MANIFEST.yaml
---

# Governor approval: Propeller progression and capability-developer activation

Codex transcribes the governor's direct statement in the active session:
“approved to proceed with Propeller implementation and capability-developer
activation remain gated.” The immediately preceding status identified those as
the two gated lanes. This record takes the narrow, execution-safe reading that
the governor approves both lanes to advance subject to the plan's named
prerequisites; it does not infer approval of a future implementation diff.

## Decisions

1. The governor freshly reaffirms activation of `capability-developer` in the
   current agent landscape. Reconstructing only that manifest activation from
   fresh `main`, with current validation and a new exact-head Gate 2 handoff,
   is authorized now. The stale MR !658 remains evidence only and must not be
   merged or rebased wholesale.
2. The governor accepts the narrow, human-directed exception to the project's
   normal two-adopter evidence preference for creating the dedicated
   `improvement-propeller` role. This is not a general weakening of that
   precedent.
3. MR !2416 exact plan head `b8a3d27d` is approved at Gate 1 **conditional on**
   the separately requested infosec-sentinel exact-head review returning no
   unresolved blocking finding. Contribution-auditor record `2b6h` is already
   NO OBJECTION. When the infosec condition is satisfied, Gate 1 becomes
   effective without another governor prompt; a blocking finding returns the
   plan to revision and requires a new exact-head decision.
4. MR !2405 is the sole normative owner of the universal 10/15-new-learning
   trigger and its projection into root/adopter `AGENTS.md` and every shipped
   specialist definition. Its canonical rule must merge before, or atomically
   with, the Propeller implementation so no shipped pointer is unresolved.
5. This decision authorizes planning and bounded implementation through fresh
   Gate 2 handoffs. It does not approve Gate 2, merge, manifest activation of
   Propeller, creation of its weekly heartbeat, tag, release, or publication.

The temporary Claude/Codex implementation allocation still requires explicit
durable consensus before either session mutates the capability-developer or
Propeller implementation branches. Human approval supplies authority; it does
not silently select the concurrent executor.
