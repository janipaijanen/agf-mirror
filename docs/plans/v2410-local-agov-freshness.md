---
id: v2410-local-agov-freshness
status: pending-gate1
topic: v2410-local-agov-freshness
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Review 20260731T100134Z-contribution-auditor-bbq5 checked operator compatibility, rebuild failure behavior, worktree isolation, release-boundary preservation, and regression coverage; it recommends CLEAR with no findings."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The change selects and verifies a repository-local executable using local source provenance. It adds no credential handling, network trust boundary, learning payload, authorization semantic, or security-control implementation."
---

# Plan: v2.4.10 repository-local agov freshness and command parity

## Decision requested

Authorize a bounded patch for issue #92 so `tooling/agov` forwards the
authoritative Rust command surface and never silently executes a binary built
from different tooling inputs. This plan does not authorize implementation
before Gate 1 approval, or any version bump, tag, release publication, branch
deletion, command-semantic change, MCP expansion, or validator change.

## Verified defects and patch classification

Two coupled defects reproduce on post-v2.4.8 `main`:

1. `tooling/agov coord publication-check --help` exits 127 while the current
   release binary accepts the command. The wrapper's handwritten
   `rust_supported()` registry also omits implemented `dist` subcommands.
2. The wrapper selects any executable release or debug binary without checking
   its embedded provenance against the current tooling sources. An older local
   binary can therefore be mistaken for a missing Rust implementation.

Both failures violate the documented repository-local operator contract. The
fix restores command reachability and source/binary agreement without changing
the commands themselves, so v2.4.10 remains patch-compatible. If implementation
requires new adopter-facing command semantics or artifact behavior, stop and
return to Gate 1 for release reclassification.

## Required design contract

### One command authority

- Remove the wrapper's handwritten command allowlist and duplicated command
  inventory/help text.
- Once a usable Rust executable is selected, forward all arguments unchanged
  and let the Rust parser accept or reject them.
- Preserve concise wrapper-only diagnostics for executable discovery,
  freshness failure, and rebuild failure. Do not restore a legacy fallback.

### Deterministic tooling-source identity

- Embed a deterministic source fingerprint in the Rust binary and expose it as
  an additive field in `agov version --format json`.
- The fingerprint must cover the Rust source, workspace/package manifests,
  lockfile, and build script inputs that can change executable behavior.
- It must be independent of paths, mtimes, file traversal order, documentation-
  only commits, and unrelated repository content.
- The build-time and wrapper/runtime calculations must share one specified
  algorithm or mechanically verified implementation; two drifting file lists
  are not acceptable.
- Binaries predating the fingerprint are stale for the repository-local
  wrapper, even if their semantic version matches.

### Selection and recovery

1. Resolve the checkout that owns the invoked wrapper and its release/debug
   candidates.
2. If the current directory belongs to a different linked worktree of the same
   repository, stop with the exact current-worktree wrapper path. Do not bind
   schemas, manifests, or binaries across worktrees silently.
3. Execute a candidate immediately only when its embedded fingerprint matches
   the current tooling inputs.
4. When no matching candidate exists and Cargo is available, perform one
   release build from the wrapper's checkout, verify the resulting fingerprint,
   and execute it.
5. If Cargo is unavailable, the build fails, or the rebuilt fingerprint still
   differs, fail closed. Never fall back to the stale executable or debug build.
6. Preserve normal released-binary use outside a source checkout; this policy
   governs the repository-local wrapper, not downloaded standalone `agov`.

The implementation must avoid fingerprint-named target directories or other
unbounded per-worktree cache multiplication. Cargo's existing target locking
may serialize concurrent builds; the wrapper must not implement a rebuild loop.

## Implementation sequence

1. Freeze the current wrapper/Rust command-tree mismatch as failing tests,
   including `coord publication-check` and every implemented `dist` command.
2. Add and test deterministic build-input fingerprinting and additive version
   provenance.
3. Replace wrapper allowlisting with transparent argument forwarding.
4. Add freshness verification, one-shot release rebuild, fail-closed handling,
   and linked-worktree mismatch diagnostics.
5. Update repository-local operator guidance and environment-variable
   documentation only where behavior changes.
6. Build clean tar and zip distributions and confirm standalone release paths
   remain self-sufficient and unaffected by repository-wrapper policy.

## Required evidence

- Wrapper and direct-binary command trees agree for all current subcommands.
- A fixture built before a Rust source change is rebuilt or rejected before
  the requested command runs.
- Rust source, manifest, lockfile, and build-script changes invalidate a build.
- Documentation-only and unrelated repository changes do not invalidate it.
- Clean, staged, unstaged, and newly added tooling inputs are handled
  deterministically; no stale binary executes after failed recovery.
- A wrapper invoked from a different linked worktree fails with an actionable
  path and does not create a target tree in the wrong checkout.
- Cargo-unavailable behavior succeeds with a matching binary and fails clearly
  with a stale or legacy binary.
- Version JSON remains backward-compatible for existing consumers.
- Full Rust workspace tests, shell portability/static checks, `agov validate`,
  strict documentation build, archive self-sufficiency, and `git diff --check`
  pass.

## Acceptance criteria

1. `tooling/agov coord publication-check --help` and every current Rust
   subcommand reach the Rust parser without wrapper inventory changes.
2. The wrapper cannot silently execute a binary whose behavior-affecting
   tooling inputs differ from its checkout.
3. Recovery performs at most one release rebuild and otherwise fails closed
   with exact operator guidance.
4. Worktree, schema, manifest, and binary paths remain bound to one checkout.
5. No validator, coordination, release-artifact, MCP, or adopter command
   semantics change.
6. Issue #92 closes only after the published v2.4.10 artifact and documented
   repository path demonstrate the fix; implementation merge is Level 4, not
   Level 5.

## Stop conditions

Stop and return to the governor if the change requires an adopter-facing
command or flag, modifies standalone artifact discovery, weakens provenance,
needs a persistent process or network service, creates unbounded build caches,
touches MCP issue #86, or expands beyond issue #92.
