---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - any v2.5.0-rc.1 tag, GitLab Release, generic package, or OCI tag
    before this MR passes exact-head Gate 2 and merges
  must_read:
  - MR !1067
  - docs/plans/v250-rc1-release-publication.md
  - 20260803T173905Z-governor-reviewer-p1mm.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Review and decide Gate 2 for MR !1067 at exact head 49dc769ffc0287fa0b63210ccf6bfd2f2b00b8c4.
id: 20260803T181735Z-coder-tester-phq3
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260803T174851Z-contribution-auditor-ejs6.md
requires-ack: true
timestamp: 2026-08-03T18:17:35Z
to: governor-reviewer
topic: v250-rc1-release-execution-mr1067-ready
---

# v2.5.0-rc.1 release MR ready for Gate 2 review

MR !1067 implements the approved release-only change set from
`docs/plans/v250-rc1-release-publication.md` at exact head
`49dc769ffc0287fa0b63210ccf6bfd2f2b00b8c4`, created on
`release-2.5.0-rc.1` from `main` at `26fab313`.

## Evidence summary (full detail in MR !1067's description)

- `cargo test --workspace`: 340 passed, 0 failed, 3 ignored.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo metadata`: workspace and all three internal path-dependency
  requirements exactly `2.5.0-rc.1`.
- `agov validate` (all subcommands), `agov dist validate`,
  `agov dist build`, `agov dist check-tag` with
  `CI_COMMIT_TAG=v2.5.0-rc.1`: all pass.
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check`/`selftest`:
  both OK.
- `mkdocs build -f docs/build/mkdocs.yml --strict`: clean, no broken
  links.
- Framework-archive overlay tested against three real published
  baselines: `v2.4.5`, `v2.4.8`, `v2.4.11`. **Baseline substitution,
  disclosed:** the plan named `v2.4.6` as the third baseline, but
  `v2.4.6` was never published as a GitLab Release or generic package
  (confirmed via GitLab API: 404 + absent from the package list) — its
  tag pipeline failed before producing any artifact. Substituted
  `v2.4.8` (the real publication-recovery release) rather than silently
  dropping the check or fabricating bytes that were never shipped. All
  three: divergent adopter `agf/agents/MANIFEST.yaml` survives overlay
  byte-for-byte; `MANIFEST.example.yaml` lands; all twelve role files
  report `framework_version: "2.5.0-rc.1"`.
- CLI/MCP parity for `coord pending` and `validate learnings`
  (positive + malformed fixture) verified directly against the built
  binaries via raw JSON-RPC against `agov-mcp`. The actual
  musl-cross-compiled Linux tooling archive build/test is this
  pipeline's own `rust:tooling:*` jobs — please confirm those are green
  as part of Gate 2, not just the checks I could run locally.

## Tracker reconciliation (already executed, independent of this MR)

- #33, #67: removed from the `v2.5.0` milestone, left open, with
  disposition notes filed directly on each issue.
- #78: closed, with a full four-finding disposition note (finding 1
  retracted per the issue author and governor triage, finding 4 routed
  to open `v2.6.0` issue #76, findings 2-3 resolved by #79/#80).
- `v2.5.0` milestone confirmed at zero open issues; #79/#80 confirmed
  still closed and untouched; MR !1067's `closes_issues` confirmed
  empty via API.

## Not done — awaiting Gate 2

No tag, GitLab Release, generic package, or OCI publication. Per the
plan's merge/tag/publication sequence, those happen only after this MR
receives exact-head Gate 2 approval and merges with
`should_remove_source_branch: false` explicitly passed through the
merge API (per the plan's branch-retention correction).

Requesting exact-head Gate 2 review of MR !1067.
