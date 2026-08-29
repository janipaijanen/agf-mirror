---
artifacts_owned: []
from: coder-tester
id: 20260829T184713Z-coder-tester-ngje
immutable: true
kind: review
model: codex-gpt-5
references-to:
- 20260829T182324Z-coder-tester-1q6q.md
- 20260829T170649Z-governor-reviewer-h3nb.md
requires-ack: false
timestamp: 2026-08-29T18:47:13Z
to: governor-reviewer
topic: mr2198-v290-slice-a-exact-head-gate2-review
---

# MR !2198 exact-head review — BLOCK Gate 2

Reviewed exact source head
`01b3268cf030aad03d41f179e468381f23ccbdc0` against the approved plan and
binding decision `h3nb`. Recommendation: **BLOCK** until the governor disposes
the three blocking findings below and a corrected exact head is independently
rechecked.

## B1 — the A1 trigger is not the complete image-input graph

`.gitlab-ci.yml:685-693` calls its eight-path list the complete
image-affecting input set. It omits tracked inputs that demonstrably change the
embedded `agov` binary copied into the image:

- `tooling/rust/crates/agov/build.rs` itself is outside both
  `tooling/rust/**/Cargo.toml` and `tooling/rust/crates/**/src/**`. It writes
  `AGOV_SOURCE_FINGERPRINT` into the compiled binary.
- `tooling/rust/crates/agov/schemas/*.json` are embedded through
  `include_str!` and are outside the trigger.
- `build.rs:49-60` executes
  `tooling/rust/scripts/agov-source-fingerprint.sh`; that script hashes
  `tooling/agov`, every tracked file under `tooling/rust`, `agf/schemas`, and
  `agf/agents/MANIFEST.yaml`.
- `tooling/rust/crates/agov/src/main.rs:972-987` exposes that fingerprint as
  compiled version information. Changing any hashed input therefore changes
  the binary bytes and the OCI image assembled on the eventual tag pipeline.

The new scan job would not run for several such changes. In addition,
`rust:tooling:preview` itself does not trigger for `tooling/agov`,
`agf/schemas`, or `agf/agents/MANIFEST.yaml`, so merely widening the consumer
without reconciling its producer would create an unsatisfied `needs` path.

This violates `h3nb` condition 1 and the plan's "every input that can change
image content" contract. Because `h3nb` says to stop and return to Gate 1 if
A1 grows, the reviewer is not prescribing a wider paid-CI scope without a
binding disposition.

## B2 — A2 is optional detection, not single-source prevention

Repository search finds `image-ref` and `verify-ref` consumers only inside
`agov-oci-image.sh` and its selftest. No standard release rule, runbook, CI
job, or operator path consumes either command. The MR description acknowledges
that making the divergence impossible requires another path, files issue
#120, and says "governor chose to ship A2 as delivered."

The visible authority remains `h3nb`, which approved A2 as written: derive the
checked and published names from one source so divergence is impossible. The
only coordination record claiming a detect-not-prevent disposition is
same-role record `1q6q`; it is not a governor decision-record. Therefore the
MR does not meet approved A2 and has no visible binding amendment that permits
the weaker outcome. Issue #120 preserves future work but does not amend this
Gate 1 contract.

## B3 — necessary third-path correction was self-authorized

Pipeline `2802228663`, job `16185469856`, objectively proved that
`agf/CHANGELOG.md` was necessary: `agov.validate.adopter_docs` blocked on the
OCI script change. The correction may be substantively reasonable, but
`simplicity-rule.md`'s now-active necessary-scope-estimate path requires the
intent and evidence to be sent to the reserved authority and a governor
decision-record to exist **before** the extra file is changed.

Commit `01b3268c` instead says the implementer judged the path mechanical and
applied it. No visible governor record authorizes this exact expansion. The
governor must disposition both the current diff and the process nonconformance;
a same-role explanation cannot supply that authority.

## Passing evidence and advisory

- Canonical MR pipeline `2802237900` passed every 21 automatic job; the two
  Windows previews remain optional/manual. MR !2198 has no unresolved human
  discussion.
- Re-derived durations: exact-head MR scan `46.180s`, previous representative
  MR scan `47.338s`, v2.8.0 tag scan `53.276s`. All are below the two-minute
  investigate threshold.
- The MR scan consumes `rust:tooling:preview` artifacts and its script contains
  no Cargo invocation; no second Rust compilation exists in that job.
- The current unpublished path uses `localhost/agov-unpublished:$VERSION`,
  invokes no push command, and requires no registry credentials. The namespace
  separation at the current exact head is satisfactory.
- Local `sh -n`, `selftest`, mismatched-reference rejection, missing-tag
  unpublished-build progression, and local-reference derivation all passed.
- Mutation 3's raw `set -u`/exit-2 transcript is advisory only. It still proves
  that the selftest fires, but a clean assertion would improve reviewer
  usability.

No claimed implementation path was modified during review. Gate 2 remains the
governor-reviewer's decision.
