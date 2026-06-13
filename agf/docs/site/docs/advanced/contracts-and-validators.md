# Contracts and validators

*Status: Provisional (v2.0 documentation). Audience: administrators and
developers integrating the framework into CI. After this page you know
which artifacts are schema-bound, which guarantees the contracts make,
and how to wire validation into your pipeline.*

## What is under contract

The framework uses validator commands: command-line checks that read the
governance files and fail when the files no longer match their contracts.
For adopters, the public command surface is `agf ...`; the current reference
implementation delegates to Elixir Mix tasks under `tooling/elixir/`. The
[tooling conformance](../../reference/tooling-conformance.md) reference
defines how future Python, Rust, Ruby, C, C++, Julia, or other implementations
can be considered AGF-compatible. Table 1 summarizes the main contracted
artifacts, the shipped checks, and the checks still treated as contract targets
rather than finished tooling.

| Artifact | Contract | Checked by |
|---|---|---|
| Coordination messages (`docs/coordination/*.md` by default) | JSON schema: front matter shape, message kind, required handoff fields | `agf coord validate` |
| Gate 1 plans (`docs/plans/*.md` by default) | JSON schema: plan front matter and required advisory consultation fields | `agf plan validate` |
| Hidden coordination obligations | Branch visibility scan against a base branch | `agf coord visibility` |
| Agent definitions (`agf/agents/*.yaml`) | JSON schema: required fields, role shape, version pin | Contract target; validator task not yet shipped |
| Learning records (`agf/learnings/*.yaml`) | JSON schema: structure, severity grades, evidence fields | Contract target; validator task not yet shipped |
| InfoSec-relevant artifacts | Anonymization and disclosure checks | Contract target; validator task not yet shipped |
| Release tags | VERSION ↔ tag alignment | Contract target; validator task not yet shipped |

The distinction matters: a schema can be binding before every validator is
implemented, but the documentation must not present future checks as shipped
capability. If your project builds one of the missing validators, that is a
contribution upstream wants.

## The stability promise

The binding contract documents (`docs/contracts/` in the v2.0 layout)
define which interfaces upstream treats as stable: schema fields,
validator exit-code semantics, the upstream-owned vs adopter-owned file
boundary. Changes to contracted surfaces are classified (BREAKING /
ADDITIVE / EDITORIAL / PREVENTIVE) under the downstream-impact
discipline, and BREAKING changes ship with migration notes — an
unclassified breaking change is a defect you should report.

## Wiring CI

Run shipped validators per merge request; they complete in seconds. Fresh
channels can validate every governed file; repositories with historical
pre-contract records should validate the changed governed files in the merge
request:

```yaml
validate-governance:
  image: hexpm/elixir:<tag-matching-your-pinned-elixir-and-erlang>
  variables:
    # Set only if this disposable runner image executes jobs as UID 0.
    AGOV_ALLOW_ROOT: "1"
  before_script:
    - cd tooling/elixir && mix deps.get && mix compile --warnings-as-errors && cd ../..
  script:
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/coordination/[0-9][0-9][0-9][0-9][0-9]-*.md' | xargs -0r ./bin/agf coord validate
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/plans/*.md' | xargs -0r ./bin/agf plan validate
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

Adapt the runner image and invocation to your stack. The `agf` wrapper is the
downstream-facing command; Mix remains the reference implementation detail.
When using Elixir 1.20 or newer, compile warnings include the compiler's
built-in type inference and gradual type-checking findings, so
`mix compile --warnings-as-errors` is part of the gate rather than a cosmetic
lint step. The current reference tooling is Mix-based and pinned in foundry's
`mise.toml`; a release archive containing only `agf/` has schemas and templates,
but not the current Mix task implementation. If your runner intentionally
executes as UID 0, use the documented root override from the
[environment variable reference](../../reference/environment-variables.md).
Two disciplines from the rule library apply directly here:

- **Gate self-test:** the MR that introduces a validator job includes a
  negative test — a deliberately malformed file, shown to fail the
  pipeline. A validator never proven to fire protects nothing.
- **Exit codes are the interface:** wire jobs to fail on non-zero, and
  do not wrap validators in `|| true` "temporarily". A silenced gate is
  the documented path back to the failure mode it guards.

## Local parity

Whatever CI runs, developers should be able to run identically before
pushing — pre-push hooks or a make target invoking the same commands.
Divergence between local checks and CI checks trains people to wait for
the pipeline instead of validating early, and gates discovered only in
CI get bypassed under deadline pressure.

## Next

- What you may safely change: [customizing the framework](customizing-the-framework.md)
