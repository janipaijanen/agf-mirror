---
kind: decision-record
from: governor-reviewer
to: all
id: 20261101T191657Z-governor-reviewer-pr6j
timestamp: 2026-11-01T19:16:57Z
model: claude
immutable: true
references-to:
  - 20261101T185810Z-governor-reviewer-7mgm.md
  - 20261101T162840Z-governor-reviewer-srra.md
topic: major-release-version-date-and-scope-answers
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Major release: version, merge, date target and tag authority

**Provenance.** The Human Governor answered the open decisions of the scope plan
`20261101T191112Z-major-release-scope` in a live session on 2026-11-01 and told the Claude
session (model: claude) to file them. The `from` field names the decision-maker.

## Decisions

- **V1.** The version is **3.0.0**.
- **S1.** MR !3482 (the forge-evidence rule) **merges to `main`** now. After it merges, any
  release cut from `main` carries the rule, so no docs-only v2.x release is planned.
- **S2.** The governor asked the session to propose the Pages Drafts disposition: !3466,
  !3467 and !3471 are included only if each finishes its independent review and a Gate 2
  by **2026-11-05T12:00:00Z** (the proposed freeze point); otherwise it is deferred to a later
  release. !3471 edits the global stylesheet and needs the narrow-display visual check.
  This is the session's proposal, adopted by the governor's delegation of the proposal; the
  governor may change it by a later record.
- **D.** The **target** release date is **2026-11-08**. It is a target, not a commitment.
  It falls after the delegation `srra` window (to 2026-11-06T16:00:00Z).
- **P.** The **Human Governor** authorizes the tag and publication.

## Not decided here

No tag, release or publication. The tag and publication remain the human governor's own
acts. The publication plan still needs its own Gate 1 and Gate 2.
