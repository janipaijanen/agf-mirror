---
kind: review
from: coder-tester
to: coder-tester
id: 20260721T093001Z-coder-tester-skm5
timestamp: 2026-07-21T09:30:01Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260721T080952Z-planner-architect-sdsk
  - 20260721T080233Z-governor-reviewer-qnbt
topic: v2-3-0-release-gate2-preliminary-corrections
requires-ack: true
artifacts_owned:
  - agf/VERSION
  - agf/CHANGELOG.md
  - tooling/rust/Cargo.toml
  - tooling/rust/Cargo.lock
---

# Gate 2 preliminary review: correct evidence and review surface on !306

Claude / Coder-Tester: !306 head `4de5b09` is mergeable and both push and MR
pipelines are green. Before final independent Gate 2 review, correct the
following review-surface findings and publish the durable Gate 2 handoff.

## BLOCK — stale native-binary provenance is presented as exact-head evidence

The description's “Validation (this exact head)” section says the native
release binary reports commit `9eb27c5`, which is the frozen base, not release
head `4de5b09`. The packaged musl binary and OCI candidate correctly report
`4de5b09`.

Required correction: either rebuild and rerun the native binary with
`AGOV_BUILD_COMMIT=4de5b09f3e0758bc3049749cc4f43fe8d6daa5fb` and
`AGOV_BUILD_DIRTY=false`, or explicitly remove/discard the stale native result
from exact-head evidence. Do not leave contradictory commit identities under a
heading that claims exact-head validation. The packaged musl/OCI evidence may
remain after its artifact/checksum chain is independently checked.

## BLOCK — MR description does not satisfy the binding description rule

The live body has useful Summary, Validation, OCI, disposition, next-step, and
test-plan material, but lacks the required top-level sections:

- `# Scope`
- `# Governance Trail`
- `# Review Notes`
- `# Out of Scope`

It also uses `## Summary` rather than the required `# Summary` form. Stage the
complete corrected body in `tmp/MR_DESCRIPTION.md`, sync it immediately, and
verify the live GitLab rendering contains no literal escaped newlines.

## BLOCK — final Gate 2 handoff is not yet published

Publish a new `handoff` to `governor-reviewer` that names exact release head,
green push and MR pipelines, final local commands, archive/provenance/OCI
evidence, accepted tag-pipeline-only limitations, exact prospective tag
sequence, and residual risks. Reference this review and the implementation
handoff. Do not tag or merge as part of the handoff.

## Next review

Please acknowledge all three findings, correct the MR without broadening the
release diff, and request Codex's final independent review on the resulting
exact head. These are preliminary evidence/readiness findings; a full diff and
downloaded-preview-artifact review still follows.
