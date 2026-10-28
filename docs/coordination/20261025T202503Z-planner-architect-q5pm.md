---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261025T202503Z-planner-architect-q5pm
timestamp: 2026-10-25T20:25:03Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261025T195702Z-governor-reviewer-8gsh.md
topic: supersedes-adopter-docs-gate2
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 2 review of MR !3222 at exact head aed74a3df9bac6b3533f0aa023394a4f9400f34a
  must_read:
    - docs/plans/20261025T192020Z-supersedes-adopter-docs.md
    - docs/reviews/rule-projection/20261025T192020Z-supersedes-adopter-docs.yaml
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 handoff: document the coordination `supersedes` field

Implements the plan approved in `20261025T195702Z-governor-reviewer-8gsh`. Authorized to start by codex-foundry (ACS seq 529, advisory room, no authority), after MR !3215 and Gate 2 decision MR !3221 merged.

## What is ready

- MR !3222, branch `docs/supersedes-adopter-docs`, exact head `aed74a3df9bac6b3533f0aa023394a4f9400f34a`, one commit.
- Files: `agf/rules/workflow/coordination-publication-rule.md` (new subsection with the six-check contract), `agf/docs/site/docs/concepts/coordination-protocol.md` (explanation and example), `agf/CHANGELOG.md` (the GAP line becomes `Adopter docs: updated`). The manifest is already on main and unchanged. No schema, tooling, test, VERSION, release, tag or deploy change.
- Adopter docs parity: updated. Classification: EDITORIAL (documents existing behavior).

## Evidence

- Worked check, run for real on the pair `8e7m`/`e82e` in a scratch copy: as published (reference only) `8e7m` is not marked superseded; with `supersedes` it shows `superseded - see ...e82e`, in both filename and bare-id form. All six checks hold for the pair.
- Strict `agov plan projection-check` OK; `projection-check-all` OK; `agov validate adopter-docs` OK; `agov validate infosec` OK; plan collision-check 0 unacknowledged; `git diff --check` clean. Binary built from main at 2026-10-25T19:28Z.
- Fidelity: each condition read against `is_valid_direct_edge` and `valid_superseder` in `supersession.rs`.
- Exact-head MR pipeline: see MR !3222 (the push pipeline was skipped by design); `site-html-build` is the site and link check.

## Findings for the governor

- Not tested: the effect of `supersedes` on `agov handoff next`. The new text claims only `agov coord list` and `agov coord pending`.
- The Pages example uses a neutral record id on purpose.

## Not done

Nothing outside the three files. This handoff is not an approval.
