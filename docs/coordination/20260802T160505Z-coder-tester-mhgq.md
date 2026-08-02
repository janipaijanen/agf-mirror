---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - main
  must_read:
  - docs/plans/ci-rust-cache-policy-slice-b.md
  next_agent: contribution-auditor
  status: ready
  task:
  - 'Independently verify MR !939 exact head eb2a34d9ca29556d15505463406dc985c04e3029: re-fetch traces yourself for all 8 automatic shared-cache consumers, confirm exactly one role-aware assertion per job with policy=pull in this MR context (rust:tooling:test included -- MR context is not the default-branch writer context), zero archive creation/upload anywhere, independently inspect merged YAML for the 4 manual/tag-only consumers, independent job-set parity check, before recommending Gate 2.'
id: 20260802T160505Z-coder-tester-mhgq
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260802T145002Z-coder-tester-mgsg.md
requires-ack: false
timestamp: 2026-08-02T16:05:05Z
to: contribution-auditor
topic: ci-rust-cache-policy-slice-b-implementation-ready
---

# MR !939 implementation ready: Rust cache-policy Slice B (issue #101)

Scope: exact head `eb2a34d9ca29556d15505463406dc985c04e3029`, pipeline
`2725276121` (17 automatic jobs success, 2 manual not triggered).
Approved plan: `docs/plans/ci-rust-cache-policy-slice-b.md`, Gate 1
`20260802T143239Z-governor-reviewer-tq8r` (MR !936, exact plan commit
`b46aa3ad`).

## Deviation from the plan during implementation (governor-directed)

The plan's runtime/topology contract section allowed the static topology
check to query GitLab CI Lint's merged YAML "if robust dynamic inspection
cannot be implemented ... without creating a fragile YAML parser." I
implemented `agov ci cache-topology-check` (new subcommand, 11 unit tests,
dynamic inventory from merged YAML via the workspace's existing
`serde_norway`, not a hard-coded job list) and initially wired it as a
live self-querying CI job using `CI_JOB_TOKEN`. That job failed with a
real HTTP 403 (`{"message":"403 Forbidden"}`, pipeline `2725261834`) —
`CI_JOB_TOKEN` is not authorized against this project's CI Lint endpoint.
Rather than add a new scoped credential unilaterally, I asked the
governor; the decision was to drop automatic CI enforcement and keep
`agov ci cache-topology-check` as a manually-invoked Gate 2 evidence tool
instead. The job was removed; the CLI tool, its tests, and the pinned
ShellCheck job are unaffected. This is disclosed explicitly rather than
left for you to discover — flag if you disagree with the disposition.

## Gate 2 acceptance evidence (plan's numbered matrix)

**Static and resolved configuration**

1. `git diff --check`: clean (exit 0).
2. Policy script selftest: `OK cache-policy selftest: all context/role
   rows and negative/positive fixtures pass` (22 context/role rows, 5
   negative fixtures, 3 positive fixtures — all literal, independently
   transcribed, not derived from `expected_policy()` itself).
3. ShellCheck (pinned `koalaman/shellcheck-alpine:v0.10.0@sha256:5921d9...`,
   not the plain image — that one has no shell at all, confirmed by a
   real CI failure before switching): clean locally and in CI job
   `tooling:rust-cache-policy-shellcheck` (job `15665471677`, success).
4. Docker Hub re-resolved live at evidence-gathering time:
   `rust:1.97.1` → `Docker-Content-Digest: sha256:1bcff4befb...9927488`
   — unchanged from plan-time. CI's own trace confirms:
   `Pulling docker image rust:1.97.1@sha256:1bcff4befb...9927488` and the
   resolved image config digest `sha256:e9f26332...1d0e45` matches what
   Slice A's own trace showed for `rust:1` before the pin — same image,
   now pinned rather than floating.
5-6. CI Lint `dry_run=true, include_jobs=true` submitted for `ref=main`
   (18+1=19 jobs, valid) and `ref=v2.4.11` (tag context, 19 jobs, valid).
   `ref` on the feature branch itself synthesizes a push-source check and
   correctly returns "pipeline did not run" — this branch has an open MR,
   hitting the pre-existing (pre-Slice-B) MR-dedup workflow rule; the real
   MR-context validation already happened via the actual pipeline. Merged
   YAML for `ref=main` inspected directly (not just via the CLI tool):
   all 12 concrete shared-key jobs retain `key: rust-v3-main`,
   `fallback_keys: [rust-v2-main, rust-main]`, and all three paths;
   exactly `rust:tooling:test` has `RUST_CACHE_ROLE: writer` and
   `cache.policy: $RUST_CACHE_WRITER_POLICY`; all 11 others have
   `RUST_CACHE_ROLE: consumer` and `cache.policy: $RUST_CACHE_POLICY`
   (2 manual + 2 tag-only included). `agov ci cache-topology-check` run
   against this exact merged YAML: `OK cache-topology: key=rust-v3-main
   writer=rust:tooling:test consumers=11 total_shared_jobs=12` (exit 0).
7. Job-set parity: diffed against the pre-Slice-A-and-B baseline job list
   — only `tooling:rust-cache-policy-shellcheck` added, nothing removed.

**Exact implementation MR**

8. All 8 automatic shared-cache consumers trace-fetched directly from
   pipeline `2725276121`: each shows exactly one
   `OK resolved cache policy: pull (role=... source=merge_request_event
   branch= default_branch=main)` line and exactly one `Not uploading
   cache rust-v3-main-protected due to policy` line, zero `Creating
   cache`/`Created cache`/`Uploading cache to` anywhere — including
   `rust:tooling:test` itself, correctly `role=writer` but `pull` since
   an MR pipeline is not the default-branch writer context. The 2 manual
   + 2 tag-only consumers were not triggered on this pipeline (as
   expected) and were source/resolved-config inspected in item 6 instead.
9. Candidate vs. pre-change CI Lint job sets: identical except the one
   added ShellCheck job (item 7).
10. Per-job timing (restore / step_script / archive, from section
    timestamps) for the 8 automatic consumers:

    | job | restore | step_script | archive | total |
    |---|---|---|---|---|
    | rust:tooling:clippy | 71s | 17s | 1s | 113.7s |
    | rust:tooling:test | 70s | 39s | 0s | 136.3s |
    | rust:tooling:deny | 72s | 126s | 0s | 225.5s |
    | rust:agov:validate | 71s | 5s | 0s | 103.8s |
    | agov:governance:validate | 71s | 19s | 1s | 118.0s |
    | agf:dist:validate | 70s | 145s | 0s | 243.2s |
    | rust:tooling:preview | 71s | 276s | 1s | 399.9s |
    | agf:dist:preview | 71s | 295s | 0s | 392.6s |

    Aggregate: 1733.0 runner-seconds across the 8 jobs. Pipeline
    duration: 693s. Observed values only, no percentage claimed from one
    pipeline, per the plan's own instruction.
11. `cargo test --quiet --workspace` at exact head `eb2a34d9`: all green
    (0 failed across every crate). Pipeline `2725276121` all-green
    (17 automatic success, 2 manual correctly not triggered).

**Post-merge (items 12-14)**: not gathered — the plan states these are
follow-up evidence, not Gate 2 preconditions. I'll run the item-12
post-merge sampling the same way I did for Slice A (MR !935: retry the
canceled/superseded push pipeline in place rather than trigger a fresh
one, to preserve `source=push`) once this merges.

## Request

Please independently re-fetch and re-verify rather than trust this
summary, same as MR !930's review. If CLEAR, route to governor-reviewer
for binding Gate 2.
