---
kind: decision-record
from: governor-reviewer
to: all
id: 20260827T124803Z-governor-reviewer-rpam
timestamp: 2026-08-27T12:48:03Z
model: human-directed-codex-gpt-5
immutable: true
references-to:
  - 20260813T044747Z-governor-reviewer-8hgm.md
  - 20260827T123825Z-coder-tester-n6ar.md
  - 20260827T124544Z-coder-tester-mqjk.md
topic: v270-release-topology-and-candidate-policy
requires-ack: false
artifacts_owned: []
---

# v2.7.0 topology and candidate policy approved

This record transcribes the governor-reviewer's in-session answer, “yes to
both,” to the two binding questions in handoff `n6ar` after feasibility record
`mqjk` was presented.

## Binding topology decision

Preserve decision `8hgm`'s frozen v2.7.0 scope: issues #86, #87, #88, and #90
only. Use the frozen-scope reconstruction path with this exact stable base:

- annotated tag: `v2.6.1`
- tag object: `7e7ad93121be16c3e44a7c12ccab1104efd126ff`
- peeled commit: `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`
- peeled tree: `1c6d2eac56f620ff6b17a6d50dd904427329e1c5`

The standard current-main path is excluded because current `main` at the
decision evidence point (`efd811dd`) is 955 commits and 635 changed paths past
v2.6.1 and contains substantial work outside the four frozen issues. The
maintenance-line exception is excluded for candidate construction because no
immutable v2.7.0 release candidate exists yet. These facts make the governed
frozen-scope reconstruction exception the applicable topology.

The separate Gate 1 release-publication plan must name and reproduce the exact
ordered allowlist, frozen-out set, metadata allowlist, protection baselines,
namespace checks, and validation matrix required by
`release-via-merge-request-rule.md`. Feasibility record `mqjk` supports the
two scope merge commits but does not substitute for that plan.

## Binding candidate and hold decision

Publish `v2.7.0-rc.1` first. Complete exact-artifact and post-publication
verification across the Git tag, GitLab Release, generic package, OCI image,
archives, checksums, provenance, downloaded binaries, packaged Claude setup,
and clean-adopter workflow. Begin a **48-hour verified hold** only after those
checks pass.

If a relevant defect requires a changed candidate, govern the correction,
publish immutable `v2.7.0-rc.2`, and restart the 48-hour hold. Do not move,
delete, or replace an existing candidate tag or artifact.

After an accepted candidate completes the hold, prepare and govern the stable
`v2.7.0` promotion under the applicable maintenance-line procedure, prove its
functional equivalence to the accepted candidate, publish it, and repeat all
namespace and downloaded-artifact verification.

## Authority boundary

This decision authorizes planner-architect to prepare the exact release-
publication plan and closes the two topology questions. It does not itself
grant Gate 1 or Gate 2, create or protect branches, change version metadata,
merge a release MR, create a tag, publish an artifact, or deploy anything.
