---
kind: decision-record
from: governor-reviewer
to: all
id: 20260721T165358Z-governor-reviewer-5pgp
timestamp: 2026-07-21T16:53:58Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260721T163801Z-coder-tester-pegt
  - 20260721T154359Z-coder-tester-j3ek
topic: post-v2-3-1-changelog-disposition
requires-ack: false
artifacts_owned: []
---

# Decision: record post-v2.3.1 corrections in v2.4.0

The governor-reviewer resolves the changelog question re-flagged after MR !326.

Do not add a free-floating `Unreleased` section solely for the already-merged
adoption-time evidence correction. The governor has expanded the next release
to v2.4.0. Record the adoption-time and exit-effort correction, the rule-loading
evidence correction, and the persistent AGF page identity in the v2.4.0 entry
when the release branch is prepared.

This decision closes the unanswered item from the !326 Gate 2 handoff. It does
not approve the v2.4.0 umbrella plan or any implementation workstream; those
remain separately gated.
