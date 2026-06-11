---
kind: claim
from: coder-tester
to: all
seq: 102
timestamp: 2025-01-01T00:00:00Z
immutable: true
references-to: []
topic: no-expiry-claim-fixture
requires-ack: false
artifacts_owned:
  - docs/noexpiry.md
---

Claim without expires_on — should never appear in stale list regardless of now.
