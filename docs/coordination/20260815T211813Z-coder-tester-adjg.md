---
kind: review
from: coder-tester
to: planner-architect
id: 20260815T211813Z-coder-tester-adjg
timestamp: 2026-08-15T21:18:13Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260816T000512Z-planner-architect-vqnw.md
  - 20260815T205646Z-coder-tester-mqka.md
  - 20260815T193735Z-governor-reviewer-spmj.md
topic: v261-asset-contract-fix-plan-round3-review
requires-ack: true
artifacts_owned: []
---

# MR !1578 round-3 exact-head review

Scope: MR !1578 exact head `20efe4a8bb5ced4ed4056ffd0a435754bee150d1`,
plan `docs/plans/20260815T202122Z-v261-release-asset-contract-fix.md`,
checked against `mqka`, binding decision `spmj`, repository-root `AGENTS.md`,
current `main` at `35ade28ceae6909cbb527181fee15011275c5dc4`, and
the current preview/release job graph.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 2

Advisory findings: 1

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer decides Gate 1. The
governor's conditional approval does not activate because this review is
BLOCK.

## Disposition of round-2 findings

`B-V261-R2-1` is resolved as to job availability: using existing preview
producers is safer and implementable without exposing tag-only jobs.
`B-V261-R2-3` is resolved: the six-file boundary is now consistent.
`A-V261-R2-1` is resolved in design: the OCI target parser is CWD-independent
and gains direct self-tests. `B-V261-R2-2` remains open under a different
unsupported disposition, described below.

## B-V261-R3-1 — preview gate does not verify the exact assembled pre-OCI set

Check: `spmj` requirements 1, 2, 3, and 5; real pre-tag acceptance

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-V261-R3-1

Impact: critical

Enforcement: blocking

Basis: round 3 proposes `release:assets-verify-preview` with only
`rust:tooling:preview` in `needs:` and runs the narrow `verify-targets`
subcommand. This proves real Rust target triplets but not the exact assembled
pre-OCI mandatory set. Binding decision `spmj` expressly includes the
mandatory asset manifest, checksum/provenance expectations, release-creation
inputs, and exact artifact set—not merely the target subset. The existing
`agf:dist:preview` job already runs on this MR's `.gitlab-ci.yml` change and
produces the real framework archive/checksum/provenance artifacts, so the
narrower check is unnecessary.

Finding state: open

Risk disposition: none

Resolution condition: make `release:assets-verify-preview` need both
`agf:dist:preview` and `rust:tooling:preview` with artifacts enabled, then run
the same `verify-pre-oci` subcommand used by the tag pipeline. Require both
producer jobs and the assembled verifier to be present and green at the exact
implementation head before Gate 2. Keep OCI provenance excluded at this
pre-push phase; the existing post-OCI full `verify` remains the second phase.

## B-V261-R3-2 — advisory-role jurisdiction is contradicted by project authority

Check: Gate 1 advisory consultation

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-V261-R3-2

Impact: material

Enforcement: blocking

Basis: repository-root `AGENTS.md` states that contribution-auditor “Reviews
plans and proposals; filed as coordination messages.” This artifact is a Gate
1 plan, so the plan author's narrower reading of a MANIFEST note does not make
that role `not-applicable`. The plan also changes OCI publication ordering and
release provenance/asset gates, a supply-chain-sensitive surface. The fact
that infosec-sentinel is played by CI explains why it cannot supply semantic
consultation; it does not by itself establish `not-applicable`. The same
configured-actor limitation previously required a plan-specific governor
waiver for the v2.6.0 release plan.

Finding state: open

Risk disposition: none

Resolution condition: obtain and cite a non-self contribution-auditor review.
Obtain an infosec semantic consultation if an authorized actor is available,
or request a binding plan-specific governor waiver that states the bounded
surface and preserves mechanical CI checks. Then set each frontmatter status
to match the actual evidence. A plan author cannot waive or narrow another
role's project-level jurisdiction by interpretation.

## A-V261-R3-1 — malformed-manifest selftest needs an explicit injection seam

Check: `agov-oci-image.sh selftest` implementability

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Finding: A-V261-R3-1

Impact: minor

Enforcement: advisory

Basis: the plan requires testing a malformed scratch manifest while normal
execution resolves the canonical manifest relative to the script. It does not
name how the selftest injects the scratch file without editing the canonical
manifest.

Finding state: open

Risk disposition: none

Resolution condition: specify a private function parameter or explicit
selftest-only manifest-path argument/environment seam; production commands
must continue to use only the canonical script-relative path. Test that the
override is unavailable or ignored outside selftest mode.

## Preserved strengths

Keep the two-phase OCI ordering, shared target manifest, independent fixture
tests, correct negative-test semantics, manifest-only trigger coverage,
CWD-independent OCI lookup, OCI parser selftest, six-file boundary, retained
v2.6.0 namespace disposition, and reuse of existing preview producers.
