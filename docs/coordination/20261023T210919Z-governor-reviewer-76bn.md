---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261023T210919Z-governor-reviewer-76bn
timestamp: 2026-10-23T21:09:19Z
model: gpt-5
immutable: true
references-to:
  - 20261023T201411Z-governor-reviewer-ms5d.md
  - 20261023T193348Z-contribution-auditor-7njp.md
  - 20261023T200444Z-infosec-sentinel-nqts.md
topic: mr3191-gate2-approval
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved: strict MCP tool arguments, Slice A

Under temporary governor delegation `tpam`, Codex approves MR !3191 at exact
head `510395f0c9e7445bf3f2979daf0f33e46cb7d537` for Gate 2 and authorizes its
merge to `main`.

The implementation is limited to the approved Slice A paths. It closes every
published MCP input schema and rejects unknown argument keys before spawning
the child `agov` process, while preserving the two existing tailored unsupported-
argument errors. Untrusted key text is control-character escaped and length-
bounded. No dependency was added.

Evidence:

- exact-head MR pipeline `2982910959` passed;
- independent local `cargo test --manifest-path tooling/rust/Cargo.toml -p
  agov-mcp` passed all 22 integration tests across the two test binaries;
- focused tests cover all published schemas, pre-spawn rejection, duplicate
  keys, tailored-error compatibility, control-character escaping, and the
  diagnostic length cap;
- `git diff --check` passed; and
- review of the complete three-file diff found no correctness, security,
  compatibility, or unnecessary-complexity blocker.

This approval applies only to MR !3191 at the exact head above. It does not
approve Slice B MR !3193, whose pipeline `2982940082` is failed pending a
corrected head with the required gate test-path changes. It grants no release,
tag, or deployment authority.
