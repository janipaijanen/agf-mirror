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
| `AGOV_ALLOW_STALE_LOCAL_BINARY` | unset | repository-local `tooling/agov` wrapper | Emergency local-development override for the wrapper freshness gate. `1` permits an existing local binary without provenance compatibility checks, emits a warning, and is rejected when `CI=true`. Never use it for routine governed work, acceptance evidence, packaging, or releases. |
| `AGOV_BUILD_COMMIT` | derived from Git when unset | Rust build scripts and CI packaging | Embeds the full source commit in `agov` and `agov-mcp`. Release CI sets it to the exact pipeline SHA. |
| `AGOV_BUILD_DIRTY` | derived from whole-tree Git status when unset | Rust build scripts and CI packaging | Embeds whole-tree dirty state. Release CI requires and verifies `false`. |
| `AGOV_BUILD_SOURCE_DIRTY` | derived from tracked freshness-sensitive paths when unset | Rust build scripts and CI packaging | Embeds the narrower tracked-source state used by the local wrapper. Release CI sets and verifies `false`; this does not replace whole-tree `dirty`. |
| `AGOV_LEARNINGS_DIR` | `agf/learnings` under the detected repo root | `agov validate learnings`, `agov validate infosec`, `agov validate contributions`, and the umbrella `agov validate` command | Overrides the default learning-record directory used by validators. Use this for a canonical adopter-owned learning corpus outside `agf/learnings`. For one-off upstream-submittable packets, prefer `agov validate learnings <path-to-learning.yaml>` instead of changing the environment. |
| `AGOV_MANIFEST_PATH` | Tool-dependent | `agov plan validate` and related plan validators | Overrides the agent manifest used to decide which advisory roles are currently available. The Gate 1 schema validates advisory-entry shape; semantic validation accepts active manifest roles, identifies known non-active roles by status, and rejects roles absent from the manifest as unknown. Release-archive adopters should set this explicitly to their local role-status authority, for example `agf/agents/MANIFEST.yaml` if they maintain a local manifest there. |
| `AGOV_MODEL` | unset | `agov coord brief` | Optional model/runtime provenance to include in generated coordination front matter. Explicit `agov coord brief --model <name>` overrides this value. This field is audit metadata only; routing and authority still come from `from` and `to` roles. |
| `AGOV_SCHEMA_DIR` | Tool-dependent | `agov coord validate`, `agov plan validate`, and related validators | Overrides the directory containing AGF JSON schemas. Release-archive adopters should set this explicitly when running the binary outside a repository layout that contains `agf/schemas`. |
| `AGOV_TOKEN_ROI` | enabled when telemetry is supported | automatic token ROI telemetry | Set to `0`, `false`, or `off` to disable automatic token ROI log writes. |
| `AGOV_TOKEN_ROI_LOG` | `<detected-project-root>/tmp/agov-token-roi.jsonl` | token ROI telemetry and `agov token roi report` | Overrides the token ROI JSONL log path. Relative values resolve from the invocation directory. A path outside the detected project root is honored with a warning. The parent directory is created automatically when logging succeeds. Choose an ignored project-local path unless your policy deliberately tracks telemetry. |

Example: point a release-binary invocation at an explicit schema directory:

```bash
AGOV_SCHEMA_DIR=agf/schemas \
  agov coord validate docs/coordination/*.md
```

Example: run inside a disposable container that intentionally uses UID 0:

```bash
AGOV_ALLOW_ROOT=1 agov coord validate docs/coordination/*.md
```

Live token ROI telemetry belongs under `tmp/` using a stable per-assistant
filename: `tmp/<ai-abbrev>-agov-token-roi.jsonl`. These files are local runtime
telemetry and are not committed. When telemetry is needed as test evidence or
showcase data, copy a dated snapshot into an explicit fixture path and document
its source, date, and confidence limits.

When multiple assistants work in the same checkout, use stable per-assistant
paths rather than the shared default. In foundry, Codex continues at
`tmp/codex-agov-token-roi.jsonl` and Claude should continue at
`tmp/claude-agov-token-roi.jsonl`:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/codex-agov-token-roi.jsonl"
```

Set that export in each project's shell or directory environment. Do not put a
`$PWD`-expanded value in a shell-global profile: the value remains fixed after
the shell starts and can silently direct later projects into the first
project's log. Without an override, `agov` discovers the current AGF project
root and uses its `tmp/agov-token-roi.jsonl`. Outside an AGF project, automatic
logging is disabled unless an explicit environment path is supplied;
`agov token roi report` similarly requires `--log` or the environment variable.
