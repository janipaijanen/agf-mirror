---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260829T214241Z-coder-tester-phe4
timestamp: 2026-08-29T21:42:41Z
model: gpt-5.4
immutable: true
references-to:
  - 20260829T211803Z-coder-tester-j1q8.md
  - 20260829T211124Z-coder-tester-rmj1.md
topic: gate-change-discipline-gate1-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate-change-discipline Gate 1 review

Scope: plan MR !2214 at exact head
`0c5d6a2fc344039157f386811b95296ab94c790c`, containing plan
`20260829T211252Z-gate-change-discipline.md` and projection manifest
`gate-change-discipline.yaml`; current target `main` was re-read at
`6f1a235bec6d2f47e11092d7d452ce3545bdd933`.

Coverage: COMPLETE for the bounded request in `j1q8`: slice C's design and
bypasses, rule-owner shape, B's asymmetric deletion treatment, the two live
changed-file validator feeds, and the directly duplicated adopter CI examples.

Recommendation: **BLOCK Gate 1 pending revision.**

Blocking findings: 3. Advisory findings: 1. Open findings with accepted risk:
0. Authority: recommendation only; the governor-reviewer decides Gate 1.

Exact-head evidence: canonical MR pipeline `2802425319` passed both jobs
(`agov:governance:validate` job `16186505218`,
`commercial-corpus:validate` job `16186505219`); the MR has zero unresolved
discussions. `agov plan validate` passes. A green plan pipeline establishes
shape, not design sufficiency.

## What passed

- `rmj1`'s corrected diagnosis is reproducible. In the live CI configuration,
  exactly two commands feed `git diff --name-only` output directly to an
  `agov` validator: coordination at lines 866-869 and plans at 870-871.
- B's asymmetry is correct in principle: a plan deletion may be intentionally
  omitted from content validation, while coordination deletion needs an
  explicit append-only rejection rather than today's accidental `file not
  found` failure. The implementation should test add, modify, and delete
  statuses independently.
- Extending `gate-self-test-rule.md` and `simplicity-rule.md` is the right
  normative-owner shape. A third rule would split one obligation across more
  authority surfaces. Discoverability belongs in the already-proposed Pages,
  runbook, and loader projections.
- The plan correctly labels C as a presence proxy rather than proof of test
  quality and leaves semantic review to humans.

## GCD-1 — The implementation boundary is not yet executable

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

The plan does not name the registry path or format, the command/check that
consumes it, the CI job that invokes it, the declared test paths, or the Rust
implementation and fixture paths. The distribution row only says that a
registry and check will ship "with tooling". Acceptance criteria 6 and 7
therefore cannot be implemented or independently reproduced from this work
order without making the load-bearing design during implementation.

Resolution condition: revise the plan and projection to name the exact
registry, parser/checker, CI integration, tests/fixtures, and distribution
paths; define the registry schema and failure semantics; and show the complete
producer-to-consumer path for the blocking result.

## GCD-2 — Diff-hunk-to-YAML-job mapping has known semantic bypasses

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

The proposed `git diff -U0` mapping sees text lines, not the effective GitLab
job. This repository already uses top-level variables, hidden jobs, `extends`,
YAML anchors/aliases, and `!reference`. A gate's behavior can change through
one of those dependencies, through an invoked script or configuration file,
or by renaming/deleting the registered job while no changed hunk intersects
the new job block. Those are concrete bypasses, not theoretical parser
tidiness. A line-range mapper must not receive blocking authority as the
first implementation.

The named fallback also does not close the plan's own motivating case: if C
registers only validator crates and hooks, it does not mechanically cover the
change to the gate job in `.gitlab-ci.yml` made by B.

Resolution condition: choose a fail-closed first version with an objective
contract. The proportionate option is to register exact gate-mechanics paths
and exact test paths, including CI, invoked scripts/configuration, the
registry, and the checker itself. Treat job-range classification as advisory
until a semantic dependency model exists. If `.gitlab-ci.yml` is too broad,
extract the gate logic into a registered script and leave only a stable,
tested invocation in YAML.

## GCD-3 — The registry/check can weaken itself

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

The plan does not protect changes to the registry or checker. In the proposed
shape, the same MR can remove a gate mapping, change its declared test path,
or weaken the detector and then satisfy the detector with any touched test
path. The rollback statement even presents removing registry entries as a
no-code rollback, although under A1 that is itself a gate modification and
must not bypass the negative-test obligation.

Resolution condition: make registry and checker changes self-triggering gate
changes; require rejection of unknown/missing registered jobs or paths; and
add negative fixtures for deregistration, rename/deletion, checker mutation,
and an unrelated test-path touch. State that rollback follows the same Gate 1
and negative-test discipline as any other gate modification.

## GCD-4 — Directly copied adopter CI examples retain the deletion bug

Check status: reviewed. Outcome: needs-review. Requirement strength:
recommended. Impact: material. Enforcement: advisory pending governor scope
decision. Finding state: open. Risk disposition: none.

The class search found only two *live foundry CI feeds*, as claimed, but also
found the same deletion-blind commands in supported adopter guidance:

- `agf/docs/site/docs/adopt/tooling-and-tokens.md:134-137`
- `agf/docs/site/docs/advanced/contracts-and-validators.md:117-120`
- the GitLab and GitHub sketches in the latter at lines 158-161 and 186-189

Leaving these unchanged fixes foundry while continuing to tell downstream
users to install the defect. This is a direct impact of B, not an unrelated
documentation cleanup. Generated `agf/site-html/` output must follow the
repository's normal site-generation policy rather than be hand-edited.

Resolution condition: governor decides whether the corrected source examples
belong in this plan. Recommendation: include them now, update the projection
manifest, and explicitly estimate the generated-site consequence before Gate
1. Because this is still a pending plan, revising the estimate now is the
least disruptive application of the necessary-scope rule.

## Recommended revised shape

Keep A and B together. Replace C's line-range mapper with a concrete,
self-protecting path registry/check contract and negative fixtures. Include
the directly copied adopter examples in B's impact boundary. A later work
order may experiment with semantic GitLab-job dependency mapping without
making that experiment a blocker.
