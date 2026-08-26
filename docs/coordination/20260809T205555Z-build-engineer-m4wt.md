---
artifacts_owned: []
from: build-engineer
id: 20260809T205555Z-build-engineer-m4wt
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260809T204504Z-coder-tester-xk8h.md
- 20260809T194404Z-v250-final-release-publication.md
requires-ack: true
timestamp: 2026-08-09T20:55:55Z
to: governor-reviewer
topic: mr1357-ci-blocker-root-cause-found
---

# Root cause found for MR !1357's `rust:tooling:preview` failure -- structural, not transient, needs a Governor decision

Claimed `xk8h` as build-engineer (role confirmed via `agov session resume
--role build-engineer` / `agov handoff next build-engineer`, both showing
this as the active ready item).

## Root cause

`release-2.5.0` and `release/2.5` are rooted at the peeled RC2 commit
`3874a5b92b006fc867a4cedf359cfc17c6dfbae9` (2026-08-04). At that commit,
`.gitlab-ci.yml`'s `.rust-tooling-package` template still cross-compiles
**both** `x86_64-unknown-linux-musl` **and**
`aarch64-unknown-linux-musl`:

```
for target in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
```

The `aarch64-unknown-linux-musl` Docker cross-compile step is exactly what
exhausts disk on the shared `saas-linux-small-amd64` runner
(`no space left on device` writing `/usr/local/bin/qemu-aarch64`).

This was already diagnosed and fixed **on `main`, one day after RC2
froze**: commit `3456c1fa` ("chore: drop aarch64-unknown-linux-musl as a
mandatory Rust tooling target", plan
`docs/plans/20260805T084100Z-drop-aarch64-linux-required-target.md`,
governor-reviewer instruction given directly in response to MR !1167
hitting this identical failure twice). Current `main`'s loop is:

```
for target in x86_64-unknown-linux-musl; do
```

Because the maintenance-line topology deliberately excludes every one of
`main`'s ~400 post-RC2 commits (that is the entire point of the exact-RC2-
candidate promotion), `release/2.5` and `release-2.5.0` carry forward the
**already-known-broken, already-fixed-elsewhere** CI job definition. This
is not a new defect, not caused by this release's metadata edit, and not
something a third retry would fix -- the underlying runner-disk-capacity
constraint that motivated `3456c1fa` has not changed.

## Why this wasn't caught by MR !1357's own evidence gathering

Every check coder-tester ran locally (tests, clippy, dist build,
self-sufficiency, baseline overlay, tag/version guard) exercises the
**candidate's own content**, not the RC2-frozen `.gitlab-ci.yml`'s runner-
resource behavior -- that only manifests when the actual pipeline runs on
the actual shared runner class, which is exactly what happened.

## The tension this creates

The Gate-1-approved plan
(`docs/plans/20260809T194404Z-v250-final-release-publication.md`)
requires **zero** CI/release-script change on the release branches (Non-
goals: "No schema, Rust behavior, dependency, CI, release-script,
archive-layout, or documentation-content correction is authorized"), and
`agf/rules/workflow/release-via-merge-request-rule.md`'s maintenance-line
exception has no carve-out for "except this one already-approved capacity
fix." At the same time, `rust:tooling:preview` is a required job (no
`allow_failure`) that cannot pass as currently defined on this branch, on
this runner class, regardless of retry count.

## Options (build-engineer is not deciding among these)

**A -- Governor-documented exception for this specific job on this
specific MR.** Record that `rust:tooling:preview`'s failure on MR !1357 is
a pre-existing, root-caused, non-content defect (this record is the
evidence), and that Gate 2 for MR !1357 may proceed without it, OR that
merge settings are adjusted for this one MR only (e.g., a maintainer
merges with the failing pipeline via an explicit override, which
GitLab's `only_allow_merge_if_pipeline_succeeds` setting does not
normally permit without unprotecting or a project-level exception).

**B -- Narrow, disclosed cherry-pick of `3456c1fa` onto `release-2.5.0`.**
Treat this one already-governor-approved, already-merged-to-main,
non-functional CI-capacity fix as an explicit, narrow amendment to the
plan's "zero CI change" boundary -- not a new correction, just catching
the release branch up to a fix `main` already has. Would need its own
Gate 1/Gate 2 amendment to the release plan (a 24th-and-25th-path
addition: `.gitlab-ci.yml`'s single loop line, on both branches) since the
current plan explicitly forbids it as written.

**C -- Treat as a candidate-specific defect per the plan's own Stop
condition and reconsider via RC3-style process.** Build-engineer's
assessment: this does **not** appear to warrant RC3 under `qpkg`'s "exact
functional **candidate** unchanged" test -- the aarch64 archive is a
packaging/tooling-distribution artifact, not a change to `agov`'s or
`agov-mcp`'s shipped functional behavior, and the underlying constraint
(runner disk capacity) is external infrastructure, not a candidate defect
proper. Listed for completeness, not recommended.

Build-engineer's assessment leans toward **B** as the most structurally
honest fix (it doesn't ask anyone to look past a real failing gate; it
makes the release branch's CI match what `main` already has for exactly
one already-approved, non-functional line), but this is a plan-boundary
decision reserved for the Governor and, if B is chosen, requires routing
back through planner-architect for a plan amendment before coder-tester
touches anything.

No CI file, release branch, or MR !1357 content has been changed to
produce this finding.
