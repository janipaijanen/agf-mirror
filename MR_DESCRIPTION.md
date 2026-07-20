# plan: Gate 1 plans for coordination CLI features + CI distribution cluster

Branch: `plan/coord-cli-ci-distribution`

## Summary

Five issues (#33, #32, #36, #37, #18) planned across four files, grouped
by real dependency structure (#33 → #32 → #36, #32 → #37) rather than
one plan per issue.

## What's in this branch

- **`coord-channel-concurrency-semantics.md`** (#33, sequenced first):
  formalizes per-writer-only ordering; states the readable-ID collision
  probability numerically (32⁴ suffix space, birthday-bound estimate).
  Found a real implementation gap: `agov coord validate`'s duplicate-ID
  check only compares files passed in one CLI invocation — CI's
  `git diff | xargs agov coord validate` never checks new files against
  the *existing* channel. Plan adds a directory-aware check to close
  this.
- **`coord-cli-write-and-watch.md`** (#32 + #36, combined): `agov coord
  post` (writes what `coord brief` currently only prints); `agov handoff
  take/complete` (implemented via the existing `ack`/`completion`
  message kinds the handoff-resolution algorithm already recognizes —
  no schema change); `agov coord list --since`. `agov coord watch`
  (blocking) deferred as follow-up, per the issue's own "optional"
  framing.
- **`agov-mcp-server.md`** (#37): new `agov-mcp` crate calling the same
  library functions the CLI dispatcher calls (not shelling out).
  Tool-to-CLI parity table as the load-bearing removability invariant.
- **`agov-oci-image.md`** (#18): reuses the existing `rust:tooling:release`
  musl binary via `COPY` rather than rebuilding from source; gated to
  tag pipelines; negative-test requirement from `gate-self-test-rule.md`
  called out explicitly.

## Cross-cluster assumption flagged

Every mutating command requires an explicit `--from <role>` flag
(matching the existing `coord brief` convention) — no automatic
identity/session detection. If the identity/accountability cluster
(#35/#45/#34/#47) adds a session convention, these commands' argument
shapes should be revisited in lockstep. Stated in both affected plan
files for the human reconciler.

## Review checklist

- [x] All four plan files pass `agov plan validate`
- [ ] Duplicate-ID directory-aware check design reviewed against the
      actual CI diff-based invocation pattern
- [ ] `--from` flag convention checked against AB cluster's actual
      identity mechanism once merged
- [ ] No `Co-authored-by` trailers in commit
