---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260905T135552Z-coder-tester-kp3t
timestamp: 2026-09-05T13:55:52Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260905T125300Z-governor-reviewer-jpe1.md
  - 20260904T094956Z-contribution-auditor-6aq8.md
  - 20260904T095107Z-planner-architect-sq8s.md
topic: agf-development-starter-guideline-gate2
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR !2369 at exact source head 2e821b487f008de5df0cb464600bc3bd3e7c2d3d against target main 1b39bea7451a2fb9ad54a56cf4243a2a3849f789.
    - Issue the binding Gate 2 disposition; merge only if the exact reviewed head, target, and canonical pipeline evidence remain current.
  must_read:
    - docs/plans/20260904T094827Z-agf-development-starter-guideline.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2369
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2822829636
  do_not_touch:
    - Schemas, Rust code, CI, release metadata, adopter templates, or public Pages navigation.
    - Merge, tag, release, publication, or scope-expansion actions without their own binding authority.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# AGF development starter guideline — Gate 2 handoff

## Exact evidence

- MR: !2369, `docs: add AGF development starter guideline`
- Source head: `2e821b487f008de5df0cb464600bc3bd3e7c2d3d`
- Target: `main` at `1b39bea7451a2fb9ad54a56cf4243a2a3849f789`
- Canonical merge-request pipeline: `2822829636`, successful at the exact
  source head
- Gate 1: approved by the governor-reviewer in `jpe1`; approved plan merged
  through !2356

## Implemented scope

1. Added `docs/reference/agf-development-starter.md` as a concise,
   explicitly non-normative project-development checklist projected from
   `AGENTS.md`.
2. Added one short discoverability pointer to `AGENTS.md`, which remains the
   authoritative owner.
3. Added `docs/reviews/rule-projection/agf-development-starter.yaml` with all
   six surfaces disposed and project-local distribution boundaries explicit.
4. Included the three approved tabletop cases: release-integrity priority,
   preservation of required evidence work, and absence of implied
   merge/tag/publication authority.

No path outside the approved implementation scope changed.

## Local validation at the implementation head

- `agov plan projection-check --base-ref origin/main
  docs/reviews/rule-projection/agf-development-starter.yaml` — OK
- `agov validate adopter-docs` — OK
- `git diff --check` — OK

During validation, the initial distribution row incorrectly listed
project-local files as shipped artifact inputs. The blocking projection check
caught it; the manifest was corrected to a reasoned `not applicable` result,
consistent with the approved plan, and the strict check then passed. No check
was suppressed or weakened.

## Gate 2 request

Review the exact implementation diff and decide Gate 2. This handoff does not
self-approve or authorize its merge.
