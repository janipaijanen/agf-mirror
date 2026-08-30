---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260830T125924Z-coder-tester-gme3
timestamp: 2026-08-30T12:59:24Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260830T122902Z-coder-tester-zc4m.md
  - 20260830T120843Z-coder-tester-hna4.md
  - 20260830T113003Z-coder-tester-2shr.md
topic: mr2237-gate-change-discipline-ab-fifth-gate2-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2237: fifth exact-head Gate 2 review

Scope: MR !2237 at exact source head
`81346b25e7e3d9bfab1ccf25dfdf43b6e3e79a6a`, against current target `main`
`ef7fc0db63b68cc831acffbedf93c8d3f05e1a0a`.

Coverage: COMPLETE for handoff `zc4m`. Recommendation: **BLOCK Gate 2**.
The positive fixture-specific rejection control closes GCD-G2-4, but the
requested attack produced a fifth false-green instance in section 10's
meta-test of that control. Blocking findings: 1. Authority: recommendation
only; the governor-reviewer decides Gate 2.

The exact source head is mergeable into the current target and has zero
unresolved discussions. Canonical MR pipeline `2803284444` passed at this
head: all 21 automatic jobs succeeded and the two Windows preview jobs remain
allowed manual jobs. Required job `governance:changed-files:selftest`
`16190520465` reported 27 passed / 0 failed. The transcript accurately covers
the current hard-coded command, but its meta-test does not prove the failure
mode it claims to prove.

## GCD-G2-4: the production assertion helper is closed

`rejects_with` now passes only when its command exits non-zero and its output
contains the specific governed-content finding the fixture must provoke.
All four add/modify assertions require either:

```text
#/kind: "not-a-real-kind"
#/status: "not-a-valid-status"
```

This correctly rejects the feeder failure constructed by `hna4`; adding more
infrastructure strings is no longer the control strategy. The two positive
validator canaries and the schema-context default remain sound.

The expected-finding strings are an acceptable maintenance trade. A deliberate
validator wording change will fail closed and require the fixture assertion to
be updated with the validator change. The current CLI exposes no structured
validation result, and these pointer-plus-invalid-value fragments are more
specific than a generic `ERR severity: blocking` marker. This is advisory
coupling, not a Gate 2 blocker.

## GCD-G2-5: the meta-test accepts a missing command as correct refusal

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

Section 10 stores the intended broken-feeder command in a quoted string and
tests it through `eval`. Its first two assertions infer their intended reasons
from bare non-zero exits:

1. any non-zero `eval "$broken_feeder"` is reported as proof that the feeder
   failed before the validator; and
2. any non-zero `eval "rejects_with ... $broken_feeder"` is reported as proof
   that the rejection control refused that feeder failure.

If `broken_feeder` is malformed or names a missing command, both assertions
still pass. The third genuine-content assertion is independent and continues
to pass, so all three meta-tests remain green even though the intended feeder
path never ran.

Mutation reproduction using the exact section-10 control flow with
`broken_feeder=command-that-does-not-exist`:

```text
PASS meta 1 accepts missing command as broken feeder
PASS meta 2 accepts missing command as control refusal
PASS meta 3 still accepts a genuine rejection
mutant_result=3_passed_0_failed
```

This is the same class for a fifth time: the expected non-zero outcome is
reached for a different reason, now inside the test of the test helper.

Resolution condition:

- replace the command string and `eval` with a shell function invoked directly;
- positively assert that the fixture command's captured output contains the
  intended `cf_changed_files` diagnostic for base `does-not-resolve`, not just
  that it exits non-zero;
- when `rejects_with` refuses it, assert the helper's recorded output/reason
  identifies that same feeder diagnostic and absence of the expected schema
  finding; a missing function, syntax error, or unrelated failure must fail
  the meta-test; and
- retain the positive genuine-content assertion.

Add a mutation-style check or equivalent setup assertion proving a missing or
malformed fixture command cannot satisfy section 10. Do not repair this by
adding another error-string blocklist to the production rejection helper.

Other exact-head evidence passed: direct-binary suite 27/0 with caller schema
unset, deliberately broken schema 19/8, shell syntax, `git diff --check`,
projection validation without planned-owner tolerance, clean current-target
merge-tree, and the exact canonical pipeline/job inventory above.

Learning `20260830T114500Z-tw3s` must remain held and must not claim the
control-test recommendation implemented until section 10 proves its intended
fixture path positively.
