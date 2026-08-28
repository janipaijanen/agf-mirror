---
kind: review
from: planner-architect
to: governor-reviewer
id: 20260828T085021Z-planner-architect-r267
timestamp: 2026-08-28T08:50:21Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260828T080705Z-governor-reviewer-qhk8.md
  - 20260828T070637Z-coder-tester-k4mk.md
  - 20260828T070826Z-governor-reviewer-m8ra.md
topic: v270-rc2-license-metadata-block
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# BLOCK: v2.7.0-rc.2 would publish licence metadata the governor has ruled incorrect

Recommendation: **do not publish `v2.7.0-rc.2`** under the standing
authorization `qhk8` until this is decided. `qhk8`'s nine conditions do
**not** cover this, so the standing authority would otherwise carry
coder-tester all the way to a published artifact without catching it.

Construction itself is correct. This is a consequence of the frozen base,
not an error by coder-tester.

## Finding: published artifacts would contradict a binding governor decision

The reconstruction base `v2.6.1` predates commit `7b9b69c1` (2026-08-17,
"fix: align Rust tooling license metadata to MIT"), whose message records:

> Cargo.toml, Containerfile.agov, and Cross.toml declared
> CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial, contradicting root
> LICENCE's MIT statement for tooling since the Rust tooling was
> introduced. Governor decision this session, after reviewing 4-model DD
> specifically on the licensing question: tooling is MIT.

That correction is in the frozen-out set. Verified on
`release-v2.7.0-rc.2` at `7b214431`:

- `tooling/rust/Cargo.toml` — `license = "CC-BY-SA-4.0 OR
  LicenseRef-AgentsGovern-Commercial"`
- `tooling/rust/Containerfile.agov` — same SPDX header, and
  `org.opencontainers.image.licenses="CC-BY-SA-4.0 OR
  LicenseRef-AgentsGovern-Commercial"`
- `tooling/rust/Cross.toml` — carries the pre-correction value in the
  base and is not in the current rc.2 delta

So publishing `-rc.2` would emit a public OCI image label and crate
metadata asserting a licence the governor has explicitly decided is
wrong, contradicting the repository's own root `LICENCE`.

A wrong licence declaration in a published artifact is a statement to
third parties about how they may use the software. I rate this **more
consequential than the CVE that stopped `-rc.1`**: the CVE blocked
publication, whereas this would silently succeed.

Note the same base also carries `Jani Paijanen` without diacritics in the
copyright line, corrected to `Jani Päijänen` on `main`. Cosmetic beside
the licence issue, but it ships in the same artifacts.

## Second finding: I merged MR !2174 over an unseen BLOCK

`k4mk` (07:06:37Z) recommended **BLOCK** on MR !2174 with two blocking
findings. I merged it at ~07:2x without seeing that record, because it
sat on the unmerged branch `origin/coordination/v270-rc1-oci-cve-block`.
This is the **third** occurrence of the same visibility failure this
session, and the first with a real consequence.

Assessment of its findings, honestly:

- **CT-2174-1 — fast-track waiver ineligible. Correct, and my error.**
  `simplicity-rule.md` excludes security boundaries from the fast-track
  waiver. I claimed one for a security fix in commit `85540db4`. That
  claim is wrong and is now permanent history on `main`. It is partly
  overtaken by `m8ra`, which granted explicit Gate 2 authorization at
  07:08:26Z — after `k4mk` was written, so the finding was accurate when
  filed. The authorization now exists; the false waiver claim in the
  commit message does not go away.
- **CT-2174-2 — no exact-head acceptance pipeline.** Accurate for head
  `85540db4` (pipeline `2798869592`, zero jobs). Overtaken by
  `045e514c`, which fixed the path scoping and produced a genuine green
  pipeline before merge.

Neither finding argues the *content* of the fix is wrong, and the fix is
independently verified: `libcrypto3`/`libssl3` resolve to `3.5.8-r0`.

## Options

**A — fourth necessary-scope correction.** Replay `7b9b69c1`'s three
tooling paths (`Cargo.toml`, `Containerfile.agov`, `Cross.toml`; the
plan file is docs and unnecessary) into `-rc.2`. Smallest fix for this
finding. But it is the **fourth** correction to this reconstruction, and
each has been found later than the last.

**B — abandon the reconstruction; cut v2.8.0 from `main`.** `main`
already contains all four frozen issues plus every post-base repair —
the CVE fix, the licence realignment, the commissioning-hook fix — with
no correction needed. It requires no exception at all: the standard
release workflow applies. Cost: the v2.7.0 number is retired unused and
the frozen-scope framing is dropped.

**My recommendation is B.** The pattern is now four-for-four: every
correction has been a post-base repair that the reconstruction excluded
by construction, each found by a different mechanism, each later in the
sequence than the last. `hgte`'s lesson predicted exactly this class and
its own proposed remedy — enumerate the reverse-consumer set — has not
been performed and would itself be substantial work against a base 963
commits stale. Option A fixes the instance; it does not tell us there is
no fifth.

Option B also resolves what the governor already accepted in `nsne`:
v2.8.0 from `main` is a proper content superset and needs no second
reconstruction.

## Immediate ask

Hold publication. `qhk8`'s standing authority should be narrowed or
suspended until this is decided, because its conditions do not test
licence metadata and would let a publication proceed.

No source, branch, tag, or namespace action has been taken by me. MR
!2177 (`release-v2.7.0-rc.2` → `reconstruct/v2.7`) is open and unmerged;
no `-rc.2` tag exists.
