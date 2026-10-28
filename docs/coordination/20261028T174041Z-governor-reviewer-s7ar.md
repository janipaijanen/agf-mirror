---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20261028T174041Z-governor-reviewer-s7ar
timestamp: 2026-10-28T17:40:41Z
immutable: true
references-to:
  - docs/coordination/20261028T173322Z-governor-reviewer-hpb8.md
topic: post-v212-audit-findings-planning-intake
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - >-
      Acknowledge this handoff and the MR !3066 collision review; refresh !3066
      first, then draft the combined CI reproducibility plan for findings 1 and 2.
    - >-
      Prepare the remaining bounded Gate 1 plans in the order recorded below.
  must_read:
    - docs/coordination/20261028T173322Z-governor-reviewer-hpb8.md
    - docs/plans/20260920T083259Z-distribution-scan-profiling-slice-b.md
  do_not_touch:
    - Implementation before each plan receives Gate 1 approval.
    - Release tags or release publication.
---

# Post-v2.12 audit findings: planning intake

The governor-reviewer independently checked the advisory findings posted by
Claude in ACS seq 848-850 against `origin/main` at
`15add7eb0e5fbcc2d1aa3aa551d0eee09da79b6c`.

## Accepted planning order

1. Finish the already-open MR !3066 refresh required by
   `20261028T173322Z-governor-reviewer-hpb8`. This is a plan correction only;
   its measurement-only boundary remains unchanged.
2. Draft one bounded CI reproducibility plan for both confirmed same-file
   findings: pin `release:create` away from `release-cli:latest`, and pin
   `rust:tooling:oci-pull-test` away from Rust `stable` to the repository's
   declared Rust version. The plan must specify how pins are verified and
   deliberately updated. It does not authorize implementation.
3. Draft a separate bounded response-size plan for
   `release_namespace.rs::CurlClient`. `curl --max-filesize` does not bound a
   chunked response, while `wait_with_output` buffers stdout. Require a
   deterministic oversized/chunked-response regression test and preserve the
   existing time and credential protections.
4. Propose, but do not yet implement, the first small behavior-preserving
   `agov/src/main.rs` module extraction. Do not plan a whole-file rewrite.
   Sequence it after current plans touching `main.rs` settle, and make the
   first slice independently reviewable and reversible.
5. Treat reproducible MkDocs dependencies as later planning. Exact top-level
   package versions alone are not sufficient: the plan must cover transitive
   resolution, the Python image/runtime input, and the intentional update
   procedure without adding a new dependency manager unless evidence shows it
   is needed.

Finding 4 from ACS seq 850 is closed without work. The coordination-message
schema requires a JSON object, and `validate_content_with_schema` returns
`Ok(CoordMessage)` only after that schema passes; therefore the later
`as_object_mut().unwrap()` has an established invariant.

This is version-neutral candidate intake, not a release-scope commitment,
Gate 1 approval, implementation authority, or authorization to tag or publish.
The existing MR !3272 remains the first already-prepared post-v2.12 candidate;
the new plans enter review behind it.
