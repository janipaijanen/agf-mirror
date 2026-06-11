---
kind: claim
from: planner-architect
to: all
seq: 103
timestamp: 2025-01-01T00:00:00Z
immutable: true
references-to: []
topic: invalid-expires-fixture
requires-ack: false
artifacts_owned:
  - docs/invalid.md
expires_on: "not-a-date"
---

Claim with invalid expires_on — ClaimTracker should return error for this.
