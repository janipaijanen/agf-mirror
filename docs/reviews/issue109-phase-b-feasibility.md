<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #109 Phase B rootless-Podman feasibility record

## Result

**BLOCKED in the selected environment before compilation.** The required
`x86_64-unknown-linux-musl` cross image could not be unpacked into the
rootless Podman graph store because the host filesystem had approximately
1.1 GB available. The pull failed after 64.38 seconds with `no space left on
device`. No target binary or archive was produced.

This result establishes a concrete local storage precondition; it does not
establish that rootless Podman is incompatible with the cross build. The
existing Option D evidence already shows that the same engine selectors can
succeed when adequate capacity and runner setup are available.

The `aarch64-unknown-linux-musl` target was not attempted after the mandatory
x86_64 prerequisite failed at image acquisition. Repeating another image pull
against the same constrained graph store would spend network and storage
without testing compilation. The optional Windows target was also not
attempted.

No account, paid service, external compute/storage, project credential,
protected variable, runner registration, CI change, hardware purchase,
Proxmox action, or externally retained artifact was used.

## Exact source and inputs

| Input | Recorded value |
|---|---|
| Source revision | `7904a4a01a63924e16dced699d22f6ba7e7d7b12` |
| Source branch | `feat/issue109-phase-b-feasibility` |
| Host OS | Linux Mint 22.3 (`zena`) |
| Kernel / architecture | `7.0.0-30-generic`, `amd64` |
| Podman | `4.9.3`, local service, rootless `true`, cgroup v2 |
| Podman storage | overlay driver; graph root `/home/devi/.local/share/containers/storage` |
| `cross` | `0.2.5` |
| Rust / Cargo | `rustc 1.98.0 (88d9e12ae 2026-08-18)`; `cargo 1.98.0 (797e8a9bc 2026-08-05)` |
| Build metadata | `AGOV_BUILD_COMMIT=7904a4a01a63924e16dced699d22f6ba7e7d7b12`, `AGOV_BUILD_DIRTY=false`, `AGOV_BUILD_SOURCE_DIRTY=false` |
| Engine selectors | `CROSS_CONTAINER_ENGINE=podman`, `CROSS_REMOTE=0` |
| Disposable target directory | `/dev/shm/agf-phaseb-20260826/target` (`CARGO_TARGET_DIR`) |
| Attempted image | `ghcr.io/cross-rs/x86_64-unknown-linux-musl:0.2.5` |
| Image digest | Not resolved locally: the manifest-list pull failed while unpacking a layer, before Podman registered the image |

The tmpfs target filesystem had approximately 4.8 GB available. The limiting
filesystem was Podman's rootless graph store on `/`, which had approximately
1.1 GB available before the pull. The build target directory remained 4 KiB,
confirming the failure happened before Cargo compilation or artifact output.

## Exact command

Run from `tooling/rust`:

```sh
RUSTUP_TOOLCHAIN=stable \
AGOV_BUILD_COMMIT=7904a4a01a63924e16dced699d22f6ba7e7d7b12 \
AGOV_BUILD_DIRTY=false \
AGOV_BUILD_SOURCE_DIRTY=false \
CROSS_CONTAINER_ENGINE=podman \
CROSS_REMOTE=0 \
CARGO_TARGET_DIR=/dev/shm/agf-phaseb-20260826/target \
cross build --release --target x86_64-unknown-linux-musl -p agov -p agov-mcp
```

`/usr/bin/time -v` wrapped the command. Standard output and error from `cross`
were retained locally while drafting this record and are reproduced in full
below.

## Target results

| Target | Result | Artifact verification |
|---|---|---|
| `x86_64-unknown-linux-musl` | **FAIL before compilation**: image layer unpack exhausted the Podman graph-store filesystem | No artifacts existed; checksum verification was not applicable |
| `aarch64-unknown-linux-musl` | **NOT ATTEMPTED** after the shared image-acquisition prerequisite failed | No artifacts existed |
| `x86_64-pc-windows-gnu` | **NOT ATTEMPTED**; optional under the approved Phase B plan | No artifacts existed |

## Resource and time observations

| Observation | Value | Limit |
|---|---:|---|
| x86_64 attempt wall time | 64.38 s | Includes registry transfer and partial unpack; no compilation |
| Maximum process RSS | 106,088 KiB | Measures the wrapping `cross`/Podman client process tree as reported by GNU `time`, not total kernel cache or storage-driver memory |
| Filesystem outputs reported by GNU `time` | 2,202,616 blocks | Indicative only; block accounting is not a direct image-size measurement |
| Root filesystem capacity before pull | 34 GB total, about 1.1 GB available | Human-readable `df`; the exact free value after automatic failed-pull cleanup was 1,048,072,192 bytes |
| tmpfs target capacity | 5,211,521,024 bytes total; 5,147,656,192 bytes available after the attempt | Cargo never started, so target output remained 4 KiB |
| Host memory after attempt | 10,423,046,144 bytes total; 3,497,578,496 bytes available | Not a peak-RAM measurement and not a controlled benchmark |

The failure created two unreferenced overlay layers totaling approximately
78 MB. `podman system prune --external --force` did not remove them because
Podman still tracks them as storage layers. They were not manually deleted:
the graph store is shared local state, and deleting tracked layer directories
outside Podman's supported commands would exceed this bounded experiment.
The single registered image in the store predates this attempt and remained
unchanged. A future retry should use an isolated disposable Podman graph root
with adequate capacity so rollback can remove the entire experiment store.

## Full x86_64 failure log

```text
info: syncing channel updates for stable-x86_64-unknown-linux-gnu

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.98.0 (88d9e12ae 2026-08-18)

info: checking for self-update (current version: 1.29.0)
Trying to pull ghcr.io/cross-rs/x86_64-unknown-linux-musl:0.2.5...
Getting image source signatures
Copying blob sha256:d429032782ed0d5b5cc35d8df48f74c106e548fcdba72fdbf08ad36b88a92b24
Copying blob sha256:c3c627bb838d28026cf329ab8784118875d3b117fec24b867c3abe970dad68f7
Copying blob sha256:b549f31133a955f68f9fa0d93f18436c4a180e12184b999a8ecf14f7eaa83309
Copying blob sha256:d9b4b549710c094eb55d9a05e7693798f99b81870134cd866690544ad3c91bcd
Copying blob sha256:168f006715ce18c516d6dbec4b38459aa02b617e1c99cf332824f8677d9fb349
Copying blob sha256:4105193e76ef5110a528e9f3e707646c8b1df3075d9507841bd2640e4a2c68c6
Copying blob sha256:91363d08c4f93048186dfa3d85a92e80e916255a2c90f8ecc544b2af9d9bc7ef
Copying blob sha256:27b3f26a9944b70005002d01a9a0405ba9535cce9a802dc7e0af5d35e67f483b
Copying blob sha256:8721398404e16b7b5f0028c28f719d6e99f0ba76c766ad83cfcbd88300cbfa5a
Copying blob sha256:ee0806ae7e37404497a72ed8e229bbf9d651b3447825e4fe144751fe1ed846e8
Copying blob sha256:a54d397f471c444073af0812763958f2dfd150aba237109281496dce4f6650d9
Copying blob sha256:954c73add16263f73cad52ded4a2688e617b71b96cf8b0cf070386e866b3e357
Copying blob sha256:948db972fa4d8c1662ea529d5523905751d3d48ab8258e8dddd789ada6e27f6c
Copying blob sha256:7691116cf0caa9988e67926635e0811533b3e7352c6f7b37f2cf27c2d1528a49
Error: copying system image from manifest list: writing blob: adding layer with blob "sha256:168f006715ce18c516d6dbec4b38459aa02b617e1c99cf332824f8677d9fb349": processing tar file(write /usr/lib/llvm-10/lib/libclangTidyModernizeModule.a: no space left on device): exit status 1
```

The full log SHA-256 while drafting this record was
`f0ecb5ac354a4f81259f185a8682d193ccec70aabce16bc658e697238eb70637`.

## Pre-run toolchain incident and recovery

The first environment probe invoked the unpinned `stable` rustup channel. It
began updating the shared stable toolchain from Rust 1.97.1 to 1.98.0 while the
root filesystem had only about 1.1 GB available, exhausted the filesystem, and
left `rustc` unusable. The target build did not start.

Recovery removed only rustup installer staging/download data: entries created
by this interrupted update plus 16-day-old files already under
`~/.rustup/tmp`. Cargo/project caches and project files were not deleted. A
forced stable installation then restored the full previously listed component
set (`cargo`, `rustc`, `clippy`, `rustfmt`, `rust-src`, `rust-docs`) and the
host, x86_64-musl, and aarch64-musl standard-library targets. Verification
after recovery showed operational Rust/Cargo 1.98.0 and all components/targets
present. The removed data was transient and re-downloadable, not user source
or durable evidence.

This incident is evidence for a future retry: pin or preflight the toolchain
before measuring the build, and keep toolchain staging plus container graph
storage out of a nearly-full shared filesystem.

## Comparison with Option D

Option D records `20260809T214119Z-contribution-auditor-6tj3.md` and
`20260810T045208Z-contribution-auditor-es1p.md` show successful x86_64 and
aarch64 musl builds with the same `CROSS_CONTAINER_ENGINE=podman` and
`CROSS_REMOTE=0` selectors on temporary protected project runner `54982037`.
They also record multiple runner-environment corrections before success and,
after cleanup, approximately 5.8 GB local disk headroom. The tag record notes
that an insufficient shared runner had already exhausted disk before the job
was moved.

The present result is consistent with that capacity sensitivity: about 1.1 GB
was insufficient even to unpack the x86_64 image. It is not a controlled
minimum-capacity measurement and must not be turned into a precise sizing
threshold. It does show that a future Phase B retry needs materially more
disposable graph-store headroom than this environment provides.

## Disposition and reproducible next step

The selected environment cannot complete Phase B without one of the actions
the approved plan deliberately withholds: destructive cleanup of shared local
state, provisioning additional storage/compute, or using another environment.
This attempt therefore stops here.

A future authorized retry should:

1. use the same exact source revision or record a new one;
2. preflight and pin the Rust toolchain without updating a shared installation;
3. use an isolated disposable rootless-Podman graph root with materially more
   than 1.1 GB free (without treating this attempt as a precise threshold);
4. rerun x86_64 first, then aarch64, preserving image digests, GNU-time data,
   artifact checksums, and full failures; and
5. delete the entire isolated graph root as rollback.

Whether to accept this blocked result as Phase B's current feasibility finding
or provide/authorize a suitable already-owned environment for a fresh run is a
governor decision. This record itself requests no scope expansion.
