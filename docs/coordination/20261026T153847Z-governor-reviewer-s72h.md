---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261026T153847Z-governor-reviewer-s72h
timestamp: 2026-10-26T15:38:47Z
model: codex
immutable: true
references-to:
  - 20261026T153209Z-contribution-auditor-1e7p
topic: next release scope intake Gate 1
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 approved: version-neutral next-release scope and intake

The human governor delegated full governor-reviewer authority to Codex in the
live session until 2026-10-26 18:00 EET (`2026-10-26T16:00:00Z`) and directed
planning of the next release plus a more systematic content-gathering method.

MR !3239 exact head `08c104367c15d98c186f99a465a4d4bac5c65382`
is **approved at Gate 1**. Exact-head pipeline `2990446176` passed all three
jobs. Independent contribution review
`20261026T153209Z-contribution-auditor-1e7p` is CLEAR with no blocking,
advisory, or accepted-risk findings and is visible on `main` through merge
commit `22ea7576ef41b710238d61fca741f26e651523f9`.

The approval is intentionally narrow:

- one version-neutral scope/intake plan is the rolling owner for the release
  after v2.11.0;
- `docs/roadmap.md`, queue ledgers, changelogs, and GitLab remain the named
  authorities/inputs for their existing domains;
- the seed table contains no `IN` commitment and chooses no version or date;
- candidate rows are evidence inputs, not implementation or shipment
  commitments; and
- every workstream retains its own Gate 1 and Gate 2.

This approval authorizes the plan's manual one-release pilot and reviewed
evidence refreshes. It does not authorize implementation, merge, milestone,
version bump, scope freeze, tag, package, release, or deployment. Candidate
membership changes, readiness/compatibility reinterpretation, promotion/exit
condition changes, any `IN` commitment, version choice, or freeze are material
amendments that return the exact plan head to `pending-gate1`, non-author
review, and a new governor decision.

The plan closes after the next release is published and independently
verified. Any following release starts a new version-neutral intake plan from
that immutable tag.
