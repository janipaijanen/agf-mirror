---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260720T224808Z-coder-tester-tnhh
timestamp: 2026-07-20T22:48:08Z
model: codex-gpt-5.6-sol-high
immutable: true
references-to:
  - 20260720T224012Z-planner-architect-rbg1
  - 20260720T224012Z-governor-reviewer-g3st
topic: adoption-surface-clarity-gate2
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/start/should-you-adopt.md
  - agf/docs/site/index.md
  - agf/docs/site/evaluate.md
  - agf/docs/site/ai-evaluation-dossier.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/future/evolution.md
  - agf/CHANGELOG.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR !275 against the approved adoption-surface-clarity plan.
    - Confirm that shipped selective commissioning and the unshipped reduced starter are distinguished without minimizing the complete-library burden.
    - Inspect the rendered first-screen placement on the home, evaluation landing, and dossier cover before deciding Gate 2.
  must_read:
    - docs/plans/adoption-surface-clarity.md
    - agf/docs/site/docs/start/should-you-adopt.md
    - agf/docs/site/evaluate.md
    - agf/docs/site/ai-evaluation-dossier.md
    - agf/docs/site/future/evolution.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2: adoption-surface clarity ready in MR !275

Implementation commit: `0fb87c5` on `docs/adoption-surface-clarity`.

The public home, AI evaluation landing, and dossier cover now carry the same
verbatim first-screen disclosure: the complete library ships, the documented
first-trial path selects a subset, and no reduced starter package currently
ships. `should-you-adopt.md` is the canonical detailed source. The evaluation
page and dossier require reviewers to assess complete inventory, activated
subset, and starter-artifact availability separately.

The future page marks the reduced three-role/four-kind/one-validator/one-CI
shape Level 1 Planned and names independent-pilot promotion evidence. The
installation page links to the canonical distinction without changing its
existing selection procedure. No profile, archive, schema, tooling, CI, role,
rule, or coordination contract was added.

Verification evidence:

- Gate 1 plan and all three new coordination records validate.
- `agov validate adopter-docs`, `agov dist validate`, and `agov dist build`
  pass.
- Umbrella `agov validate` passes with pre-existing transitional-learning-ID,
  assistant-drift, and README rule-count warnings.
- MkDocs strict build passes with no broken-link finding for the new anchors.
- Built home, evaluation, and dossier pages were rendered at 1440x1200 and
  visually inspected; the disclosure is above the fold on all three.
- The built dossier contains the curated Flow 04.02a section, canonical link,
  and generated source anchor for the planned starter hypothesis.

One non-semantic hygiene warning is disclosed rather than hidden:
`git diff --check HEAD^ HEAD` reports a blank line at EOF in the already
published immutable Gate 1 handoff `rbg1`. The source-document diff was clean
before filing; the published coordination record was not rewritten merely to
remove whitespace.
