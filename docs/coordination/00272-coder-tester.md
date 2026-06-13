---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 272
timestamp: 2026-06-14T03:52:00+03:00
immutable: true
references-to:
  - docs/coordination/00270-coder-tester.md
topic: agf-dist-manifest-gate2-bridge
requires-ack: true
artifacts_owned:
  - .worktrees/agf-dist-manifest
model: codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch `feat/agf-dist-manifest`.
    - Decide Gate 2 for the AGF release artifact manifest and builder.
    - Confirm that GitLab CI should publish archives produced by `mix agov.dist.build` rather than maintain separate archive include/exclude rules.
  must_read:
    - .worktrees/agf-dist-manifest/agf/dist/release-manifest.yaml
    - .worktrees/agf-dist-manifest/docs/coordination/00271-coder-tester.md
    - .worktrees/agf-dist-manifest/agf/docs/site/reference/release-artifacts.md
    - .worktrees/agf-dist-manifest/tooling/elixir/lib/agov/dist_manifest.ex
    - .worktrees/agf-dist-manifest/tooling/elixir/test/agov/dist_manifest_test.exs
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

Visibility bridge for the AGF release artifact manifest Gate 2 handoff.

Branch: `feat/agf-dist-manifest`

Branch commit: `dd045f1 feat(dist): add AGF release manifest tooling`

Branch-local record:

- `00271-coder-tester.md` is the detailed Gate 2 handoff.

Summary:

- Adds `agf/dist/release-manifest.yaml` as the allowlist-first source of truth
  for AGF source release archive contents.
- Adds `mix agov.dist.validate` and `mix agov.dist.build`.
- Builds both `dist/agf-2.0.0.tar.gz` and `dist/agf-2.0.0.zip` from the same
  manifest.
- Keeps generated root `dist/` ignored while explicitly tracking `agf/dist/**`.
- Adds public release artifact docs with a GitLab CI shape where CI publishes
  builder outputs instead of deciding archive contents.

Validation on the branch:

```bash
cd tooling/elixir
mise exec -- mix agov.dist.validate
mise exec -- mix agov.dist.build
rm -f /tmp/agov_token_roi_*.jsonl && rm -rf /tmp/agov_session_resume_* /tmp/agov_dist_manifest_* && mise exec -- mix test
mise exec -- mix compile --warnings-as-errors
cd ../..
mise exec -- uvx --from 'mkdocs>=1.6,<2' mkdocs build -f agf/docs/build/mkdocs.yml --strict
git diff --check
```

Results: OK. Dist manifest selected 150 files; tar and zip inspections found no
forbidden-root matches. Full suite passed with 138 tests. MkDocs strict build
passed.
