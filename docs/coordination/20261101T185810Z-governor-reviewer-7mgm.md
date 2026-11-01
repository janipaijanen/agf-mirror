---
kind: decision-record
from: governor-reviewer
to: all
id: 20261101T185810Z-governor-reviewer-7mgm
timestamp: 2026-11-01T18:58:10Z
model: claude
immutable: true
references-to:
  - 20261101T105902Z-governor-reviewer-kqgp.md
  - 20261101T160106Z-governor-reviewer-k6bq.md
  - 20261101T162840Z-governor-reviewer-srra.md
  - 20261101T122612Z-coder-tester-5tem.md
topic: major-release-approved-in-principle
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Major release approved in principle

**Provenance.** The Human Governor gave this decision in a live session on 2026-11-01 ("major
release approved. proceed.") and told the Claude session (model: claude) to file it. The
`from` field names the decision-maker. The governor may correct or supersede it by a later
record.

## Decision

A **major release** is approved in principle. This is the "separately approved major-release
decision" that D1 option A (`kqgp`) and plan acceptance criterion 4a require before the
forge-evidence rule implementation (MR !3482) may receive a Gate 2 decision and merge.
Handoff `5tem` is unblocked by this record.

## Not decided here (still excluded from the delegation `srra`)

- No version number, release date, milestone or tag.
- No release content beyond the forge-evidence rule's own entry. Other candidates (see plan
  `20261101T115616Z-release-content-inventory`) are not selected.
- No release, tag, publication or issue closure.
- The session reads "approved" as allowing Gate 2 and merge of !3482 to `main` once the
  usual conditions hold (CLEAN independent review of the exact head, green pipeline, `main`
  merged in). Because that places the BREAKING rule on `main`, the session will ask before
  merging if any release other than this major release could still be cut from `main`.
