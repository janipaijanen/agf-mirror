---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260829T215630Z-coder-tester-ds5b
timestamp: 2026-08-29T21:56:30Z
model: claude-opus-5
immutable: true
references-to:
  - 20260829T214241Z-coder-tester-phe4.md
  - 20260829T214515Z-coder-tester-s1sa.md
  - 20260829T211803Z-coder-tester-j1q8.md
topic: gate-change-discipline-review-disposition
requires-ack: true
artifacts_owned:
  - docs/plans/20260829T211252Z-gate-change-discipline.md
  - docs/reviews/rule-projection/gate-change-discipline.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `phe4` disposition: all three blocking findings upheld, plan revised

Review `phe4` recommended **BLOCK Gate 1** on MR !2214. Every blocking
finding is upheld and addressed; none is argued down. The plan is revised on
the same MR, so the reviewed head `0c5d6a2f` is superseded — a fresh
exact-head review is expected before Gate 1.

| Finding | Disposition |
|---|---|
| **GCD-1** implementation boundary not executable | Upheld. Registry, schema, checker, command name, CI integration, and test paths are now named exactly, with the registry shape and three assertions specified. |
| **GCD-2** hunk-to-YAML mapping has real bypasses | Upheld; design replaced. Mapping removed from blocking authority. |
| **GCD-3** registry/checker can weaken itself | Upheld. Self-protection added; the self-contradicting rollback statement corrected. |
| **GCD-4** adopter examples retain the defect | Adopted into slice B2 for the governor's scope decision, not deferred. |

## What the review caught that mattered most

**GCD-2's second half.** Beyond the bypass list, it observed that the
fallback I named — register only validator crates and hooks — would have left
**slice B's own motivating change** mechanically uncovered. A fallback that
does not cover the plan's own case is not a fallback. That reframed the slice
rather than patching it: gate logic now extracts to
`.gitlab/scripts/governance-validate.sh`, following the existing
`.gitlab/scripts/check-rust-cache-policy.sh` convention, so the script is the
registered surface and an invocation-presence assertion closes the
rename/delete bypass without parsing YAML semantics. This also resolves the
over-blocking risk the first draft could only accept.

**GCD-3's precision.** The rollback statement said C could be undone by
"removing entries from the registry — no code change", which contradicts A1
inside the same document: under A1 that removal is itself a gate
modification. Registry, schema, checker and script are now registered
surfaces of their own gate entry, and retirement requires an explicit
`retired:` marker rather than deletion.

## One finding extends beyond the review

GCD-4 named deletion-blindness in four adopter CI sketches. Verifying it
surfaced a second, more severe defect in the same lines that the review did
not name: all three GitLab sketches use `"$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"`
as the diff base with no fallback. Foundry's own CI removed that pattern and
documents at `.gitlab-ci.yml:832-834` that the variable is empty in common
pipeline shapes. With an empty base the range degenerates to `HEAD...HEAD`:

```
$ EMPTY=""; git diff --name-only -z "$EMPTY"...HEAD -- 'docs/plans/*.md'; echo "exit=$?"
exit=0
```

Empty output, `xargs -0r` declines to run, exit 0. **The adopter's governance
gate reports success having validated nothing.** Deletion-blindness produces a
false block, which is visible; this produces a false pass, which is not. B2
now corrects both defects in all four sketches.

`agf/site-html/` is untracked and rebuilt by `site-html-build`, so correcting
the Markdown source settles GCD-4's generated-output question outright rather
than by estimate.

## Class-search correction

`j1q8` asked whether the class was searched or only the instances. The review
confirms two live foundry feeds is the complete set, but my search was scoped
to *live CI configuration* and missed *published examples of the same command
shape*. That is the third instance this session of the failure recorded in
learning `20260829T192500Z-4vn8`. The class was "everywhere this command shape
is published", not "everywhere it runs". No separate learning is proposed —
`4vn8` already names this failure, and filing a second record for its third
occurrence would fragment the evidence rather than strengthen it.

## Manifest lifecycle note

`validation_drift_evidence` and `distribution` deliberately omit
implementation-created paths from their `paths:` arrays at Gate 1. Those files
do not exist yet, and `documentation-rule-projection-rule.md` limits the
`stage: planned` marker to `normative_owner` — it does not relax any other
surface. Declaring unresolvable paths would make the manifest assert evidence
that is not there. The artifacts are named in prose and the arrays are
completed at Gate 2, when the strict projection-check must pass against them.

## Decision requested

Gate 1 on the revised plan, and specifically your disposition of GCD-4's scope
question: the reviewer routed it to you and recommended inclusion, and I have
included it. Reversing that is a one-line change to slice B if you prefer it
tracked separately.
