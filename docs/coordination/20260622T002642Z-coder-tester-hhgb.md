---
kind: handoff
from: coder-tester
to: governor-reviewer
model: gpt-5-codex
id: 20260622T002642Z-coder-tester-hhgb
timestamp: 2026-06-22T00:26:42Z
immutable: true
references-to:
  - 20260622T001623Z-coder-tester-4tss.md
topic: release-2.1.2-gate2
requires-ack: false
artifacts_owned:
  - agf/VERSION
  - agf/CHANGELOG.md
  - docs/coordination/20260622T002642Z-coder-tester-hhgb.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review release MR for v2.1.2."
    - "If approved and merged, tag the resulting main merge commit as v2.1.2 and push the tag immediately per release-via-merge-request-rule."
  must_read:
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/VERSION
    - agf/CHANGELOG.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: release v2.1.2

Release branch `release/2.1.2` is ready for governor-reviewer review.

Reason for `2.1.2` instead of `2.1.1`:

- MR !28 merged the `2.1.1` version/changelog bump, but no `v2.1.1` tag or
  GitLab release exists.
- `main` has since advanced through additional merged work, including the
  principal-bound AI delegation docs.
- Creating `v2.1.1` now would attach that old version number to newer content
  that was not described by the `2.1.1` changelog entry.
- `2.1.2` is therefore the next clean immutable release.

Release changes:

- `agf/VERSION`: `2.1.1` -> `2.1.2`
- `agf/CHANGELOG.md`: add `2.1.2` entry dated `2026-06-22`

Validation performed from `tooling/elixir/`:

```bash
mix deps.get
mix agov.dist.check_tag
mix agov.dist.validate
mix agov.dist.build
mix test
```

Results:

- `mix deps.get`: completed; dependency resolution unchanged.
- `mix agov.dist.check_tag`: OK, local build guard skipped because
  `CI_COMMIT_TAG` is not set.
- `mix agov.dist.validate`: OK, release manifest valid for package `agf 2.1.2`.
- `mix agov.dist.build`: OK, produced:
  - `dist/agf-2.1.2.tar.gz`
  - `dist/agf-2.1.2.zip`
  - `dist/agf-2.1.2-provenance.json`
- `mix test`: 217 passed (1 doctest, 216 tests).

Known local noise:

- Linked-worktree commands print the existing `mise.toml` trust warning.
- Dependency compilation emits upstream warnings from `yamerl`/`ulid`.
- Existing test warnings remain:
  - unused `@tag` before `describe` in `coord_config_test.exs`
  - unused default optional arg in `claim_tracker_test.exs`

Post-merge obligation:

Per `release-via-merge-request-rule.md`, if this release MR is reviewed,
approved, and merged, the merged main commit authorizes immediate tag creation
and push:

```bash
git tag v2.1.2
git push origin v2.1.2
```

That tag pipeline should create the GitLab release entry and package links.
