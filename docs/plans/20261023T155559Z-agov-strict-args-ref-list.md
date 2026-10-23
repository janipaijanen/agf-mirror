---
id: 20261023T155559Z-agov-strict-args-ref-list
status: pending-gate1
topic: agov-strict-mcp-arguments-and-ref-aware-coordination-list
gate1_approved_by: ""
gate1_approved_at: ""
depends_on:
  - agov-mcp-transport-protocol-correctness
  - mcp-coordination-dir-contract-fix
  - agov-coordination-read-path-reliability
  - "!3105"
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      Requested via `20261023T155752Z-planner-architect-ehh4` because Slice A
      tightens the MCP request trust boundary and Slice B passes an untrusted
      revision string to local Git. Review must confirm fail-closed unknown-key
      handling, argument separation, option injection resistance, and no
      implicit network access.
  - role: contribution-auditor
    status: pending
    reason: >-
      Requested via `20261023T155752Z-planner-architect-ehh4` to verify that
      the two independently testable slices match the governor-approved
      proposal without absorbing the existing MCP coordination-directory
      plan, input-provenance issue #49, or watcher state issue #133.
affected_paths:
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/coord_list_ref.rs
  - tooling/rust/crates/agov-mcp/src/main.rs
  - tooling/rust/crates/agov-mcp/tests/stdio.rs
  - agf/docs/site/docs/adopt/mcp-server.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/docs/site/reference/tooling-conformance.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: strict MCP arguments and ref-aware coordination deltas

## Decision and scope

The governor approved the proposal in the live Codex session on 2026-10-23
and directed: plan first, then implement. This file is the exact Gate 1
candidate; implementation still waits for completed advisory dispositions,
validation, publication, and a binding governor-reviewer record that names
this plan. No ADR is needed: both slices extend existing command/adapter
contracts without changing architecture, storage, authority, or the
repository-native coordination protocol.

This plan contains two narrow slices because they close the two new defects
proven in the same operator audit and can share one exact-head CLI/MCP parity
run. Either slice remains independently revertible and testable.

### Slice A — reject unknown MCP tool arguments

`agov-mcp` currently ignores unknown keys for most tools. A reproduced call
to `agov_coord_list` with misspelled `sence` returned success and executed
`agov coord list --format json`, silently dropping the intended cursor.
Only 4 of 21 advertised schemas currently set
`additionalProperties: false`; server dispatch also does not enforce the
published property set for most tools.

Implement one shared pre-dispatch check derived from each published tool's
`inputSchema.properties`. Every unknown key fails before the child `agov`
process starts, with a diagnostic naming the tool and key. Set
`additionalProperties: false` on all 21 published schemas so capable clients
reject the same request before transport. Preserve the existing tailored
errors for deliberately known-but-unsupported `include_forge` and `output`
arguments. Do not add a second hand-maintained key registry or a dependency.

### Slice B — read `coord list` from a fetched Git ref

`agov coord list` currently reads only the checked-out filesystem. Live proof
at the audit baseline: branch `coordination/b1-claude-review-split` did not
contain coordination record `20261023T152434Z-coder-tester-a8uk`, fetched
`origin/main` did contain it, and
`agov coord list --since 2026-10-23T15:00:00Z --format json` returned exit 2
with “No coordination messages found.” This makes the required fetch-first
poll path capable of a false-empty delta unless the checkout is already at
the fetched target; the broader visibility scan eventually recovers the
record but defeats the compact first check.

Add optional `agov coord list --ref <commit-ish>` and the equivalent MCP
`ref` string. Omission preserves current filesystem behavior byte-for-byte,
except for the existing empty-JSON defect described below. Ref mode:

1. resolves `<commit-ish>^{commit}` once, with Git option parsing terminated;
2. performs no fetch and no forge/network call;
3. reads only direct lowercase `.md` channel members under the selected
   coordination directory, matching the governed channel-membership rule;
4. uses one bounded tree inventory and one batched blob read, reusing the
   existing visibility plumbing rather than spawning one process per file;
5. validates, sorts, applies `--since`, and computes supersession exactly as
   filesystem mode does; and
6. reports an unavailable ref or unreadable/incomplete Git input as exit 2,
   never as a clean empty result.

For text output, ref mode prints one source line containing the requested ref
and resolved commit OID before message rows. For JSON, ref mode returns an
object with `source_ref`, `source_oid`, `coordination_dir`, and `messages`.
That object shape is new and applies only when the new `--ref` flag is used;
existing non-ref JSON remains an array.

Today `coord list --format json` emits the plain-text sentence “No
coordination messages found” when the result is empty. Correct that zero-row
edge in this slice: non-ref JSON returns `[]`; ref JSON returns its provenance
object with `messages: []`; both retain exit 2 as the documented expected
polling-empty status. Text output and exit semantics remain unchanged.

## Collision and sequencing disposition

Fresh fetch and open-MR inspection found one current implementation overlap:
MR !3105 touches both Rust entrypoints and `mcp-server.md` while retiring
Windows support. It does not implement either behavior here. Rebase this work
after !3105 if that MR merges first; otherwise re-run the exact range diff and
resolve only those already-reviewed mechanical removals. Do not merge this
implementation ahead of an unresolved textual or behavioral conflict.

The declared plan dependencies acknowledge older live-plan path overlaps:
the transport-correctness plan owns JSON-RPC framing, the directory-contract
plan owns `dir` → `coordination_dir`, and the read-path-reliability plan owns
the earlier cursor/visibility fixes. This plan neither supersedes nor edits
their contracts. The collision gate must be rerun after this file is drafted;
any additional reported live-plan overlap requires an explicit sequencing
disposition before Gate 1.

## Tests and acceptance

Slice A:

- A table-driven test sends one extra key to every advertised MCP tool and
  proves a JSON-RPC error names the tool/key and the mock child was not run.
- `tools/list` proves every schema has `additionalProperties: false`.
- Existing valid argument mapping, bespoke CLI-only/output diagnostics,
  malformed JSON-RPC, bounded-line, and CLI/MCP parity tests remain green.

Slice B:

- A new integration fixture proves a record present only at another ref is
  returned after a cursor while the checkout remains unchanged.
- Tests cover annotated/lightweight ref resolution as applicable, missing
  ref, a ref-like value beginning with `-`, non-commit objects, empty channel,
  invalid channel records, custom in-repository `--dir`, and an absolute or
  outside-repository directory rejection in ref mode.
- Filesystem mode output remains unchanged for non-empty text and JSON cases.
- Empty `--format json` is valid JSON at exit 2 in both filesystem and ref
  modes.
- CLI and MCP return equivalent ref-mode status/stdout/stderr, with the MCP
  command vector showing `--ref` as a separate argument.

Required verification:

```text
cargo fmt --check --manifest-path tooling/rust/Cargo.toml
cargo clippy --workspace --all-targets --manifest-path tooling/rust/Cargo.toml -- -D warnings
cargo test --workspace --manifest-path tooling/rust/Cargo.toml
agov plan validate docs/plans/20261023T155559Z-agov-strict-args-ref-list.md
agov plan collision-check --base-ref origin/main --plans-only --changed-only --fail-on-unacknowledged-collision
git diff --check
```

Gate 2 evidence must name the exact head, exact pipeline/jobs, the reproduced
pre-fix typo and false-empty cases, and the corresponding post-fix results.

## Security, compatibility, and rollback

- Slice A narrows accepted input only: documented keys keep their behavior;
  misspelled/extra keys that previously appeared successful become explicit
  errors.
- Slice B is read-only and local. The ref and directory are separate Git argv
  values, option parsing is terminated, the resolved commit OID is pinned for
  the full read, and no shell interpolation or implicit fetch is introduced.
- No schema, coordination record, plan lifecycle, role authority, or gate
  semantics change. No new dependency, daemon, cache, database, or watcher is
  added.
- Roll back either slice by reverting its commits before release. Existing
  no-ref CLI behavior and all current MCP key names remain the compatibility
  baseline.

## Explicitly out of scope

- MCP child-binary identity/version attestation (issue #49 is the adjacent
  provenance owner).
- The existing `dir`/`coordination_dir` contract fix.
- Watcher, inbox, and work-queue convergence tracked by issue #133.
- Adding MCP tools for release, CI, distribution, usage, or token reports.
- Repository initialization, automatic fetch, GitLab/ACS transport, or an
  external polling daemon.
