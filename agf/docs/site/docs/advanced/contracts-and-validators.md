# Contracts and validators

*Status: Released (v2.1 documentation). Audience: administrators and
developers integrating the framework into CI. After this page you know
which artifacts are schema-bound, which guarantees the contracts make,
and how to wire validation into your pipeline.*

## What is under contract

The framework uses validator commands: command-line checks that read the
governance files and fail when the files no longer match their contracts. The
adopter-facing command is `agov`. Table 1 summarizes the main contracted
artifacts, the shipped checks, and the checks still treated as contract targets
rather than finished tooling.

Contracts define what tooling may check; they do not make tooling the
governor. A validator failure becomes a blocking gate only when a named AGF
rule, contract, CI policy, or recorded governor decision says that check is
blocking. Otherwise the result is advisory evidence for the responsible role.
This distinction lets teams add useful automation without accidentally giving a
script authority the framework never granted.

| Artifact | Contract | Checked by |
|---|---|---|
| Coordination messages (`docs/coordination/*.md` by default) | JSON schema: front matter shape, message kind, required handoff fields | `agov coord validate` |
| Gate 1 plans (`docs/plans/*.md` by default) | JSON schema: plan front matter and required advisory consultation fields | `agov plan validate` |
| Hidden coordination obligations | Branch visibility scan against a base branch | `agov coord visibility` |
| Agent definitions (`agf/agents/*.yaml`) | JSON schema: required fields, role shape, version pin | `agov validate agents` |
| Learning records (`agf/learnings/*.yaml`) | JSON schema: structure, severity grades, evidence fields | `agov validate learnings` |
| InfoSec-relevant artifacts | Anonymization and disclosure checks | `agov validate infosec` |

The distinction matters: a schema can be binding before every validator is
implemented, but the documentation must not present future checks as shipped
capability. If your project builds one of the missing validators, that is a
contribution upstream wants.

## Advisory and blocking checks

Every validator integration should answer two questions:

| Question | Required answer |
|---|---|
| What artifact is being checked? | Path pattern and schema or rule reference |
| Is failure advisory or blocking? | `advisory` unless a named rule or decision grants blocking authority |

Good CI job names make the authority visible:

- `advisory:agf-coordination-review` for a non-blocking report.
- `blocking:agf-coordination-schema` when the coordination schema is a required gate.

Do not turn every useful warning into a blocker. Advisory checks are valuable:
they let evaluators, reviewers, and local models surface friction without
stalling work. Reserve blocking checks for violations that the project has
explicitly decided must stop a merge or release.

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
  image: debian:stable-slim
  script:
    - tar -xzf agov-<version>-x86_64-unknown-linux-musl.tar.gz
    - install -m 0755 agov-<version>-x86_64-unknown-linux-musl/agov /usr/local/bin/agov
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/coordination/*.md' |
        xargs -0r agov coord validate
    - git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- 'docs/plans/*.md' |
        xargs -0r agov plan validate
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

Download the `agov` release archive for your platform from the
[GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).
See [Tooling and token savings](../adopt/tooling-and-tokens.md) for the full
setup guide including local development options.

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
