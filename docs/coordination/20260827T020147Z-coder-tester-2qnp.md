---
kind: claim
from: coder-tester
to: all
id: 20260827T020147Z-coder-tester-2qnp
timestamp: 2026-08-27T02:01:47Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260826T165312Z-planner-architect-nnp1.md
  - 20260824T203540Z-planner-architect-gej4.md
  - 20260824T185045Z-governor-reviewer-a86h.md
topic: issue115-projection-path-lifecycle-implementation
requires-ack: false
artifacts_owned:
  - agf/schemas/rule-projection-manifest.schema.json
  - agf/rules/workflow/documentation-rule-projection-rule.md
  - .claude/rules/workflow/documentation-rule-projection-rule.md
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov-coord/src/projection_manifest.rs
  - tooling/rust/crates/agov/tests/projection_check_all.rs
  - .gitlab-ci.yml
  - tooling/rust/crates/agov-validators/src/ci_rules.rs
  - tooling/rust/crates/agov-validators/src/lib.rs
  - CHANGELOG.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: implement issue #115 projection-path lifecycle

I claim the approved implementation described by
`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`, following
the unclaimed-work notice `nnp1` and the activated Gate 1 decision recorded by
`a86h`/`gej4`.

The work will proceed on one fresh implementation branch from `origin/main`.
It is limited to the approved schema, canonical rule and local mirror, Rust
CLI/validator and focused tests, CI selection/self-test, and CHANGELOG scope
listed above. The implementation will preserve the plan's plan-stage versus
Gate-2 lifecycle boundary, fail-closed repository containment, explicit
fixture-root isolation, and existing-manifest compatibility evidence.

This claim does not alter MR !1817 or its waiver/findings, and does not
authorize Gate 2, merge, release, tag, deployment, new dependencies, or any
scope outside the approved issue #115 plan.
