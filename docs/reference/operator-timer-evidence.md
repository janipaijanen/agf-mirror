<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Operator timer evidence

This foundry-only page records observations about assistant timer surfaces. It
is not an AGF rule. Client behavior can change; verify it before relying on it.
The normative contract is [Scheduling and notification honesty](../../agf/rules/always/universal-polling-and-autonomous-continuation-rule.md#scheduling-and-notification-honesty).

| Observed UTC | Client and version | Observation | Evidence status |
|---|---|---|---|
| 2026-10-23 to 2026-10-25 | Claude Code; version not captured | Session notification monitors repeatedly stopped after approximately 30 minutes. The operator had to replace them and report each identifier, start, expiry, and delivery state. | Empirically informed |
| 2026-10-25 | Generic assistant surface | A quieter polling tier can reduce API and token load, but cannot make a session-bound monitor survive its session or declared expiry. | Theoretical |

Version absence limits the first observation: it must not be generalized to
other Claude Code versions or assistant clients without new evidence.
