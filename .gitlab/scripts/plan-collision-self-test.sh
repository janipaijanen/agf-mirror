#!/bin/sh
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
#
# Row 6 S4 (#123), S4-I2 fix: proves the plan-collision-block gate
# actually fires on a known-bad input BEFORE evaluating the real
# candidate in agov:governance:plan-collision-gate. A synthetic fixture
# with two live plans declaring an identical affected_paths entry, no
# depends_on, must produce exit 1. If it doesn't, the gate cannot be
# trusted to fire on the real candidate either -- fail loudly here
# rather than silently evaluating the real candidate under a broken gate.
#
# This is gate mechanics per agf/rules/always/gate-self-test-rule.md:
# registered as one of the plan-collision-block gate's own test paths in
# .agf/gates.yaml, alongside tests/plan_collision.rs.
set -eu

agov_bin="${1:?plan-collision-self-test.sh: agov binary path required}"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

git init -q -b main "$tmp_dir"
(
  cd "$tmp_dir"
  git config user.email "selftest@example.invalid"
  git config user.name "Gate Self-Test"
  mkdir -p docs/plans
  : > docs/plans/.keep
  git add -A
  git commit -q -m base

  cat > docs/plans/existing.md <<'PLAN'
---
id: existing
status: approved
affected_paths: ['shared.rs']
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
  - role: contribution-auditor
    status: consulted
---
Self-test fixture -- not a real plan.
PLAN
  git add -A
  git commit -q -m existing
  base_sha="$(git rev-parse HEAD)"

  cat > docs/plans/candidate.md <<'PLAN'
---
id: candidate
status: approved
affected_paths: ['shared.rs']
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
  - role: contribution-auditor
    status: consulted
---
Self-test fixture -- not a real plan.
PLAN
  git add -A
  git commit -q -m candidate

  set +e
  "$agov_bin" plan collision-check --plans-only --changed-only \
    --fail-on-unacknowledged-collision --base-ref "$base_sha"
  result=$?
  set -e

  if [ "$result" -ne 1 ]; then
    echo "ERR: plan-collision-block self-test failed -- expected exit 1 on a known unacknowledged collision (two live plans, identical affected_paths, no depends_on), got $result. The gate cannot be trusted to fire; refusing to evaluate the real candidate under it." >&2
    exit 1
  fi

  echo "plan-collision-block self-test passed: a known unacknowledged collision correctly returned exit 1."
)
