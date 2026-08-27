---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260827T100251Z-coder-tester-7bkm
timestamp: 2026-08-27T10:02:51Z
model: codex
immutable: true
references-to:
  - 20260827T072557Z-coder-tester-8gpt.md
  - 20260825T200220Z-planner-architect-4dgq.md
topic: assistant-discovery-archive-setup
requires-ack: false
artifacts_owned:
  - AGENTS.md
  - .claude/CLAUDE.md
  - agf/agents/MANIFEST.yaml
  - agf/agents/MANIFEST.example.yaml
  - agf/templates/AGENTS.md
  - agf/templates/CLAUDE.md
  - agf/ADOPTING.md
  - agf/docs/site/docs/adopt/assistant-loader-configuration.md
  - agf/docs/site/docs/adopt/mcp-server.md
  - agf/project-specific-examples/python-flask/CLAUDE.md
  - examples/claude/.mcp.json
  - .gitlab-ci.yml
  - tooling/rust/crates/agov-validators/src/assistant_loader.rs
  - tooling/rust/crates/agov/src/dist.rs
  - tooling/rust/scripts/check-release-self-sufficiency.sh
  - docs/reviews/rule-projection/assistant-discovery-archive-setup.yaml
  - agf/learnings/foundry.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR !2138 at exact head 26cd74e0d5be0677a26906fd8d9e91dd0b8107f0 for Gate 2.
    - If approved, make the binding Gate 2 and merge decision; this handoff grants neither.
  must_read:
    - docs/plans/assistant-discovery-archive-setup.md
    - docs/reviews/rule-projection/assistant-discovery-archive-setup.yaml
    - agf/learnings/foundry.yaml#20260827T091844Z-as7k
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2138
  do_not_touch:
    - .claude/settings.local.json
---

# Gate 2 handoff: assistant discovery and archive setup

MR !2138 implements the approved MR !807 plan for issues #87 and #90 on
`feat/assistant-discovery-archive-setup`. Exact head
`26cd74e0d5be0677a26906fd8d9e91dd0b8107f0` is mergeable; exact-head pipeline
`2795981450` passed with 20 successful jobs and two optional manual Windows
jobs. No discussion is unresolved.

## Delivered behavior

- `supported_assistant_loaders` declares `codex` and `claude-code`, accepts
  `github-copilot` as the frozen alias, validates every declared surface, and
  preserves legacy any-loader behavior when the field is absent.
- The live and shipped `.claude/CLAUDE.md` bridge is exactly
  `@../AGENTS.md`; a one-level resolver follows that relative import for
  version freshness with component-aware containment, regular/readable-file
  checks, and a hard 64 KiB + 1 read bound.
- Every rejected import condition stays a stable visible advisory; a deliberate
  unknown-token run produced the named warning and process exit status 0.
- Linux and Windows tooling archive staging includes the exact sanitized
  `examples/claude/.mcp.json`, self-contained README instructions, and
  negative missing/stale/unreferenced-template checks.
- Clean framework archive extraction proves the neutral manifest and both
  shipped loader templates configure a scratch adopter without repository-only
  files.

## Verification

Local verification passed: full Rust workspace tests, workspace clippy with
warnings denied, strict MkDocs build, GitLab CI lint, full `agov validate`, plan
validation, strict six-surface projection validation, learning validation,
infosec scan, archive self-sufficiency self-tests, MCP JSON sanitization, and
`git diff --check`. The current exact-head CI independently passed every
automatic job, including real framework/tooling archive previews, governance,
commercial corpus, dependency deny/CVE, OCI, clippy, and workspace tests.

The only unperformed plan item is the explicitly permitted live-surface check:
no Claude Code session was available here. After merge, run `/context` and
confirm `.claude/CLAUDE.md` and imported root `AGENTS.md` content appear under
**Memory files**. This is recorded as post-merge monitoring, not claimed as
completed text-only evidence.

## Necessary expansion and learning

Four approved-criterion owner paths were absent from the plan's affected-file
estimate: the self-sufficiency script, `dist.rs` clean-archive test, structured
projection record, and root `AGENTS.md` version comment (the plan incorrectly
said the comment already existed). Each was disclosed and added only because
omitting it would leave an approved criterion incomplete. Learning
`20260827T091844Z-as7k` records the miss and proposes deriving file estimates
from acceptance-criterion owner closure plus verifying every “already exists”
premise against the target tree.

## CI correction and duplicate cost

Earlier head `9f1255aa` pipeline `2795911940` failed only in job
`16142684381`: its permission-mode unreadable fixture assumed a non-root runner,
while GitLab ran as root. Commit `26cd74e0` replaced that assumption with a
deterministic post-resolution open failure. The paid failed job was not retried;
the corrected SHA received the next required acceptance run.

Redundant same-SHA push pipeline `2795907982` was canceled in favor of the
canonical MR pipeline. Fifteen jobs had started and consumed 673.29 aggregate
runner-seconds before cancellation: canceled jobs `16142658594` and
`16142658601` through `16142658606`; completed jobs `16142658592` through
`16142658600`, excluding canceled `16142658594`.

This handoff stops at Gate 2. It does not authorize merge, tag, release,
publication, deployment, or any broader loader/import/MCP behavior.
