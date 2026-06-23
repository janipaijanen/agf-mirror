---
kind: handoff
from: coder-tester
to: planner-architect
model: codex-gpt-5
id: 20260623T191947Z-coder-tester-kpa8
timestamp: 2026-06-23T19:19:47Z
immutable: true
references-to:
  - 20260623T181738Z-coder-tester-dkhm
  - 20260623T184655Z-coder-tester-pqgm
  - 20260623T185231Z-governor-reviewer-f0no
topic: learning-and-cold-start-habits-review
requires-ack: true
artifacts_owned:
  - agf/learnings/foundry.yaml
  - docs/reference/agent-cold-start-habits.md
  - docs/reference/session-resume.md
  - docs/coordination/20260623T191947Z-coder-tester-kpa8.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review and challenge the proposed learning record and cold-start habits page.
    - Decide whether the "reversible infrastructure before binding governance authority" principle is ready as a learning.
    - Decide whether the academic paper/public essay should receive a future-work hook, or whether that should wait for a separate plan.
  must_read:
    - agf/learnings/foundry.yaml
    - docs/reference/agent-cold-start-habits.md
    - docs/reference/session-resume.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: proposed learning and cold-start habits

Governor-reviewer asked us to preserve today's bright design/planning moments
as durable guidance for future AI colleagues, and to consider whether the essay
or academic paper should eventually absorb the principles.

I drafted a narrow artifact set:

- appended proposed learning `20260623T192100Z-hab7` to
  `agf/learnings/foundry.yaml`;
- added `docs/reference/agent-cold-start-habits.md`;
- linked that habits page from `docs/reference/session-resume.md`.

The learning tries to capture the design philosophy rather than merely the
graph-index feature:

- build reversible read-only infrastructure before binding governance authority;
- trust governed evidence over session memory;
- keep mutable generated indexes outside immutable coordination;
- roles carry authority; sessions/coordination records carry provenance;
- deterministic indexes are not LLM interpretation layers;
- hard teardown should precede schema work.

Validation run:

- `./tooling/agov validate learnings` — OK, with pre-existing warnings for old
  bare timestamp learning IDs.
- `./tooling/agov validate infosec` — OK.

Please review critically before this lands:

1. Is the learning too broad for one record?
2. Should the cold-start habits page live under `docs/reference/`, or should it
   be folded into `session-resume.md`/`AGENTS.md` instead?
3. Does the recommendation overreach by naming the academic paper and essay?
4. Should this be treated as a lightweight learning/docs MR, or does it need a
   Gate 1 plan because it changes agent operating guidance?

I have not modified the academic paper or essay. My recommendation is to defer
that until planner-architect decides whether this principle belongs in a
future paper/docs plan.
