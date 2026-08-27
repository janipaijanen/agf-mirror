---
kind: decision-record
from: governor-reviewer
to: all
id: 20260815T174820Z-governor-reviewer-jj5s
timestamp: 2026-08-15T17:48:20Z
model: human
immutable: true
references-to:
  - 20260815T173111Z-governor-reviewer-kpkk.md
  - 20260813T154648Z-governor-reviewer-2r8r.md
topic: v260-aarch64-drop-after-success
requires-ack: false
artifacts_owned: []
---

# Binding post-test decision: drop aarch64 from mandatory targets

The approved bounded necessity procedure completed on the exact throwaway
state before this decision:

- immutable base: annotated `v2.5.0` tag object
  `b9215bd65aec59bea45f1576d789d00ca3f3e4ae`, peeled to
  `f1932a1a486b772ca4353c7fa71c2279132fe0dc`;
- unprotected, never-to-be-merged branch:
  `throwaway/v260-aarch64-necessity-20260815`;
- exact test SHA: `76c9f5663b49867219f55fd86682a79dc791f36b`, whose
  parent is the peeled base and whose only diff is the one-run manual CI rule;
- pipeline: `2762845613`;
- decisive job: `15916862141`, `rust:tooling:release`, run on shared runner
  `4-green.saas-linux-small-amd64.runners-manager.gitlab.com/default`;
- result: **SUCCESS** in 530.315 seconds. The job built and packaged both
  `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`, wrote the
  aarch64 provenance record, passed release self-sufficiency for both
  archives, uploaded the artifacts, and ended with `Job succeeded`.

The containing pipeline is canceled because unrelated automatically-created
jobs were canceled to keep the governed procedure bounded; the decisive
manual job itself completed successfully. This success is evidence that the
tested runner handled the mandatory aarch64 path. It is **not** evidence of
disk exhaustion or inadequate runner capacity, and it does not retroactively
satisfy `2r8r`'s failure-based necessity condition.

After receiving that result, Jani issued the explicit binding direction:
“drop aarch64 from mandatory targets.” The governor therefore decides:

1. `aarch64-unknown-linux-musl` is dropped from the mandatory Rust tooling
   target set for v2.6.0 despite the successful bounded run.
2. Exact commit `3456c1fac7ee3112674fd2b4ebfd884e56501896` is effective
   for the v2.6.0 frozen-scope allowlist. Use the exact commit; do not partially
   cherry-pick or recreate it.
3. This is a new post-evidence product and operating-policy decision. It
   supersedes only the success-stop disposition in `2r8r`/`kpkk` for this
   exact release and commit; it does not mischaracterize the test result or
   establish a general rule that successful necessity checks may be ignored.

This decision resolves the conditional-target disposition. It does not itself
authorize Gate 2, merge MR !1492, create a reconstruction or release branch,
create a version bump or tag, publish a release, deploy, or close issues.
Those actions remain behind their existing explicit gates.
