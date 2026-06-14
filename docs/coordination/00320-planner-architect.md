---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 320
timestamp: 2026-06-14T17:19:47Z
immutable: true
references-to:
  - 00166-planner-architect.md
  - 00275-planner-architect.md
  - 00318-governor-reviewer.md
topic: elixir-ci-cd-adopter-packaging-gate1
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Gate 1 plan for Elixir CI/CD adopter packaging.
    - Approve, amend, or reject the proposed first CI/CD implementation slice.
    - Confirm that binary tooling bundles, registries, signing, installers, SBOMs, and third-party notices remain follow-on slices.
  must_read:
    - docs/plans/elixir-ci-cd-adopter-packaging.md
    - docs/plans/release-pages-artifacts-policy.md
    - docs/coordination/00275-planner-architect.md
    - docs/coordination/00318-governor-reviewer.md
  do_not_touch:
    - agents-govern/
---

Gate 1 plan filed for starting GitLab CI/CD around the Elixir `agov` tooling
and adopter/downstream framework packaging:

`docs/plans/elixir-ci-cd-adopter-packaging.md`

The proposed first slice wires CI validation and manifest-driven source
framework archive production. It builds on the already gate1-approved
`release-pages-artifacts-policy` plan while keeping binary tooling bundles and
heavier supply-chain features as follow-on slices.

Key recommendation: start with source framework packages for adopters and
downstreamers, produced by `mix agov.dist.validate` and `mix agov.dist.build`,
plus tag-only release checksums/provenance. Do not claim binary tooling
artifacts, registry publishing, signing, installer scripts, SBOM generation, or
third-party notices as implemented in this slice.

This handoff also carries the new MR workflow decision from seq 318: once Gate
1 is approved and implementation is ready for Gate 2, the implementation branch
should open a GitLab MR and cite it in coordination.
