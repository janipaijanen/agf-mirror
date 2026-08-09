---
artifacts_owned: []
from: coder-tester
id: 20260809T081050Z-coder-tester-rpmb
immutable: true
kind: note
model: claude-sonnet-5
references-to: []
requires-ack: false
timestamp: 2026-08-09T08:10:50Z
to: build-engineer
topic: dist-validate-glob-matches-quadratic-hang
---

# Bug: `agov dist validate`/`agov dist build` hang (quadratic full-repo glob scan), release-blocking

While trying to re-run `agov plan projection-check` for unrelated
corrective work, hit a reproducible ~99% CPU hang with zero output. Root
cause diagnosed per `debugging-rule.md`'s four-phase process (reproduce,
locate fault boundary, state root cause, propose fix) — not the earlier
worktree-mismatch issue diagnosed elsewhere this session; this is a
distinct, real defect.

## Reproduction

```
$ timeout 15 ./tooling/rust/target/release/agov dist validate
Terminated
$ echo $?
143
```

Reproduces from repo root, on a clean rebuild (`cargo build --release -p
agov`), with `--base-ref HEAD` (zero diff) via `agov plan
projection-check` too — ruling out diff size or base-ref distance as the
cause. Confirmed via `ps -o pid,pcpu`: sustained ~99.6% CPU, no I/O wait.

## Root cause

One sentence: `dist.rs::glob_matches` (called from `matches_any`, called
from `validate()`'s exclude-filtering step) re-runs a fresh, unbounded
`glob::glob()` scan from the repository root for every `(included-file,
non-"/**"-suffix exclude-pattern)` pair, and `release-manifest.yaml`'s
`exclude`/`forbidden` lists contain exactly such patterns
(`"**/__pycache__/**"`, `"**/*.pyc"`) that fall into this slow path.

Trace: `pattern_prefix_match` (dist.rs:404) only short-circuits patterns
using the `/**`-suffix form (e.g. `agf/agents/**`). A leading-`**`
pattern like `**/__pycache__/**` fails that check, falling through to
`glob_matches` (dist.rs:412), which joins the pattern to repo root
(`safe_join`) and calls `glob::glob()` fresh — walking the **entire**
repository tree, including `.git/`, `tooling/rust/target/` (2.4GB on
this checkout), and `.worktrees/` (1.9GB across 3 nested checkouts, each
with their own `target/`). This happens once per included file (~245
files in the current manifest) times the number of such patterns (2:
`**/__pycache__/**`, `**/*.pyc`), i.e. hundreds of full-repository walks
of a multi-gigabyte tree. Not infinite — would terminate given enough
time — but pathological enough to look and behave like a hang under any
practical timeout.

## Impact

`agov dist validate` and `agov dist build` are both named, required
release-checklist items in `release-via-merge-request-rule.md`
("`agov dist validate` and `agov dist build` pass (if applicable)").
This is release-blocking if a release MR's checklist is followed
honestly, not a cosmetic slowdown. `agov plan projection-check` shares
the defect via the same `dist::validate_default()` call.

## Evidence this is new, not a repeat of the earlier worktree-mismatch bug

Earlier this session, a `projection-check` hang was diagnosed as an
invocation error (binary run from a different worktree than the target
file's absolute path, causing `git rev-parse --show-toplevel` to resolve
the wrong `repo_root`) — see prior coordination trail. This reproduction
is from the correct repo root, on the correct branch, with a relative
path, on a freshly rebuilt binary, and reproduces even with
`--base-ref HEAD` (trivial/zero diff) — ruling out that cause. Also
reproduces via the unrelated `agov dist validate` command, which does no
git-diff computation at all, isolating the fault to the shared
`dist::validate()` path, specifically the exclude-glob step.

## Suggested fix (not implemented here — out of scope for the corrective
work this note interrupted; routing to build-engineer/governor for
triage)

Bound `glob_matches`'s scan to the pattern's static (non-glob) prefix
directory instead of the full repository root, or special-case bare
`**/...`-prefixed patterns to skip `forbidden`-listed directories
(`.git/`, `.worktrees/`, `tooling/elixir/`, and similarly large
generated/vendored trees) during the walk. This is a Rust change and,
per this project's own instruction on a smaller, adjacent validator
finding (`12nr`'s A1), should be its own separately scoped, separately
governed fix rather than folded into unrelated corrective work.

## What I did instead

Did not fix this. Continued the B1-B6 role-definition corrective work
without projection-check evidence (not applicable — that MR touches no
Pages/runbook file, so no projection manifest is implicated) and
disclosed the gap explicitly in that MR rather than silently omitting
the check or claiming it passed.
