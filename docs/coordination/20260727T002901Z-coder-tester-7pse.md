---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260727T002901Z-coder-tester-7pse
timestamp: 2026-07-27T00:29:01Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260727T002901Z-governor-reviewer-m6g4
  - 20260727T001923Z-coder-tester-hrn6
  - 20260726T234538Z-planner-architect-snhe
  - 20260726T233427Z-coder-tester-p1qh
  - 20260726T233848Z-coder-tester-nqpk
topic: downstream-extension-namespace-doc-contract-gate1-planning
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a separate bounded Gate 1 plan for the v2.5.0 documentation and contract slice authorized by the governor decision.
    - Keep the deliverable explicitly Level 2 Surface-defined and define acceptance evidence that prevents claims of machine enforcement.
    - Include the compatibility position for existing bare IDs, the boundary between adopter presentation profiles and AGF's own supported UI locales, and worked rule, skill, and command examples.
    - Declare advisory consultation status for contribution-auditor and infosec-sentinel and route the completed plan for human Gate 1 review.
    - Track schema, registry, validator, migration, wrapper-collision, and negative-fixture enforcement as a separate v2.6.0 candidate without implementing it.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/671
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/672
    - docs/coordination/20260727T001923Z-coder-tester-hrn6.md
    - docs/coordination/20260727T002901Z-governor-reviewer-m6g4.md
    - agf/docs/site/docs/advanced/customizing-the-framework.md
    - agf/docs/site/docs/adopt/assistant-loader-configuration.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/start/adopter-faq.md
  do_not_touch:
    - docs/plans/v2.5.0-portfolio-discovery.md
    - agf/schemas
    - tooling
    - agf/VERSION
    - any release branch or tag
---

# Gate 1 planning authorized for the documentation-only slice

The governor has selected the documentation-only v2.5.0 slice recommended by
the independent challenge. Draft the smallest coherent plan that makes the
identity and extension contract understandable without presenting the proposed
schema or validator behavior as implemented.

This handoff authorizes planning only. Do not edit product documentation or
code while the plan is pending Gate 1, do not fold the work into blocked
MR !661, and do not treat the separately tracked v2.6.0 enforcement candidate
as approved implementation scope.
