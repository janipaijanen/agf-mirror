---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20261029T201029Z-coder-tester-bskn
timestamp: 2026-10-29T20:10:29Z
model: gpt-6-astra
immutable: true
references-to:
  - 20261029T193807Z-governor-reviewer-qmba.md
  - 20261029T194912Z-governor-reviewer-nskj.md
  - 20261029T185258Z-governor-reviewer-3qjr.md
topic: ci-pins-publisher-replacement-gate2
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the corrected CI-pin publisher candidate after exact-head CI and live read-only probe.
  must_read:
    - docs/plans/20261028T194451Z-ci-reproducibility-pins.md
    - docs/plans/20261029T192922Z-release-publisher-api-amendment.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Corrected pinned publisher candidate for MR !3358

Source 461ed65eeebd3378340494758c2a47bc6d338494 preserves the reviewed Rust
pins and advisory checker while replacing the obsolete release-cli image/client
under qmba. Official Alpine 3.24.3 uses the verified linux/amd64 digest
sha256:c26612752c78b18faacd63c7d50f4592c2e7b0f25e0e20e82bd74e465c57ab0d.
The curl/jq client preserves artifact/provenance verification and package uploads,
checks tag SHA and Release absence before uploads and again before one ref-free
POST, and verifies the response and follow-up. No update/delete/tag creation,
redirect following or POST retry exists. Tokens are supplied through stdin;
uncertain POST outcome stops for operator reconciliation.

Independent review initially BLOCKED local source 180ebcdb for malformed 404 and
multiple-document JSON acceptance, plus missing test/probe coverage. Corrections
require one JSON document, explicit missing-Release response shapes, exact payload
and link assertions, and in-flight credential/cleanup observation. A live read-only
GET against existing tag v2.7.0-rc.1 confirmed GitLab's generic
`{"message":"404 Not Found"}` body; a regression preserves that valid contract
without admitting malformed responses. Final bounded independent re-review at
461ed65e is CLEAR. No production write was used for testing.

Validation: publisher release-profile target 9/9 locally, with the live CI probe
explicitly skipped outside canonical same-project MR CI. Earlier full agov tests
passed before the bounded response/test correction; shell syntax, formatting and
diff checks pass afterward. Pin selftest 14/14, release-assets selftest and affected
CI/gate targets passed. Final gate change-check passes; CI-parity plan collisions
are zero unacknowledged / one acknowledged against current main. GitLab CI lint
returns valid with no errors or warnings. The next corrected MR pipeline must
exercise the actual read-only job-token probe for protected existing tag, Release
and collection endpoints, as well as required tests. Prior pipeline 3005412510's
DNS failure fetching index.crates.io is retained; no unchanged retry was requested.

Fresh Trivy 0.76.1 evidence with 2026-10-29 database, linked in !3358 note
4119889717, reports zero HIGH/CRITICAL findings across 16 base and 34 installed
runtime packages; independent evidence review is CLEAR. This is bounded package
scan evidence, not vulnerability freedom or full APK reproducibility. The prior
release-cli findings were not accepted or suppressed.

The nskj overlap corrections are preserved, including separate v2.13 profiling
and publisher acceptance text. A merge-tree check with profiling candidate
7212e438 is clean. Do not merge implementation until final review, exact-head CI,
live probe evidence and a fresh binding Gate 2 decision. No release/tag/deployment
or extension of delegation is requested by this handoff.
