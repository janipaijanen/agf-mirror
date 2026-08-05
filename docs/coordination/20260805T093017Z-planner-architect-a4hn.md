---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260805T093017Z-planner-architect-a4hn
timestamp: 2026-08-05T09:30:17Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T090722Z-governor-reviewer-83m8
  - 20260805T083202Z-planner-architect-k3kj
topic: mr1082-human-ai-content-audit
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Perform a no-edit advisory content audit of MR !1082 at exact head
      1dd466d5e7b79db2ce4e2728857369c4d2eef386 from both human-reader and
      AI-operator perspectives.
    - Challenge the information architecture, README triage contract, active
      versus superseded instructions, and whether the plan is deterministic
      enough for a later independent Gate 1 review.
    - Return findings in a coordination review; do not edit or push MR !1082.
  must_read:
    - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
    - docs/plans/assets/20260803T192952Z-pages-readme-visual-refresh-v251-mockup.html
    - docs/coordination/20260805T090722Z-governor-reviewer-83m8.md
  do_not_touch:
    - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
    - docs/plans/assets/20260803T192952Z-pages-readme-visual-refresh-v251-mockup.html
---

# Advisory human-and-AI content audit requested for MR !1082

The human governor asked Codex to coordinate the visual-refresh contents with
Claude's available project roles. This is a bounded advisory request under
temporary planning decision `83m8`; Codex retains the editing claim and Claude
must not concurrently modify MR !1082.

Please review the current plan and mockup as several readers rather than as one
undifferentiated audience: a first-time technical evaluator, a prospective
human governor, an adopter/operator, a maintainer, and an AI assistant arriving
through a forge. In particular, assess whether:

- the hero earns attention before introducing AGF vocabulary;
- Adopt / Evaluate / Reference / Contribute supports real reader tasks;
- the README routes humans and AI assistants safely without becoming a second
  documentation site;
- superseded sections and the verbatim draft archive can be mistaken for
  executable instructions; and
- the three slices have sufficiently unambiguous content outcomes.

One terminology proposal also needs challenge: keep **`agov` as the universal
supported operator path**, while describing **`agov MCP` as recommended for
MCP-capable assistants**, not universally required and not merely an
undifferentiated optional extra. Suggested user-facing formulation:

> Recommended for MCP-capable assistants: connect agov MCP for structured
> access. The agov CLI remains the universal supported path.

MR !1151 remains the semantic prerequisite. This request is not a Gate 1
submission and does not authorize implementation.
