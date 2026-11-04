---
kind: decision-record
from: governor-reviewer
to: all
id: 20261104T162918Z-governor-reviewer-qksh
timestamp: 2026-11-04T16:29:18Z
model: claude
immutable: true
references-to:
  - 20261104T061445Z-planner-architect-qrqd.md
  - 20261104T052528Z-governor-reviewer-2st2.md
  - 20261101T195927Z-v300-release-publication.md
topic: gate2-status-sync-3507-and-sources-radar-3510-bridge-record
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 for MRs !3507 and !3510 at their exact heads (bridge record, provisional)

**Provenance.** The Human Governor said in a live session on 2026-11-04: "gate 2 approved for
both. proceed." and told the Claude session (model: claude) to file this record. The `from`
field names the decision-maker; the session is the filer. This is a bridge record: provisional
until the governor confirms it in a later record, and correctable by a later record. The human
did not name MR numbers; the session reads "both" as the two MRs it had named as waiting for
Gate 2 in its last report, !3507 and !3510.

## Decision

Gate 2 is approved, and merge without squash at the exact SHA is authorized, for:

| MR | Exact head | Pipeline at head | Paths |
|---|---|---|---|
| !3507 status sync for the v3.0.0 publication | `bec7a7c8141a8f8274202ea73c1fe64f651504ad` | success | `agf/docs/governance/manifest.md`, `agf/docs/site/roadmap.md`, `docs/roadmap.md` |
| !3510 sources Batch 44 (two repositories) and one radar question | `942f701b150340ba8bada3d557ff809a3cf6d90a` | success | `docs/sources.md`, `docs/radar.md` |

Each merge is made only if the head is unchanged and the pipeline at that head is green at merge
time; otherwise this decision lapses for that MR. After each merge the session verifies the
first and second parents and the file list.

## Evidence the governor relied on or may check

- !3507: published-state evidence is in `qrqd` (tag pipeline 3025636346 all 23 jobs, Release,
  anonymous downloads with matching SHA-256, OCI manifest digest equal to its provenance).
  Strict `mkdocs build` exit 0; no `2.13.1` left in the three files.
- !3510: sources fast path (`simplicity-rule.md`). Range qualification PASS on target `6ca0cac7`;
  only the two allowed files changed. Immediate pre-Gate-2 collision scan at 2026-11-04T16:28:57Z
  against target `6ca0cac7`: the only other open MR is !3507, whose paths are the three above and do
  not overlap the allowed files; no relevant state change. Evidence class for both sources:
  self-reported, one summarized read each, behaviour untested.

## Not decided here

The BREAKING classification of the role guidance, confirmation of the earlier provisional records
(`g8jp`, `egbp`, `rkjm`, `1jkm`, `rqeb`, `2st2`), the prior-art source, and any further
release step.
