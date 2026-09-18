---
kind: decision-record
from: governor-reviewer
to: all
id: 20260918T035118Z-governor-reviewer-megj
timestamp: 2026-09-18T03:51:18Z
immutable: true
references-to:
  - 20260917T201055Z-contribution-auditor-j4ea
requires-ack: false
topic: gate2-approved-mr2942-mr2943-mr2944
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved: MR !2942, !2943, and !2944

The governor-reviewer approved Gate 2 in the active session for the full
MR !2651 publication-structure fix (`j4ea` CA-2651-4):

- MR !2942 at exact head `3d95e7e38e80ccc382f19f8f5381e648b60fc53e`
  (claim record). Merged into `main` as
  `f0d35e38eaa0a75de1e4ceabd1a770a2bdf1de2a`.
- MR !2943 at exact head `2dde3d12a8a1d6166450b7965997b5fa4397ed4f`
  (four orphaned coordination records — `et3p`, `asjp`, `emdh`, `jh3g` —
  published byte-identical from MR !2651's branch). Merged into `main`
  as `c71668b1156e544f7782dad440a57751207baa92`.
- MR !2944 at exact head `241409ce524ba528c11a8bf5fe289703ee0475a7`
  (MR !2651 rebuilt as a learning-only branch carrying the corrected
  `qvzj` entry, byte-identical content). Merged into `main` as
  `1cfd1f7be232f67eb03c549d2949f4cf44fd61d8`.

MR !2651 itself was closed as superseded by !2943 + !2944 (note added to
the MR explaining the split).

Filed by the active session at the governor-reviewer's explicit direction.
