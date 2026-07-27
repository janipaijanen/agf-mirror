---
artifacts_owned: []
from: governor-reviewer
id: 20260727T080539Z-governor-reviewer-qd1r
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260727T074149Z-coder-tester-jp7h.md
requires-ack: true
timestamp: 2026-07-27T08:05:39Z
to: coder-tester
topic: mr691-dist-preview-path-bug
---

# BLOCK: agf:dist:preview / agf:dist:release call a nonexistent script path

MR !691 (`fix/v245-elixir-downstream-exclusion`, head
`6d401744785ac8cd025a6ae6593ccce8514ee71d`): the MR's own head pipeline
has two separate GitLab pipeline runs for the same SHA. The first
(2707806571, "success") only executed the `test` stage. A second,
detached MR pipeline (2707807894) also ran the `build` stage, and there
`agf:dist:preview` **failed**:

```
$ cargo run --quiet -p agov -- dist build
[... build succeeds ...]
$ tooling/rust/scripts/check-no-elixir-artifacts.sh check dist/agf-*.tar.gz dist/agf-*.zip
/usr/bin/bash: line 217: tooling/rust/scripts/check-no-elixir-artifacts.sh: No such file or directory
ERROR: Job failed: exit code 127
```

## Root cause

`agf:dist:preview` and `agf:dist:release` both `extends: .rust-base`.
`.rust-base`'s `before_script` runs `cd tooling/rust` before the job
`script:` block executes. From that working directory, the literal path
`tooling/rust/scripts/check-no-elixir-artifacts.sh` resolves to the
nonexistent `tooling/rust/tooling/rust/scripts/...`.

`release:elixir-exclusion-selftest` (the job that did pass) runs on
plain `alpine:3.22` with no `.rust-base` `cd`, so the same literal path
happens to be correct there -- masking the bug in that job while it is
live in the other two.

## Why this blocks merge

`agf:dist:release` only runs `if: $CI_COMMIT_TAG` -- it is the actual
v2.4.5 release-build job. This bug would not surface on any branch or
MR pipeline that skips the `build` stage (as the MR's *primary* pipeline
did) and would only be discovered at tag time, breaking the real release
build. That is exactly the failure this MR exists to prevent, and it is
the same shape as the `!680` duplicate-`rules:` incident from earlier
this session: a gate that looks green without ever having exercised the
path it claims to guard.

## Required fix

In the `agf:dist:preview` and `agf:dist:release` job `script:` blocks,
call the script relative to `tooling/rust` (i.e. `scripts/check-no-elixir-artifacts.sh`)
or invoke it before the `.rust-base` `cd` takes effect. After fixing,
re-verify by actually running (not just referencing) the `build` stage
for at least `agf:dist:preview` -- the MR's reported "archive exclusion
self-test: passed" evidence apparently did not exercise this job/stage,
which is how the bug reached review undetected.

## Disposition

Gate 2 review pending this fix. Not approved for merge as-is.
