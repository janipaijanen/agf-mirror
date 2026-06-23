# Tooling conformance

*Status: Draft AGF v2 contract.*

AGF defines contracts, not a sacred implementation language. For adopters, the
current distributed tooling surface is the platform-specific Rust `agov` binary.
Source-checkout contributors may also run development Mix tasks, but those tasks
are not part of the release archive contract.

Future Python, Ruby, C, C++, Julia, or other implementations can be
AGF-compatible if they conform to the same command behavior, schemas, exit
codes, and conformance fixtures.

## Compatibility terms

| Term | Meaning |
|---|---|
| AGF release implementation | The platform-specific Rust `agov` binary distributed for adopters. |
| AGF-compatible implementation | Independently implemented, but passes the AGF conformance suite for the claimed version. |
| Development implementation | Source-checkout tooling used by framework contributors; useful as implementation evidence, but not itself a release artifact. |
| Experimental implementation | Useful for evaluation, but not yet passing the conformance suite. |
| Not verified for AGF v2 | Inherited, partial, or third-party tooling with no current AGF v2 conformance result. |

## Current and Target CLI Surface

The current adopter command is `agov`. A future public interface may become
`agf`, but released v2.0 archives document `agov`.

Command status uses the [maturity vocabulary](maturity-vocabulary.md): Level 4
Validated means implemented and tested in a branch or checkout; Level 5
Deployable means the supported operator path or release artifact includes the
command and the documentation matches that path. Do not describe a command as
deployable merely because a wrapper advertises it.

The table lists the adopter-facing command surface. Source-checkout Mix tasks
may exist for development and parity checks, but they are not required for a
release-archive adopter.

| Adopter command | Purpose | Status |
|---|---|---|
| `agov coord validate <file>...` | Validate coordination messages against the schema. | Release surface |
| `agov plan validate <file>...` | Validate Gate 1 plan front matter and advisory consultation metadata. | Release surface |
| `agov validate [subcommand]` | Validate framework artifacts such as adoption, agents, learnings, infosec, contributions, or framework config. | Release surface |
| `agov coord brief [args...]` | Generate a coordination message ID and front matter. | Release surface |
| `agov handoff next <role> [args...]` | Show the next unresolved handoff for one role. | Release surface |
| `agov coord status [args...]` | Summarize coordination channel activity, roles, pending handoffs, and claims. | Release surface |
| `agov session resume [args...]` | Print cold-start or role-specific session context. | Release surface |
| `agov coord stale [args...]` | Report expired artifact claims. | Release surface |
| `agov coord visibility [args...]` | Detect coordination obligations hidden on non-base branches. | Release surface |
| `agov coord graph [args...]` | Render role-to-role coordination flows as Graphviz dot or text. | Release surface |
| `agov token roi report [args...]` | Summarize local deterministic-tool token savings telemetry. | Release surface |
| `agov usage report [args...]` | Count active roles, senders, message kinds, and unknown senders. | Release surface |

Developer/source-checkout users should use the same `agov` command surface as
release-archive adopters. Public docs must not require a release-archive
adopter to have Elixir installed.

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
