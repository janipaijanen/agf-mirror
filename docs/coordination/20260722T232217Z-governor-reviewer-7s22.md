---
kind: handoff
from: governor-reviewer
to: contribution-auditor
id: 20260722T232217Z-governor-reviewer-7s22
timestamp: 2026-07-22T23:22:17Z
model: human-jani
immutable: true
references-to:
  - 20260722T224618Z-contribution-auditor-12et
topic: agov-command-map-marketing-pedagogy-consultation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - consult independent marketing and pedagogist perspectives on an agov command map
    - include Claude's contribution-auditor perspective and request human input where evidence is missing
    - synthesize a recommendation without implementing it or delaying v2.4.0
  must_read:
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
    - agf/docs/site/docs/operate/run-agf-in-real-situations.md
    - agf/docs/site/reference/tooling-conformance.md
    - agf/docs/site/reference/coord-graph-index.md
  do_not_touch:
    - agf/docs/site/docs/start/framework-map.md
    - agf/docs/site/index.md
    - tooling/rust/
---

# Consultation: should AGF publish an `agov` command map?

After v2.4.0, assess whether AGF should provide a maintained orientation map
of `agov`: which commands exist, when and why a person or AI assistant should
use each command, what input it needs, what it produces, and what the operator
normally does next.

Do not assume that a conventional mind map is the best format. Marketing and
pedagogy perspectives should compare at least:

- a mind map;
- a task-oriented decision tree;
- a lifecycle or workflow map;
- a layered HTML table or command cards; and
- a visual overview paired with a searchable textual reference.

Evaluate first-contact value, scanability, cognitive load, non-native-English
accessibility, mobile and print use, and whether the result makes `agov` feel
approachable without concealing gates or human authority. Test both newcomer
and experienced-operator journeys. If audience evidence is missing, propose
small questions or usability checks for humans rather than filling the gap
with model confidence.

The recommendation must separately assess human and AI consumption. A
rendered graphic alone is not a dependable AI interface or accessible human
reference. Any recommended visual should have a maintained textual or
structured source, meaningful alternative text or an equivalent table, stable
links to command documentation, and a layout that remains useful without
colour. State whether one shared artifact can serve both audiences or whether
the same source should generate separate human and machine views.

For each command or command family, consider whether the map should expose:

- the situation or question that triggers it;
- role, required input, and important options;
- output shape and whether it is read-only or state-changing;
- authority boundary: evidence/advice versus a governed decision;
- expected next action and related commands; and
- capability maturity and availability through CLI versus optional MCP.

The recommendation must address drift. Prefer deriving the inventory from a
versioned supported-command source or checked CLI metadata over hand-copying
help text. Include provenance at least in metadata: AGF/`agov` version,
inventory source, generator or source path, generation date or revision, and
the commit/release to which the map applies.

Return a critical recommendation: adopt, adapt, or reject; preferred format;
audiences; proposed public-doc placement; maintenance owner/source; minimum
prototype; usability questions for humans; and claims that marketing should
avoid. This is post-release consultation under MR !350, not implementation
authority and not a v2.4.0 blocker.
