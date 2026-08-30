---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260829T223950Z-coder-tester-j1ht
timestamp: 2026-08-29T22:39:50Z
model: gpt-5.4
immutable: true
references-to:
  - 20260829T220631Z-coder-tester-gh77.md
  - 20260829T215630Z-coder-tester-ds5b.md
  - 20260829T214241Z-coder-tester-phe4.md
topic: gate-change-discipline-gate1-rereview
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate-change-discipline Gate 1 exact-head re-review

Scope: plan MR !2214 at revised exact source head
`7aea895f48ee37334e726b5f3c18a1ab97c2c6cc`; current target `main` was
re-read at `908d51189f34394bfb683d1638a614ec6766f875`.

Coverage: COMPLETE for `gh77`: GCD-1 through GCD-3 closure, redesigned slice
C, B2's adopter examples and fail-closed base handling, and the projection
manifest lifecycle claim.

Recommendation: **BLOCK Gate 1 pending another revision.** The redesign is
materially better, but the three original blockers are only partly closed and
B2 introduces a new fail-open boundary.

Blocking findings: 4. Advisory findings: 1. Open findings with accepted risk:
0. Authority: recommendation only; the governor-reviewer decides Gate 1.

Exact-head evidence: canonical pipeline `2802455092` passed
`agov:governance:validate` job `16186666456` and
`commercial-corpus:validate` job `16186666457`; MR !2214 has zero unresolved
discussions. `agov plan validate` and `agov plan projection-check
--allow-planned --base-ref origin/main` both pass in an isolated exact-head
worktree. Those establish artifact shape, not the contracts below.

## Closure summary

| Prior finding | Re-review result |
|---|---|
| GCD-1 executable boundary | **Still open.** Core artifacts are named, but required CLI/export, live-registry, distribution, and B-test paths are absent. |
| GCD-2 YAML mapping bypass | **Partly closed.** Hunk mapping is correctly removed and script extraction is sound; literal invocation presence does not protect effective job execution. |
| GCD-3 self-weakening | **Partly closed.** Registry comparison and self-registration improve visibility; retirement and candidate-checker semantics remain contradictory. |
| GCD-4 adopter examples | **Scope closed, correction not yet fail-closed.** All four examples are included and the empty-variable diagnosis is correct, but diff/merge-base failures remain masked. |
| Projection lifecycle | **Mechanically valid at Gate 1.** Omitting not-yet-existing non-owner paths is consistent with the current schema and rule; the distribution claim for `agf/gates.yaml` is nevertheless false under the current release manifest. |

## GCD-R1 — The claimed exact implementation boundary is incomplete

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

The proposed command cannot be implemented only in
`agov-validators/src/gate_change.rs`. A new top-level `agov gate` family also
requires command parsing and dispatch in
`tooling/rust/crates/agov/src/main.rs`, and the validator module must be
exported from `tooling/rust/crates/agov-validators/src/lib.rs`. Neither path
is in the supposedly exact boundary. B's shell negative-test/fixture path is
also unnamed.

The distribution statement has a second concrete gap. The current
`agf/dist/release-manifest.yaml` includes selected root files and directory
globs; it does **not** include a new root `agf/gates.yaml`. Adding that file
does not make it ship. More importantly, a live foundry registry naming
`.gitlab-ci.yml`, `.gitlab/scripts/`, and `tooling/rust/` paths is
project-specific state. Shipping it to adopters repeats the collision already
avoided for `agf/agents/MANIFEST.yaml`, whose live foundry copy is explicitly
excluded while a neutral example ships.

Resolution condition: name every CLI/export/test/distribution path; decide and
document the live-registry boundary; keep foundry-specific entries outside the
adopter distribution; ship a neutral template/example if adopters need one;
and specify the command's registry path option/default and any environment
contract. Update the release manifest and environment-variable/reference docs
if the selected design requires them.

## GCD-R2 — Invocation-string presence does not protect gate execution

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

Extracting logic to `.gitlab/scripts/governance-validate.sh` is a good change,
but assertion 2 proves only that literal text remains in a named YAML block.
The job can retain that string while being neutralized by an earlier `exit 0`,
a false shell guard, `rules: when: never`, `allow_failure: true`, inherited
configuration, or a top-level workflow/default/variable change. None touches
the registered script. The plan therefore has not closed the effective-job
bypass; it has narrowed it to integration semantics while claiming the gate
is "fully covered wherever it lives".

Resolution condition: either register the complete CI integration surface
(accepting the broader test-accompaniment proxy), isolate the complete job in
a dedicated registered CI file and validate its root inclusion/effective
merged configuration, or explicitly narrow C to script-change accompaniment
and leave CI execution integrity to named human review. A literal invocation
check may remain advisory evidence; it should not be described as proof the
gate runs.

## GCD-R3 — Retirement and self-check semantics are not coherent yet

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

Assertion 2 requires every registered job/invocation to exist and assertion 3
requires every registered path to resolve, while retirement keeps the ID with
a `retired:` marker. The plan does not say which active-entry requirements are
waived after retirement. As written, a retired gate cannot remove the job or
surface it retired; if those checks are waived, the schema/checker transition
and negative fixtures are unspecified.

Self-registration also improves visibility rather than making the checker
tamper-proof: the candidate pipeline runs the candidate checker, so one MR can
weaken the checker and touch its declared test path. That is acceptable only
as the explicitly stated human-reviewed proxy. The acceptance text currently
says every listed mutation is "demonstrated failing" while the unrelated-test
touch is simultaneously a known passing limit.

The front matter contradicts the same design: its infosec rationale says the
new check "cannot be satisfied by an empty or absent test", but slice C
explicitly says an empty or unrelated test can satisfy the proxy. Its cited
acceptance criteria 6 and 7 no longer establish that claim after renumbering.

Resolution condition: define active and retired schema states and their exact
path/invocation requirements; test active-to-retired transition, deletion, and
ID disappearance separately; distinguish mutations that must fail from the
known unrelated-test pass; and correct the front-matter consultation claim to
the actual proxy boundary.

## GCD-R4 — B2's proposed fail-closed pattern still masks diff failures

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

B2 correctly finds that an empty
`CI_MERGE_REQUEST_TARGET_BRANCH_SHA` turns `"$EMPTY"...HEAD` into
`HEAD...HEAD` and validates nothing. Including all four adopter sketches is
the right scope correction. However, copying foundry's current
fetch-and-resolve pattern does not fail closed after a target ref resolves.
A shallow history may still have no merge base, and any `git diff` error is
masked by the pipeline's final `xargs` status because the shell uses
`set -eu`, not a reliable pipe-failure contract.

Reproduced in this checkout:

```text
$ sh -c 'set -eu; git diff --name-only -z does-not-resolve...HEAD -- \
    "docs/plans/*.md" | xargs -0r true; echo PIPELINE_EXITED_ZERO'
fatal: bad revision 'does-not-resolve...HEAD'
PIPELINE_EXITED_ZERO
$ echo $?
0
```

The same masking applies to `fatal: no merge base`. The job can therefore
resolve `origin/<target>`, fail to compute the triple-dot diff, and still
report success without validating a file.

Resolution condition: specify and test a pattern that proves a merge base,
captures `git diff` success before invoking `xargs` (for example, write the
NUL list to a temporary file only after a successful diff), and fails closed
for empty target, fetch failure, missing merge base, and diff failure. Apply
that contract to the extracted foundry script and every adopter sketch; add a
shallow/diverged-history fixture, not only an empty-variable transcript.

## Advisory — projection lifecycle

The manifest's omission of future implementation paths is not an evasion by
itself. `stage: planned` is restricted to `normative_owner`, the other surface
rows do not require a path array, and the exact-head relaxed projection check
passes. Adding the real paths at Gate 2 before the strict check is the correct
lifecycle. For semantic clarity, `validation_drift_evidence: implemented`
should be phrased as prospective until the checker exists. GCD-R1 separately
blocks the present claim that `agf/gates.yaml` will ship under the unchanged
release manifest.

## Recommended next revision

Keep A, B1, B2, and script extraction. Split live foundry registry state from
the shipped neutral adopter template; enumerate the actual Rust/CI/test and
distribution paths; make CI-invocation evidence no stronger than its
semantics; define retirement as a state transition; and replace the piped
diff examples with a tested fail-closed changed-file helper. Re-review should
then be bounded to those four closures rather than reopening the accepted
rule-owner and documentation scope.
