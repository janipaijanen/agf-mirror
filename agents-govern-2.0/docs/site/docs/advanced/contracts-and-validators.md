# Contracts and validators

*Status: Provisional (v2.0 documentation). Audience: administrators and
developers integrating the framework into CI. After this page you know
which artifacts are schema-bound, which guarantees the contracts make,
and how to wire validation into your pipeline.*

## What is under contract

| Artifact | Contract | Checked by |
|---|---|---|
| Agent definitions (`agents/*.yaml`) | JSON schema: required fields, role shape, version pin | `agov-validate-agents` |
| Learning records (`learnings/*.yaml`) | JSON schema: structure, severity grades, evidence fields | `agov-validate-contributions` |
| InfoSec-relevant artifacts | Anonymization and disclosure checks | `agov-validate-infosec` |
| Release tags | VERSION ↔ tag alignment | `agov-pre-tag-check` |

Coordination channel files are governed by convention and review, not by
an automated validator — a known gap, stated here rather than papered
over. If your project builds one, that is a contribution upstream wants.

## The stability promise

The binding contract documents (`docs/contracts/` in the v2.0 layout)
define which interfaces upstream treats as stable: schema fields,
validator exit-code semantics, the upstream-owned vs adopter-owned file
boundary. Changes to contracted surfaces are classified (BREAKING /
ADDITIVE / EDITORIAL / PREVENTIVE) under the downstream-impact
discipline, and BREAKING changes ship with migration notes — an
unclassified breaking change is a defect you should report.

## Wiring CI

Run validators per merge request; they complete in seconds, so no path
scoping is needed:

```yaml
validate-governance:
  script:
    - uv run agov-validate-agents
    - uv run agov-validate-contributions
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

Adapt the runner image and invocation to your stack; the validators are
plain Python entry points. Two disciplines from the rule library apply
directly here:

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
