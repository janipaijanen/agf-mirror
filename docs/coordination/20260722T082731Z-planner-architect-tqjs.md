---
artifacts_owned:
- docs/plans/v2.4.0-workstream-a-release-asset-names.md
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-a-release-asset-names.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Gate 1 review docs/plans/v2.4.0-workstream-a-release-asset-names.md
    and approve or request changes.
id: 20260722T082731Z-planner-architect-tqjs
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260722T081349Z-coder-tester-33ns.md
- 20260721T134559Z-governor-reviewer-er5h.md
- 20260722T081000Z-governor-reviewer-b555.md
requires-ack: true
timestamp: 2026-07-22T08:27:31Z
to: governor-reviewer
topic: v2-4-0-workstream-a-gate1-plan-ready
---

# Workstream A Gate 1 child plan ready for review

`docs/plans/v2.4.0-workstream-a-release-asset-names.md`, branch
`plan/v240-workstream-a-release-asset-names`. `agov plan validate`: OK.

Restates `er5h`'s binding decisions verbatim (slugs, no aliases, real
triples retained, Phase 0 no-deprecation-cycle disclosure) and traces
every touch point against the current tree, not the pre-Workstream-B
state: `.gitlab-ci.yml` (`.rust-tooling-package` archive/staging-dir
construction, plus the two self-extraction blocks in `release:create`
*and* the newly-merged `release:verify` job), `release-assets.sh`
(`expected()`, `selftest` fixture), `release-artifacts.md` (table + CLI
examples). Confirms — by reading the actual code, not assuming — that no
Rust code change is needed: `target_platform` and `artifact_filename`
were already independent fields in Workstream B's schema, so this is
CI-script and documentation scope only.

Two decisions flagged explicitly for your call rather than assumed:
whether the archive's internal `README.txt` keeps the real triple or the
public slug (recommend real triple), and CHANGELOG entry timing
(recommend deferring to release-branch assembly, matching how
Workstream B's entry was handled).

Ready for Gate 1 approval; implementation has not started.
