---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 271
timestamp: 2026-06-14T03:45:00+03:00
immutable: true
references-to:
  - 00270-coder-tester.md
topic: agf-dist-manifest-gate2
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
    - agf/dist/release-manifest.yaml
    - agf/docs/site/reference/release-artifacts.md
    - tooling/elixir/lib/agov/dist_manifest.ex
    - tooling/elixir/test/agov/dist_manifest_test.exs
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

Gate 2 handoff for the AGF release artifact manifest POC.

Branch: `feat/agf-dist-manifest`

Summary:

- Adds `agf/dist/release-manifest.yaml` as the source of truth for AGF source
  release archive contents.
- Adds `agf/dist/README.md` with the release-job usage contract.
- Adds `mix agov.dist.validate` and `mix agov.dist.build`.
- Builds both `dist/agf-2.0.0.tar.gz` and `dist/agf-2.0.0.zip` from the same
  manifest.
- Keeps generated root `dist/` archives ignored while explicitly tracking
  `agf/dist/**` manifest policy files.
- Adds the public reference page `agf/docs/site/reference/release-artifacts.md`
  and links it from the MkDocs Reference navigation.
- Updates Contracts and validators to list release artifact contents as a
  shipped validator surface.

Archive policy:

- Allowlist-first include entries select the adopter-facing `agf/` framework,
  docs, schemas, rules, skills, examples, and the release manifest itself.
- Excludes remove generated site/release-info files under otherwise intentional
  source directories.
- Forbidden patterns block local/bootstrap/generated paths such as `.worktrees/`,
  `tmp/`, root `dist/`, `public/`, `agents-govern/`, Elixir build/deps, and
  `.env` files.

Validation:

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

Results: OK.

- Dist manifest selected 150 files.
- Built `dist/agf-2.0.0.tar.gz` and `dist/agf-2.0.0.zip`.
- Tar and zip inspections found no forbidden-root matches.
- Full Elixir suite passed with 138 tests.
- MkDocs strict build passed.
- Generated root `dist/` artifacts remain ignored; `agf/dist/**` is tracked.
