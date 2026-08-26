---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - main
  must_read:
  - docs/plans/ci-rust-cache-policy-slice-a.md
  next_agent: contribution-auditor
  status: ready
  task:
  - 'Independently verify MR !930 exact head adfe4576123309568f2d29f2d8b295b6faeddb59: full trace enumeration of all 12 shared-cache consumers (8 automatic already trace-checked by Claude, 2 manual source-checked, 2 release-only source-checked), confirm no archive_cache upload occurs on any non-default-branch job, and independent job-set parity check before recommending Gate 2.'
id: 20260802T100519Z-coder-tester-m3rb
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260802T091308Z-coder-tester-bgkb.md
requires-ack: false
timestamp: 2026-08-02T10:05:19Z
to: contribution-auditor
topic: ci-rust-cache-policy-slice-a-implementation-ready
---

# Slice A implementation ready for independent trace verification

MR !930 exact head `adfe4576123309568f2d29f2d8b295b6faeddb59`. Two prior
heads on this branch failed and were fixed in the open:

- `a1bbe340`: first implementation, pipeline never checked whether the
  runtime assertion actually ran everywhere.
- `f69dcb43`: fixed 4 of 12 consumers silently skipping the assertion
  (`extends` doesn't merge `before_script` arrays; used `!reference` to
  compose instead) — but this introduced a duplicate `cd tooling/rust`,
  which broke `rust:tooling:preview` (`cd: tooling/rust: No such file or
  directory`).
- `adfe4576` (current): removed the now-redundant `cd tooling/rust` lines.
  Pipeline `2724978736` passed in full.

## Evidence gathered so far (my own pass, independent verification requested)

- **8 automatic shared-cache consumers**, all trace-checked directly:
  `rust:tooling:clippy`, `rust:tooling:test`, `rust:tooling:deny`,
  `rust:agov:validate`, `agov:governance:validate`, `agf:dist:validate`,
  `rust:tooling:preview`, `agf:dist:preview`. Every trace shows
  `OK resolved cache policy: pull (source=merge_request_event ...)` from
  the runtime assertion, `Checking cache for rust-v2-main-protected...`,
  and `Not uploading cache rust-v2-main-protected due to policy` — zero
  `Uploading cache to`/`Created cache` lines in any of the 8.
- **2 manual consumers** (`rust:tooling:build:windows:preview`,
  `rust:tooling:preview:windows-package`): not invoked in this pipeline;
  source-checked instead — neither overrides `cache:`, both now correctly
  reference `.rust-base`'s `before_script` via `!reference` rather than
  silently dropping it.
- **2 release-only consumers** (`agf:dist:release`, `rust:tooling:release`):
  not present in an MR pipeline by design (tag-only `rules:`); source-
  checked the same way — no `cache:` or `before_script` override.
- **Job-set parity**: cache policy and job-selection `rules:` are
  orthogonal YAML keys; no job's `rules:` references cache state, so job
  selection cannot be affected by this change by construction. GitLab CI
  Lint API (`dry_run=true`) against this exact content returns the same
  18-job set for `main` and the `v2.4.11` tag across every version of this
  diff, including before any cache-policy change existed.
- `sh .gitlab/scripts/check-rust-cache-policy.sh selftest` and
  `agov validate` both pass locally on the exact head.

Requesting your own independent re-derivation of this evidence — especially
re-fetching the 8 traces yourself rather than trusting my summary — before
recommending Gate 2 to the governor.
