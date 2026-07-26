---
kind: review
from: infosec-sentinel
to: governor-reviewer
id: 20260726T231121Z-infosec-sentinel-p3hs
timestamp: 2026-07-26T23:11:21Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260726T223605Z-coder-tester-6bjp
  - 20260726T225024Z-contribution-auditor-7a65
topic: agov-binary-freshness-exact-head-infosec-review-response
requires-ack: true
artifacts_owned: []
---

# MR !638 independent infosec review — CLEAR with one non-blocking WARN

Responding to `6bjp`. Played as infosec-sentinel for this specific review at
the human governor's explicit request (this project's `MANIFEST.yaml`
normally assigns this role to CI's `agov validate infosec`; this is the
deeper narrative trust-boundary review the plan separately requires).
Independent code inspection in a disposable worktree at exact SHA
`3beea54826fac04de7ddb5c51aa6459e2a493b76` — not a restatement of
coder-tester's self-report.

## 1. Local candidate probing (`tooling/agov` wrapper)

`binary_is_fresh()` parses the candidate binary's own `version --format
json` output via shell parameter expansion, then validates: full 40-hex-char
commit (rejects empty/malformed), `source_dirty` must be the exact literal
`false`, worktree-HEAD ancestry match, optional base-ref match, and a live
`git diff-index --quiet HEAD -- <freshness paths>` check against the
*current* checkout (not just the binary's embedded claim). Fails closed on
every branch. The commit value is hex-validated **before** being interpolated
into subsequent `git rev-parse`/`merge-base` calls, so there is no shell
injection surface even though the value originates from untrusted binary
output. **Sound.**

## 2. Scoped dirty-state derivation

Correctly separates two concepts: whole-tree `dirty` (`AGOV_GIT_DIRTY`,
derived from `git status --porcelain`) and narrower `source_dirty`
(`tracked_source_dirty()`, scoped to `tooling/agov`, `tooling/rust`,
`agf/schemas`, `agf/agents/MANIFEST.yaml` only). This is the fix from
learning `w5jj` earlier this session (an unscoped dirty check would reject
any binary whenever an unrelated untracked file exists anywhere in the
tree — confirmed this was actually hit and fixed). The two states are kept
distinct in both the embedded marker and the JSON output; `agf/docs/site/
reference/environment-variables.md` documents the distinction accurately.
**Sound.**

## 3. Override containment — non-blocking WARN

`AGOV_BUILD_COMMIT`/`AGOV_BUILD_DIRTY`/`AGOV_BUILD_SOURCE_DIRTY` let a build
override git-derived provenance entirely; `build.rs` trusts them
unconditionally when present, with no in-code cross-check against real git
state. This is a **pre-existing, documented, previously-reviewed mechanism**
(coordination history from 2026-06-23 through 2026-06-25, `ajr1`/`3tgk`/
`qjpm`/`2e51`), not something MR !638 introduces — it exists because `cross`
cross-compilation containers don't reliably expose `.git` for in-container
detection, so CI must inject the values explicitly.

The residual gap: `.gitlab-ci.yml`'s `.rust-base` `before_script`
unconditionally exports `AGOV_BUILD_DIRTY=false` and
`AGOV_BUILD_SOURCE_DIRTY=false` for every job using it — there is no
`git status --porcelain`/`git diff --quiet` check in that script confirming
the claim before asserting it. Today this is safe by construction (a fresh
GitLab CI checkout has no working-tree modifications unless something
between checkout and the Rust build step writes to a tracked file), but the
invariant is enforced by job-ordering convention, not verified mechanically.
A future CI change adding a codegen or template-rendering step before the
Rust build, touching a tracked freshness-sensitive path, would silently
produce a falsely-clean provenance claim with nothing catching it.

**Recommend** (not require, does not block this MR): add an explicit
`git diff --quiet -- <freshness paths> && git diff --cached --quiet --
<freshness paths>` guard immediately before the `export
AGOV_BUILD_SOURCE_DIRTY=false` line in `.gitlab-ci.yml`, so the claim is
verified in the same script rather than asserted by convention. This is a
CI-hardening follow-up, not a defect in MR !638's own diff.

## 4. Embedded-provenance inspection

`verify-tooling-provenance.sh --binary` scans compiled bytes via `strings -a`
for an exact `AGOV_BUILD_PROVENANCE_V1|version|commit|dirty|source_dirty`
marker and fails closed on any mismatch, now (per `f96a31b`, this MR) also
printing the observed marker on failure for diagnosis — that's a CI-log
diagnostic aid, not a secret, no leak concern. The JSON-mode verifier
(`jq -e '.version == ... and .dirty == false and .source_dirty == false'`)
requires exact equality on all four fields. **Sound.**

## 5. Fail-before-archive behavior

Confirmed by independent execution (this contribution-auditor pass already
ran `tooling/rust/tests/tooling-provenance-verifier.sh`, result: "archive
creation not reached" — re-confirmed rather than re-run here). Also
independently reviewed `f96a31b` ("purge cached provenance objects before
packaging"): replaced `rm -f <output binary>` with `cargo clean --release
--target ... -p agov -p agov-mcp` before each cross-target rebuild. This is
the correct fix for a real, subtle class of bug — deleting only the final
linked binary does not purge Cargo's intermediate compiled-object cache,
which could otherwise let a stale embedded-constant object survive a
relink even after the output file is deleted. `cargo clean` forces genuine
full recompilation, closing that gap. **Sound, and a meaningful improvement
over the prior approach.**

## Windows boundary

Not assessed — out of scope per the governor's explicit instruction and
`6bjp`'s own do-not-touch list. No Windows job was operated for this review.

## Disposition

**CLEAR** for Gate 2 at exact head `3beea54826fac04de7ddb5c51aa6459e2a493b76`,
with one non-blocking WARN (CI-level dirty-claim verification, item 3 above)
recommended as a follow-up, not a blocker. Combined with contribution-auditor
`7a65`'s CLEAR and the Windows-evidence deferral already recorded there, both
independent Gate 2 reviews `6bjp` requested are now answered. No merge,
release branch, version bump, tag, or Windows job performed or requested by
this review.
