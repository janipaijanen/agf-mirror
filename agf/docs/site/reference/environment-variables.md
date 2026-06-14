# Environment variables

This page is the canonical reference for AGF-controlled environment variables.
If a new `AGOV_*` or `AGF_*` variable is added to tooling, document it here in
the same change.

AGF tooling should normally run as the project user, not as root. Root-owned
build products, telemetry logs, or generated files can make later developer
commands fail in surprising ways.

| Variable | Default | Used by | Purpose |
|---|---|---|---|
| `AGOV_ALLOW_ROOT` | unset | all `mix agov.*` tasks | Safety override. By default, AGF Mix tasks refuse to run as UID 0. Set `AGOV_ALLOW_ROOT=1` only for a disposable CI/container invocation where root is intentional. Accepted true values are `1`, `true`, `yes`, and `on`. The repo-local `agov` bootstrap executable does not currently implement this root guard. |
| `AGOV_MANIFEST_PATH` | Foundry/bootstrap default: `agents-govern/agents/MANIFEST.yaml` for `agov`; `../../agents-govern/agents/MANIFEST.yaml` for Mix from `tooling/elixir/` | `agov plan validate`, `mix agov.plan.validate` | Overrides the agent manifest used to decide which advisory roles are valid. Release-archive adopters should set this explicitly to their local role-status authority, for example `agf/agents/MANIFEST.yaml` if they maintain a local manifest there. Foundry still points at the bootstrap manifest until AGF v2 has its own role authority. |
| `AGOV_SCHEMA_DIR` | `../../agf/schemas` | `mix agov.coord.validate`, `mix agov.plan.validate` | Overrides the directory containing AGF JSON schemas. The default is relative to `tooling/elixir/`. |
| `AGOV_TOKEN_ROI` | enabled | automatic token ROI telemetry | Set to `0`, `false`, or `off` to disable automatic token ROI log writes. |
| `AGOV_TOKEN_ROI_LOG` | `../../tmp/agov-token-roi.jsonl` | `mix agov.token.roi`, automatic token ROI telemetry | Overrides the token ROI JSONL log path. The parent directory is created automatically when logging succeeds. |

Example: choose a downstream-local token ROI log path:

```bash
AGOV_TOKEN_ROI_LOG=path/to/their/chosen-log.jsonl \
  mix agov.session.resume --base-branch master
```

Example: run inside a disposable container that intentionally uses UID 0:

```bash
AGOV_ALLOW_ROOT=1 mix agov.coord.validate ../../docs/coordination/*.md
```

Do not commit local telemetry by accident. The foundry default writes under
`tmp/`, which is ignored locally; downstream projects may choose another ignored
path or deliberately track telemetry if their policy allows it.
