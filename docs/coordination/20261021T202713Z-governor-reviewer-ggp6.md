---
kind: decision-record
from: governor-reviewer
to: all
id: 20261021T202713Z-governor-reviewer-ggp6
timestamp: 2026-10-21T20:27:13Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T220706Z-governor-reviewer-e7mr
requires-ack: true
topic: otgr-superseded-external-implementation
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor decision transcribed from the current task

Jani instructed directly: "there is OTGR implemented outside of this
project. Close !2587, !2592." Claude transcribes the human decision
under governor-reviewer attribution; authority is the human's, not
AI-originated.

**OTGR (purpose-built relay service for machine-speed AI-to-AI
communication) is superseded by an external implementation outside
this project.** The sequencing decision in `e7mr` ("OTGR stays parked
until v2.11 ships") no longer applies — there is nothing left to
unpark. The entire OTGR thread in this project is closed, not merely
deferred.

**Closed, not merged:**

- MR !2587 (`plan/otgr-relay-service`) — Gate 1 plan for a purpose-built
  OTGR relay service. Closed as superseded.
- MR !2592 (`proposal/otgr-relay-api-comms-spec`) — draft OTGR relay API
  and technical communications specification v0.1. Closed as superseded.

Both source branches are left in place (not deleted) as historical
reference; neither MR's content merges to `main`.

No further OTGR work is in scope for this project. Any future AI-to-AI
relay need should be evaluated fresh against the external
implementation's actual capabilities, not by resuming this thread.
