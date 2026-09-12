---
kind: review
from: build-engineer
to: independent-implementation-reviewer
id: 20260912T104104Z-build-engineer-pkgg
timestamp: 2026-09-12T10:41:04Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T103533Z-planner-architect-jerh
requires-ack: false
artifacts_owned: []
topic: cpb2-filesystem-feasibility-advisory
---

# CP-B-2 filesystem-protocol feasibility/footprint — advisory report

Read-only advisory, no dependency added, no plan changed, no
implementation started.

## Known facts (verified this session, not estimates)

- `agov-coord`'s actual shipped dependencies today (`Cargo.toml`):
  `agov-core`, `serde`, `serde_json`, `serde_norway`, `time`. Zero
  filesystem-abstraction or platform-capability crates. `tempfile` is a
  **dev-dependency only** (tests), not shipped in the binary.
- `rustix` and `libc` already appear in `Cargo.lock`, but only
  transitively — via `tempfile` (dev-only) and `xattr` (an unrelated
  packaging/archive concern, not filesystem-read logic). Neither is a
  direct or shipped dependency of any `agov` crate today.
- No `same-file`, `walkdir`, or equivalent stable-file-identity crate is
  anywhere in the current dependency tree.
- Git access in `agov-coord` (`publication_check.rs`) shells out to the
  `git` binary directly (`Command::new("git")`) — no `git2`/`gix`
  library dependency exists. The existing coordination-record reads
  (introducing-commit blobs) already carry **zero** filesystem-race
  concerns by construction: Git blobs are immutable and content-addressed,
  so there is nothing to double-check for concurrent mutation.

## Estimates (general Rust-ecosystem knowledge, not verified/tested this session)

- Portable stable-file-identity (inode+device on Unix, file index via
  `GetFileInformationByHandle` on Windows) is a solved, low-risk problem
  — the `same-file` crate (MIT/Unlicense, small, no `unsafe` beyond what
  std itself needs, widely used by `walkdir`/ripgrep-adjacent tooling)
  is the standard choice, not a heavy addition.
- True no-follow-open is asymmetric across platforms: Unix has a direct
  `O_NOFOLLOW` custom-flag path; Windows symlink/reparse-point handling
  is less standardized and would need its own explicit test coverage on
  the Windows CI runners this project already has
  (`rust:tooling:preview:windows-package` and friends) — a genuine CI
  footprint increase specific to Windows, not just an extra crate.
- A `cap-std`/`cap-tempfile`-family sandboxed-filesystem-capability
  dependency would be substantially heavier (larger `unsafe` surface,
  bigger audit burden) than this single read-only status command needs.
  Not recommended if the narrower `same-file`-plus-manual-flags approach
  covers the required cases.

## Git-input comparison

The plan's own design already treats Git reads (merge-base + target-tip
+ candidate-addition blobs) as the **primary** input; the filesystem
`--pending-dir` is an explicit, bounded, opt-in secondary source layered
on top — not a parallel architecture. So "compare against a Git-only
first implementation" isn't really choosing between two designs; it's
recognizing that the plan's primary path already needs **zero** new
dependencies or platform-specific hardening (reuses the existing
shell-out-to-`git` pattern, no filesystem TOCTOU concerns at all since
Git objects are immutable).

**What a Git-only-first implementation would lose:** exactly the one
behavior the plan itself names — visibility into a hold published on a
branch not yet merged to `main`, via the local pending-directory
extension. Everything else in the command's contract (target
resolution, hold/urgency classification, exit codes, JSON/text/MCP
parity) is independent of the filesystem question.

## Advisory recommendation

Not a blocker for this plan's design. A concrete sequencing option
worth the plan author's/reviewer's consideration: implement and ship
the Git-based primary path first (no new dependency, no new CI
footprint), and treat `--pending-dir` as a small follow-up increment
that adds `same-file` plus the platform-conditional no-follow-open
handling, with its own Windows test coverage, rather than gating the
whole command's first implementation on solving cross-platform
filesystem hardening upfront. This is a sequencing suggestion, not a
scope change to what the plan already proposes for the finished
command.

No concrete blocker identified. This report doesn't itself clear the
plan for Gate 1 — that remains the trial reviewer's and governor's
decision.
