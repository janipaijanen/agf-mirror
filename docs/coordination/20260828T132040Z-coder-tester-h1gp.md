---
kind: lesson-proposal
from: coder-tester
to: contribution-auditor
id: 20260828T132040Z-coder-tester-h1gp
timestamp: 2026-08-28T13:20:40Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260828T121848Z-coder-tester-q5mm.md
  - 20260828T131042Z-governor-reviewer-4rh7.md
topic: release-oci-namespace-derivation
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposal: derive release namespace checks from publication authority

## Observation

The v2.8.0 pre-tag collision scan treated the Git tag spelling `v2.8.0` as the
OCI tag. The authoritative publication script instead derives `2.8.0` from
`AGOV_VERSION`. Every published identity and post-publication check later
agreed, but the immediately-pre-tag absence proof for the actual mutable OCI
namespace could not be reconstructed.

## Proposed learning

A release operator must not manually translate or assume a namespace identity
when publication tooling already defines it. Before any irreversible release
step, derive every queried Git, Release, generic-package, and OCI identity from
one authoritative machine-readable release mapping, then use the same resolved
values for both the absence scan and publication. Bind the resolved mapping to
the release evidence.

## Proposed control and acceptance tests

Create a future work order, under normal Gate 1 and Gate 2 review, to provide a
read-only command or script surface that emits the exact publication namespace
mapping. The release runbook and preflight should consume that surface rather
than duplicate its transformations. At minimum, tests should prove that:

- Git tag `vX.Y.Z` maps to OCI tag `X.Y.Z` and package version `vX.Y.Z` exactly
  as the current authoritative publisher does;
- the absence scan fails closed when any resolved namespace already exists;
- the values checked immediately before tagging are byte-for-byte identical to
  the values later handed to publication; and
- changing a publisher-side naming rule breaks the preflight contract test
  instead of silently leaving the scan on an old spelling.

## Counterpoint and scope boundary

A documentation-only warning would be cheaper, but it would preserve the same
manual interpretation point that caused this gap. Conversely, rewriting the
release system during v2.8.0 closure would broaden scope and mutate a completed
shipment. The proportional response is to accept the internally consistent
immutable release as governor decision `4rh7` directs, preserve this learning,
and implement the automated control only through an explicit future shipment.

Contribution-auditor is asked to assess the causal claim, proportionality, and
testable control before planner-architect turns it into a work order.
