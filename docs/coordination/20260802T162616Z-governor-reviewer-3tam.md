---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260802T162616Z-governor-reviewer-3tam
timestamp: 2026-08-02T16:26:16Z
immutable: true
references-to:
  - 20260802T162308Z-contribution-auditor-mmbb
  - 20260802T143239Z-governor-reviewer-tq8r
topic: ci-rust-cache-policy-slice-b-scope-amendment
requires-ack: false
artifacts_owned:
  - docs/plans/ci-rust-cache-policy-slice-b.md
---

# Gate 1 scope amended: Rust cache-policy Slice B

The governor-reviewer directed: "amend the approved scope, please" in response
to the contribution-auditor's MR !939 finding that the implemented topology
checker is manual rather than an automatic CI control.

Binding amendment:

- Automatic per-pipeline second-writer topology enforcement is no longer a
  required Slice B outcome.
- `agov ci cache-topology-check` remains mandatory Gate 2 evidence and must run
  against a fresh authenticated GitLab CI Lint merged-YAML response for the
  exact implementation head.
- The one-writer topology itself remains mandatory at the reviewed head.
- Automatic per-job role/context assertions remain mandatory and fail closed
  for missing/unknown roles or wrong resolved policy.
- No new credential, permission, or project setting is authorized for this
  slice. The observed `CI_JOB_TOKEN` 403 is accepted as the reason automation
  was removed.
- The accepted residual risk is that a later deliberate second writer override
  is found by explicit topology review rather than automatically by every
  pipeline. The fail-safe base still makes an ordinary new inheritor pull-only.

This decision resolves finding B1 once the plan text is revised consistently
and validated. It does not approve Gate 2 or merge MR !939; the
contribution-auditor must re-evaluate the exact head against the amended plan,
then the governor-reviewer makes the separate binding Gate 2 decision.
