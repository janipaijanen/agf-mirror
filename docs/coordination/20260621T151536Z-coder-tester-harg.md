---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T151536Z-coder-tester-harg
timestamp: 2026-06-21T15:15:36Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260621T151036Z-coder-tester-rnt1.md
topic: learning-record-id-policy-ci-addendum
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Include the CI optional-needs fix in Gate 2 review for MR !63.
  must_read:
    - .gitlab-ci.yml
    - docs/coordination/20260621T151036Z-coder-tester-rnt1.md
  do_not_touch: []
---

# Gate 2 addendum: optional preview bundle needs

MR !63 initially failed to create a pipeline:

```text
'agf:bundle:preview' job needs 'agov:tooling:preview' job, but
'agov:tooling:preview' does not exist in the pipeline.
```

Root cause: `agf:bundle:preview` can run when `agf/**/*` changes, while
`agov:tooling:preview` only runs when `tooling/elixir/**/*` or `.gitlab-ci.yml`
changes. GitLab rejects a hard `needs:` edge to a job omitted by `rules`.

Fix applied on branch `chore/learning-id-policy`:

- marked `agf:bundle:preview` need on `agf:dist:preview` as `optional: true`;
- marked `agf:bundle:preview` need on `agov:tooling:preview` as `optional: true`.

Validation:

```bash
glab api projects/83321848/ci/lint -X POST -F "content=$(cat .gitlab-ci.yml)"
git diff --check
```

GitLab CI lint returned `valid: true` with no errors or warnings.
