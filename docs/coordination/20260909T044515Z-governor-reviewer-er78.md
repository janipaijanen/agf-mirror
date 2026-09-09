---
kind: decision-record
from: governor-reviewer
to: all
id: 20260909T044515Z-governor-reviewer-er78
timestamp: 2026-09-09T04:45:15Z
model: human-via-codex
immutable: true
references-to:
  - 20260909T013812Z-capability-developer-jprj.md
  - 20260909T040330Z-capability-developer-5jde.md
topic: v211-propeller-protocol-proposals-promoted
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Include the Propeller coordination-protocol proposals in v2.11

The human governor asked whether the Propeller proposals were included in the
release scope and directed: “they should.” This is a binding v2.11 scope
membership decision for the coordination-protocol proposals assessed in
`jprj` and handed to the planner in `5jde`.

The v2.11 plan must therefore list all of the following as IN rather than
CANDIDATE or unowned:

- #121 session-level addressing;
- #122 self-announcing supersession;
- #123 structured affected paths and advisory collision detection;
- MR !2027 priority and bounded material challenge-response;
- CP-A substantive-response versus acknowledgement state;
- CP-B safe multi-record coordination publication plus a non-urgent
  release-lane hold and urgent bypass; and
- CP-C branch-local obligation visibility and archive-path semantics.

This decision supersedes only the earlier candidate disposition of #123 and
CP-A/CP-B. It does not approve the materially revised release-plan head, grant
implementation authority, collapse child Gate 1 or Gate 2, or authorize a
release or tag. The revised v2.11 plan requires fresh exact-head advisory
reviews and a new binding Gate 1 decision.
