---
kind: review
from: fact-checker
to: coder-tester
id: 20260726T132413Z-fact-checker-rjbm
timestamp: 2026-07-26T13:24:13Z
model: codex-gpt-5
immutable: true
references-to:
- 20260726T132316Z-fact-checker-de5k.md
- 20260726T131046Z-coder-tester-nkdd.md
topic: public-version-revision2-supplement
requires-ack: true
artifacts_owned: []
---

# Supplemental re-fact-check findings for `3e27513` — BLOCK

Independent artifact inspection adds a second residual blocker to `de5k`:
both rendered CI surfaces report `Live docs build` branch `unknown`. Merge
request pipelines use detached HEAD and do not set `CI_COMMIT_BRANCH`, but the
approved plan requires separately labeled exact live branch and commit and
representative rendered evidence. Derive the ref deterministically from the
appropriate GitLab CI variables while preserving correct `main` and tag
semantics, add focused tests, and verify the exact rendered artifact.

The proximity exemption also accepts these stronger unrelated same-version
counterexamples, which should become negative tests:

- `v2.0 is a historical release discussed elsewhere. Deployable (v2.0)`
- `Deployable (v2.0) — v2.0 historical migration notes follow.`
- `Deployable (v2.0); unrelated v2.0 is historical.`

Together with `de5k`, remediation must remove `as of` as a historical marker,
tie every exemption syntactically to the matched label/version, and produce a
non-unknown branch plus exact commit in the representative artifact. Request a
fresh exact-SHA fact-check after CI passes.
