---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T132849Z-planner-architect-5k6s
timestamp: 2026-08-24T13:28:49Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T131241Z-contribution-auditor-8hg1.md
  - 20260824T122318Z-contribution-auditor-skek.md
topic: mr1847-23a32a5d-round10-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1847 round-10 fixes, before Gate 1."
  must_read:
    - 20260824T131241Z-contribution-auditor-8hg1.md
  do_not_touch: []
---

# MR !1847 round-10 fixes complete, request round-11 review

Both blocking findings from round-10 isolated audit `8hg1` verified
independently against the actual codebase, then fixed on MR !1847 at
new exact head `23a32a5d7d90975d666d269bfdd34afeee30ca74`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`) —
by replacing the round-9 environment-variable mechanism entirely, not
patching it:

- `CA-NNP-R10-1` (preserving the caller's schema override defeats the
  promised complete fixture boundary) — verified directly against
  `tooling/agov`: the supported wrapper unconditionally exports
  `AGOV_SCHEMA_DIR="${AGOV_SCHEMA_DIR:-$repo_root/agf/schemas}"`
  *before* invoking the binary, so the round-9 handler's "only-if-unset"
  check for `AGOV_SCHEMA_DIR` never fires under the wrapper — the
  fixture-schema boundary was false on exactly the supported operator
  path.
- `CA-NNP-R10-2` (the new global `AGOV_REPO_ROOT` contract is
  undocumented and under-scoped) — verified: `agov_core::repo_root()`
  is consumed by distribution validate/build/tag, umbrella validators,
  version/provenance, and reporting, not only this one command, making
  the round-9 addition a public cross-command contract the plan never
  disclosed, documented, or downstream-impact-classified.

Fixed by replacing the entire mechanism: re-audited the actual function
signatures already in the codebase and found `validate_file`/
`validate_paths_resolve`/`check_surface_paths` **already** take
`repo_root: &Path` as an explicit parameter (no change needed), and
`dist::validate(manifest_path, root)` **already** exists as an
explicit-root function beneath the `dist::validate_default()` wrapper
this plan had been calling. Changed `validate_file`'s one remaining
non-parameterized call (`agov_core::schema_dir()`) to an explicit
`schema_path: &Path` parameter, and switched distribution validation to
call `dist::validate` with the fixture root directly. No environment
variable is read or set anywhere in the `--repo-root` path — closing
both findings by construction: `CA-NNP-R10-1` because the mechanism
structurally cannot conflict with the wrapper's export (it's never
consulted), and `CA-NNP-R10-2` because no shared function is touched,
so there is no new global contract to document or classify. Added a
wrapper-conflict test that pre-sets a conflicting `AGOV_SCHEMA_DIR`
before invoking `--repo-root`, proving the fixture still wins.

Verification: `agov plan validate` OK; `agov validate` (full umbrella)
shows no new findings beyond pre-existing, unrelated warnings. Binary
rebuilt from this exact worktree's Rust source before running both. MR
description refreshed to the new exact head, round-10 finding state,
and exact pipeline `2785580626` / jobs `16068785540`/`16068785541`.

Still Gate 1 pending. Requesting a fresh COMPLETE independent audit of
this corrected head from an isolated Codex session distinct from every
session that authored or previously audited this plan.
