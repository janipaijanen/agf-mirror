# Tooling conformance

*Status: Draft AGF v2 contract.*

AGF defines contracts, not a sacred implementation language. The current
foundry operator entrypoint is `agov` for bootstrap validation, and the Elixir
tooling is the reference/development implementation for the wider command set.
Future Python, Rust, Ruby, C, C++, Julia, or other implementations can be
AGF-compatible if they conform to the same command behavior, schemas, exit
codes, and conformance fixtures.

## Compatibility terms

| Term | Meaning |
|---|---|
| AGF reference implementation | Maintained in this repository and used as the contract oracle for AGF v2. |
| AGF-compatible implementation | Independently implemented, but passes the AGF conformance suite for the claimed version. |
| Experimental implementation | Useful for evaluation, but not yet passing the conformance suite. |
| Not verified for AGF v2 | Inherited, partial, or third-party tooling with no current AGF v2 conformance result. |

## Current and Target CLI Surface

The current foundry operator command is `agov`. The long-term adopter-facing
interface may become `agf`; Mix tasks are developer and
reference-implementation internals. The `agov` wrapper routes each command to
the best available implementation: Rust binary if built and supported, compiled
Elixir escript next, Mix as final fallback.

The "Rust" column shows which commands have a Rust implementation that passes
the byte-for-byte parity test against the Elixir reference output.

| Current operator command | Long-term public command | Reference command | Rust |
|---|---|---|---|
| `agov coord validate <file>...` | `agf coord validate <file>...` | `mix agov.coord.validate <file>...` | ✓ Phase 1 |
| `agov plan validate <file>...` | `agf plan validate <file>...` | `mix agov.plan.validate <file>...` | ✓ Phase 1 |
| `agov validate [subcommand]` | `agf validate [subcommand]` | `mix agov.validate [subcommand]` | ✓ Phase 1 |
| `agov coord brief [args...]` | `agf coord brief [args...]` | `mix agov.coord.brief [args...]` | ✓ Phase 1 |
| `agov handoff next <role> [args...]` | `agf handoff next <role> [args...]` | `mix agov.handoff.next <role> [args...]` | ✓ Phase 1 |
| `agov coord status [args...]` | `agf coord status [args...]` | `mix agov.coord.status [args...]` | ✓ Phase 2 |
| `agov session resume [args...]` | `agf session resume [args...]` | `mix agov.session.resume [args...]` | Phase 2 in progress |
| `agov coord stale [args...]` | `agf coord stale [args...]` | `mix agov.coord.stale [args...]` | Phase 2 in progress |
| not wrapped yet | `agf coord visibility [args...]` | `mix agov.coord.visibility [args...]` | Phase 2B |
| not wrapped yet | `agf usage report [args...]` | `mix agov.usage.report [args...]` | Phase 2B or later |

## Exit codes

| Code | Meaning |
|---|---|
| `0` | Success, no findings requiring failure. |
| `1` | Invocation or validation failure. |
| `2` | No work found / clean negative result where that is the command contract. |
| `3` | Reserved for cross-artifact invariant failure. |
| `4` | Reserved for infrastructure or input/output failure. |

## Conformance suite

A future `agf/conformance/` directory should test fixtures, exit codes, JSON
output shape where specified, path handling, deterministic ordering, and
lifecycle behavior such as transitive handoff resolution.

The contract source lives in `agf/docs/contracts/tooling-conformance.md`.
