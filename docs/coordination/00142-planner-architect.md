---
kind: handoff
from: planner-architect
to: coder-tester
seq: 142
timestamp: 2026-06-13T11:20:09Z
immutable: true
references-to:
  - 00141-coder-tester.md
topic: public-docs-pedagogical-review-needed
requires-ack: true
artifacts_owned:
  - feat/v2-visual-system-figures
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Perform a pedagogical review of the v2.0 public documentation before treating Slice A as complete.
    - Coordinate between Claude and Codex on a full reader-facing structure proposal before large rewrites.
    - "Make tables and figures follow first-reference discipline: introduce the table or figure in prose before the artifact appears."
  must_read:
    - agents-govern-2.0/docs/site/
    - agents-govern-2.0/docs/site/docs/advanced/contracts-and-validators.md
    - agents-govern-2.0/docs/site/docs/operate/day-to-day.md
    - agents-govern-2.0/docs/site/content-plan.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
    - agents-govern/
---

# Pedagogical review request for public v2.0 documentation

Governor-reviewer raised a documentation architecture concern: the public HTML
documentation is still too weak for human comprehension at its current
abstraction level.

This is not just a prose polish item. The documentation needs a pedagogical
review of the whole public path: chapter order, page order, section structure,
introductory scaffolding, visual aids, and the progression from beginner
concepts to advanced operation.

## Immediate findings

The page `agents-govern-2.0/docs/site/docs/advanced/contracts-and-validators.md`
opens its first section with a table whose `Checked by` column lists:

- `agov-validate-agents`
- `agov-validate-contributions`
- `agov-validate-infosec`
- `agov-pre-tag-check`

Those command names are not introduced before the table. They appear later as
commands in the CI example, and they are described in older/internal contract
documentation outside the public learning path, but the public page itself asks
the reader to interpret them before teaching what they are.

Pedagogically, the page should first introduce "validators" as shipped command
line checks, explain what a validator command is expected to do, then present
the contract table. The table should be referenced in prose before it appears.

## Requested documentation rule

Before any table or figure, include a short first reference in the preceding
paragraph. The reference should tell the reader what the artifact is for and how
to read it, similar to academic writing:

- "Table 1 summarizes the validator commands and the artifacts they check."
- "Figure 2 shows the daily loop and separates delegated execution from human
  approval authority."

This should apply across public docs, not only the advanced page.

## Requested collaborative review

Please have a pedagogical documentation role review the public documentation
contents run by both Codex and Claude. The reviewer may propose any full
structure: chapter structure, section structure, page order, new figures,
introductory pages, or page splits/merges. Claude and Codex should discuss and
settle an implementation proposal through coordination before a broad rewrite.
The human governor-reviewer will review the resulting plan.

## Suggested review outputs

- A public-docs learning path from evaluator to new adopter to operator to
  advanced adopter.
- A figure/table inventory with the required preceding first-reference text.
- A glossary or "tools and commands" introduction before command names are used
  as if already known.
- A list of weak pages and the specific missing teaching step on each.
- A Gate 1 plan for public documentation restructure if the proposed changes are
  larger than a local fix.
