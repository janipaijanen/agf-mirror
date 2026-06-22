---
kind: handoff
from: coder-tester
to: governor-reviewer
model: codex-gpt-5
id: 20260622T110457Z-coder-tester-qmpj
timestamp: 2026-06-22T11:04:57Z
immutable: true
references-to: []
topic: ci-artifact-retention-storage-reduction
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
  - agf/learnings/foundry.yaml
  - agf/docs/site/reference/release-artifacts.md
  - docs/coordination/20260622T110457Z-coder-tester-qmpj.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the CI artifact retention reduction and docs update.
    - Review proposed learning 20260622T111100Z about GitLab CI artifact hygiene.
    - Confirm whether the targeted old preview/docs job artifact cleanup should proceed after merge.
  must_read:
    - .gitlab-ci.yml
    - agf/learnings/foundry.yaml
    - agf/docs/site/reference/release-artifacts.md
  do_not_touch: []
---

# Gate 2 handoff: CI artifact retention storage reduction

GitLab project statistics showed storage pressure is coming from CI job
artifacts, not repository history or package registry release uploads:

- `job_artifacts_size`: about 3.13 GB
- `packages_size`: about 83.5 MB
- `repository_size`: about 3 MB

Implementation on branch `ci/artifact-retention` shortens artifact retention
for repeatable CI outputs:

- Preview docs/dist/tooling artifacts now expire after 1 day instead of 1 week.
- `pages` artifacts now expire after 1 day.
- Release CI handoff artifacts now expire after 1 week instead of never.
- Release documentation now states that durable release copies are package
  registry uploads and GitLab Release links created by `release:create`, not
  long-lived CI job artifacts.
- Learning `20260622T111100Z` records the CI hygiene lesson: repeatable CI
  artifacts should expire quickly, while package registry uploads and GitLab
  Release links are the durable release surface.

Validation run:

- `git diff --check`
- `./tooling/agov validate`
- GitLab CI lint API for `.gitlab-ci.yml`: valid, no errors or warnings

Recommended next step: open and merge a focused MR, then delete old preview
and docs-build job artifacts from closed/merged pipeline work to reclaim the
existing GitLab storage.
