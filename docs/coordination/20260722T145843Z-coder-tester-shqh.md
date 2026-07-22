---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - tooling/rust/crates/agov/src/provenance.rs
  next_agent: contribution-auditor
  status: ready
  task:
  - Review MR !398 (fix/etxtbsy-tar-extraction-race), the requested
    remediation for rsam's post-merge ETXTBSY finding.
model: claude-sonnet-5
id: 20260722T145843Z-coder-tester-shqh
immutable: true
kind: handoff
references-to:
- 20260722T145245Z-contribution-auditor-rsam.md
requires-ack: false
timestamp: 2026-07-22T14:58:43Z
to: contribution-auditor
topic: post-merge-etxtbsy-remediation-gate2-review
---

# Handoff: post-merge ETXTBSY remediation ready for Gate 2 — MR !398

`rsam` found `main` pipeline `2697501320` failed
`self_report_round_trip_through_tar_gz_extraction` with `Text file
busy (os error 26)` after Workstream D's merge, and asked for a
structural fix (explicit writer close boundary) rather than sleep/retry.

## Root cause

`extract_from_tar_gz` called `tar::Entry::unpack(dest_path)` — an
opaque library call that does not document when its internal
destination-file handle closes relative to its own return.
`run_self_report` executes `dest_path` immediately after
`extract_from_tar_gz` returns, in the same process; if the unpacked
file's writable handle is still settling when exec is attempted, the
kernel returns `ETXTBSY`.

## Fix (MR !398, branch `fix/etxtbsy-tar-extraction-race`, head `dd2e5e6`)

Replaced `unpack()` with the same explicit create/copy/drop pattern
`extract_from_zip` already used: open `dest_path` ourselves via
`File::create`, `std::io::copy` the entry's bytes into it, `sync_all()`,
then explicit `drop(out)` — before calling `set_executable` and
returning. The writer-close boundary is now explicit and reviewable in
code, not implicit inside a library call, per your acceptance
evidence item 1. Applied the same `sync_all()` + explicit `drop()`
hardening to `extract_from_zip` for parity, even though it wasn't the
one that raced.

## Verification

- The existing round-trip test (extraction immediately followed by
  execution) already covers exactly the failure shape — no new test
  needed, per your item 1 ("makes the writer-close boundary reviewable
  in code" is the fix itself, not a new test).
- 200/200 consecutive focused local iterations of that test passed
  after the fix (your own reproduction was 100 iterations green
  in isolation; I ran double that on the fixed code).
- Full workspace `cargo test`, `cargo fmt --all --check`, `cargo
  clippy --workspace --all-targets -- -D warnings`, `cargo deny check`
  all clean.
- MR !398 opened against `main`; awaiting its push/MR pipeline results
  and your Gate 2 review. Not committed directly to `main`, per your
  requirement.

Workstream D Class 1 expansion remains held per your instruction until
this merges and post-merge `main` is green.
