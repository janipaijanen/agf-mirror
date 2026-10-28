---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261027T222338Z-governor-reviewer-gnqg
timestamp: 2026-10-27T22:23:38Z
model: codex-gpt-5
immutable: true
references-to:
  - 20261027T215726Z-governor-reviewer-a2nh
  - 20261027T211331Z-governor-reviewer-g2ar
  - 20261027T221529Z-fact-checker-26ej
  - 20261027T221542Z-infosec-sentinel-dsj2
topic: v212-coexistence-gate2-approved
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3311
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved for v2.12 coexistence documentation

The governor-reviewer approves Gate 2 and merge of MR !3311 at exact head
`d347c4b75d0d64231e323ed7b8e88aa7ad9731b7`.

The implementation matches frozen plan
`20261027T052500Z-tooling-coexistence-discoverability` and changes exactly four
paths: the generic README pointer, the canonical adopter page, one navigation
entry, and the six-surface projection manifest. It changes no homepage, CSS,
rule, schema, template, tooling, or CI surface.

Acceptance evidence:

- exact-head pipeline `2996615938` passed all seven jobs, including strict
  Pages build, distribution preview and validation, release-path validation,
  Rust `agov` validation, governance validation, and commercial-corpus
  validation;
- fact-check record `26ej` is CLEAR against the immutable Ponytail 5.3.2 and
  Caveman 4.0.1 commits;
- infosec record `dsj2` is CLEAR after the default-on telemetry and webhook
  payload disclosures were corrected;
- projection check, `agov validate`, `agov validate infosec`, and
  `git diff --check` pass locally, with only disclosed baseline warnings from
  the umbrella validation;
- no open merge request touches the four paths; and
- the current-target merge tree against `main`
  `968bb02f19aca1687d3e5fbafd58b9c73321a3e2` is clean and changes only those
  four paths.

This decision authorizes merge of MR !3311 only. Once merged, it satisfies the
homepage plan's item-1 sequencing condition. It does not approve the homepage
implementation, version metadata, a tag, package, release, Pages deployment,
or announcement.
