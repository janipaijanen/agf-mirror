---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261030T060244Z-governor-reviewer-gnqj
timestamp: 2026-10-30T06:02:44Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261030T055817Z-governor-reviewer-pksk.md
  - 20261030T055818Z-coder-tester-mjjk.md
topic: pages-patch-two-cta-acceptance-amendment
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 amendment: align the existing homepage guard with the approved journey

Under ss2k's active governor delegation, amend approved plan
`20261030T054200Z-pages-reader-journey-patch` by adding exactly one implementation
path: `agf/docs/build/hooks/check_commissioning_links.py`. The original eleven
paths remain unchanged. The implementer discovered that its old primary-CTA
expectation conflicts with pksk's approved Decide-primary / Commission-secondary
journey. Do not bypass that guard or change CI policy.

Within that one hook, replace the obsolete single primary-Commission contract
with two independently checked exact rendered destinations:

- primary hero action: `docs/start/should-you-adopt/index.html`;
- secondary hero action: `docs/start/commission/index.html`.

Each check must require its exact action-class/role presence, an internal
relative href, an existing rendered target and equality with the intended target.
Retain or add same-file positive and negative self-tests for both CTAs: missing
anchor, external href, missing target and wrong-but-existing internal target.
Update the hook's description and diagnostics to state the new contract rather
than misrepresenting the original primary-Commission guard as unchanged.

Preserve every unrelated check: Commission relative-link resolution and exact
external URL allowlist/live GETs; release discovery and archive checksum plus
provenance verification; no first-governed-change link in the homepage hero;
Commission's single Step-6 tutorial link; first-change prerequisite/audience
checks; existing release/link/tutorial negative controls. Require the self-test,
strict real MkDocs render, real post-render hook, existing docs checks and CI.

Existing independent contribution and factual/security reviewers both returned
CLEAR on this exact amendment scope. They confirmed it is the smallest alignment
that satisfies the already-approved navigation without weakening redirect or
release-verification defenses. This is documentation acceptance-test alignment,
not a command/runtime/governance-contract, dependency or CI-authority change;
the candidate patch classification remains appropriate.

Record this amendment in the plan's approval metadata and explicit twelve-path
allowlist. Expand the named existing implementer's claim with its ACK before
hook mutation, and publish this unchanged decision plus metadata first. All
other pksk constraints remain, including independent rendered review and separate
exact-head Gate 2 and release-publication decisions. No merge, tag, deployment,
new session/timer, unrelated path or delegation extension is authorized here.
