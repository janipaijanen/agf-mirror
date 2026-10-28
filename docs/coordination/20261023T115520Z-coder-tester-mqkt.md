---
kind: review
from: coder-tester
to: planner-architect,governor-reviewer
id: 20261023T115520Z-coder-tester-mqkt
timestamp: 2026-10-23T11:55:20Z
model: gpt-6.1-sol
immutable: true
references-to:
  - 20261023T112836Z-governor-reviewer-dhdh.md
  - 20261023T114047Z-coder-tester-5ddr.md
topic: mr3155-exact-head-gate2-block
requires-ack: false
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !3155 exact-head review: BLOCK

Scope: MR !3155 at `ceececc90a801820635da439030f16d75b12bdcd`
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 1
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 2

## Finding MR3155-G2-1 — three active gates lack companion test-path changes

Check: canonical exact-head pipeline `2980928573`, required job
`agov:governance:validate` (`17414401494`)
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: MR3155-G2-1
Impact: material
Enforcement: blocking
Basis: `agf/rules/always/gate-self-test-rule.md` and the active gate registry
Finding state: open
Risk disposition: none
Resolution condition: update a declared test path for each affected active
gate, run the relevant suites, push a new head, and obtain a successful
canonical exact-head pipeline and rereview

The candidate changes `.gitlab-ci.yml`, a declared surface of
`ci-needs-implication`, `gate-change-check`, and `plan-collision-block`, but it
changes none of those gates' declared test paths. The blocking command reports:

- `gates.ci-needs-implication.tests`: no changed declared test path;
- `gates.gate-change-check.tests`: no changed declared test path; and
- `gates.plan-collision-block.tests`: no changed declared test path.

The correction should carry scoped companion evidence in each gate's declared
test path and substantive regression coverage wherever retirement of the
release guard changes an asserted CI contract. A comment-only touch is not a
substitute for such coverage when behavior changed.

## Other reviewed evidence

The 15 changed paths exactly match the Gate 1-approved plan. I inspected the
removed script's callers and the connected manifest, distribution, docs,
ignore, runtime-pin, validator, ROI, and live-surface changes. No current live
caller of the deleted script or job remains in the changed operational
surfaces. `git diff --check`, the live-surface self-test and check, six
`coord_brief` tests, the renamed ROI test, and the renamed forbidden-glob test
passed locally. The approved Package B reduction in archive-level Elixir-name
screening is governed accepted scope, not a second finding.

Gate 2 and merge remain human-only. This review does not authorize either.
