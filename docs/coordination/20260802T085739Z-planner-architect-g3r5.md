---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260802T085739Z-planner-architect-g3r5
timestamp: 2026-08-02T08:57:39Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260801T181444Z-coder-tester-ggs2
  - 20260802T074541Z-contribution-auditor-rs7b
topic: ci-rust-cache-policy-slice-a
requires-ack: false
artifacts_owned:
  - docs/plans/ci-rust-cache-policy-slice-a.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Publish an independent adversarial review of the exact pending Gate 1 plan; do not modify the plan, implement CI changes, or issue a gate decision.
    - Recompute the full direct and indirect shared-cache consumer inventory from current source instead of accepting the five-job sample or plan prose.
    - Challenge the fail-safe workflow-variable mechanism, versioned shared key and fallback, context matrix, CI Lint limitations, and per-trace Gate 2 evidence.
    - Decide whether Slice A is independently safe and valuable while deliberately leaving concurrent default-branch writers for the separately gated Slice B.
    - Test whether the Slice B boundary is strong enough to require exactly one automatic writer across the resolved default-branch graph, with manual consumers pull-only.
  must_read:
    - docs/plans/ci-rust-cache-policy-slice-a.md
    - GitLab issue #101 and its relation to #53
    - MR !910 diagnosis at exact head e61374d426451f26eb47f2f835690a82a308779f
    - MR !920 independent review at exact head 79a2dc7a77508257fa222981288722a82cac437c
  do_not_touch:
    - docs/plans/ci-rust-cache-policy-slice-a.md
    - .gitlab-ci.yml
    - .gitlab/scripts/
    - .claude/settings.local.json
---

# Adversarial review requested for Rust cache-policy Slice A

The governor approved proceeding from the cache-policy proposal into governed
planning and requested active coordination with Claude. The plan narrows the
first implementation to a fail-safe pull-only policy outside ordinary
default-branch push pipelines. It explicitly does not claim to solve the
remaining concurrent writers on `main`; that topology is reserved for a
measured, separately approved Slice B.

A read-only inventory audit corrected the motivating shorthand: the shared
`.rust-base` cache has 12 concrete consumers, not only the five jobs observed
colliding in pipeline `2724116160`. Review the resolved inheritance graph and
all context-specific jobs directly. Please publish findings as a new
coordination MR and state CLEAR or blocking findings against this exact plan
head. Gate authority remains with the human governor-reviewer.
