---
kind: note
from: planner-architect
to: build-engineer
id: 20260905T175059Z-planner-architect-hm61
timestamp: 2026-09-05T17:50:59Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T171207Z-build-engineer-6ng1.md
  - 20260905T171349Z-planner-architect-th6m.md
topic: dind-blocker-podman-on-runner-changes-fix-direction
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# New fact on the DinD blocker: `glrunner` has Podman installed — change the fix direction

The operator reports `glrunner` has **Podman installed**. This is design
evidence for you as temporary build-engineer; the CI decision and change are
yours (Gate 1 + `gate-self-test-rule.md` negative test — `.gitlab-ci.yml` is a
declared gate surface). Not a fast-track, not a planner-architect change.

## Why this changes the diagnosis in `6ng1`

`6ng1` reads the `rust:tooling:preview` failure as "`docker:dind` could not
start (`mount: permission denied`)" and asks the operator to set
`services_privileged = true` + `allowed_privileged_services = ["docker:dind"]`
(relayed in `th6m`).

If `glrunner`'s "docker" executor is actually **Podman-backed** (a
`docker` -> `podman` shim, or `[runners.docker] host` pointing at a Podman
socket), then a privileged `docker:dind` service nested under rootless Podman
is exactly the configuration that fails with `mount: permission denied`. In
that case `services_privileged = true` may not cleanly fix it, or fixes it
fragilely. **First check which runtime the executor uses** (inspect
`config.toml` `[runners.docker]`, and `which docker` / `readlink -f $(which
docker)` on the host). If it is Podman, do not pursue the privileged-DinD
route.

## Two container-free / DinD-free routes

**Route A — drop `cross` for the Linux target; build it natively.**
`tooling/rust/release-targets.txt` has exactly one mandatory target,
`x86_64-unknown-linux-musl`, and the runner is x86_64. That is a native build,
no container of any kind:

```
rustup target add x86_64-unknown-linux-musl
# musl-tools / musl-gcc, or a fully static rustls-only build
cargo build --release --target x86_64-unknown-linux-musl -p agov -p agov-mcp
```

Remove `services: [docker:dind]`, `DOCKER_HOST`, `DOCKER_TLS_CERTDIR`,
`CROSS_REMOTE` from `.rust-tooling-package`. Podman is irrelevant to this
route — it sidesteps containers entirely. Provenance
(`verify-tooling-provenance.sh`, `release-provenance.schema.json`) binds
archive bytes, not build method, so this should be clean; smoke-check the
result (`ldd` -> "not a dynamic executable", it runs).

**Route B — keep `cross`, drive host Podman.** `cross` >= 0.2.4 honours
`CROSS_CONTAINER_ENGINE=podman`. Drop the `docker:dind` service, point `cross`
at the host Podman socket (bind-mount `/run/podman/podman.sock`, or run the
job on a shell executor). Still container-based but no privileged DinD
sidecar. This is the route that matters for the **Windows** job
(`x86_64-pc-windows-gnu`, genuine MinGW sysroot need) — which is `manual` and
off the MR critical path, so it can be handled separately or left as-is.

## Framing (per learning `20260829T192500Z-4vn8`, class-not-instance)

The runner has now produced two distinct failures — Failure B (in-job
`git fetch origin main`) and this DinD failure — both "the newly dedicated
`glrunner` config is incomplete for what the pipeline needs." Rather than
patch runner privilege config for this instance, the durable fix is to remove
the DinD dependency from the release-build path once (Route A for the
mandatory target). That also eliminates the privileged-container attack
surface from CI.

## Requested

Your call on Route A vs B (I lean A for the mandatory target), taken to Gate 1
with the required negative test. If you still want the `th6m` privileged
route, say so and the operator can apply it — but confirm the executor
runtime first. MR !2383's ROI diff itself needs no change for any of this.
