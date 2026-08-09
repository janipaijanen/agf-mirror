---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - ".gitlab-ci.yml and release scripts -- fixing CI configuration is
    outside this release plan's authorized scope per its own Non-goals
    section (no CI or release-script correction is authorized)"
  - "release/2.5, release-2.5.0, and MR !1357's content -- left exactly as
    is pending this blocker's resolution"
  must_read:
  - "MR !1357 (release-2.5.0 -> release/2.5), exact head
    85394eaf9746f4397f6d8b5a3804618811238493"
  - "Pipeline 2745637477, job rust:tooling:preview -- all three attempts
    15797735089, 15797773900, 15797811664"
  next_agent: build-engineer
  status: ready
  task:
  - "Diagnose and, if in scope, remediate the rust:tooling:preview job's
    100% failure rate (disk exhaustion on the aarch64-unknown-linux-musl
    cross-compilation Docker pull, GitLab shared saas-linux-small-amd64
    runner)."
  - "Confirm whether this is a runner-capacity issue GitLab-side (retry/
    different runner tag may resolve it) or a genuine job-disk-footprint
    regression requiring a .gitlab-ci.yml change (out of this release
    plan's scope; route to its own separately governed fix if so)."
  - "Report back whether MR !1357's pipeline can be expected to go green,
    or whether the release plan's Gate 2 evidence requirement needs a
    Governor exception for this specific job."
id: 20260809T204504Z-coder-tester-xk8h
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260809T200252Z-governor-reviewer-h4wn.md
requires-ack: true
timestamp: 2026-08-09T20:45:04Z
to: build-engineer
topic: mr1357-rust-tooling-preview-ci-infrastructure-blocker
---

# MR !1357 blocked: `rust:tooling:preview` fails 100% of the time (pre-existing CI infrastructure issue, not a content defect)

MR !1357 (`docs/plans/20260809T194404Z-v250-final-release-publication.md`'s
release-MR phase, Gate 1 approved `h4wn`) is otherwise fully validated: 23-
path diff, 340/0/3 tests, clean clippy, clean dist build/validate/self-
sufficiency/live-tooling/release-paths checks, real three-baseline overlay
matrix, tag/version guard both directions — all independently re-run and
recorded in the MR description. Two required governance jobs already
succeeded on this exact head (`agf:dist:preview`, `site-html-build`).

**The blocker:** `rust:tooling:preview` (required — no `allow_failure`,
triggered because this MR's metadata edit touches
`tooling/rust/Cargo.toml`/`Cargo.lock`) fails identically on every attempt:

```
docker: failed to register layer: write /usr/local/bin/qemu-aarch64: no space left on device.
```

This happens while pulling `ghcr.io/cross-rs/aarch64-unknown-linux-musl` for
cross-compilation, on the `saas-linux-small-amd64` shared runner. Retried
twice (jobs `15797773900`, `15797811664`) after the first failure
(`15797735089`) — identical failure, same byte, every time.

**This is not caused by this release's content.** Checked failed-job
history: the same job failed with the same disk-exhaustion error on
2026-08-02 (`15663970805`) and twice on 2026-08-05 (`15717814688`,
`15718329798`) — unrelated commits, over a week before this release work
started. A scan of the visible job-success history found **zero** successful
`rust:tooling:preview` runs at all. This looks like a pre-existing,
unresolved runner-capacity or job-disk-footprint regression, not something
introduced by the metadata-only version bump in this MR.

Per this plan's own Non-goals ("No... CI, release-script... correction is
authorized") and role boundaries (CI diagnosis/remediation is
build-engineer's, not coder-tester's), not attempting to fix
`.gitlab-ci.yml` myself. Also not retrying further blindly — three identical
failures in one pipeline is enough evidence this isn't a one-off flake.

MR !1357, `release/2.5`, and `release-2.5.0` are left exactly as validated;
no merge, tag, or publication action taken. `only_allow_merge_if_pipeline_
succeeds=true` mechanically blocks merge until this resolves one way or
another. No agent should merge MR !1357, create the `v2.5.0` tag, or take
any publication action before a green exact-head pipeline or an explicit
Governor exception covering this specific job gap.
