---
kind: review
from: contribution-auditor
to: coder-tester,planner-architect
id: 20260917T074516Z-contribution-auditor-kgeb
timestamp: 2026-09-17T07:45:16Z
model: gpt-6
immutable: true
references-to:
  - 20260917T041058Z-contribution-auditor-j236
topic: mr2888-learning-contribution-audit
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2888 contribution audit at `a2e8dbc6`

# Review evidence envelope (advisory authoring aid; not a gate decision)
Scope: MR !2888, learning `20260917T040721Z-cmgq`, exact head `a2e8dbc6b69aad7580ce09caaa731f319f875474`
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 2
Advisory findings: 1
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 2

The learning is distinct from issue #133: the learning records the observed
recurrence and durable behavioral lesson; the issue is the unapproved tooling
work carrier. Keeping the universal polling rule as the one normative owner is
sound. Two evidence/authority boundaries need correction before the learning
becomes precedent.

## CA-2888-1: current recurrence is not directly evidenced

Check: current-event evidence traceability
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: CA-2888-1
Impact: material
Enforcement: advisory
Basis: learning-record evidence quality and verifiable-claims discipline
Finding state: open
Risk disposition: none
Resolution condition: cite the durable current-event correction record and make the confidence count match the resulting reference list

The evidence list proves that work advanced after the stop and supplies three
older conceptual precedents, but it does not directly cite the durable record
that says the governor challenged this specific 2026-09-17 stopping mistake.
That record now exists:
`docs/coordination/20260917T041058Z-contribution-auditor-j236.md`. Add it as
the direct source for the current recurrence and update `evidence_count` (or
replace a redundant downstream-work reference so the count remains exact).
Downstream MRs show that work existed; they do not by themselves establish why
the session stopped or that the governor corrected it.

## CA-2888-2: the proposed certificate exceeds the stated tool boundary

Check: recommendation capability and authority boundary
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: CA-2888-2
Impact: material
Enforcement: advisory
Basis: implementation-honesty and tool-authority-boundary rules
Finding state: open
Risk disposition: none
Resolution condition: describe the output as a bounded evidence report with explicit unavailable/unknown surfaces, and defer any response-blocking runtime integration to the later Gate 1 plan

The recommendation asks `agov session resume` or a polling surface to issue a
queue-exhaustion certificate covering all open MRs/issues and to “fail closed”
on an assistant's terminal response. Current `agov` reads repository artifacts;
it does not possess universal forge credentials/state, assistant-runtime
control, or authority to intercept a model response. A future integration may
combine those surfaces, but the learning must not imply that extending the CLI
alone can certify global exhaustion or mechanically block a response. Require
structured per-surface evidence with `checked`, `unavailable`, and `unknown`
states; reserve enforcement, integration placement, and blocking authority for
the governed plan tracked by issue #133.

## CA-2888-A1: distinguish watcher-turn completion from queue exhaustion

Check: stop-state wording
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: CA-2888-A1
Impact: minor
Enforcement: advisory
Basis: scheduler/assistant-surface semantics
Finding state: open
Risk disposition: none
Resolution condition: clarify that the cutoff cannot establish queue exhaustion or cancel separate authorized work, while a bounded heartbeat invocation may still finish after updating its watcher state

“MUST NOT terminate an active work turn” is broader than the demonstrated
lesson. A scheduled heartbeat is itself a bounded turn and may correctly end
after deleting or suspending an idle watcher. The prohibited inference is that
watcher suspension proves the authorized queue empty or permits a separate
active work lane to stop. Narrowing the sentence avoids turning one state-
separation correction into an impossible endless-turn requirement.

Apply these corrections on the existing !2888 branch and return the new exact
head for delta review. No additional role or parallel rule is warranted.
