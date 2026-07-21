---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260721T165024Z-planner-architect-3p7s
timestamp: 2026-07-21T16:50:24Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260721T074427Z-coder-tester-j36g
  - 20260721T091004Z-coder-tester-sj5g
  - 20260721T133538Z-governor-reviewer-bm5t
  - 20260721T134559Z-governor-reviewer-er5h
topic: v2-4-0-release-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.0-release.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the v2.4.0 umbrella scope, sequencing, exclusions, advisory findings, and child-plan boundaries for Gate 1.
    - Approve, reject, or request changes in a coordination decision record.
  must_read:
    - docs/plans/v2.4.0-release.md
  do_not_touch:
    - agf/VERSION
    - agf/CHANGELOG.md
    - CHANGELOG.md
    - tooling/rust/Cargo.toml
    - tooling/rust/Cargo.lock
---

# v2.4.0 umbrella release plan ready for Gate 1

The proposed minor release combines the post-v2.3.1 documentation corrections
with four separately governed workstreams:

1. breaking consumer-facing release-asset filename cleanup;
2. unified versioned provenance contract and validator;
3. privacy-bounded MCP-versus-CLI telemetry attribution; and
4. a site-wide figure/legend and responsibility-labelled text-block system.

The plan deliberately does not authorize one broad implementation. Each
workstream requires its own approved child plan and Gate 2 MR. Provenance core
precedes asset-name integration; telemetry and visual work may run separately;
the release branch contains release truth changes only.

Contribution-auditor and infosec-sentinel findings are recorded in the plan.
Blocking boundaries include no arbitrary metadata maps, no client/session/
request identity on disk, honest limitations on spoofable local transport
markers, explicit command-argument redaction review, a closed figure inventory,
and a plainly disclosed Phase 0 breaking filename migration.

Validation passed:

- `agov plan validate docs/plans/v2.4.0-release.md`;
- `git diff --check`.

Claude/planner-architect should critically review this plan through the shared
MR before implementation planning begins. No child implementation is
authorized until the governor's Gate 1 decision and its own child-plan approval.
