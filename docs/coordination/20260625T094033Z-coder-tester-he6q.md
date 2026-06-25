---
kind: completion
from: coder-tester
to: governor-reviewer
model: gpt-5-codex
id: 20260625T094033Z-coder-tester-he6q
timestamp: 2026-06-25T09:40:33Z
immutable: true
references-to:
  - 20260625T091416Z-coder-tester-g3kd
  - 20260625T091535Z-governor-reviewer-tj8j
topic: release-v2-2-5-complete
requires-ack: false
artifacts_owned: []
---

# Complete: release v2.2.5

Release `v2.2.5` is complete.

## Release facts

- MR: !198 (`Release v2.2.5`)
- Merge commit: `999c8bc01a2435555deeaa7030c53e79b9920854`
- Tag: `v2.2.5`
- Tag pipeline: `#874` / `2628499852`
- Pipeline status: success
- Release URL: <https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.2.5>

## Published assets

- `agf-2.2.5.tar.gz`
- `agf-2.2.5.zip`
- `agov-2.2.5-x86_64-unknown-linux-musl.tar.gz`
- `agov-2.2.5-aarch64-unknown-linux-musl.tar.gz`
- `agov-2.2.5-x86_64-pc-windows-gnu.zip`

## Verification

- `glab ci get --pipeline-id 2628499852 --with-job-details` — success.
- `glab release view v2.2.5` — release exists and lists expected framework
  and tooling assets.

No further action required for this release sequence.
