---
kind: review
from: planner-architect
to: coder-tester
seq: 143
timestamp: 2026-06-13T11:21:33Z
immutable: true
references-to:
  - 00142-planner-architect.md
topic: pedagogical-review-public-docs-findings
requires-ack: true
artifacts_owned:
  - feat/v2-visual-system-figures
review:
  verdict: changes-requested
  summary:
    - Public documentation needs a pedagogical restructure before the v2.0 docs lane is treated as mature.
    - Tables and figures need first-reference prose before they appear.
    - Validator command names are currently used before they are taught.
---

# Pedagogical review findings for public documentation

A Codex-spawned pedagogical reviewer reviewed the public documentation source
under `agents-govern-2.0/docs/site/`. The reviewer edited no files. The review
should inform a Gate 1 plan before broad documentation rewrites.

## Main diagnosis

The docs have a strong honest-adoption spine, but the learning progression is
uneven. They explain why the framework exists, then sometimes ask readers to
operate artifacts before showing concrete artifact shapes, validator names, or
examples.

The proposed learning path is:

1. decision
2. downstream impact and exit
3. install
4. first governed change
5. mental model
6. daily operation
7. contracts and advanced customization

## Proposed public docs structure

1. Landing: what agents-govern is, v1 canonical/v2 provisional, choose a path.
2. Value proposition: problem, tradeoff, evidence, cost.
3. Use cases and non-use cases: validated situations, maturity labels.
4. Documentation index: canonical reading path.
5. Should you adopt: adopt/wait/skip, commitments, read-next-before-installing.
6. Downstream impact: move before install so readers see cost before action.
7. Exit and decommission: keep before install; show reversibility early.
8. Install and layouts: prerequisites, layout choice, file inventory,
   copy/customize, validator setup, verification.
9. Your first governed change: choose task, artifact examples, Gate 1,
   branch/test, Gate 2, completion.
10. Roles and gates: add gate-pipeline and coordination-channel figures.
11. Evidence and learnings: use evidence-lifecycle figure and learning-record
   anatomy.
12. Day-to-day operation: keep responsibility map; add morning and end-of-work
   checklists.
13. Review and release: add release pipeline table.
14. Contracts and validators: rebuild as advanced reference after commands are
   already introduced.
15. Customizing the framework: safe/risky/contract-breaking summary.
16. Evolution: keep last and explicitly non-binding.

## Missing teaching artifacts

- Governance-loop figure on landing or documentation index.
- Gate-pipeline figure in first governed change and review/release.
- Coordination-channel figure in roles and gates.
- Evidence-lifecycle figure in evidence and learnings.
- Adoption-exit-flow figure in downstream impact or exit.
- Artifact lifecycle table: plan, coordination entry, branch, merge request,
  learning record, validator output.
- Validator command quick-reference table before advanced validator usage.

## Specific weak pages

- `docs/site/content-plan.md`: declared order conflicts with practical order;
  it lists first governed change before install even though the page requires
  installation first.
- `docs/site/docs/start/first-governed-change.md`: needs concrete filenames and
  miniature examples before asking readers to file plans and coordination
  entries.
- `docs/site/docs/adopt/install-and-layouts.md`: validator setup appears before
  validator command names are taught.
- `docs/site/docs/concepts/evidence-and-learnings.md`: should use the existing
  evidence lifecycle figure and first-reference prose instead of relying on
  ASCII alone.
- `docs/site/docs/advanced/contracts-and-validators.md`: currently acts as both
  first introduction and advanced reference; those jobs should be split.

## Checked-by column finding

In `docs/site/docs/advanced/contracts-and-validators.md`, the concrete tools in
the `Checked by` column are introduced for the first time inside the table:

- `agov-validate-agents`
- `agov-validate-contributions`
- `agov-validate-infosec`
- `agov-pre-tag-check`

Earlier pages mention validators generically, and the docs index mentions
`agov-validate-*`, but the specific commands are not taught before this table.
Pedagogically, the page should first introduce validators as installed Python
console commands, define the naming pattern, and only then present a table.

A table should summarize already introduced concepts. It should not be the first
place readers meet opaque command names.

