# Release artifacts

AGF source release archives are built from
`agf/dist/release-manifest.yaml`. The manifest is the release policy; GitLab CI
or any other runner should call the AGF tooling instead of maintaining a
separate artifact include/exclude list.

From `tooling/elixir/`:

```bash
mix agov.dist.validate
mix agov.dist.build
```

`mix agov.dist.validate` expands the manifest and fails if selected files escape
the repository, match forbidden paths, or would collide in the archive.
`mix agov.dist.build` writes the configured `.tar.gz` and `.zip` archives under
`dist/`.

The current archive policy is allowlist-first:

- include the adopter-facing `agf/` framework, docs, schemas, rules, skills,
  and release manifest;
- exclude generated site output and generated release-info pages;
- forbid local or bootstrap paths such as `.worktrees/`, `tmp/`, `dist/`,
  `agents-govern/`, Elixir build directories, and `.env` files.

Downstream projects may fork the manifest or pass `--manifest` to use their own
release policy. If they intentionally track generated release artifacts, that is
a downstream policy choice; AGF's default keeps generated archives out of git
through the root `dist/` ignore rule.

## GitLab CI shape

Use the manifest-driven commands as the source of truth:

```yaml
agf-release-artifacts:
  image: hexpm/elixir:1.20.1-erlang-28.0.2-debian-bookworm-20250630-slim
  before_script:
    - cd tooling/elixir
    - mix deps.get
  script:
    - mix agov.dist.validate
    - mix agov.dist.build
  artifacts:
    paths:
      - dist/*.tar.gz
      - dist/*.zip
```

The `artifacts.paths` block publishes files produced by the builder; it does
not decide what goes into those files.
