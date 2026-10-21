<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue 109 Phase B2: Windows Cross under rootless Podman

**Result: FAIL. Stop Windows job migration under the approved plan.** The
Windows cross build reached linking through rootless Podman, then
`agov-mcp` failed with an unresolved `GetHostNameW` symbol. This is not
proof that Podman itself is unsuitable: the Rust 1.99.1 standard library
and the `cross` 0.2.5 MinGW image may have a linker/sysroot mismatch. The
plan directs a failed spike to stop and return any fallback choice to the
governor rather than change the two optional Windows CI jobs.

## Source and host

- Repository source: `6dd899c134922ddbe36a78a456e1611d1880956d`,
  approved plan MR !3100. The Rust sources match the main-branch base.
- Host: Ubuntu 24.04.5 LTS, kernel `6.8.0-146-generic`, `x86_64`.
- Rootless Podman `4.9.3`, overlay storage, `crun` runtime. Podman and
  `fuse-overlayfs` were installed from Ubuntu's package repository for
  this bounded local experiment; no GitLab runner, protected variable,
  project credential, account, or paid resource was used.
- Cross `0.2.5`, installed with `cargo install cross --locked --version
  '^0.2'`. The local `stable` toolchain updated during this command's
  startup to `rustc 1.99.1` / `cargo 1.99.1`; the prior observed Rust
  version was `1.98.1`. This version change limits comparison with the
  earlier Linux-only Phase B run, which used Rust `1.98.0`.
- Windows image:
  `ghcr.io/cross-rs/x86_64-pc-windows-gnu:0.2.5`, image ID
  `5fbb89c8cb4b9e2904e52ebc085fda5825d538bc643dc464c4d67631a0324374`,
  reported digest
  `sha256:21de770ed1804a5f4114d783ef6bace2e253c27c394f0f8b7784fff8790d3ca0`,
  size 2,897,018,180 bytes. The digest is the local platform image
  digest reported by Podman.

## Exact command and result

From the `tooling/rust` directory of the source checkout:

```sh
CROSS_CONTAINER_ENGINE=podman \
CROSS_REMOTE=0 \
CARGO_TARGET_DIR=/tmp/agf-windows-target \
cross build --release --target x86_64-pc-windows-gnu -p agov -p agov-mcp
```

The command exited `101`. The complete 8,138-line combined stdout/stderr
is retained in the adjacent
`issue109-phase-b2-windows-feasibility.log.gz` (gzip with no embedded
original-name/time header): compressed SHA-256
`d57221bcb5e9ee8cbda9931ae607da8e425a33aa8ffabc0e5957e185355ef902`;
uncompressed SHA-256
`151237fa90c520478becd02ae708f3ca0c3ddda30bb01c97f1b076fbc02f5948`.
Inspect with `gzip -dc docs/reviews/issue109-phase-b2-windows-feasibility.log.gz`.

The terminal failure in that log is:

```text
/rustc/58cf8f9c35b355199727627d07ae34a4e2846d67/library\\std\\src\\sys\\net\\hostname/windows.rs:15: undefined reference to `GetHostNameW'
collect2: error: ld returned 1 exit status
error: could not compile `agov-mcp` (bin "agov-mcp") due to 1 previous error
```

The linker also emitted many `.drectve` warnings; the complete warnings
are in the compressed log. No accepted `agov.exe` or `agov-mcp.exe`
artifact was present under `/tmp/agf-windows-target` after failure, so
there is no artifact checksum to report. No archive/package staging was
attempted.

## Resource observations and limits

The pulled image reported about 2.90 GB. The isolated Cargo target
directory occupied about 311 MB after failure. Free space on `/` was
about 7.5 GB after the run; these are retained-size and point-in-time
observations, not measured peak disk, RAM, or runtime. No deliberate
parallel experiment ran on this host, although the host is shared.
The image pull and compilation were not independently timed. No claim
about production runner capacity follows from this local run.

This experiment ran Cross on a rootless-Podman host. It did not test
nested Podman execution inside a GitLab job image. Because the local
build failed before producing artifacts, the approved plan's PASS branch
cannot be followed. The two Windows CI jobs remain on DinD pending a
separate governor decision on retrying with a pinned older Rust toolchain,
using a scoped privileged runner, using a native Windows runner, or
dropping optional Windows packaging.
