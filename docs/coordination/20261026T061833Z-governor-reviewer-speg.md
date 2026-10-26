---
kind: decision-record
from: governor-reviewer
to: all
id: 20261026T061833Z-governor-reviewer-speg
timestamp: 2026-10-26T06:18:33Z
model: codex
immutable: true
references-to:
  - 20261026T053900Z-governor-reviewer-g7pj
topic: v2.11.0 post-merge tag authorization
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3235
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.11.0
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11.0 metadata Gate 2 and conditional tag authorization

The human governor delegated full governor-reviewer authority to Codex in the
live session until 2026-10-26 18:00 EET (`2026-10-26T16:00:00Z`) and directed
the approved release plan to completion. Exercising that bounded authority:

1. MR !3235 is approved at Gate 2 and its completed merge is accepted. The
   independently reviewed exact head
   `b98fdbe18f48dc558230650eef9af276c41bffac` passed canonical MR pipeline
   `2988254014` with all 17 expected latest jobs green. It merged to `main` as
   `c4f332cd3ef0a3b8e78ea2c86f009a097d035426`, whose tree is the tested tree
   `b28de17aabad34b089755e4704aa893b686b76b1` and whose parents are frozen base
   `24d098a108ffe9cb227bd55c1c8208cac6506123` and the reviewed head.
2. The clean unpublished head preflight returned `ABSENT` for the Git tag,
   GitLab Release, generic package, and derived OCI reference. The exact
   protected-tag policy is `v2.11.0`, creation restricted to Maintainers
   (access level 40; access-level object `44387056`).
3. One annotated tag named `v2.11.0` is authorized on the final proven live
   `main` commit after this coordination decision is merged, but only if the
   exact-main pipeline `2988294802` and this record's coordination-MR pipeline
   are green and the immediately preceding fail-closed checks all pass:
   four-namespace `pre-push` absence, byte-equivalent protected-tag policy,
   frozen live-main SHA/tree/ancestry, and sufficient runner/DinD capacity.
4. The tag push is a single irreversible action. This decision grants no
   authority to move, delete, or recreate the tag. Any ambiguity or drift
   stops publication. Completion still requires the exact tag pipeline,
   `release:verify`, an independent anonymous post-publication check, and the
   explicit annotated-tag object check required by the release plan.

This is the separate post-metadata-merge binding decision required by Step 6
of `docs/plans/20260909T203636Z-v211-release-publication.md`. Publication
evidence and ship-in issue closures belong in the later completion record.
