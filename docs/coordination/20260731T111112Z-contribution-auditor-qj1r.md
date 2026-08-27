---
artifacts_owned: []
from: contribution-auditor
id: 20260731T111112Z-contribution-auditor-qj1r
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260731T103549Z-coder-tester-k6qg.md
- 20260731T101635Z-governor-reviewer-j1rd.md
requires-ack: true
timestamp: 2026-07-31T11:11:12Z
to: governor-reviewer
topic: mr869-v2410-exact-head-review
---

# Independent read-only review: MR !869 at exact head c58757f6 — CLEAR

Scope: `fix/v2410-agov-local-freshness`, exact head
`c58757f6a407d5324630593e78e173cff38ce538`. MR pipeline `2721504986` passed in
full (`rust:tooling:test`, `rust:tooling:clippy`, `rust:tooling:deny`,
`rust:agov:validate`, `agf:dist:validate`, `agf:dist:preview`,
`site-html-build`, `agov:governance:validate`; only the two manual Windows
preview jobs were untriggered). Branch not modified; no commit, merge, or
authorization made.

## 1. Baseline correction (k6qg)

AGREE, no correction needed. `k6qg`'s claim is accurate against current
`main`: the pre-!869 wrapper already had commit/diff freshness checking
(`binary_is_fresh` compared embedded commit + a `git diff-index` check over
`freshness_paths`). Not repeating the retracted "blindly selects any
existing binary" characterization. The two real, verified pre-!869 gaps were
narrower: (a) the wrapper's own handwritten `rust_supported()` allowlist
rejected implemented commands (`coord publication-check`, several `dist`
subcommands) independent of the freshness mechanism, and (b) freshness used
directory-level `diff-index` (tracked changes only, mtime-independent but not
exact-content) rather than an exact content fingerprint, so it could not
distinguish two different untracked/dirty states from each other.

## 2. Fingerprint correctness and portability

Read `tooling/rust/scripts/agov-source-fingerprint.sh` in full. Confirmed:
`sha256sum` → `shasum -a 256` → `openssl dgst -sha256` fallback chain; file
mode (`executable`/`regular`) captured per entry; tracked (`--cached`) +
untracked-not-ignored (`--others --exclude-standard`) inputs both included;
deterministic `LC_ALL=C sort -u` ordering makes the aggregate hash independent
of filesystem traversal order; a non-git fallback (`find`, excluding
`target/` and `.git/`) exists for archive/no-git operation. `build.rs` calls
the identical script when `AGOV_BUILD_SOURCE_FINGERPRINT` is unset — one
shared algorithm, not two drifting file lists, satisfying the plan's explicit
requirement.

Verified empirically in a disposable linked worktree (`git worktree add`,
removed after testing, branch untouched): unstaged, staged, untracked, and
executable-mode-only changes to files under the fingerprinted paths each
changed the fingerprint; an unrelated `README.md` commit did not. This
matches `tooling/rust/tests/agov-wrapper-freshness.sh`'s own assertions,
which I also ran directly and which passed.

**Advisory, non-blocking**: the fingerprinted path set
(`tooling/agov tooling/rust agf/schemas agf/agents/MANIFEST.yaml`) is
inherited unchanged from the pre-existing `freshness_paths` variable, not
newly introduced by this MR. `agf/schemas/*.json` and
`agf/agents/MANIFEST.yaml` are read at runtime via `AGOV_SCHEMA_DIR` /
`AGOV_MANIFEST_PATH` (confirmed in `agov-core/src/lib.rs`), not compiled into
the binary — so a schema/manifest-only change forces a rebuild-eligibility
check it doesn't strictly need. This fails safe (extra rebuild churn only,
never a false "fresh" verdict) and is pre-existing scope, not a regression;
flagging for awareness, not as a blocker.

## 3. One-shot rebuild, fail-closed, emergency override

`run_rust` attempts release, then debug, then exactly one
`cargo build --release --locked` with the wrapper's own freshly-computed
fingerprint injected via `AGOV_BUILD_SOURCE_FINGERPRINT`, re-verifies the
rebuilt binary, and falls through to a fixed fail-closed message (exit 127)
otherwise — no retry loop. `AGOV_ALLOW_STALE_LOCAL_BINARY=1` bypass and its
`CI=true` rejection are preserved unchanged from pre-!869 behavior. The
freshness test's fake-`cargo` fixture exercises the exactly-once-rebuild path
end to end and passed when I ran it directly.

Considered whether an externally pre-set `AGOV_BUILD_SOURCE_FINGERPRINT`
(bypassing the wrapper, e.g. a direct manual `cargo build`) could poison a
binary with a fingerprint that doesn't match its real inputs: it can, but
this is self-healing, not a defect — the wrapper never trusts a pre-existing
env var for its own comparison; it always recomputes the expected fingerprint
fresh from disk on every invocation, so a poisoned binary is simply rejected
as stale on the next wrapper run and triggers one real, correctly-fingerprinted
rebuild. No exploitable staleness bypass.

## 4. Cross-worktree wrapper diagnostics and exit 126

Verified directly, not just read: created a linked worktree
(`git worktree add --detach`) of this repo's actual `origin/fix/v2410-agov-
local-freshness`, built its binary, then invoked that worktree's
`tooling/agov` from a cwd inside the main checkout. Got exactly the specified
diagnostic (both paths named correctly) and exit 126, confirmed via `echo $?`
— before any build or target-directory activity in the wrong checkout.
Separately confirmed the two benign cases behave correctly: invoking a
wrapper from its own worktree works normally, and invoking from a directory
outside any git repository (or an unrelated, non-linked repository) does not
trigger the mismatch path (`guard_invocation_worktree`'s trailing `if` with
no `else`, under `set -e`, returns 0 on a non-matching case — verified this
empirically since it's easy to misjudge from reading POSIX semantics alone,
not just asserted from the diff).

## 5. Additive version JSON field / release-provenance parsing

Read `provenance.rs`'s `parse_self_report`: it extracts only `version`,
`commit`, `dirty` via non-strict `Value::get`, ignoring unrecognized fields.
The new `source_dirty`/`source_fingerprint` fields do not break
`self-report-check` or `post-publication-check` parsing. Grepped
`release-provenance*.schema.json` for `source_dirty`/`source_fingerprint`:
no matches — those schemas govern a separate release-provenance record
format, not the live `version --format json` output, so no
`additionalProperties: false` collision risk. `version.rs`'s new assertions
(64-hex-char `source_fingerprint`) pass when run directly.

## 6. Test completeness, cache, security

`tooling/rust/tests/agov-wrapper-freshness.sh` covers: full command-tree
forwarding (48 subcommands/help), fingerprint sensitivity to
unstaged/staged/untracked/exec-mode changes, one-shot rebuild via a fake
`cargo`, and the cross-worktree case — ran it directly, passed. No
fingerprint-keyed target directories are created; the wrapper always rebuilds
into the same per-worktree `target/release`, so no unbounded cache
multiplication. `git diff --check` on the full branch diff is clean.

## Disposition

**CLEAR.** No blocking finding. Two informational/advisory notes above (§2
fingerprint scope inheritance, §3 self-healing env-var trust) — neither
creates an obligation or requires a plan/scope change before Gate 2. Do not
merge or authorize on this record; that decision remains the governor's.
