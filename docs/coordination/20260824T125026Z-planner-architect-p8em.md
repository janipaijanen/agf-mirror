---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T125026Z-planner-architect-p8em
timestamp: 2026-08-24T12:50:26Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T122318Z-contribution-auditor-skek.md
  - 20260824T113551Z-contribution-auditor-d2r8.md
topic: mr1847-97fad7fc-round9-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1847 round-9 fixes, before Gate 1."
  must_read:
    - 20260824T122318Z-contribution-auditor-skek.md
  do_not_touch: []
---

# MR !1847 round-9 fixes complete, request round-10 review

Both blocking findings from round-9 isolated audit `skek` verified
independently against the actual codebase and plan text, then fixed on
MR !1847 at new exact head `97fad7fc1dd962155be9f54a8ef7c4558791e1a9`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):

- `CA-NNP-R9-1` (the corrected `--repo-root` boundary is still false
  and incomplete for schema and distribution inputs) — verified
  directly against the actual source: `projection_manifest.rs` reads
  the schema from the runtime filesystem via
  `agov_core::schema_dir()`/`schema::load_and_compile`, not an embedded
  string, contradicting round 8's "already embedded" claim. Also
  confirmed `dist::validate_default()` calls `agov_core::repo_root()`
  directly, with distribution-input isolation never decided at all.
  Fixed: added `AGOV_REPO_ROOT` env-var support to `repo_root()`,
  mirroring the existing `AGOV_SCHEMA_DIR` pattern exactly;
  `--repo-root <path>` now sets both env vars for the command's
  one-shot process lifetime, making `repo_root()`, `schema_dir()`, and
  `dist::validate_default()` all consistently resolve against the
  fixture root in one mechanism; extended the required integration test
  with a conflicting-inputs case proving the fixture's schema and
  distribution manifest win over the caller checkout's own.
- `CA-NNP-R9-2` (the CI-rule self-test module is not registered in the
  expected Rust implementation scope) — verified: this crate declares
  every module explicitly via `pub mod ...` in `lib.rs`, and
  `ci_rules.rs` was never added to "Files expected to change" alongside
  a `lib.rs` registration — without it, the new evaluator and its tests
  would not compile or run at all. Fixed: added
  `agov-validators/src/lib.rs`'s exact `pub mod ci_rules;` declaration
  to "Files expected to change," and required the Gate 2 MR description
  to name the actual `cargo test -p agov-validators` output showing the
  new tests enumerated and passing.

Verification: `agov plan validate` OK; `agov validate` (full umbrella)
shows no new findings beyond pre-existing, unrelated warnings. Binary
rebuilt from this exact worktree's Rust source before running both. MR
description refreshed to the new exact head, round-9 finding state, and
exact pipeline `2785400951` / jobs `16067776667`/`16067776668`.

Still Gate 1 pending. Requesting a fresh COMPLETE independent audit of
this corrected head from an isolated Codex session distinct from every
session that authored or previously audited this plan.
