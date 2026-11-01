---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261031T201447Z-governor-reviewer-pgjh
timestamp: 2026-10-31T20:14:47Z
model: codex
immutable: true
references-to:
  - 20261031T200726Z-governor-reviewer-m1d5.md
  - 20261031T201124Z-governor-reviewer-4s2p.md
  - 20261031T134635Z-governor-reviewer-jda1.md
topic: a3-one-original-zip-sharing-trial
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# One-original ZIP trial after direct-JPEG transport failure

Under jda1 through 2026-11-01T05:00:00Z, root authorizes only the exact two public payloads below, after publication. This narrowly supersedes m1d5's prohibition on an archive for this one original; no broader archive or batch approval. No capture, source change, merge or Gate 2 authority.

The direct-JPEG trial stopped correctly: upload 1457950291 returned 147268 bytes SHA-256 4966ca60034d753c3e43bdb150027a94adcd824363f62645c12ae97d29d5673b through both official upload-id GET and returned public path, instead of the approved 147624-byte original 2d595dd7af68df737226fdd2cfa747a0007efde1aec93d268ee22a26e9ce6a43. The metadata changed; no server/CLI causal attribution or color/pixel equivalence is asserted. No manifest or batch was uploaded. Preserve the failed pilot, original and response/diagnostic evidence; do not delete or silently relabel it successful.

Read-only triage report tmp/a3-public-original-pilot/read-only-transport-triage.md SHA-256 39d416144ceb4f81d6f896fdd2a7cd4cd9d7d1649399a1cac42a038d9c98cf6f identifies APP0/ICC replacement with EXIF, matching coded stream but possible appearance consequences; original-byte equality remains mandatory. Official references: https://docs.gitlab.com/administration/raketasks/uploads/sanitize/ and https://docs.gitlab.com/api/project_markdown_uploads/. No undocumented preservation option is assumed.

## Exact public whitelist

Root inspected the ZIP central directory, CRC and extracted member bytes; approved public candidate pixels/sanitized facts are unchanged from m1d5. Authorize only:

- a3-first-original-byte-preserving-pilot.zip: 147810 bytes, SHA-256 e9f473d386d2c0e1441724a5640231b0afa8498ec6b51897555599976f801f40. Exactly one flat regular member desktop-light-why-meter-reader-context-0.jpg, 147624 bytes, SHA-256 2d595dd7af68df737226fdd2cfa747a0007efde1aec93d268ee22a26e9ce6a43. ZIP_STORED; no comments/extras/encryption/symlinks/directories/other files. Fixed 1980 ZIP timestamp is packaging only, not capture time.
- The unchanged a3-first-original-public-manifest.json already reviewed in m1d5: 1624 bytes, SHA-256 6f050c05c0cf09a5630366c8cddcd62cb8c540e814fa40b0a4a20826a96508c5. Its source, capture and original-image identity remains valid; this decision separately authorizes the outer carrier. No private paths/logs/chat/account data are included.

Use existing official project 83321848 upload API. Verify exact source payload bytes, POST the ZIP once, preserve returned id/full_path, download via official upload-id GET, require outer-byte equality, CRC and extracted-original hash equality before manifest POST. Then POST only the exact manifest once and verify its roundtrip too. Any uncertainty or mismatch stops; inspect existing returned upload metadata before a retry, no duplicate blind POST. No raw directory/log archive, changed/resized/cropped/collaged image, new attachment beyond these two payloads, repository binary commit or new CI pipeline is authorized.

Give Claude actual returned carrier/manifest URLs, outer and inner hashes and source 6d47f8db. Receiver must download, verify ZIP and member hashes, open the extracted original and record actual observations/limits. No transport-success claim until receiver confirms actual access. No accepting transformed JPEG pixels as the original. Image-only review does not independently replay capture/keyboard/focus/network/lifecycle behavior or establish novice/whole-site acceptance.

A2 deployment hold in 4s2p stays active: no further main merge or A3 capture until verified main CI/Pages/live routes/counts. A3 desktop-dark fault remains pending precise residual disposition; eighteen groups unattempted. This transport trial does not close any of those obligations, parent R1/R2/A1, final release or governor expiry. No additional timer/session, source change, permission change or deletion is authorized.
