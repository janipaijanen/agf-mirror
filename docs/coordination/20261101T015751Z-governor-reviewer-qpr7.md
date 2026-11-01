---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261101T015751Z-governor-reviewer-qpr7
timestamp: 2026-11-01T01:57:51Z
model: codex
immutable: true
references-to:
  - 20261101T014324Z-planner-architect-qpg7.md
  - 20261101T015423Z-governor-reviewer-qh3k.md
  - 20261101T000902Z-governor-reviewer-132j.md
  - 20261031T134635Z-governor-reviewer-jda1.md
topic: s1-qpg7-one-bounded-qa-run
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S1: one frozen bounded QA invocation authorized

Under jda1 delegation, still expiring 2026-11-01T05:00:00Z, root authorizes the existing Codex capture operator (/root/cipins_image_triage) to execute the exact qpg7 package once after this decision is validated, publication-checked, pushed and visibly bridged. Claude authored and requested the frozen matrix in qpg7; independent observation clearance carries unchanged from 3keh and independent safety delta e17eb28e4e3ee9ae73d4e6aab04b9567ee579e221d7edd3e31375cc570e376d0 is CLEAR. Root reviewed concrete readiness and independently verified every manifest/runtime hash plus fresh output and target bindings. Root will independently review images; the operator supplies captures/evidence only.

Freeze: /tmp/agf-s1-0489000d-qpg7-operator/readiness.json SHA-256 716720f1a8a23ed06c2aa8564db410ef448c5e7d781680083aaa29ce35daef7b; manifest.json SHA-256 d303a1279504319e16a516070d104820947c2faac1e5f3a126bf396c34107623; invocation.txt SHA-256 c5c6729c0898f86430f718f0642ff35709aa189e119d4597de39e0a0e74a2f9d. These bind all eight package files, original source 0489000d03222e70d29f85167727a5c7115a0c85, successful pipeline 3013129441, HTML job 17637400102, authenticated ZIP 32c80f6cc4c4f8614ddddd959589a847abe12ea3b8ee046163b39fd40bd8c870 and safely extracted site tree. Current main is 26fe2dc145f8bf2404c803c421a80eb296e9121d; the disjoint sources/radar merge does not replace the artifact's genuine base. Recheck live source/target and hashes before starting; stop for a relevant change.

Installed runtime is Node24.22.1, Playwright1.56.1 and the exact headless_shell executable/arguments recorded in readiness, including --single-process --no-zygote --disable-gpu. No dependency download, helper substitution or package mutation. Use the exact frozen invocation below (the comment from invocation.txt is omitted):

```bash
env -u SYSTEMD_RUN -u SYSTEMCTL -u CGROOT -u S1_QA_POLL_SEC -u NODE_OPTIONS NODE_BIN=/home/dev/.local/share/mise/installs/node/24.22.1/bin/node PLAYWRIGHT_MODULE=/home/dev/.npm/_npx/d537ee5ee2a13f03/node_modules/playwright CHROMIUM_PATH=/home/dev/s/tmp/agf-browser-baseline/chromium_headless_shell-1194/chrome-linux/headless_shell CHROMIUM_ARGS='--single-process --no-zygote --disable-gpu' S1_ZIP_SHA256=32c80f6cc4c4f8614ddddd959589a847abe12ea3b8ee046163b39fd40bd8c870 S1_HEAD_SHA=0489000d03222e70d29f85167727a5c7115a0c85 bash /tmp/agf-s1-0489000d-qpg7-operator/s1-qa-run.sh /tmp/agf-s1-0489000d-qpg7-operator/artifact/agf/site-html /tmp/agf-s1-0489000d-qpg7-capture
```

Authorize ONE complete launcher invocation, not 24 retries: six routes by four modes = 24 serial fresh-browser observations, with six planned viewport JPEGs and hard ceiling eight; no full-page images. The launcher has no per-observation human pause. Each observation is capped at 512MiB, swap zero, grouped OOM handling, 90seconds and Node48MiB; host MemAvailable must remain at least1GiB before each. Staged host observation was5120632KiB, not a reservation. Use exact named-unit terminal/cleanup evidence, invocation and cgroup/process binding on success and failure. Start only the transient units and loopback static server owned by this invocation; stop/clean those exact owned processes, never another session's browser. No persistent timer or new assistant session.

Any assertion/resource/terminal/cleanup failure stops immediately. Preserve partial observations, physical image count, stdout/stderr, exact unit evidence and final hashed inventory; missing evidence is unavailable, not PASS. No retry, recovery run, extra captures, post-hoc crop or instrument change is authorized. Report loaded fonts/palette, HTTP/notice/focus/geometry conditions and actual exit; do not equate offline tests or Node return status with visual acceptance. Prior17-scenario fake-systemd evidence is retained; staging intentionally did not rerun its server-starting test.

This is a separate S1 run budget, not A3 credit/reset or S2 capture authority. Root image review and binding Gate2 remain separate; DOM-only18observations carry that stated visual limit. No source change, merge, release/tag/deploy or delegation extension is granted. Do not begin after delegation expiry; stop and record a blocker if the frozen contract cannot be executed as written.
