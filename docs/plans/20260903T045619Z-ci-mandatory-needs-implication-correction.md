---
id: 20260903T045619Z-ci-mandatory-needs-implication-correction
status: approved
topic: ci-mandatory-needs-implication-correction
candidate_release: v2.9.0
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-03T13:47:09Z
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Recorded as waived, not consulted. No infosec-sentinel review of this
      plan exists, and describing a change as security-relevant does not
      establish that a consultation happened. The first version of this plan
      applied that provenance distinction correctly to contribution-auditor
      below and incorrectly claimed consultation here; Codex raised it as
      CNI-G1-A1. The substance is unchanged and belongs in the body, not in
      a metadata field asserting a review that did not occur: this
      correction restores a security control whose absence is silent,
      because pipeline creation fails before the CVE scan is ever evaluated.
  - role: contribution-auditor
    status: waived
    reason: >-
      No contribution-auditor disposition exists for this plan and the
      governed enum has no pending state. Recorded as waived rather than
      consulted so the metadata does not assert an audit that has not
      happened. This is gate mechanics, so an independent review is
      requested regardless; the governor may hold Gate 1 for an auditor
      pass.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CI mandatory-needs implication correction

Discharges the blocker `tk2n` orders ahead of everything else: A1's container
CVE scan cannot fire on a Containerfile-only change, so the v2.9.0 release
path is stopped.

**This is a gate change.** It is ineligible for the fast-track waiver
regardless of diff size, per `simplicity-rule.md`, and it ships with the
negative test `always/gate-self-test-rule.md` requires for a modification —
demonstrated firing on known-bad input after the change, not before.

## The defect class

The invariant that `.gitlab-ci.yml` currently violates:

> In every pipeline source and changed-path class where consumer `C` is
> included, every non-optional producer `P` in `C.needs` must also be
> included.

When it is violated, GitLab rejects the pipeline **at graph creation**, before
any `rules:` block is evaluated and before any job runs. The observable
symptom is a pipeline that does not exist — which is indistinguishable, from
the outside, from a pipeline that ran and passed. That is why the gap survived:
nothing failed.

Twelve explicit mandatory `needs:` edges across six consumers were audited at
`origin/main` `86e92ad9` (`s5t7`, corrected from eleven to twelve by `mmms`).
Four consumers are coherent — `release:create`, `release:verify`,
`release:assets-verify-pre-oci`, and `agov:oci:release` are all tag-only on
both sides. Two fail.

### NRI-1 — `agov:oci:mr-build-scan` outruns `rust:tooling:preview`

The scan is eligible in merge-request pipelines, the first-push
non-default-branch fallback, and later branch contexts with matching changes.
Its producer `rust:tooling:preview` is eligible only for
`merge_request_event` or the default branch. Every non-default-branch push
context therefore includes the consumer and excludes the producer.

Already reproduced: `rhqb` records eight API-created non-default-branch
pipelines rejected for exactly this.

### NRI-2 — `release:assets-verify-preview` outruns `agf:dist:preview`

The verifier's path set is `tooling/rust/**/*` plus `.gitlab-ci.yml`.
`agf:dist:preview`'s is `agf/**/*`, `.gitlab-ci.yml`, two review documents,
and the single file `tooling/rust/scripts/check-release-self-sufficiency.sh`.
A change to `tooling/rust/Containerfile.agov` alone includes the verifier and
excludes the AGF producer.

Already reproduced: MR !2313's three attempted pipelines, all rejected at
graph creation on that exact single path (`k6m3`).

## A finding the graph analysis surfaced, and which outlives it

`agf:dist:preview` builds its artifact with `cargo run --quiet -p agov --
dist validate` and `dist build` (`.gitlab-ci.yml:423-424`). **Its output is
produced by the Rust binary, yet its path anchor excludes `tooling/rust/**/*`
except one script.**

So a change to `agov`'s own `dist build` implementation changes the shipped
AGF archive while leaving the job that validates that archive unrun. This is
not a consequence of the `needs:` mismatch — it is an independent,
pre-existing coverage gap in the same file, of exactly the shape A1 itself
exists to fix: an incomplete anchor silently produces no coverage, as
`045e514c` already demonstrated once.

It matters for the correction because it settles a choice that would otherwise
be a judgment call. Adding `tooling/rust/**/*` to `agf:dist:preview` is not a
graph-parity hack that happens to fix NRI-2 — it is the semantically correct
anchor for that job on its own terms, and fixing NRI-2 is a side effect of
correcting it.

## Correction: producer parity, not `optional: true`

`needs: optional: true` is rejected for both defects, and the reason is
semantic rather than stylistic. Both consumers declare `artifacts: true` and
consume the producer's artifact — `agov-oci-image.sh build-unpublished`
requires the packaged Rust archive; `release-assets.sh verify-pre-oci`
requires the complete AGF **and** Rust artifact set. An optional missing
producer would let the graph build and then fail on absent inputs, converting
a loud creation error into a quieter runtime one. A lint returning
`valid: true` would prove the graph coheres, not that the consumer functions.

### NRI-1 source matrix

The consumer's fourth rule is `if: $CI_COMMIT_BRANCH` with no source guard and
no `compare_to` (`.gitlab-ci.yml:706-707`). GitLab evaluates `rules:changes`
without `compare_to` as **true** when there is no Git push event
(<https://docs.gitlab.com/ci/yaml/#ruleschanges>), so that rule silently
includes the consumer in every no-push branch context. The first version of
this plan described these only as "later branch contexts with matching
changes", which was wrong. Codex raised it as CNI-G1-2.

C = `agov:oci:mr-build-scan`, P = `rust:tooling:preview`. "Changed paths from"
states what supplies the `changes:` comparison.

| Pipeline source | Branch | C today | P today | Changed paths from | Result today |
|---|---|---|---|---|---|
| `merge_request_event` | any | in (r1) | in | MR diff | coherent |
| `push`, first push (`BEFORE_SHA` all-zero) | non-default | in (r2) | **out** | `compare_to` default branch | **FAIL** |
| `push`, first push, no matching change | non-default | out (r3 `never`) | out | — | coherent |
| `push`, later | non-default | in (r4) | **out** | previous commit | **FAIL** |
| `push` | default | in (r4) | in | previous commit | coherent |
| `schedule` | default | in (r4) | in | **nothing — evaluates true** | coherent, but unintended |
| `schedule` | non-default | in (r4) | **out** | **nothing — evaluates true** | **FAIL** |
| `web`, `api`, `trigger`, `pipeline`, `parent_pipeline` | default | in (r4) | in | **nothing — evaluates true** | coherent, but unintended |
| `web`, `api`, `trigger`, `pipeline`, `parent_pipeline` | non-default | in (r4) | **out** | **nothing — evaluates true** | **FAIL** |
| tag | — | out (no tag rule) | out | — | coherent; `agov:oci:release` scans instead |

### Selected NRI-1 design: narrow the consumer, then align the producer

Both halves are chosen here, at Gate 1, because they decide security coverage
and runner cost.

1. **Add an explicit `push` source guard to the consumer's fourth rule**, so
   it no longer matches `schedule`, `web`, `api`, `trigger`, `pipeline`, or
   `parent_pipeline`.
2. **Extend `rust:tooling:preview` to the non-default-branch `push` context**
   the consumer legitimately needs, with the same `compare_to` the consumer's
   first-push rule already uses.

**Why narrowing rather than widening.** The mandatory CVE coverage is
merge-request pipelines and default-branch pushes, because every path to a tag
passes through one of them; the first-push rule exists only to cover the
MR-registration race. A scheduled or web-triggered pipeline on a feature
branch is not on the release path, and a `changes:` clause that is
unconditionally true there would schedule an 8.5-minute Rust build plus a
container build on every such run for no security gain.

**This gives up no observed coverage.** Across the 300 most recent pipelines
(2026-08-29 to 2026-09-03) the only sources present are
`merge_request_event` (154) and `push` (146). **Zero** scheduled, web, API,
trigger, pipeline, or parent-pipeline runs exist in that window, so the
narrowed contexts are currently unexercised. If that changes, the guard is one
line to revisit — and it would then be a deliberate decision with a stated
security reason, which is exactly what the current source-agnostic rule
prevents.

### NRI-2 correction

Add `tooling/rust/**/*` to `agf:dist:preview`'s `changes:` in both its
merge-request and default-branch rules. Justified on that job's own terms by
the anchor finding above, not merely as graph parity.

### Measured cost, and the stop condition

Medians from the 400 most recent successful jobs, read from the jobs API — not
estimated:

| Job | n | Median | Max |
|---|---|---|---|
| `rust:tooling:preview` | 5 | 510s | 513s |
| `agf:dist:preview` | 9 | 510s | 541s |
| `agov:oci:mr-build-scan` | 3 | 48s | 48s |
| `release:assets-verify-preview` | 5 | 12s | 13s |

NRI-2 dominates: `agf:dist:preview` newly runs on every merge-request and
default-branch pipeline touching `tooling/rust/**/*`. In the sampled ~5-day
window, five pipelines ran `rust:tooling:preview`, so the added load is
approximately 5 x 510s ≈ 42 minutes per 5 days, or **under 10 minutes of
additional runner time per day at observed frequency**.

NRI-1 adds `rust:tooling:preview` (510s) plus the scan (48s) on
non-default-branch pushes that touch the scan's paths and have no open merge
request — the workflow dedup rule already suppresses branch pushes once an MR
exists. 31 of the 300 sampled pipelines were non-default-branch pushes, and
only those touching `.gitlab-ci.yml`, `agf/VERSION`, or `tooling/rust/**/*`
qualify. These contexts were already *attempting* the scan and failing at
graph creation, so this converts a broken pipeline into a working one rather
than adding a new trigger class.

**Pre-approved ceiling.** 30 minutes of additional runner time per day,
averaged over any rolling 7-day window — roughly three times the estimate.
**Stop condition:** if the measured addition exceeds that ceiling, stop and
return to Gate 1 to split `release:assets-verify-preview` into independently
meaningful AGF and Rust artifact checks, rather than silently absorbing the
cost or quietly dropping full-set proof.

## Zero-cost preflight

`POST /ci/lint` accepts `content` plus `ref`, so a candidate configuration is
evaluated in a chosen branch context **before it is written to disk,
committed, or pushed**, consuming no runner time (`ersd`, confirming and
extending `b8ja`'s GET form):

```sh
glab api --method POST projects/83321848/ci/lint \
  -f content="$(cat .gitlab-ci.yml)" \
  -f dry_run=true \
  -f ref=<branch>
```

For a defect whose entire symptom is pipeline creation failing, creation-time
linting observes it directly. Verified today: current content at
`ref=docs/site-adoption-rust-tooling` returns the NRI-1 error with `jobs: 0`;
the same content at `ref=main` returns `valid: true` with 24 jobs.

This is the preflight, not the acceptance evidence. It cannot discharge the
semantic question above, and it cannot reproduce merge-request context.

## Gate registry ownership

The first version of this plan listed its affected files as `.gitlab-ci.yml`
plus one new test, and that was wrong. `.agf/gates.yaml:3-16` registers
`.gitlab-ci.yml` as an active `gate-change-check` surface whose **declared**
tests are only `tooling/rust/crates/agov/tests/gate_change_check.rs` and
`tooling/rust/crates/agov/tests/fixtures/gate-change/**`. The checker at
`tooling/rust/crates/agov-validators/src/gate_change.rs:120-135` raises a
blocking finding when an active surface changes and no changed path matches
that gate's declared tests. A new test at
`tooling/rust/crates/agov/tests/ci_needs_implication.rs` matches neither
pattern, so **the implementation this plan describes would have been blocked
by the project's own gate-change discipline.** Codex raised it as CNI-G1-1;
it is a real defect in the plan, not a technicality.

### The design, and the option rejected

`.agf/gates.yaml` gains a **second gate entry**:

```yaml
  - id: ci-needs-implication
    status: active
    surfaces:
      - .agf/gates.yaml
      - .gitlab-ci.yml
    tests:
      - tooling/rust/crates/agov/tests/ci_needs_implication.rs
      - tooling/rust/crates/agov/tests/fixtures/ci-needs/**
```

**Rejected: adding `ci_needs_implication.rs` to the existing
`gate-change-check` entry's test list.** It is the smaller diff and it would
satisfy the checker, but it weakens an existing gate. Accompaniment is
satisfied by *any* declared test changing, so a future change to
`gate_change.rs` could be accompanied by an edit to `ci_needs_implication.rs`
— which proves nothing about gate-change-check. Making one gate's diff smaller
by widening what counts as proof for another gate is the exact shape
`gate-self-test-rule.md` exists to prevent, and it is indistinguishable at
diff level from a deliberate weakening.

### Verified by execution, and it corrected the file set

The claim that the original plan would be blocked was an inference, made by
me and repeated by Codex. Neither of us had run it, so I ran all three cases
locally against `origin/main` — read-only, no pipeline:

| Simulation | Changed paths | `agov gate change-check` |
|---|---|---|
| **A** — plan as first written | `.gitlab-ci.yml`, `ci_needs_implication.rs` | **exit 1** — `active gate 'gate-change-check' has a changed surface but no changed declared test path` |
| **B** — plus the new registry entry and its fixtures | A + `.agf/gates.yaml`, `fixtures/ci-needs/**` | **exit 1** — same finding |
| **C** — plus a real `fixtures/gate-change/**` case | B + `fixtures/gate-change/probe-multi-gate.yaml` | **exit 0** — `OK gate change-check .agf/gates.yaml` |

Simulation A confirms the inference. **Simulation B falsifies the first
version of this section**, which implied the second registry entry was
sufficient. It is not: adding the entry *changes `.agf/gates.yaml`*, which is
itself a `gate-change-check` surface, so that gate demands one of **its own**
declared tests change too.

The corrected implementation file set is therefore five paths, not four:

1. `.gitlab-ci.yml` — the CI correction
2. `.agf/gates.yaml` — the new registry entry
3. `tooling/rust/crates/agov/tests/ci_needs_implication.rs` — the new invariant
4. `tooling/rust/crates/agov/tests/fixtures/ci-needs/**` — its fixtures
5. `tooling/rust/crates/agov/tests/fixtures/gate-change/**` — a real case
   covering a **two-gate registry that shares a surface**

Item 5 is not the token edit Codex's correction warns against. A registry
that has only ever held one gate is now growing a second that shares
`.gitlab-ci.yml`, and `gate-change-check` has no existing case for that shape.
Writing one is exactly the accompaniment the rule is asking for. Had this
been discovered at implementation instead, the tempting move would have been
to touch an existing gate-change fixture to make the check go green — the
precise failure mode Codex named.

**The disclosed cost.** Accompaniment is evaluated per gate, so with two
active gates naming `.gitlab-ci.yml`, every future edit to that file must
change a declared test of *both*. That is a real ongoing burden and it is
stated rather than discovered later. The `fixtures/ci-needs/**` glob is what
keeps it honest: a substantive CI change lands a fixture describing the new
job shape, so the accompaniment is evidence rather than a token edit. If the
governor judges the doubled burden unacceptable, the alternative is to merge
the two entries and accept the weakening above explicitly — that is a
governor decision, not one to make quietly inside an implementation.

## The negative test

Per `gate-self-test-rule.md`, the gate must be shown firing on known-bad input
**after** the change. Reusing the pre-change reproductions is explicitly
forbidden by that rule.

A deterministic checker asserts `consumer inclusion => producer inclusion`
across all twelve mandatory edges, bound to the finite source matrix above
rather than to an open-ended "supported pipeline sources". It lives at
`tooling/rust/crates/agov/tests/ci_needs_implication.rs`, following the
existing `roi_redaction_audit_matches_real_clap_surface` precedent of
asserting against the real parsed artifact rather than a copy. No new tool,
script convention, or CI job type is introduced.

Four transcripts, all visible in the MR description:

| # | Input | Required result |
|---|---|---|
| 1 | Revert `rust:tooling:preview`'s non-default-branch push coverage | Checker **fails**, naming the NRI-1 edge |
| 2 | Remove `tooling/rust/**/*` from `agf:dist:preview` | Checker **fails**, naming the NRI-2 edge |
| 3 | Restore the consumer's source-agnostic fourth rule (drop the `push` guard) | Checker **fails** on a no-push context, naming `schedule` |
| 4 | Reorder two entries within a `changes:` list, changing no semantics | Checker **passes** |

Transcript 3 is new: it binds the checker to the source matrix, so a future
edit that reintroduces the no-push hazard is caught rather than merely
described in prose.

Transcript 4 replaces the first version's fourth case, which called itself a
mutation and then said the coherent edge was "left alone" — ambiguous, and
already covered by the unmutated tree passing. Codex raised it as CNI-G1-A2.
A semantics-preserving reordering is a real mutation with a reproducible
expected result, and it proves the checker discriminates rather than failing
on any CI edit at all.

## Acceptance for A1

Graph correction alone does not discharge A1. Its acceptance criterion 1
requires a fresh vulnerable-base merge request that:

1. **constructs a valid pipeline graph** — proving this correction worked; and
2. **fails specifically at the Trivy gate** — proving the scan ran and caught
   a real CVE.

A pipeline that fails at creation proves nothing about the scan. The prior
attempt reverted `Containerfile.agov` to `alpine:3.22.1` for this purpose and
never produced a pipeline at all.

## Cost

The incremental runner cost of NRI-1's broader `rust:tooling:preview`
eligibility and NRI-2's broader `agf:dist:preview` eligibility is measured and
disclosed before Gate 2, not estimated at Gate 1. CI quota is currently
constrained, which is a further reason the preflight loop above carries the
iteration and paid pipelines are spent only on acceptance.

`ci-trigger-scoping-rule.md` cuts toward inclusion here: a lockfile or build
input wrongly scoped out costs a missed regression, and its own guidance is
"when in doubt, include the path."

## Scope boundary

Touches `.gitlab-ci.yml`, `.agf/gates.yaml`, and
`tooling/rust/crates/agov/tests/fixtures/gate-change/**`, and adds one test
file plus its fixture directory — the five-path set verified as simulation C
above. Does not touch
`tooling/rust/scripts/agov-oci-image.sh`, `refs/tags/v2.8.0`, or
`release-2.9.0`, all of which `tk2n` places off limits.

`tk2n` also lists `.gitlab-ci.yml` as do-not-touch. That list scopes the
**queue-convergence plan** `tk2n` commissioned; the same handoff orders the
CI-rule correction completed first, which is not possible without editing that
file. This plan reads the restriction as plan-scoped rather than global. If
that reading is wrong, this plan stops at Gate 1.

## Test and validation strategy

1. `agov plan validate` on this plan — passing at submission.
2. **`agov gate change-check --base-ref origin/main` — passing on the
   implementation MR.** No longer an assumption: simulations A, B and C above
   were executed locally, and C is the exact five-path set the implementation
   must reproduce. A run that passes with fewer than those five paths means
   the registry changed underneath this plan and it returns to Gate 1.
3. CI Lint preflight for every candidate, in both MR and branch contexts,
   with no pipeline created.
4. The four negative-test transcripts above.
5. One canonical exact-head pipeline on the correction MR.
6. The vulnerable-base MR discharging A1 acceptance criterion 1.
7. Measured cost against the pre-approved ceiling, recorded in the MR
   description — verifying the Gate 1 estimate, not making a first design
   decision.

## Risks

- **Broader producer rules raise CI cost while quota is constrained.**
  Bounded by the measured estimate and the pre-approved 30-minute daily
  ceiling above, with a named stop condition rather than an open-ended
  acceptance. The zero-cost preflight absorbs iteration so paid pipelines are
  spent only on acceptance.
- **The checker encodes today's twelve edges and silently rots.** Mitigated by
  asserting against the real parsed `.gitlab-ci.yml`, so a thirteenth edge is
  seen rather than missed. Transcript 4 guards the opposite failure.
- **Two active gates now name `.gitlab-ci.yml`, doubling accompaniment cost.**
  Disclosed above rather than discovered later. The `fixtures/ci-needs/**`
  glob channels the burden into substantive fixture cases; if it still proves
  too heavy, merging the entries is a governor decision with a stated
  weakening, not a quiet implementation choice.
- **The narrowed source guard removes coverage that is currently unexercised
  but might matter later.** Zero scheduled, web, or API pipelines exist in the
  300-pipeline sample, so nothing observed is lost — but "unexercised today"
  is not "never needed". Reversing the guard is one line and would then be a
  deliberate decision with a security reason attached, which the present
  source-agnostic rule does not require of anyone.
- **Correcting `agf:dist:preview`'s anchor surfaces pre-existing failures.**
  Possible, and would be a real finding rather than a regression — the job has
  not been running on Rust changes that affect its output. Any such failure
  returns to Gate 1 rather than being absorbed here.

## Rollback

Revert all five paths together: the `.gitlab-ci.yml` rules changes, the
`.agf/gates.yaml` registry entry, the new test and its fixtures, and the added
`fixtures/gate-change/**` case. Reverting the
registry entry and the CI change together keeps `agov gate change-check`
coherent at every point in the rollback; reverting only one would leave an
active gate naming a surface with no declared test present. No schema, tag,
release artifact, or external state is mutated.

## Evidence

- `docs/coordination/20260902T090221Z-planner-architect-at7q.md` — A1 scan does not fire on a Containerfile change
- `docs/coordination/20260902T181632Z-coder-tester-k6m3.md` — pipeline-creation root cause, MR !2313 transcripts
- `docs/coordination/20260902T182111Z-planner-architect-m2nh.md` — mechanism confirmed, and my own overclaim corrected
- `docs/coordination/20260902T182347Z-coder-tester-s5t7.md` — full twelve-edge audit, NRI-1 and NRI-2
- `docs/coordination/20260902T191658Z-coder-tester-mmms.md` — edge-count erratum
- `docs/coordination/20260902T192529Z-coder-tester-b8ja.md` — no-runner preflight and acceptance shape
- `docs/coordination/20260902T193228Z-planner-architect-ersd.md` — POST /ci/lint content+ref, verified
- `docs/coordination/20260903T040339Z-coder-tester-tk2n.md` — ordering constraint
- `docs/coordination/20260903T053215Z-coder-tester-kjs7.md` — Codex's Gate 1 review; CNI-G1-1, CNI-G1-2, CNI-G1-A1, CNI-G1-A2 are all applied in this revision
- `.gitlab-ci.yml:418-449, 665-700, 706-707, 715-736, 1453-1478` — the four jobs and the source-agnostic consumer rule, read at `013c4a04`
- `.agf/gates.yaml:3-16` and `tooling/rust/crates/agov-validators/src/gate_change.rs:120-135` — the active registry entry and the accompaniment check the first version would have failed
- GitLab jobs and pipelines API, read 2026-09-03 — job medians over the 400 most recent successful jobs; pipeline-source frequency over the 300 most recent pipelines (2026-08-29 to 2026-09-03)
