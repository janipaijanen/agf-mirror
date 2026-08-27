<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #109 Phase B rootless-Podman feasibility record

## Result

**PASS for both required Linux targets.** At the same exact source revision as
the earlier storage-blocked attempt, a fresh rootless-Podman reproduction built
both `agov` and `agov-mcp` for `x86_64-unknown-linux-musl` and
`aarch64-unknown-linux-musl`. Both output pairs were present and SHA-256
checksummed. The engine substitution was exactly
`CROSS_CONTAINER_ENGINE=podman` with `CROSS_REMOTE=0`.

The reproduction used a new, empty, disposable Podman graph and run root on a
5.21 GB tmpfs. The selected host's root filesystem had only 2.25 GB available
at preflight, less than the 5.5 GB reported when the retry was authorized, so
shared caches and the shared Podman store were not changed. The isolated image
store reached 3,988,884,076 bytes for the two images and was deleted in full
after evidence capture. Disposable Cargo output was also deleted. No account,
paid service, external compute or storage, project credential, protected
variable, runner registration, CI change, hardware purchase, Proxmox action,
or externally retained artifact was used.

The first aarch64 command exposed a separate, reproducible cache-boundary
failure: host build scripts produced under the x86_64 cross image required
newer glibc symbols than the aarch64 image provided. Repeating aarch64 with its
own disposable `CARGO_TARGET_DIR` passed. This is a Phase D design input, not a
Phase B blocker: sequential target builds must isolate host-side Cargo state or
perform an equivalent clean. Merely cleaning the target-triple directory may
leave incompatible host build scripts in the shared `target/release` tree.

The optional Windows preview was not attempted. After the mandatory targets,
the root filesystem had 1.56 GB available and the isolated tmpfs had 1.06 GB
available. Spending that remaining capacity on an optional third image was not
necessary to answer Phase B's required Linux feasibility question.

## Exact source and inputs

| Input | Recorded value |
|---|---|
| Source revision | `7904a4a01a63924e16dced699d22f6ba7e7d7b12` (detached worktree; same revision as the blocked attempt) |
| Host OS | Linux Mint 22.3 (`zena`, Ubuntu `noble` base) |
| Kernel / host architecture | `7.0.0-30-generic`, `x86_64` |
| Host capacity | 6 CPUs; 10,423,046,144 bytes RAM; no swap |
| Podman | `4.9.3`, rootless `true`, cgroup v2, `crun` 1.14.1 |
| Podman storage | rootless overlay through `fuse-overlayfs` 1.13-dev; isolated graph and run roots on tmpfs |
| `cross` | `0.2.5` |
| Rust / Cargo | `rustc 1.98.0 (88d9e12ae 2026-08-18)`; `cargo 1.98.0 (797e8a9bc 2026-08-05)` |
| Installed Rust targets | `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl` |
| Build metadata | `AGOV_BUILD_COMMIT=7904a4a01a63924e16dced699d22f6ba7e7d7b12`, `AGOV_BUILD_DIRTY=false`, `AGOV_BUILD_SOURCE_DIRTY=false` |
| Engine selectors | `CROSS_CONTAINER_ENGINE=podman`, `CROSS_REMOTE=0` |
| x86_64 image | `ghcr.io/cross-rs/x86_64-unknown-linux-musl:0.2.5`, image ID `449c67afebe0...`, digest `sha256:77db671d8356a64ae72a3e1415e63f547f26d374fbe3c4762c1cd36c7eac7b99`, size 2,099,610,462 bytes |
| aarch64 image | `ghcr.io/cross-rs/aarch64-unknown-linux-musl:0.2.5`, image ID `1024ee12e0a3...`, digest `sha256:702154f52b2d8091671aa2c84d5582d849f949977228c735ff8462f93cc0e1e4`, size 1,918,294,894 bytes |

The recorded image digests are Podman's platform-specific image digests. The
pull output also exposed manifest-list digests
`sha256:48da142669b71e8a4ef9f01eec80358dd9c8b174732dd672155041fe4a0f9c2d`
for x86_64 and
`sha256:53a761857a806b4f73b209a15bf71eacc38a82d5a02e05b166300c4794d7ad83`
for aarch64.

## Rootless isolation configuration

The actual commands resolved `podman` through a disposable wrapper placed
first on `PATH`. It invoked the system Podman with these global options before
forwarding the `cross` arguments unchanged:

```sh
exec /usr/bin/podman \
  --root /dev/shm/agf-phaseb-retry-20260826T213125Z/graphroot \
  --runroot /dev/shm/agf-phaseb-retry-20260826T213125Z/runroot \
  --storage-driver overlay \
  --storage-opt overlay.mount_program=/usr/bin/fuse-overlayfs \
  "$@"
```

`podman info` verified the exact graph root and run root above, an empty image
store before the pull, tmpfs backing, rootless execution, cgroup v2, and the
FUSE overlay mount program. A preliminary configuration probe also established
that rootless Podman ignores a plain `graphroot` entry unless
`rootless_storage_path` is set; the verified wrapper avoids that ambiguity.

Compressed-transfer scratch and Cargo outputs were placed on `/` in the
detached worktree, so they did not compete with the image layers for the same
5.21 GB tmpfs. The two target builds used separate disposable Cargo directories
after the cache-boundary finding.

## Exact command sequence

The image for each target was pulled explicitly first so image acquisition and
compilation have separate timing and failure evidence:

```sh
PATH="$PHASEB/bin:$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH" \
TMPDIR="$RUNTIME/tmp" \
/usr/bin/time -v podman pull \
  ghcr.io/cross-rs/x86_64-unknown-linux-musl:0.2.5

PATH="$PHASEB/bin:$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH" \
TMPDIR="$RUNTIME/tmp" \
RUSTUP_TOOLCHAIN=stable \
AGOV_BUILD_COMMIT=7904a4a01a63924e16dced699d22f6ba7e7d7b12 \
AGOV_BUILD_DIRTY=false \
AGOV_BUILD_SOURCE_DIRTY=false \
CROSS_CONTAINER_ENGINE=podman \
CROSS_REMOTE=0 \
CARGO_TARGET_DIR="$RUNTIME/target" \
/usr/bin/time -v cross build --release \
  --target x86_64-unknown-linux-musl -p agov -p agov-mcp

PATH="$PHASEB/bin:$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH" \
TMPDIR="$RUNTIME/tmp" \
/usr/bin/time -v podman pull \
  ghcr.io/cross-rs/aarch64-unknown-linux-musl:0.2.5
```

The first aarch64 compile used the same environment as x86_64 and the same
`$RUNTIME/target` directory. It failed with the complete log below. The
corrected command changed only this one value:

```sh
CARGO_TARGET_DIR="$RUNTIME/target-aarch64"
```

It then ran:

```sh
/usr/bin/time -v cross build --release \
  --target aarch64-unknown-linux-musl -p agov -p agov-mcp
```

The `cross` prelude reported the stable toolchain unchanged at 1.98.0 on each
build; no toolchain install or update occurred.

## Target results and artifact verification

| Target | Result | Artifact SHA-256 |
|---|---|---|
| `x86_64-unknown-linux-musl` | **PASS** | `agov` (8,747,376 bytes): `36e5d0d4e84bbb16c9a2d00f71178ee9b9016af10a1b3492f564580fd10406ad`; `agov-mcp` (910,136 bytes): `149937a59e8d757cfbb28e376472d3cc2c8e6aa366ff9facc2d70d551739e985` |
| `aarch64-unknown-linux-musl` | **PASS after target-directory isolation** | `agov` (8,040,288 bytes): `18f99b08ade361659c33acbae0a5dd22569f00bd3a367e0ee80903d562fc6aa1`; `agov-mcp` (901,200 bytes): `752a2a511e71e9b42ced6be4541b6b3ee3bf85bed9e257feba5f6e415eeb28f7` |
| `x86_64-pc-windows-gnu` | **NOT ATTEMPTED**; optional under the approved plan | Not applicable |

The output binaries were verified by direct existence, byte-size, and
SHA-256 checks before rollback. They were not executed on the x86_64 host for
the aarch64 target, and no archive/package staging was in Phase B scope.

## Resource and time observations

| Operation | Wall time | GNU `time` maximum RSS | Retained storage observation |
|---|---:|---:|---:|
| x86_64 image pull | 1m33.61s | 106,292 KiB | Image 2,099,610,462 bytes |
| x86_64 build | 7m36.68s | 63,776 KiB | Cargo directory 340,787,506 bytes |
| aarch64 image pull | 1m35.67s | 106,804 KiB | Image 1,918,294,894 bytes |
| aarch64 shared-target failure | 2.27s | 54,840 KiB | No accepted artifact |
| aarch64 isolated-target build | 2m44.43s | 54,964 KiB | Cargo directory 333,734,387 bytes |

Preflight free capacity was 2,254,204,928 bytes on `/` and 5,144,858,624
bytes on `/dev/shm`. After both accepted builds it was 1,560,432,640 bytes and
1,060,544,512 bytes respectively. `podman unshare du -sb` measured the final
isolated graph store at 3,988,884,076 bytes. The two retained Cargo directories
totalled 674,521,893 bytes.

These are indicative sizing observations, not controlled benchmarks. GNU
`time`'s RSS is the maximum of the wrapped `cross` or Podman client process,
not a whole-host or container-cgroup peak; host page cache and the container's
full memory are excluded. End-of-command image and Cargo sizes are retained
disk high-water proxies and lower bounds on true peak disk use; transient
downloads, temporary linker files, and container upper-layer writes may have
made the instantaneous peak higher. Host `free` snapshots are not presented as
peak-RAM measurements. No parallel heavy job was intentionally started, but
the host is shared, so unrelated activity can affect wall time and free-space
snapshots.

Evidence-log SHA-256 values before local rollback were:

- preflight: `903c3bd8659048de80f6eded29763b8803ef34893b345c0b6ab0e8b64b580711`
- x86_64 image pull: `8d52ffabf19cd13e25b449960f63922ccdf67c213fd90b7fe0175f5622aa2901`
- x86_64 build: `96a0b3bb128d7dda522477d31946c11b9c4c5cff0fadd68f86be88d9d3ee9cf8`
- aarch64 image pull: `c7b2b6eaf68f39b9ee48f41c0f7215559a3359e7607e5e4c19ec2af65471f405`
- aarch64 shared-target failure: `f14629b04b694d1f285c56be1b82ee2c62a2421d40a4591d9ec311f05e1c3fff`
- aarch64 corrected build: `0d8035d602702eb34835d9bac8dfe4fbd02c6ad99b19d3e31872e09e67fe6b14`

## Full aarch64 shared-target failure log

```text
info: syncing channel updates for stable-x86_64-unknown-linux-gnu

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.98.0 (88d9e12ae 2026-08-18)

info: checking for self-update (current version: 1.29.0)
   Compiling cfg-if v1.0.4
   Compiling stable_deref_trait v1.2.1
   Compiling libc v0.2.186
   Compiling memchr v2.8.2
   Compiling zerofrom v0.1.8
   Compiling num-traits v0.2.19
error: failed to run custom build command for `libc v0.2.186`

Caused by:
  process didn't exit successfully: `/target/release/build/libc-85c1105c875ab78d/build-script-build` (exit status: 1)
  --- stderr
  /target/release/build/libc-85c1105c875ab78d/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.28' not found (required by /target/release/build/libc-85c1105c875ab78d/build-script-build)
  /target/release/build/libc-85c1105c875ab78d/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.29' not found (required by /target/release/build/libc-85c1105c875ab78d/build-script-build)
  /target/release/build/libc-85c1105c875ab78d/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.30' not found (required by /target/release/build/libc-85c1105c875ab78d/build-script-build)
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `num-traits v0.2.19`

Caused by:
  process didn't exit successfully: `/target/release/build/num-traits-818ff06d1e04fbcb/build-script-build` (exit status: 1)
  --- stderr
  /target/release/build/num-traits-818ff06d1e04fbcb/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.28' not found (required by /target/release/build/num-traits-818ff06d1e04fbcb/build-script-build)
  /target/release/build/num-traits-818ff06d1e04fbcb/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.29' not found (required by /target/release/build/num-traits-818ff06d1e04fbcb/build-script-build)
  /target/release/build/num-traits-818ff06d1e04fbcb/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.30' not found (required by /target/release/build/num-traits-818ff06d1e04fbcb/build-script-build)
error[E0463]: can't find crate for `zerofrom_derive`
  --> /cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerofrom-0.1.8/src/lib.rs:33:9
   |
33 | pub use zerofrom_derive::ZeroFrom;
   |         ^^^^^^^^^^^^^^^ can't find crate

For more information about this error, try `rustc --explain E0463`.
error: could not compile `zerofrom` (lib) due to 1 previous error
Command exited with non-zero status 101
	Command being timed: "cross build --release --target aarch64-unknown-linux-musl -p agov -p agov-mcp"
	User time (seconds): 0.27
	System time (seconds): 0.43
	Percent of CPU this job got: 31%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:02.27
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 54840
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 111
	Minor (reclaiming a frame) page faults: 61927
	Voluntary context switches: 3041
	Involuntary context switches: 198
	Swaps: 0
	File system inputs: 627264
	File system outputs: 256
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 101
```

The earlier storage-blocked x86_64 failure log remains in this file's Git
history and in coordination record
`20260826T153457Z-coder-tester-hkn4.md`; this retry supersedes its environment
disposition without erasing the evidence.

## Comparison with Option D

Option D records `20260809T214119Z-contribution-auditor-6tj3.md` and
`20260810T045208Z-contribution-auditor-es1p.md` reported successful x86_64 and
aarch64 musl builds with the same Podman selectors on temporary protected
runner `54982037`. They also reported repeated runner-environment corrections,
an earlier disk-exhaustion event, and about 5.8 GB headroom after cleanup.

This reproduction independently confirms the central Option D result on a
fresh local rootless store without runner registration or protected context.
It also sharpens the capacity evidence: the two pinned images alone occupied
about 3.99 GB, while retained Cargo output for the two independently isolated
targets occupied about 675 MB. The earlier 1.1 GB graph-store headroom could
not unpack one image; a 5.21 GB isolated graph store held both, with about
1.06 GB remaining. These observations are not a precise minimum or a production
sizing recommendation.

The new deviation from Option D is the cross-image Cargo cache finding. The
first aarch64 command inherited host build scripts produced under the x86_64
image and failed on glibc symbol versions. Independent target directories
removed that coupling and produced a pass. A Phase D plan should therefore
make target cache boundaries explicit and test the actual multi-target job
sequence rather than infer safety from two individually successful targets.

## Disposition

Phase B's required Linux rootless-Podman feasibility reproduction is complete.
It provides acceptance evidence for future Phase C and D planning but does not
authorize either phase, a runner, CI migration, provisioning, purchase, merge,
release, tag, or deployment. Phase D still needs its own Gate 1 plan and must
address the target-specific Cargo-state boundary identified here. Phase C
still needs the independent infosec review required by governor decision
`20260826T111710Z-governor-reviewer-2jq3.md` before its own Gate 1 approval.
