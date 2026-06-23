# Maturity Vocabulary

AGF keeps the 1.x maturity vocabulary because it prevents overclaiming.
Use these labels whenever documentation, review notes, coordination records,
or release evidence describe what a feature, command, domain, or governance
claim can actually do.

## Capability Maturity

Use this five-level ladder for individual features, commands, domains, and
tooling capabilities.

| Level | Name | Generic meaning | CLI/tooling example |
|---|---|---|---|
| 5 | Deployable | Full operational support | Command implemented, tested, documented, and included in a release or supported operator distribution. |
| 4 | Validated | Verified but not yet deployable | Command implemented and tested in a branch or checkout, but not yet released or merged into the supported operator path. |
| 3 | Instantiated | Partially used, incompletely verified | Command partially implemented or exercised manually without complete tests. |
| 2 | Surface-defined | Interface exists only | Flag or subcommand is declared, documented, or advertised but not wired to working behavior. |
| 1 | Planned | Identified only | Named in a plan, roadmap, issue, or design note. |

Do not use binary yes/no capability tables for non-trivial claims. A reader
needs to know whether the capability is merely planned, surfaced, implemented,
validated, or actually deployable.

Example:

```text
agov token roi report is Level 4 Validated in an implementation branch when it
is implemented and tested there. It is Level 5 Deployable only after the
supported operator path or release artifact includes the command and the docs
match that path.
```

This distinction matters when a wrapper or README advertises a command before
the selected binary contains it. In that state, the command is not Deployable
for that operator path, even if it is Validated somewhere else.

## Evidence Status

Use these labels for framework-level claims, principles, and recommendations.

| Status | Meaning |
|---|---|
| Empirically validated | Exercised by multiple independent projects with consistent results. |
| Empirically informed | Derived from one project, one sustained self-application thread, or adjacent-domain evidence. Plausible, but not independently corroborated. |
| Theoretical | Derived from principles or design reasoning without direct project evidence. |

Evidence status describes the strength of a framework claim. It is separate
from capability maturity. A command can be Level 5 Deployable while the broader
claim about its organizational benefit is only empirically informed.

## Compliance Profile

Use compliance profiles for an adopter's chosen verifiability target.

| Profile | What is enforced |
|---|---|
| Full | Process-aware plus binding identifiers, trailers, or sidecars that connect artifacts to execution traces. |
| Process-aware | Artefact-only plus gate execution evidence and recorded decisions. |
| Artefact-only | Schema and cross-field validation on governance artifacts. |

Compliance profile answers "which verifiability layers does this project
enforce?" It is separate from adoption intensity, capability maturity, and
evidence status.

## AI Contributor Rule

When an AI agent makes a maturity-sensitive claim, include the label. Prefer:

```text
Level 4 Validated in this branch; not yet Level 5 Deployable in the supported
operator path.
```

over:

```text
Supported.
```

Short labels keep future agents honest during cold start, review, and release
work.
