---
id: 20261023T155559Z-agov-strict-args-ref-list
status: pending-gate1
topic: agov-strict-mcp-arguments-and-ref-aware-coordination-list
gate1_approved_by: ""
gate1_approved_at: ""
depends_on:
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle
  - 20260913T165127Z-row5-s4-cp-a-ack-pending-display
  - 20260913T195450Z-row5-s2-self-announcing-supersession
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - agov-mcp-transport-protocol-correctness
  - mcp-coordination-dir-contract-fix
  - agov-coordination-read-path-reliability
  - 20261021T220000Z-retire-windows-support
  - "!3105"
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      The exact-head InfoSec review is to be performed and published by coder-tester (Codex) after revision
      R2 (ACS seq 229). Evidence already available, which is not that decision: the CI validator
      `agov validate infosec` passes (it checks learning records only), and a trust-boundary read-through by
      a different assistant than the plan's author (contribution-auditor record
      `20261023T193348Z-contribution-auditor-7njp`, findings 6-10) found no blocking issue and two WARNs,
      resolved in R2.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Record `20261023T193348Z-contribution-auditor-7njp` (Claude, independent of the plan's author
      Codex): scope matches the governor's `ptds` proposal; no blocking finding; two WARNs (missing
      classification/impact/migration statement, undeclared high-risk `agov-coord` reuse boundary) plus INFOs,
      addressed in revision R2 by Claude under the allocation in ACS seq 226. R2 was written by the reviewer at
      the author's request, so the author (Codex) must still check it independently before Gate 1.
affected_paths:
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/src/coord_list_ref.rs
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

## Revision R2 (2026-10-23, Claude, per ACS seq 226): resolutions of review 7njp

This revision adds the statements and test requirements the independent review asked for. It does
not change the two slices' behavior. Mapping: finding 1 -> "Classification and impact"; finding 2 ->
"Where the ref reader lives"; finding 3 -> sequencing note; finding 4 -> `depends_on` note; finding 5 ->
"JSON shapes"; findings 6-9 -> "Trust boundary requirements"; finding 10 -> "Documentation notes".

### Classification and impact

Mode: **STRICT** (the MCP request boundary is a security boundary, and `tooling/rust/` is a
`downstream-impact-rule` trigger path).

```
Impact:
  Foundry:          operators and the polling rule gain a ref-aware first check; MCP calls with stray keys now fail.
  Direct adopters:  Slice A BREAKING under the Parnas test (requests that silently succeeded with a stray or
                    misspelled key now return an error); Slice B ADDITIVE (new optional --ref, new JSON shape only
                    when --ref is used); the empty-result JSON fix is BREAKING for any consumer that matched the
                    plain sentence in --format json (it now receives []).
  Downstream:       MCP clients that cached a looser argument set must send only the documented keys.
  Migration:        required. See "Migration note" below. Evidence: reproduced typo `sence` and the false-empty
                    delta in this plan.
```

**Migration note.** For Slice A, an MCP client should diff the keys it sends against each tool's published
`inputSchema.properties` from `tools/list` and remove any extra key; the error names the tool and key. For the
empty-result JSON change, `coord list --format json` now emits `[]` (non-ref) or an object with `messages: []`
(ref); exit status 2 for "no messages" is unchanged. Consumers that string-matched
`No coordination messages found` in JSON mode must parse the JSON instead.

Adopter docs parity: **updated** (the three documentation paths in `affected_paths` change in the same work).

### Where the ref reader lives

Ref mode is implemented in the `agov` crate (the new `coord_list_ref.rs` module and `main.rs`). It **calls**
the existing `agov-coord` read and validation functions; it does **not edit** anything under
`tooling/rust/crates/agov-coord/**` (a `high-risk-operations-rule` area: coordination integrity mechanics and
publication-hold code). If during implementation a shared helper in `agov-coord` must change or be moved, that is
a plan amendment that adds the path, names the high-risk handling and returns to Gate 1 before the change is made.
The Git-reading approach (`rev-parse`, `ls-tree`, `cat-file --batch`) mirrors the existing visibility code but is
re-implemented or called, never edited, here.

### JSON shapes

Without `--ref`, JSON stays an array (and `[]` when empty). With `--ref`, JSON is an object
(`source_ref`, `source_oid`, `coordination_dir`, `messages`). MCP clients must branch on whether they passed `ref`.

### Trust boundary requirements

1. **Leading-dash values.** The MCP layer passes the ref to the child process as the single argument
   `--ref=<value>` (never as two arguments), so a value such as `-x` is a value, not a flag. A CLI test and an MCP
   command-vector test both use a ref starting with `-`.
2. **Echoed untrusted strings.** Text output and diagnostics that include `source_ref` or a rejected key name escape
   control characters (C0, C1, DEL, ESC) as `\xNN`, cap the displayed value at 200 characters with a trailing
   marker, and never print raw newlines. JSON output is unaffected. Tests use a value containing ESC and a newline.
3. **Bounds and file types (ref mode).** Fail closed with exit 2 above 50,000 channel entries, 1 MiB per blob or
   128 MiB total (the current channel is about 4,400 records, so the bounds leave room). Read only regular blobs
   (modes 100644 and 100755); skip symlinks and submodules and report the skipped count in the coverage line.
   Reject a `--dir` that is absolute, contains `..`, a backslash or NUL, or starts with `-`.
4. **Duplicate keys.** Duplicate JSON-RPC argument keys keep today's last-wins behavior for a known key; a
   duplicated or single unknown key is rejected. A test covers both cases so a repeated key cannot smuggle a known
   key past the check.

### Documentation notes

Clients that enforce `additionalProperties: false` will reject `include_forge` and `output` before the tailored
errors can appear; the MCP documentation says so and points to the tailored text.

### Sequencing note

The two slices are independently revertible and may land as two implementation MRs, in either order, sharing one
exact-head CLI/MCP parity run when they land together. `depends_on` lists the plan `20261021T220000Z-retire-windows-support`, which acknowledges the
collision with the now-merged Windows retirement, and `"!3105"` (a merge request, not a plan), which only records
sequencing. MR !3105 merged on 2026-10-23 as `c2a73e5e`, so implementation starts from a `main` that already contains
it and no rebase over it is needed.

## Collision and sequencing disposition

Fresh fetch and open-MR inspection (R1) found one implementation overlap: MR !3105 touches both Rust
entrypoints and `mcp-server.md` while retiring Windows support. It did not implement either behavior here.
**Update in R2:** MR !3105 merged on 2026-10-23 (`c2a73e5e`), so the overlap is now between this plan and a plan
already on `main`; it is acknowledged through `depends_on` and implementation starts from the merged state. Do not
merge this implementation ahead of an unresolved textual or behavioral conflict, and re-run the changed-only
collision check against the then-current `main` before Gate 2.

The declared plan dependencies acknowledge older live-plan path overlaps:
the transport-correctness plan owns JSON-RPC framing, the directory-contract
plan owns `dir` → `coordination_dir`, and the read-path-reliability plan owns
the earlier cursor/visibility fixes. This plan neither supersedes nor edits
their contracts. The five timestamped Row 5/Row 6 plans reported by the
exact-head CI collision gate all overlap only through the shared
`tooling/rust/crates/agov/src/main.rs` entrypoint. Their established
coordination lifecycle, plan lifecycle, acknowledgement display,
supersession, and collision-gate behavior stays intact; this work starts from
their integrated `main` state and adds only the new `coord list --ref` path.
They are named in `depends_on` so that sequencing is explicit rather than
silently treating the common entrypoint as free. Any further reported live
plan overlap requires an explicit disposition before Gate 1.

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
  command vector containing the single argument `--ref=<value>` (for a ref `-x`, exactly `--ref=-x`) and
  no standalone `--ref` argument.


R2 additions to the acceptance evidence:

- Ref starting with `-` passes through both the CLI and the MCP command vector (`--ref=-x`) as a value.
- A ref or key containing ESC and a newline appears escaped and length-capped in text output; JSON is valid.
- Bounds: a fixture over each limit fails closed with exit 2; a symlink and a submodule entry in the channel are
  skipped and counted; each rejected `--dir` form (`/abs`, `..`, backslash, NUL, leading `-`) fails.
- Duplicate-key cases from "Trust boundary requirements" item 4.
- `git diff` shows no change under `tooling/rust/crates/agov-coord/**`.

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
