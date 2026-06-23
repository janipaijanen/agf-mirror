# Environment variables

This page is the canonical reference for AGF-controlled environment variables.
If a new `AGOV_*` or `AGF_*` variable is added to tooling, document it here in
the same change.

AGF tooling should normally run as the project user, not as root. Root-owned
build products, telemetry logs, or generated files can make later developer
commands fail in surprising ways.

| Variable | Default | Used by | Purpose |
|---|---|---|---|
| `AGOV_ALLOW_ROOT` | unset | `agov` and development tasks that implement the root guard | Safety override. Set `AGOV_ALLOW_ROOT=1` only for a disposable CI/container invocation where root is intentional. Accepted true values are `1`, `true`, `yes`, and `on`. |
| `AGOV_LEARNINGS_DIR` | `agf/learnings` under the detected repo root | `agov validate learnings`, `agov validate infosec`, `agov validate contributions`, and the umbrella `agov validate` command | Overrides the learning-record directory used by validators. Use this when validating a bootstrap or adopter-owned learning corpus outside the default `agf/learnings` path. |
| `AGOV_MANIFEST_PATH` | Tool-dependent | `agov plan validate` and related plan validators | Overrides the agent manifest used to decide which advisory roles are valid. Release-archive adopters should set this explicitly to their local role-status authority, for example `agf/agents/MANIFEST.yaml` if they maintain a local manifest there. |
| `AGOV_SCHEMA_DIR` | Tool-dependent | `agov coord validate`, `agov plan validate`, and related validators | Overrides the directory containing AGF JSON schemas. Release-archive adopters should set this explicitly when running the binary outside a repository layout that contains `agf/schemas`. |
| `AGOV_TOKEN_ROI` | enabled when telemetry is supported | automatic token ROI telemetry | Set to `0`, `false`, or `off` to disable automatic token ROI log writes. |
| `AGOV_TOKEN_ROI_LOG` | Tool-dependent | token ROI telemetry and `agov token roi report` | Overrides the token ROI JSONL log path. The parent directory is created automatically when logging succeeds. Choose an ignored project-local path unless your policy deliberately tracks telemetry. |

Example: point a release-binary invocation at an explicit schema directory:

```bash
AGOV_SCHEMA_DIR=agf/schemas \
  agov coord validate docs/coordination/*.md
```

Example: run inside a disposable container that intentionally uses UID 0:

```bash
AGOV_ALLOW_ROOT=1 agov coord validate docs/coordination/*.md
```

Do not commit local telemetry by accident. The foundry project writes under
`tmp/`, which is ignored locally; downstream projects may choose another ignored
path or deliberately track telemetry if their policy allows it.
