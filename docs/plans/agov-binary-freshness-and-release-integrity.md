---
id: agov-binary-freshness-and-release-integrity
status: pending-gate1
topic: agov-binary-freshness-and-release-integrity
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: No independent infosec response is available in this planning session; require an infosec review before Gate 1 approval because release-binary provenance is a supply-chain boundary.
  - role: contribution-auditor
    status: waived
    reason: No independent contribution-auditor response is available in this planning session; require an independent plan audit before Gate 1 approval because the safeguards must cover both local execution and public artifacts.
---

# Plan: fail closed on stale agov binaries and verify release artifacts

**Date:** 2026-07-26T18:13:00Z  
**Status:** PENDING GATE 1 APPROVAL

## Incident and intent

During the 2026-07-26 Codex cold start, `tooling/agov` silently selected an
executable repository-local release binary reporting `agov 2.2.6` at commit
`7dd69050fb1f`, while fetched `origin/main` and the current public line were
v2.4.4 at `e4afd034bb32`. The wrapper's current selection rule checks only
whether a binary is executable. It does not check embedded provenance against
the checkout, the requested cold-start base branch, or freshness-sensitive
source changes.

The release pipeline is stronger: tag jobs compile from source with explicit
`AGOV_BUILD_COMMIT=$CI_COMMIT_SHA` and `AGOV_BUILD_DIRTY=false`. However, the
packaging job does not independently inspect the binaries it stages and prove
that their embedded version, commit, and dirty state match the tag before the
archives become release inputs.

The desired invariant is:

> A stale or provenance-inconsistent `agov`/`agov-mcp` binary must be rejected
> before it can govern a session or enter a preview, release archive, or OCI
> image.

## Scope

### 1. Local wrapper freshness gate

Change `tooling/agov` so an existing debug/release binary is probed before
execution rather than trusted because it is executable.

The gate must:

- parse machine-readable embedded version/commit/dirty provenance;
- reject `unknown` or malformed provenance;
- verify that the embedded commit exists locally and is an ancestor of the
  current worktree `HEAD`;
- reject the binary when freshness-sensitive files differ between the embedded
  commit and `HEAD` (`tooling/agov`, `tooling/rust/**`, `agf/schemas/**`, and
  `agf/agents/MANIFEST.yaml` at minimum);
- when `session resume --base-branch <ref>` is requested, apply the same
  ancestry and freshness-sensitive-diff checks against that resolved base ref,
  so an old branch cannot silently validate a newer fetched base;
- fail closed with the observed and expected commits plus a safe rebuild
  command. It must not silently continue or silently build from a different
  branch;
- provide only an explicit, documented development override, if one is proven
  necessary. The override must never be used by CI packaging or release jobs.

No mtime-only decision is acceptable: copied files, caches, rebases, and
worktrees make timestamps insufficient evidence.

### 2. Binary provenance parity

Give both shipped executables a non-interactive provenance surface. `agov`
already has `version --format json`; `agov-mcp` must expose equivalent
version/commit/dirty information without starting the MCP server.

Both binaries built from the same job must report the same source commit and
dirty state. Tests must cover the contract.

### 3. Preview and release packaging gate

Strengthen `.rust-tooling-package` before archive creation:

- force the final `agov` and `agov-mcp` artifacts to be rebuilt for the current
  job rather than accepting a cached final executable;
- inspect each just-built target binary through a target-appropriate mechanism
  and assert version equals `agf/VERSION`, commit equals the full
  `$CI_COMMIT_SHA`, and dirty is `false`;
- verify the staged archive contents again before publishing artifacts;
- fail before checksums, provenance JSON, OCI inputs, package upload, or
  `release:create` when any assertion differs;
- ensure `release:create` and `agov:oci:release` consume only artifacts from the
  successful verified release job.

If a target binary cannot execute on the runner, implementation must use a
reviewed target-capable runner/emulator or a deterministic embedded-provenance
inspector. Grepping strings is not sufficient.

### 4. Durable incident evidence and operator guidance

- Extend learning `20260726T121943Z-w5jj` with this concrete stale-wrapper
  occurrence and the distinction between local freshness and release
  integrity, without rewriting any immutable coordination record.
- Update root and adopter-facing tooling guidance so cold start explicitly
  verifies binary provenance. Do not distribute machine-specific paths or the
  incident's local worktree layout.
- Document that matching versions alone are insufficient; commit and dirty
  state are part of the decision.

## Affected files

| File | Intended change |
|---|---|
| `tooling/agov` | Add fail-closed local provenance/freshness selection. |
| `tooling/rust/crates/agov-mcp/**` | Add non-interactive provenance output and tests. |
| `tooling/rust/crates/agov/**` | Share or test the provenance contract as needed. |
| `.gitlab-ci.yml` | Add final-binary rebuild and pre-archive/post-stage verification. |
| `agf/learnings/foundry.yaml` | Add this incident as bounded evidence to the existing sync-drift learning. |
| `AGENTS.md`, `agf/templates/AGENTS.md`, relevant tooling docs | State the enforced cold-start provenance check and recovery path. |

Exact files may narrow during implementation, but expanding beyond freshness,
provenance, packaging verification, tests, and matching guidance returns to
Gate 1.

## Acceptance tests

1. A fixture binary whose embedded commit predates a freshness-sensitive
   change is rejected before its requested command runs.
2. A binary built at an ancestor commit with no freshness-sensitive changes is
   accepted for ordinary feature-branch work.
3. `session resume --base-branch origin/main` rejects a binary compatible with
   an old branch but stale relative to the fetched base—the exact incident
   reproduced here.
4. Unknown, missing, malformed, non-ancestor, and dirty provenance each have
   explicit tested behavior.
5. Release and debug selection follow the same freshness policy; release
   remains preferred only when it passes.
6. `agov` and `agov-mcp` report matching version, full commit, and dirty state
   from one build.
7. A negative packaging fixture injects a stale final executable and the job
   fails before archive/checksum/provenance creation.
8. A negative packaging fixture injects a version, commit, or dirty-state
   mismatch and the job fails before publication.
9. A positive tag-equivalent build proves every staged executable and
   provenance JSON agree with `agf/VERSION` and the exact source SHA.
10. Archive and OCI paths consume the same verified binaries; no second
    unverified build path is introduced.
11. Rust workspace tests, shell tests, `agov validate`, distribution preview,
    and the relevant GitLab pipeline are green at the exact implementation
    head.

## Security and operational boundaries

- This is a provenance/integrity gate, not a claim of reproducible builds.
  Byte differences may be legitimate when embedded dirty state or toolchains
  differ, but the asserted source identity must be exact and machine-readable.
- Do not fetch or execute a replacement binary automatically.
- Do not trust a remote ref without the normal authenticated Git transport.
- Do not weaken tag immutability, checksums, or existing provenance JSON.
- The local gate may be advisory only during explicit Rust debugging; public
  packaging and release verification are blocking with no override.

## Migration and rollback

No schema migration is required. Existing local binaries that cannot prove
freshness will stop with rebuild instructions. Rollback may remove the local
gate only through a governed change; release-side verification must remain
fail closed once shipped.

## Gate 2 evidence required

- Exact-head test and CI links.
- Transcript of the reproduced v2.2.6-vs-v2.4.4 rejection.
- Negative packaging transcripts proving no archive is produced.
- Positive staged-binary/provenance report for every supported release target.
- Independent contribution audit and infosec review.
- Governor-reviewer binding decision before merge.
