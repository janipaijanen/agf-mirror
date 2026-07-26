---
kind: note
from: coder-tester
to: all
id: 20260726T225814Z-coder-tester-hmh4
timestamp: 2026-07-26T22:58:14Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T225151Z-coder-tester-8nds
topic: session-closeout-before-compact
requires-ack: false
artifacts_owned: []
---

# Session closeout before compact: gate consultation and Windows boundary

## Resume first

Run:

```bash
agov session resume --role coder-tester --base-branch main
```

## Human instruction controlling future work

Do **not** trigger, retry, play, or otherwise operate any Windows build,
preview, or packaging job until further notice. Authorization requires a new
external request from the human governor. In particular, leave both
`rust:tooling:preview:windows-package` and
`rust:tooling:build:windows:preview` untouched.

## Completed consultation

- MR !655: recommend accepting/merging the independent exact-head PASS first.
- MR !651: recommend human Gate 2 approval at exact head `d7d4da2`; pipeline
  `2707015076` passed, including the public-site build.
- MR !589: substantive blockers are cleared at exact head `b3c73bd3`; pipeline
  `2707035323` and independent re-audit passed. Recommend human Gate 2 after a
  description-only truth refresh; do not rebase or change the reviewed head.
- MR !613: both prior protocol blockers are cleared at exact head `34a1882f`;
  pipeline `2707037504` and independent re-audit passed. Recommend human Gate 1
  after a description-only truth refresh; no review execution before the
  binding decision is visible.
- MR !654: recommend Gate 1 BLOCK pending revision. Exact-tree inspection found
  44 Claude-side Markdown files, 47 AGF-side files, 42 shared files, and a
  49-file union. After normalizing provenance wrappers, 26 shared files are
  equivalent and 16 diverge, contradicting the plan's `44 shared / 39 trivial /
  8 divergent` account. The plan also omits a divergent development-workflow
  rule, understates adopter impact from canonical `agf/rules/` changes, proposes
  migrating foundry/legacy coordination language as-is, and lacks adequate
  consultation and Windows/Git symlink portability evidence.

No listed MR needs rebasing now: all were mergeable and newer `main` changes
were coordination-only with no affected-path overlap. MR !638, the stale
coord-scanner-resolution Gate 1 item, and the remaining backlog were not
changed or assessed beyond the previously recorded state.

## Public-positioning answer

AGF is a repository-local, file-based governance framework with CLI and MCP
tooling, not an SDK, agent runtime, orchestrator, or hosted service. Public
`evaluate.md` and the AI evaluation dossier already say it is not an SDK. A
small separate documentation follow-up after MR !651 may add the same identity
line to the home and assistant-loader pages without invalidating !651's reviewed
SHA.

## Do not assume

- Green CI or advisory review is not human Gate 1 or Gate 2 authority.
- A supported tool interface is not an SDK and does not acquire governance
  authority of its own.
- No merge, release branch, version bump, tag, or Windows job is authorized by
  this note.
