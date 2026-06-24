<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Tooling Conformance

Status: draft AGF v2 contract. This page defines the language-neutral command
surface that AGF tooling implementations should provide. The Rust `agov` binary
is the reference implementation; alternative implementations may be written in
any language if they conform to the same behavior.

## Principle

AGF defines contracts, not a sacred implementation language.

- `agf/schemas/` are the machine-readable artifact contracts.
- `agf/docs/contracts/` defines behavior, exit codes, and compatibility terms.
- `tooling/rust/` is the reference implementation (`agov` binary).
- Alternative implementations are conforming implementations only if they pass
  the same conformance fixtures and command behavior.

## Compatibility Terms

| Term | Meaning |
|---|---|
| AGF reference implementation | Maintained in this repository and used as the contract oracle for AGF v2. |
| AGF-compatible implementation | Independently implemented, but passes the AGF conformance suite for the claimed version. |
| Experimental implementation | Useful for evaluation, but not yet passing the conformance suite. |
| Not verified for AGF v2 | Inherited, partial, or third-party tooling with no current AGF v2 conformance result. |

Do not call a tool "AGF-compatible" unless its version, conformance result, and
claimed AGF contract version are visible to users.

## Public CLI Surface

The adopter interface is the `agov` command.

| Command |
|---|
| `agov coord validate <file>...` |
| `agov coord status [args...]` |
| `agov coord stale [args...]` |
| `agov coord visibility [args...]` |
| `agov coord brief [args...]` |
| `agov plan validate <file>...` |
| `agov handoff next <role> [args...]` |
| `agov session resume [args...]` |
| `agov usage report [args...]` |

Future commands such as `agf learning validate`, `agf workitem validate`, and
`agf provenance validate` should be added to this table before they are treated
as adopter-facing contract.

## Exit Codes

Conforming implementations must use these meanings:

| Code | Meaning |
|---|---|
| `0` | Success, no findings requiring failure. |
| `1` | Invocation or validation failure. The command ran, but input or arguments were invalid. |
| `2` | No work found / clean negative result where that is the command contract, such as no pending handoff. |
| `3` | Reserved for cross-artifact invariant failure. |
| `4` | Reserved for infrastructure or input/output failure. |

This table is the target AGF CLI contract. The Rust `agov` implementation is
the authoritative reference for exit code behavior.

## Output Modes

Human-readable text is the default. Machine-readable JSON should be added
command by command only when there is a documented consumer.

Rules:

- Default text output must be stable enough for humans, not for parsers.
- JSON output, once documented for a command, is a compatibility surface.
- Adding a field to JSON output is additive.
- Removing or renaming a field is breaking.
- Error output must include the command name or artifact path when available.

## Conformance Suite

A future `agf/conformance/` directory should contain:

```text
agf/conformance/
  fixtures/
    coord/
      valid/
      invalid/
    plans/
      valid/
      invalid/
  expected/
    coord-validate.json
    plan-validate.json
  README.md
```

The conformance suite should test:

- accepted and rejected artifact fixtures;
- exit codes;
- JSON output shape where JSON is specified;
- path handling from repository root and from a tooling directory;
- deterministic ordering where a command lists records;
- lifecycle behavior such as transitive handoff resolution.

## Implementation Registry

AGF may later publish an implementation registry:

| Implementation | Status | Language | Command | Notes |
|---|---|---|---|---|
| Rust reference | Reference | Rust | `agov` | Maintained in this repository (`tooling/rust/`). |
| Python | Not verified for AGF v2 | Python | TBD | v1.x heritage; must pass AGF v2 conformance before compatibility claims. |

The registry is descriptive, not exclusive. Downstream projects may build their
own tooling. Upstream acceptance is required only for "reference
implementation" status.

## Licensing

Framework content and contracts are licensed as documented in `agf/LICENSE`:
CC BY-SA 4.0, with separate commercial licensing available for alternative
terms. Official tooling implementations in this repository are MIT licensed.
Third-party implementations may choose their own license, but their license does
not change the AGF content license and does not imply upstream endorsement.
