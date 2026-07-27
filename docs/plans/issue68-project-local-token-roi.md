---
id: issue68-project-local-token-roi
status: approved
topic: issue68-project-local-token-roi
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-27T01:58:19Z"
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: >-
      Issue 68 is a maintainer-filed local tooling defect with explicit
      acceptance criteria, not an external contribution or precedent-setting
      governance claim; implementation evidence remains subject to Gate 2.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The change resolves local filesystem paths for metadata-only ROI JSONL
      and adds warnings; it does not add secrets, authentication, network
      transport, learning records, or cross-record data flow. Tests must still
      prove warnings do not print telemetry contents.
---

# Plan: project-local token ROI telemetry (issue 68)

## Trigger and release placement

Governor decision `20260727T014112Z-governor-reviewer-hjha` includes issue #68
in v2.4.5. The defect is that a long-lived `AGOV_TOKEN_ROI_LOG` value expanded
from another directory can silently mix unrelated projects, while invocation
outside an AGF checkout silently falls back to the current directory.

This plan is a child scope of v2.4.5. It does not change the standardized
documentation-review protocol or authorize release preparation.

## Decision

Use existing AGF project-root discovery. Do not add a dotfile or a new config
key in this slice.

Resolution precedence is:

1. explicit `--log` for `agov token roi report`;
2. `AGOV_TOKEN_ROI_LOG`;
3. `<detected-project-root>/tmp/agov-token-roi.jsonl`.

An explicit path remains honored. If `--log` or `AGOV_TOKEN_ROI_LOG` resolves
outside the detected project root, print one warning per process naming the
resolved path and the suggested project-local path. Do not print log contents.

If no AGF project root is detectable and no explicit path is supplied, ROI
logging is disabled and `token roi report` returns a clear non-success result
that says no project root was found; it must not create a user-global or
arbitrary-CWD log. An explicit path outside a project remains usable with a
warning that project isolation cannot be verified.

Relative explicit paths are resolved against the invocation directory and then
normalized for comparison and reporting. Symlink/canonicalization failures use
lexical absolute normalization and are reported honestly; the implementation
must not require the target file to exist before resolving it.

The default remains generic. Multiple assistants in one project continue to use
explicit stable per-assistant paths. Automatically deriving an assistant name
from `AGOV_MODEL` would conflate models, surfaces, and assistants and is out of
scope.

## Implementation scope

- Add a reusable ROI path-resolution result in `agov-core` carrying the path,
  source (`cli`, `environment`, or `project-default`), detected root, and an
  optional warning.
- Reuse `find_repo_root_from`; do not change unrelated callers of `repo_root()`.
- Make automatic telemetry writing and `token roi report` consume the same
  resolver so reports cannot disagree with writers.
- Preserve `AGOV_TOKEN_ROI=0/false/off` as the higher-level logging disable
  switch.
- Keep `--log` on `token roi report`; do not introduce a global CLI flag.
- Update CLI help, `docs/reference/tooling-token-roi.md`, the environment
  variable reference, and foundry guidance so `$PWD` exports are explicitly
  per-project and not recommended as shell-global configuration.

## Tests and acceptance evidence

- Two temporary AGF roots resolve to different default logs.
- Nested directories resolve to the same root-local log as their project root.
- An inherited outside-project `AGOV_TOKEN_ROI_LOG` is honored and emits the
  resolved-path warning plus a project-local suggestion.
- An explicit `--log` has precedence over the environment and receives the
  same boundary warning behavior.
- Invocation without a recognizable project and without an explicit path does
  not create a log and reports the missing root.
- Invocation without a project but with an explicit path works with an
  unverifiable-isolation warning.
- Disabled logging remains disabled.
- Existing record atomicity and report-summary tests remain green.
- CLI integration tests assert warnings contain paths but never JSONL content.
- Rust formatting, clippy, unit/integration tests, governance validation, and
  the exact-head MR pipeline pass.

## Explicit exclusions

- No `.agovrc`, `.env`, or new `agov-config.yaml` key.
- No telemetry migration or deletion outside the repository.
- No assistant-name inference or renaming of existing logs.
- No telemetry schema/content change, network transport, release branch,
  version bump, tag, or release artifact.

## Gate boundary

Implementation begins only after a human governor changes this plan to
`approved` and records Gate 1 approval. Gate 2 remains a separate exact-head
human review and merge decision.
