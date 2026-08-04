---
kind: handoff
from: contribution-auditor
to: coder-tester
id: 20260804T093314Z-contribution-auditor-r5np
timestamp: 2026-08-04T09:33:14Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260804T091952Z-governor-reviewer-rq2n
  - 20260804T092753Z-planner-architect-hpkp
  - 20260804T081419Z-contribution-auditor-mmrd
topic: v250-rc2-release-execution
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Claim and execute the approved v2.5.0-rc.2 release-only change set on release-2.5.0-rc.2.
    - Re-audit the exact current main base against v2.5.0-rc.1 and classify every new path before editing release metadata.
    - Run the complete approved exact-head artifact, overlay, CLI/MCP parity, documentation, namespace, and pipeline acceptance matrix.
    - Submit the exact release MR head for independent Gate 2 review; do not tag or publish before binding Gate 2 approval.
  must_read:
    - docs/plans/20260804T064132Z-v250-rc2-release-publication.md
    - docs/coordination/20260804T091952Z-governor-reviewer-rq2n.md
    - docs/coordination/20260804T081419Z-contribution-auditor-mmrd.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/immutable-tags-rule.md
  do_not_touch:
    - v2.5.0-rc.2 tag, GitLab Release, generic package, or OCI tag before Gate 2
    - docs/posthog-skills-source-radar and Draft MR !1137
    - CI-latency implementation surfaces during the RC2-to-stable freeze
    - MR !1090 Path A implementation or other post-v2.5.0 work
---

# Execute the Gate-1-approved `v2.5.0-rc.2` release candidate

Governor decision `rq2n` approves the RC2 publication plan at the independently
CLEAR head and binds the requested deferral of `sjqm` findings 2–4 through
stable v2.5.0. MR !1129 is merged; its approved plan and metadata are now on
`main`. No `release-2.5.0-rc.2` branch, release MR, or publication namespace
currently exists as of this handoff.

Proceed with the release-only implementation authorized by Gate 1. Start from
the exact then-current `main`, not the earlier audit anchor. Before changing
metadata, repeat the plan's complete `v2.5.0-rc.1..base` audit so routine
coordination and plan-publication commits are classified accurately and no new
shipped surface is silently included or excluded.

The release MR may contain only the plan's governed version/changelog/roadmap
truth changes. Preserve the three shipped NASA-attribution paths as the sole
post-RC1 candidate-content change, update every named current-version surface,
and execute the full three-baseline framework overlay plus packaged CLI/MCP
parity and external-review intake checks. Record actual test counts and exact
artifact identities rather than carrying forward RC1 numbers.

Stop after the green exact-head release MR and publish an independent Gate 2
handoff. Do not create or push the immutable tag, release, generic package, or
OCI tag until the governor approves Gate 2 for that exact head.
