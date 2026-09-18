---
kind: decision-record
from: governor-reviewer
to: all
id: 20260918T040100Z-governor-reviewer-44bd
timestamp: 2026-09-18T04:01:00Z
immutable: true
model: human
references-to:
  - 20260917T230437Z-improvement-propeller-qtrn
topic: propeller-issue44-legal-surface-boundary
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Approved boundary for Propeller handling of issue #44

The governor-reviewer approves the handling boundary proposed after the
2026-09-17 Improvement Propeller wake stopped at possible legal content in
GitLab issue #44.

For future Propeller cycles, issue #44 is a metadata-visible,
content-excluded legal surface. The role may observe and retain these bounded
GitLab metadata fields:

- project-local issue identifier and title;
- open or closed state;
- labels and milestone;
- assignees; and
- creation, update, and closure timestamps.

The role must not inspect, emit, retain, hash, quote, or forward the issue
description, comments, discussions, attachments, linked documents, embedded
URLs, quoted legal text, or any other content-bearing field. Tool queries and
output must be limited or filtered so those excluded fields are not exposed to
the role.

If an otherwise allowed metadata field itself contains sensitive or legal
content, the role must stop processing that issue, retain only the identifier
and fixed classification marker `excluded legal surface`, and route the
boundary exception to the governor-reviewer.

Encountering the already-recorded exclusion is a complete governed
disposition for discovery purposes and must not repeatedly block later
Propeller cycles. Allowed metadata changes may update the retained metadata
without opening excluded content. Any request to widen this boundary requires
a new governor decision.

This decision authorizes resuming the blocked Propeller cycle from its
unchanged durable cursors and baselines. It does not approve, prioritize, or
otherwise disposition issue #44 itself.
