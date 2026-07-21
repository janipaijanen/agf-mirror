---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260721T080952Z-planner-architect-sdsk
timestamp: 2026-07-21T08:09:52Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260721T080233Z-governor-reviewer-qnbt
  - 20260721T075136Z-governor-reviewer-dpm8
  - 20260721T075344Z-planner-architect-tphp
topic: v2-3-0-release-implementation
requires-ack: true
artifacts_owned:
  - agf/VERSION
  - agf/CHANGELOG.md
  - agf/templates/AGENTS.md
  - tooling/rust/Cargo.toml
  - tooling/rust/Cargo.lock
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - implement the approved v2.3.0 release-only changes as Claude / Coder-Tester
    - publish complete validation, packaged-artifact, provenance, and OCI evidence for independent Gate 2 review
  must_read:
    - docs/plans/v230-release-preparation.md
    - docs/coordination/20260721T080233Z-governor-reviewer-qnbt.md
    - docs/coordination/20260721T075136Z-governor-reviewer-dpm8.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
    - refs/tags/v2.3.0
---

# Handoff: Claude implements v2.3.0; Codex independently reviews

Claude / Coder-Tester: implement the approved v2.3.0 release plan on branch
`release-v2.3.0`, created from the exact frozen and green `main` commit:

```text
39aa4d263c43ab3823597208f1190ace7cc500bc
```

Do not include later `main` merges or currently open coordination/future-work
MRs. Keep the branch release-only as defined by
`docs/plans/v230-release-preparation.md`.

## Responsibility split

- **Claude / Coder-Tester:** release metadata implementation, changelog/public
  truth, version consistency, full local and GitLab validation, preview archive
  inspection, packaged-binary provenance checks, OCI build/smoke/scan evidence,
  and the Gate 2 handoff.
- **Codex:** independent review of the final diff and evidence, including stale
  pipeline detection, archive/provenance consistency, OCI evidence, release
  state, and exact tag target.
- **Human governor:** binding Gate 2 merge decision.

## Non-negotiable boundaries

- Apply the exact v2.2.6 prepared/merged/unpublished wording from `dpm8`.
- Do not describe 2.2.6 as a prerelease or completed release.
- Do not implement !293, !294, !295, !297, !299, !300, or the deferred
  philosophical-lenses additions.
- Do not create or push `v2.3.0` from the release branch. Tagging is permitted
  only after the release MR is approved and merged, on its exact `main` merge
  commit, following the approved plan.
- A green pipeline on an older head is not evidence for the final release head.

Please acknowledge this handoff before implementation and publish the release
MR URL when ready for independent review.
