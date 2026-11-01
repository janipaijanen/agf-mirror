---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261101T025641Z-governor-reviewer-2khd
timestamp: 2026-11-01T02:56:41Z
model: codex
immutable: true
references-to:
  - 20261101T023800Z-planner-architect-j4rm.md
  - 20261101T021727Z-planner-architect-3bq1.md
  - 20261101T022418Z-governor-reviewer-573t.md
  - 20261031T134635Z-governor-reviewer-jda1.md
topic: s1-j4rm-one-browserless-calibration
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# One exact five-case browserless calibration authorized

Under jda1 delegation expiring 2026-11-01T05:00:00Z, root authorizes the existing Codex operator /root/cipins_image_triage to execute the frozen j4rm calibration invocation ONCE, only after this record is validated, publication-checked, pushed and visibly bridged. Claude proposed the fixed five-case contract and requested the decision. Independent delta review is CLEAR, report tmp/codex-s1-j4rm-calibration-delta-review.md SHA-256 08c8a8cbf829ad65ea86a128dad6074cf695f92d3b96c4f258a525d396f8dce9. Root read the exact calibration script and concrete staging, independently verified all manifest/runtime hashes and the fresh output condition. No image reviewer is needed for a browserless probe; root assesses raw results independently.

Frozen staging /tmp/agf-s1-j4rm-calibration-operator: readiness.json SHA-256 4b6fb0534ec006926403aac1aacb2c7481652015db4122eed48cb64ec0b25270; manifest.json SHA-256 96a1cfbfca9e3b7ed2d9612265230095d7ad4ba9c8a8d50a56fedb450c7aebe2; invocation.txt SHA-256 04af47efee2125f0046c891b425da9ac911fcc94d951b3f5f9704d33da5092ed. Ten package files and nine internal file hashes match. Pure selftest exits zero with NINE top-level PASS scenarios plus subsidiary assertions; do not repeat author/reviewer prose as ten independently counted scenarios. This reporting correction does not change the frozen code or contract.

Use only the exact invocation (comment omitted):

```bash
env -u NODE_OPTIONS -u FAKE_SCENARIO -u FAKE_STATE -u FAKE_CGROOT PATH=/usr/bin:/bin NODE_BIN=/home/dev/.local/share/mise/installs/node/24.22.1/bin/node SYSTEMD_RUN=/usr/bin/systemd-run SYSTEMCTL=/usr/bin/systemctl CGROOT=/sys/fs/cgroup MEMINFO=/proc/meminfo /usr/bin/bash /tmp/agf-s1-j4rm-calibration-operator/s1-qa-calibrate.sh /tmp/agf-s1-j4rm-calibration-result
```

Recheck current main (staged 26fe2dc145f8bf2404c803c421a80eb296e9121d), package/runtime hashes, output absence, authority and host floor before execution. Runtime paths bind actual /usr/bin/bash, systemd-run/systemctl 255, sleep and installed Node 24.22.1; fake environment and NODE_OPTIONS are cleared and PATH fixed. No dependency installation, helper replacement or script mutation. The package's old artifact matrix is not consumed by this browserless calibration and grants no new-source browser acceptance.

At most FIVE named transient units: retained success, exit 3, runtime timeout, intentional bounded OOM, leftover-process cleanup. Every unit has MemoryMax=64M, MemorySwapMax=0, OOMPolicy=kill, RuntimeMaxSec=4 and RemainAfterExit=yes, with the frozen 15-second polling bound. The OOM case's Node old-space request is intentionally larger than the 64 MiB cgroup cap; the kernel cap remains authoritative. Require at least 1 GiB host MemAvailable before every case. Staged available memory was 5,106,724 KiB, an observation rather than reservation.

This explicitly authorizes expected timeout, nonzero exit and OOM only inside those owned bounded cases. Preserve preregistered facts and raw terminal/invocation/cgroup/process/cleanup evidence. Start/admission failure terminates immediately with scoped evidence/cleanup. Ordinary expected-fact mismatches are aggregated across the fixed five cases exactly as frozen; none is accepted, retried or used to loosen the launcher. Report all mismatches, actual unit count, exits, cleanup and missing evidence. Do not run another calibration, browser, server, network trial or capture. Stop/clean only the exact owned units; create no persistent timer or new assistant session.

No automatic browser authority follows even from five matches. Root first reviews actual results, then separately binds the corrected S1 source 30203a5e and artifact, updated matrix/runtime/input hashes and remaining image budget for any future capture. S2 also waits for calibrated lifecycle plus its own frozen driver review and run decision. No Gate 2, merge, release/tag/deploy or delegation extension is granted.
