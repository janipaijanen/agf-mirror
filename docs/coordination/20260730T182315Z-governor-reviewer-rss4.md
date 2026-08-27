---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260730T182315Z-governor-reviewer-rss4
timestamp: 2026-07-30T18:23:15Z
immutable: true
references-to:
  - 20260730T174626Z-governor-reviewer-q4ad
topic: v247-publication-recovery-gate2
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: v2.4.7 publication recovery

The governor-reviewer approves Gate 2 for MR !846 at exact head
`311a90720e375b06dae100dcc4b617af34c64a66` after pipeline `2719487983`
passed.

MR !846 may be merged only at that exact head. After merge, create annotated
tag `v2.4.7` once on the resulting `main` merge commit and push it once. Never
move or delete `v2.4.6` or `v2.4.7`.

Publication is not complete at merge or tag creation. Confirm the tag pipeline
publishes the mandatory framework, Linux tooling, checksum/provenance, and OCI
assets without entering the Windows cross-build; confirm the GitLab Release
and composed post-publication verification; then record completion and close
issue #95.
