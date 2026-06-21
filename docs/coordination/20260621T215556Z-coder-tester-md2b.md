---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T215556Z-coder-tester-md2b
timestamp: 2026-06-21T21:55:56Z
model: GPT-5
immutable: true
references-to:
  - 20260621T214311Z-coder-tester-sjqb.md
topic: learning-threshold-governor-effort
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/concepts/evidence-and-learnings.md
  - agf/docs/site/docs/start/adopter-faq.md
  - agf/docs/site/docs/start/should-you-adopt.md
  - agf/docs/site/docs/adopt/downstream-impact.md
  - docs/coordination/20260621T215556Z-coder-tester-md2b.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the learning-record threshold and governor-effort docs slice.
  must_read:
    - agf/docs/site/docs/concepts/evidence-and-learnings.md
    - agf/docs/site/docs/start/adopter-faq.md
    - agf/docs/site/docs/start/should-you-adopt.md
    - agf/docs/site/docs/adopt/downstream-impact.md
  do_not_touch:
    - agf/rules/
    - agf/schemas/
    - agf/agents/
---

# Gate 2 handoff: learning threshold and governor effort docs

## Summary

This branch addresses the remaining documentation-only findings from External
Review work item #4 after MR !92 landed the assistant-loader and minimum
structural guardrail material.

The change adds:

- a concrete "record / do not record / escalate" threshold for learning
  records;
- adopter-facing examples for normal iteration, first commissioning, repeated
  friction, rule catches, rule workarounds, near-misses, and irreversible
  incidents;
- realistic governor attention estimates for first-trial and non-trivial
  changes;
- a clearer boundary that "no new infrastructure" does not mean "no human
  effort".

## Files changed

- `agf/docs/site/docs/concepts/evidence-and-learnings.md`
- `agf/docs/site/docs/start/adopter-faq.md`
- `agf/docs/site/docs/start/should-you-adopt.md`
- `agf/docs/site/docs/adopt/downstream-impact.md`

## Validation

- `git diff --check`
- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`
- `agov coord validate docs/coordination/20260621T215556Z-coder-tester-md2b.md`

## GitLab work items

- Work item #3 appears addressed by MR !90. Recommendation: close it with a
  note that canonical role names, gate timing, and GitLab login guidance were
  incorporated.
- Work item #4 is partially addressed by MR !92 and completed by this branch:
  MR !92 covered assistant-loader configuration and the minimum structural
  guardrail; this branch covers learning-record threshold and governor
  time-cost expectations.

## Review focus

- [ ] The learning threshold is practical enough for a first-time adopter.
- [ ] The governor effort estimates are honest without overpromising.
- [ ] The docs do not turn README or the start path into a long operations
      manual.
- [ ] The distinction between no new technical infrastructure and real human
      gate ownership is clear.
