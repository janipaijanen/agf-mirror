# docs+fix: restore self-governance epigraph + CI cargo-install cache guards

Branch: `docs/self-governance-epigraph`

## Summary

**1 — Self-governance epigraph (docs)**
Restores the one-line epigraph under the h1 in three locations:

> *The framework that cannot govern itself has no business governing anything else.*

Added to `README.md`, `agf/docs/site/index.md`, and `agf/docs/site/evaluate.md`.
Two lines each (blank line + italic line), no other content changed.

**2 — CI `cargo install` cache guards**
Two CI jobs were reinstalling `cargo-deny` and `cross` from crates.io on every
pipeline run, even when the cached binary was already present. Replaced bare
`cargo install` calls with a check-then-install guard:

```bash
cargo install --list 2>/dev/null | grep -qE "^cargo-deny v0\.19\.[0-9]+:" \
  || cargo install cargo-deny --locked --version "^0.19" --force
```

Same pattern for `cross`. This avoids a network round-trip and a full
recompile when the cached version is already correct.

## Files changed

| File | Change |
|---|---|
| `README.md` | +2 lines: epigraph under h1 |
| `agf/docs/site/index.md` | +2 lines: epigraph under h1 |
| `agf/docs/site/evaluate.md` | +2 lines: epigraph under h1 |
| `.gitlab-ci.yml` | `rust:tooling:deny` + `rust:tooling:parity` + `rust:tooling:build:windows:preview` — check-then-install guard for `cargo-deny` and `cross` |

## Review checklist

- [ ] Epigraph text is identical across all three files
- [ ] CI cache guards use `grep -qE` (quiet exit code, no output on hit)
- [ ] Version pattern matches the pinned version constraint in the same job
- [ ] No `Co-authored-by` trailers in commits
