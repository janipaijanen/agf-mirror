---
id: 20260905T175920Z-native-musl-release-build
status: pending-gate1
topic: native-musl-release-build
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      No infosec-sentinel review exists yet. This is recorded as waived rather
      than consulted. The proposed direction removes a privileged DinD service
      and its unauthenticated TCP endpoint from the mandatory release path, so
      an independent security review is requested before binding Gate 1.
  - role: contribution-auditor
    status: waived
    reason: >-
      No contribution-auditor disposition exists yet. The planner-architect
      supplied the route analysis in coordination record
      20260905T175059Z-planner-architect-hm61.md, but that is not an independent
      contribution audit. Review is requested before binding Gate 1.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Native musl packaging for the mandatory Linux release path

## Decision requested

Approve replacing Cross plus Docker-in-Docker with Rust's native musl target
build for the mandatory `x86_64-unknown-linux-musl` tooling archive. Keep the
optional Windows preview on a separately isolated Cross/DinD template and do
not retry MR !2383's known-broken pipeline before either the runner is repaired
or this approved correction is applied to its exact head.

This is a Gate 1 plan only. It does not authorize implementation, merge,
release, tag, deployment, runner reconfiguration, or enabling privileged
services.

## Release blocker and evidence

MR !2383 is release-critical and its exact-head local workspace tests are
green, but canonical pipeline 2822977825 failed before tests could run. Job
16325589120 could not start `docker:dind` (`mount: permission denied`), after
which Cross could not reach `docker:2375` (`no route to host`). This is a
runner/runtime mismatch, not code evidence.

Coordination record `20260905T175059Z-planner-architect-hm61.md` identifies two
possible corrections and recommends the smaller mandatory-release route:
remove Cross/DinD for Linux and compile the one mandatory target directly with
Cargo. The release manifest currently contains exactly:

```text
x86_64-unknown-linux-musl linux-x86_64
```

A controlled local feasibility run installed that target with `rustup target
add`, then successfully built both workspace binaries with:

```text
cargo build --manifest-path tooling/rust/Cargo.toml --release \
  --target x86_64-unknown-linux-musl -p agov -p agov-mcp
```

`file` reported both outputs as x86-64 static PIE executables, `ldd` reported
them statically linked, and both executed `--version`. No `musl-gcc`, Docker,
Podman, or Cross executable was required. This proves build-method feasibility;
it is not release evidence because it was not produced by an exact-head
canonical pipeline.

## Scope and design

1. In `.rust-tooling-package`, remove the `docker:dind` service,
   `DOCKER_HOST`, `DOCKER_DRIVER`, `DOCKER_TLS_CERTDIR`, and `CROSS_REMOTE`.
   Remove `docker.io` and the `cargo install cross` step from that template.
2. After the existing fail-closed release-target manifest validation, install
   every declared Rust target with `rustup target add "$target"` and replace
   the mandatory Linux loop's `cross build` with `cargo build`. Preserve the
   exact packages, target, release profile, staging, checksums, provenance,
   archive verification, and downstream artifact names.
3. Preserve `rust:tooling:preview` and `rust:tooling:release` as consumers of
   the now container-free mandatory template. Do not alter their selection
   rules or `needs` relationships.
4. Preserve optional Windows preview capability without leaking its container
   requirements back into mandatory Linux jobs. Introduce a private Windows
   packaging template that extends the common packaging script but adds the
   existing DinD variables/service and Cross installation; make only
   `rust:tooling:preview:windows-package` extend it. Keep
   `rust:tooling:build:windows:preview` optional/manual and otherwise
   unchanged. A future Podman-native Windows route is separate work.
5. Do not change the required target manifest, artifact contract, release
   version, runner tags, or any source code in `agov`/`agov-mcp`.

The split is intentional: tag releases and ordinary MR previews stop depending
on privileged nested containers immediately, while an optional Windows job may
remain unavailable on the current runner without blocking a Linux release.

## Gate accompaniment and negative test

`.gitlab-ci.yml` is a registered surface for both `gate-change-check` and
`ci-needs-implication`. The implementation must therefore change a declared
test path for each gate and provide real negative evidence, not token edits.

1. In `ci_needs_implication.rs`, add a live-configuration invariant for the
   mandatory packaging path: `rust:tooling:preview` and
   `rust:tooling:release` inherit a template with no DinD service or Docker
   transport variables, and the Linux target loop uses `rustup target add`
   plus `cargo build`, not `cross build`. Exercise the same checker against a
   deliberately mutated template that restores `docker:dind`/`cross build`;
   it must fail for the expected reason. Run the live assertion before the CI
   edit (RED) and after it (GREEN).
2. In `gate_change_check.rs`, add a focused synthetic repository case where
   `.gitlab-ci.yml` changes and only the `ci-needs-implication` test family is
   changed. `agov gate change-check` must still exit 1 and name
   `gate-change-check` as lacking its own declared evidence. Then add the
   gate-change test change and demonstrate the combined implementation diff
   passes. This re-demonstrates that the shared-surface gate still fires after
   the CI modification.
3. Put the reproducible negative-test commands, expected failures, actual
   failures, and exit codes in the implementation MR description. The
   canonical exact-head pipeline log is the visible verification artifact
   required by `gate-self-test-rule.md`.

MR !2373 proposes different live-registry/default-runner-tag tests for MR
!2362. The implementation must scan and rebase against that work before
editing either Rust test file. It may reuse helpers after !2373 merges, but it
must retain the distinct packaging-path assertions and fresh negative run
above; the existence of another pending plan is not evidence for this change.

## Acceptance criteria

1. `agov gate change-check --base-ref origin/main` passes on the complete
   implementation diff.
2. Both deliberate negative cases fail for their documented expected reason,
   then the corresponding exact implementation passes.
3. `cargo test --manifest-path tooling/rust/Cargo.toml --workspace` passes.
4. The canonical MR pipeline for the exact implementation head passes,
   including `rust:tooling:preview`; no mandatory job starts `docker:dind` or
   contacts `docker:2375`.
5. Downloaded preview artifacts contain executable `agov` and `agov-mcp`,
   retain their expected filenames/checksum/provenance records, pass the
   existing self-sufficiency and no-Elixir checks, and are statically linked
   x86-64 Linux executables.
6. CI configuration inspection confirms the optional Windows preview remains
   manual and allowed to fail, while tag releases remain Linux-only.
7. No paid failed job is retried merely to seek green status; the accepted run
   must contain the correction.

## Risks and mitigations

- Native musl support could regress if a future dependency adds an external C
  toolchain requirement. The exact-head packaging job and executable/static
  artifact checks remain the release evidence; Cargo failure stays blocking.
- Splitting inherited YAML can silently drop `before_script` setup. The
  implementation must keep the existing `!reference [.rust-base,
  before_script]` in every overriding template and the live CI test must check
  the effective mandatory path.
- The optional Windows job remains dependent on nested container support. Its
  manual/allowed-failure status and isolation make that limitation explicit;
  this plan must not claim Windows deployability.
- Two pending CI plans may touch the same tests. Rebase/collision inspection is
  mandatory before implementation; do not overwrite another contributor's
  test changes.

## Rollback

Revert the implementation commit. This restores the prior mandatory
Cross/DinD template and leaves runner configuration untouched. If rollback is
needed after a release attempt, do not retry until the restored runner
dependency is known operational.

## Evidence and dependencies

- `20260905T175059Z-planner-architect-hm61.md` — runtime diagnosis and Route A
  recommendation.
- `20260905T171349Z-coder-tester-7m4q.md` — exact-head Gate 2 handoff for
  !2383.
- MR !2383 / pipeline 2822977825 / job 16325589120 — blocked release-critical
  implementation and exact infrastructure failure.
- `.gitlab-ci.yml` `.rust-tooling-package` and the two manual Windows preview
  jobs — current build paths.
- `.agf/gates.yaml` and `agf/rules/always/gate-self-test-rule.md` — required
  gate accompaniment and negative-test evidence.
- `tooling/rust/release-targets.txt` — mandatory target authority.

