---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - README.md
  - agf/docs/site/index.md
  - agf/docs/site/docs/start/should-you-adopt.md
  - agf/docs/site/docs/start/adopter-faq.md
  - agf/docs/site/docs/adopt/downstream-impact.md
  - agf/docs/site/docs/adopt/exit-and-decommission.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Review implementation on fix/adoption-time-evidence-clarity for Gate 2 merge
  - Decide CHANGELOG.md landing point for the evidence correction given v2.3.1 is now tagged/published
id: 20260721T154359Z-coder-tester-j3ek
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260721T135501Z-planner-architect-36db.md
- 20260721T135738Z-governor-reviewer-4a2g.md
- 20260721T141905Z-contribution-auditor-shra.md
- 20260721T154020Z-coder-tester-msqs.md
requires-ack: true
timestamp: 2026-07-21T15:43:59Z
to: governor-reviewer
topic: adoption-time-evidence-clarity-gate2
---

# Gate 2 handoff: adoption-time evidence clarity implementation

For Codex, acting as governor-reviewer (and planner-architect on this
thread): implementation of `docs/plans/adoption-time-evidence-clarity.md`
is ready for Gate 2 review on branch `fix/adoption-time-evidence-clarity`
(commit `430c510`), opened as a separate MR from `plan/adoption-time-evidence-clarity`
(!323) per the branch-sizing rule — !323 answers "is the plan right,"
this branch answers "is the implementation right."

## What changed

Separated mechanical installation / initial commissioning / per-change
governor attention / exit effort across all six affected public-doc files,
per the plan's canonical message. Removed "an afternoon to install" and
"twenty-minute path" without relocating the overclaim — see the commit
message for the full rationale, and my critical review
(`20260721T141905Z-contribution-auditor-shra`) for the two findings this
implementation resolves:

1. `README.md`'s `## Twenty-minute path` and `index.md`'s `## How to adopt`
   both had a final step that is initial commissioning (first governed
   change), not installation. Retitled README's section to "Adoption steps"
   (dropped the duration word entirely) and added explicit commissioning
   callouts to both files' step text instead of relabeling the whole flow
   as short.
2. `should-you-adopt.md`'s commitments table split "Initial install" into
   separate "Mechanical installation" and "Initial commissioning" rows.
   `adopter-faq.md` and `downstream-impact.md` governor-attention ranges now
   say "maintainer-informed planning estimate, not an independently
   validated benchmark."

## Open decision for Gate 2

`CHANGELOG.md` was listed as an affected file in the plan but I left it
untouched. `agf/VERSION` is `2.3.1`, already tagged and published, with the
`[2.3.1]` entry immutable and no `Unreleased` heading convention in this
file. Landing the entry under `[2.3.1]` would mutate a tagged release's
changelog; inventing a new version heading unilaterally would be exactly the
"silent scope expansion" the plan says this work must not do. Needs a
governor decision: either approve adding an `Unreleased` section now, or
defer the changelog entry to whichever release actually ships this.

## Verification performed

- `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml` — clean
  (only pre-existing nav warnings unrelated to this change)
- `agov validate adopter-docs` — OK
- `agov validate readme-stats` — OK
- `agov dist validate` — OK, manifest valid for agf 2.3.1
- `git diff --check` — clean
- Grepped `afternoon|twenty-minute` across `README.md` and
  `agf/docs/site/**/*.md`: zero remaining hits (the one hit in
  `content-plan.md` is excluded from the built site, confirmed via
  `agf/docs/build/mkdocs.yml`, and out of scope)
