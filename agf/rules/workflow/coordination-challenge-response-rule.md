---
id: coordination-challenge-response-rule
scope: workflow
applies_to: [llm-agent, human-agent]
strength: MUST
status: active
provenance: foundry-native
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Advisory coordination challenge-response

A challenge MUST remain visible until a valid governor terminal record closes
it. These records are advisory evidence, never an automatic merge/gate blocker.
Role and session declarations do not authenticate a person or grant authority.

Use `kind: review` and the optional `challenge` object. A raising record has
`resolution: open` or omits resolution, mints a corpus-unique `challenge_id`,
and supplies `window_hours` (1–720) and `disputed_record_id`. The latter MUST
identify exactly one disputed record and appear in `references-to`; contextual
citations do not identify additional parties. Ordinary requires-ack eligibility
and satisfaction remain independent of the challenge lifecycle.

A response MUST carry the same challenge ID and reference the raising record's
own ID or filename. `replied` is evidence only: recognize it as a party reply
only when its nonempty `session` matches the raising or disputed record's
session. Matching a role alone is insufficient. A third-party opinion remains
coordination content and cannot close or set party-replied state. Only a
`from: governor-reviewer` review may declare `resolution: escalated`, the
terminal action. The governor's prose states the disposition; an ordinary
escalation request is not a terminal judgment.

Before accepting any terminal candidate, validate the raising record and
resolve its required disputed source uniquely. Missing, invalid or ambiguous
sources yield `needs-review`, even when a purported terminal record exists.
Responses are effective only when their timestamps lie between the raising
timestamp and observation time, inclusive. A valid terminal dominates replies;
later replies never reopen it. Same-class reply display sorts by timestamp,
then filename. Missing raising records and collisions remain diagnostic rows.

Compute the deadline with checked arithmetic. Overflow yields needs-review.
At exactly the deadline the challenge is still open; strictly afterward,
without a recognized reply, display window-expired/escalate-to-governor.
A recognized reply, including a late one, displays decision-pending until
terminal disposition. Future raising records remain visibly not-yet-effective
with their activation timestamp, outside open/expired counts.

Once immutable records use this field, retain schema readability on rollback.
Disabling evaluation MUST retain conservative disabled/needs-review rows and
ordinary ack tracking. The evaluator's disabled mode deliberately may show
previously closed raising records again rather than trusting disabled closure
logic. Older strict readers that reject the field are unsupported after use.
