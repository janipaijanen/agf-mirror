---
kind: decision-record
from: governor-reviewer
to: all
id: 20261028T163912Z-governor-reviewer-ss2k
timestamp: 2026-10-28T16:39:12Z
model: codex
immutable: true
references-to:
  - docs/plans/20261027T235807Z-v212-release-publication.md
  - docs/coordination/20261028T161358Z-coder-tester-gm8j.md
topic: v212-status-sync-scope-renewal
requires-ack: false
artifacts_owned: []
---

# Renew v2.12 status-sync scope

Jani Päijänen delegated governor-reviewer authority to Codex in the live
session through 2026-10-30T18:00:00Z (20:00 EET). This record transcribes that
delegation before any further binding or irreversible action.

The approved v2.12 publication plan's post-publication status-sync scope is
renewed to include exactly the two `Observed at` cells in:

- `agf/docs/site/roadmap.md`; and
- `docs/roadmap.md`.

Both cells must become `2026-10-28T16:13:58Z`, the timestamp of the published
completion record after the tag pipeline and anonymous verification completed.
Keeping the pre-publication 2026-10-26 timestamp beside a v2.12 status and
v2.12 observation baseline would be internally contradictory. The existing
enumerated B cells, including each observation-baseline cell and the manifest's
tag date, remain in scope unchanged from the reviewed draft.

No additional path or cell is authorized. The three-file status-sync MR still
requires validation, independent advisory review, exact-head pipeline evidence,
and a separate binding Gate 2 decision before merge.
