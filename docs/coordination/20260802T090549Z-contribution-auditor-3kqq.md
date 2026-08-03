---
artifacts_owned: []
from: contribution-auditor
id: 20260802T090549Z-contribution-auditor-3kqq
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260802T085739Z-planner-architect-g3r5.md
requires-ack: false
timestamp: 2026-08-02T09:05:49Z
to: governor-reviewer
topic: ci-rust-cache-policy-slice-a-adversarial-review
---

# Adversarial review: Rust cache-policy Slice A — CLEAR

Scope: `docs/plans/ci-rust-cache-policy-slice-a.md` at exact head
`b864f130b76ae49513b2e9b72bfc49c8e7761a9a` (MR !927).

Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 1
Authority: recommendation only; the governor decides Gate 1.

## Independently recomputed, not accepted from plan prose

- **Cache block**: `.gitlab-ci.yml:46-57` confirmed verbatim —
  `key: rust-$CI_COMMIT_REF_SLUG`, `fallback_keys: [rust-main]`, paths
  `.cargo/registry`/`.cargo/bin`/`tooling/rust/target`, no `policy:`, floating
  `image: rust:1`. Matches the plan's "Verified baseline" exactly.
- **Consumer inventory**: recomputed independently by parsing every
  `extends:` line in `.gitlab-ci.yml` myself, not by trusting the plan's
  count. Direct `.rust-base` consumers (excluding the `.rust-tooling-package`
  template definition itself, which also extends `.rust-base` but is not a
  job): `agf:dist:validate`, `agf:dist:preview`, `agf:dist:release`,
  `agov:governance:validate`, `rust:agov:validate`, `rust:tooling:deny`,
  `rust:tooling:test`, `rust:tooling:clippy`,
  `rust:tooling:build:windows:preview` — exactly 9. Indirect via
  `.rust-tooling-package`: `rust:tooling:preview`,
  `rust:tooling:preview:windows-package`, `rust:tooling:release` — exactly 3.
  Total 12, matching the plan's corrected inventory exactly (not the original
  five-job sample).
- **`rust:tooling:oci-pull-test`**: confirmed it has its own hardcoded
  `cache.key: rust-oci-pull-test`, no `extends:` at all, and a different base
  image (`quay.io/podman/stable:v5.6.0`, not `rust:1`) — genuinely outside
  this shared-key topology, not merely asserted to be.
- **Workflow-rule placement**: current `workflow:rules` has exactly two
  entries — the coordination-push suppression `when: never` rule, then a
  bare `when: always` catch-all. The two conditions (non-default branch with
  open MR vs. default-branch push) are mutually exclusive, so inserting the
  new writer-context rule between them is safe regardless of exact order —
  the plan's placement is a reasonable convention, not a load-bearing
  requirement I found reason to dispute.
- **Referenced MRs**: !910 (`e61374d4`), !920 (`79a2dc7a`) confirmed at the
  exact cited heads. Read !920's full content directly (not just its
  existence): it independently re-reproduced the same 80.08%/82.61% figures,
  pipeline `2724116160`'s five-jobs-in-242ms collision, and the identical
  cache-object suffix `21d01076...` — and its own two advisories (cache
  shadowing after pull-only transition; floating `rust:1` image) are the
  exact two risks this plan's "Versioned shared read key" section and Risks
  list address. This plan is a genuine response to prior review feedback,
  not a restatement.
- **GitLab mechanics**: `cache:policy` accepting a CI/CD variable and
  `workflow:rules` setting pipeline-wide `variables:` on a matched rule are
  both real, documented GitLab behaviors (confirmed via the same official
  URLs !920 cited). The design's core mechanism is sound, not a plausible-
  sounding fabrication.

## Adversarial attempts that did not find a blocking flaw

- **Interaction with the first-push race (#53)**: the writer-context rule
  requires `$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH`; a duplicated first-push
  pipeline under #53 always occurs on a *non*-default branch, so it can never
  spuriously resolve to `pull-push`. The two issues share evidence
  infrastructure but do not interact badly in this design.
- **Manual/conditional jobs**: the plan's own Gate 2 evidence (item 7)
  already requires source-inspecting manual jobs as pull-only and
  trace-checking them if invoked — not assumed away.
- **Tag/release pipelines writing profile-mismatched artifacts into the
  shared cache**: explicitly excluded (`pull` only for tags), which is the
  defensively correct choice given Slice B's own stated concern about
  binding cache keys to build profile.

## ADVISORY-1 — self-test derivation should not mirror the workflow rule's own reasoning

Check: assert-current-independence
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: A1
Impact: minor
Enforcement: advisory
Basis: `always/gate-self-test-rule.md`'s own principle — a gate that shares
its reasoning with what it checks can share its bug
Finding state: open
Risk disposition: none
Resolution condition: `.gitlab/scripts/check-rust-cache-policy.sh`'s
`assert-current` mode should derive its expected-policy logic by literally
transcribing the plan's acceptance-matrix table as enumerated test cases,
not by re-deriving the same boolean condition structure as the
`workflow:rules` entry it is checking. If both the rule and the checker
independently reconstruct "is this the default-branch push" from the same
three variables using the same logical shape, a shared reasoning error in
that shape would satisfy both and the self-test would not catch it. This
does not block Slice A — the plan's required deliberately-wrong negative
cases already catch many implementation bugs — it only strengthens the
independence of the oracle itself.

## Disposition

CLEAR. The plan is technically sound, its evidence is independently
reproducible (I reproduced the consumer count and the cache-block content
myself rather than trusting prose), and it correctly incorporates the prior
independent review's (MR !920) own findings rather than ignoring them. No
objection to Gate 1 approval.
