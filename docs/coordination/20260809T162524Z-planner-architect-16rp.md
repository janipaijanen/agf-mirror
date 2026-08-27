---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
    - .gitlab-ci.yml, release scripts, branch protection, versions, tags, packages, OCI, or GitLab Release state
    - current release rule or its projections before Gate 1
    - .claude/settings.local.json
  must_read:
    - docs/coordination/20260809T155404Z-planner-architect-d58p.md
    - docs/coordination/20260809T161825Z-governor-reviewer-2mmn.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - .gitlab-ci.yml
  next_agent: build-engineer
  status: ready
  task:
    - Assess whether current MR and tag pipelines correctly support a protected maintenance target rooted at an older main commit.
    - Freeze any required branch-protection, pipeline, artifact, cache, source-commit, and post-publication evidence in the plan without changing CI unless a proven gap requires it.
id: 20260809T162524Z-planner-architect-16rp
immutable: true
kind: handoff
model: gpt-5
references-to:
  - 20260809T155404Z-planner-architect-d58p.md
  - 20260809T161825Z-governor-reviewer-2mmn.md
requires-ack: true
timestamp: 2026-08-09T16:25:24Z
to: build-engineer
topic: maintenance-line-release-rule-build-consultation
---

# Build consultation: maintenance-line release exception

The Governor approved planning a narrow protected maintenance-line exception
for exact-RC2 `v2.5.0` promotion. The tag would point to an MR merge commit on
`release/2.5`, not current `main`; a later metadata-only MR would forward-sync
release truth to current main.

Please verify from current CI and GitLab behavior whether MR pipelines can
validate a source branch targeting `release/2.5`, whether tag-only jobs build
and publish solely from the tag commit regardless of default-branch ancestry,
and which protected-branch/API settings must be frozen. Identify any required
CI or release-script change; prefer no CI change if the existing tag pipeline
already binds archives, provenance, packages and OCI to the immutable tag
commit. Also define the exact checks that prove the metadata-forward merge
preserves every nonmetadata path from its current-main base.

No implementation or release mutation is authorized by this request.
