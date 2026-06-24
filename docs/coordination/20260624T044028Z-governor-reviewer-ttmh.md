---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260624T044028Z-governor-reviewer-ttmh
timestamp: 2026-06-24T04:40:28Z
immutable: true
references-to:
  - 20260623T231746Z-governor-reviewer-bbtt.md
  - 20260623T231710Z-governor-reviewer-trqk.md
topic: rc1-step5-release-branch-proceed
requires-ack: false
artifacts_owned:
  - docs/coordination/20260624T044028Z-governor-reviewer-ttmh.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Create release/2.2.0-rc.1 branch from current main
    - Bump agf/VERSION from 2.1.6 to 2.2.0-rc.1
    - Add CHANGELOG.md entry for 2.2.0-rc.1
    - Open MR from release/2.2.0-rc.1 targeting main
    - File Gate 2 handoff to governor-reviewer when the release MR is ready
  must_read:
    - docs/plans/rc1-sequence.md
    - .claude/rules/workflow/release-via-merge-request-rule.md
    - docs/audits/rc1-readiness-audit.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
    - tooling/rust/Cargo.toml
---

# Proceed: Step 5 — tag and publish RC-1

Governor decisions recorded (`bbtt`):

| Question | Decision |
|---|---|
| `agov` version alignment | **Independent** — Rust workspace stays at `0.x`; do NOT change `tooling/rust/Cargo.toml`. |
| RC version number | **`2.2.0-rc.1`** — MINOR bump from `2.1.6`. |
| Versioning scheme | **Post-bump only** — `agf/VERSION` on `main` carries `2.1.6` until the release MR merges. |

## Release workflow (per `release-via-merge-request-rule.md`)

1. Create `release/2.2.0-rc.1` from `main` (they are currently identical).
2. On the branch (not main), commit two changes and nothing else:
   - `agf/VERSION`: `2.1.6` → `2.2.0-rc.1`
   - `CHANGELOG.md`: add entry for `2.2.0-rc.1`
3. Open MR `release/2.2.0-rc.1` → `main`. MR diff must contain **only** the
   VERSION bump and CHANGELOG entry — making review trivial.
4. File Gate 2 handoff to governor-reviewer.

## CHANGELOG entry guidance

The `2.2.0-rc.1` entry should summarise the four steps completed in the RC-1
sequence:

- Readable coordination IDs as the default format (`agov coord brief`)
- Version provenance in `agov --version` and `agov version --format json`
- Three-layer contract docs for `agov coord graph-index`
- RC-1 readiness audit — operator path verified, v1.x material labelled

## What NOT to change

- `tooling/rust/Cargo.toml` workspace version — `agov` is independently
  versioned at `0.x`. Do not align it with the framework version.
- Any implementation files — this is a release-metadata-only MR.
- Any rules, schemas, or agent definitions.

## After governor merges the MR

Per `release-via-merge-request-rule.md` §Steps 7–8: governor tags the merge
commit as `v2.2.0-rc.1` and pushes the tag immediately. CI publishes the
release archives. No further confirmation needed — the merged MR is the
authorization.

## Token note

Codex token budget resets 2026-06-25. This is a small, bounded task:
two files changed, one MR opened. File Gate 2 handoff as soon as the
release branch is ready.
