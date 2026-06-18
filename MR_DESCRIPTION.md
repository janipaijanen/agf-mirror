# Summary

Fix `release:create` CI job shell syntax error on Alpine sh.

# Scope

- `.gitlab-ci.yml` `release:create` job: remove `2>/dev/null` from the
  `for f in ...` glob list.

# Why

The `release-cli:latest` image runs Alpine sh (ash/dash), not bash.
`2>/dev/null` after a glob list in a `for`-`in` statement is valid bash
but a syntax error in POSIX sh. The `[ -f "$f" ] || continue` guard
inside the loop already skips unmatched globs, so the redirect was
redundant.

# Verification

- `mix test` unchanged (CI-only change).
- Syntax verified: `sh -n .gitlab-ci.yml` passes locally.

# Out of Scope

No behavior changes beyond fixing the sh syntax error.
