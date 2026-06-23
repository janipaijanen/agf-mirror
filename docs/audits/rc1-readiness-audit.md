# RC-1 Readiness Audit

Status: Step 4 audit evidence for RC-1.  
Date: 2026-06-23T23:10:00Z.  
Scope source: `docs/coordination/20260623T225930Z-governor-reviewer-p4rc.md`.

## Operator Path

Verified from a source checkout using the Rust release build and repo-local
launcher:

```bash
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov
export PATH="$PWD/tooling:$PATH"
agov --version
agov version --format json
agov coord status --dir docs/coordination
agov coord graph-index
```

Expected `agov --version` shape:

```text
agov {version} ({sha} {dirty|clean})
```

The command must report the build commit SHA and dirty state. `agov version
--format json` must emit `version`, `commit`, and `dirty` fields.

Observed during this audit:

```text
HEAD=f412d23101b5
agov 0.1.0 (f412d23101b5 dirty)
{"version":"0.1.0","commit":"f412d23101b5","dirty":true}
OK  wrote graph index to tmp/rc1-graph-index.json
```

## Documentation Consistency Findings

Resolved in this audit MR:

- `AGENTS.md` no longer describes `tooling/agov` as falling back from Rust.
- `AGENTS.md` examples now use `main` as the base branch for session resume and
  coordination visibility.
- README and public site command tables now include version provenance,
  `coord graph`, and `coord graph-index`.
- Tooling conformance now lists `agov --version` and
  `agov version --format json` as release-surface commands.

Known non-blocking warning carried from Gate 2 approval `qjpm`:

- `agov --help` can still expose clap's bare package version in generated help
  contexts; the critical operator path `agov --version` reports SHA and dirty
  state correctly.

## Inherited v1.x Public Material

High-risk public-facing inherited paths reviewed for this staged audit:

| Path | Disposition | RC-1 action |
|---|---|---|
| `agf/ADOPTING.md` | `adapted-from-v1x` | Added an explicit top-of-file warning that Mix, Elixir validator, or not-yet-shipped command passages are historical/provisional unless confirmed by current Rust `agov` docs/tests. |
| `agf/framework.md` | `adapted-from-v1x` | Added an explicit top-of-file warning that the monolithic reference carries inherited v1.x material and current AGF v2 operator truth lives in public docs, schemas, rules, and Rust command contracts. |
| `agf/docs/site/` primary adopter/evaluator path | current AGF v2 docs | Scanned for legacy Mix/Elixir fallback claims; no public-site path was found requiring Elixir fallback for normal adopter work. |

Deferred beyond RC-1:

- Full historical audit of every rule, role, schema, and long-form reference.
- Inline disposition labels on every inherited paragraph in `agf/framework.md`
  and `agf/ADOPTING.md`.

## Adopter-Facing Claim Check

The audited docs now consistently state:

- Rust `agov` is the supported operator path.
- Elixir/Mix is not required for release-archive adopters.
- Tool output is evidence unless a named rule or governor decision makes it
  blocking.
- Capability claims use the Level 1-5 maturity vocabulary and avoid describing
  planned or branch-only work as deployable.

Residual risk:

- Some older, shipped long-form reference content remains inherited and broad.
  It is labelled for RC-1, but a complete item-by-item disposition remains a
  follow-up quality gate.
